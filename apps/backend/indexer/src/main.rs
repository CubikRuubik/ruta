use std::{env, time::Duration};

use dotenvy::dotenv;

use axum::serve;
use database::{
    entity::evm_chains::EvmChains, entity::evm_sync_logs::EvmSyncLogs, initialize_database,
};
use error::AppError;
use service::ListenerService;
use tokio::sync::broadcast;
use tokio::task::JoinSet;
use tower::{Service, ServiceBuilder, ServiceExt};

mod erc20;
mod error;
mod server;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();

    let db_pool = initialize_database().await.unwrap();

    let (transfer_tx, _) = broadcast::channel::<erc20::Erc20Transfer>(100);

    let app_state = server::AppState {
        db_pool: db_pool.clone(),
        transfer_tx: transfer_tx.clone(),
    };
    let app = server::create_router(app_state);

    let server_handle = tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        serve(listener, app).await.unwrap();
    });

    let chains = EvmChains::find_all(&db_pool).await?;
    if chains.is_empty() {
        eprintln!("No chains configured in database");
        return Ok(());
    }

    let contract_addresses = env::var("CONTRACT_ADDRESSES").unwrap_or_default();
    for chain in &chains {
        for addr in contract_addresses.split(',') {
            let addr = addr.trim();
            if !addr.is_empty() {
                EvmSyncLogs::find_or_create_by_address(addr, chain.id as u64, &db_pool).await?;
            }
        }
    }

    let mut service_futures = JoinSet::new();

    for chain in chains {
        let chain_id = chain.id as u64;
        let block_time = chain.block_time.unwrap_or(12) as u64;

        let sync_logs = EvmSyncLogs::find_all_by_chain_id(chain_id, &db_pool).await?;

        for sync_log in sync_logs {
            let mut service = ServiceBuilder::new()
                .rate_limit(1, Duration::from_secs(block_time))
                .service(ListenerService {
                    chain_id,
                    address: sync_log.contract_address.clone(),
                    db_pool: db_pool.clone(),
                    transfer_tx: transfer_tx.clone(),
                });

            let address = sync_log.contract_address.clone();
            let future = async move {
                loop {
                    if service.ready().await.is_ok() {
                        match service.call(()).await {
                            Ok(()) => {}
                            Err(err) => {
                                eprintln!("Failed to index {}: {:?}", address, err);
                                tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
                            }
                        }
                    }
                }
            };

            service_futures.spawn(future);
        }
    }

    tokio::select! {
        _ = server_handle => {},
        _ = service_futures.join_all() => {},
    }

    Ok(())
}
