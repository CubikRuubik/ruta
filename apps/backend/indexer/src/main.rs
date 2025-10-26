use std::{env, time::Duration};

use database::{
    entity::evm_chains::EvmChains, entity::evm_sync_logs::EvmSyncLogs, initialize_database,
};
use error::AppError;
use service::ListenerService;
use tokio::task::JoinSet;
use tower::{Service, ServiceBuilder, ServiceExt};

mod error;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_pool = initialize_database().await.unwrap();

    // Get all supported chains
    let chains = EvmChains::find_all(&db_pool).await?;
    if chains.is_empty() {
        eprintln!("No chains configured in database");
        return Ok(());
    }

    let mut service_futures = JoinSet::new();

    for chain in chains {
        let chain_id = chain.id as u64;
        let block_time = chain.block_time.unwrap_or(12) as u64;

        // Get all contract addresses for this chain
        let sync_logs = EvmSyncLogs::find_all_by_chain_id(chain_id, &db_pool).await?;

        for sync_log in sync_logs {
            let mut service = ServiceBuilder::new()
                .rate_limit(1, Duration::from_secs(block_time))
                .service(ListenerService {
                    chain_id,
                    address: sync_log.contract_address.clone(),
                    db_pool: db_pool.clone(),
                });

            let address = sync_log.contract_address.clone();
            let future = async move {
                loop {
                    if service.ready().await.is_ok() {
                        match service.call(()).await {
                            Ok(()) => {}
                            Err(err) => {
                                eprintln!("Failed to index {}: {:?}", address, err);
                                // Sleep on error to avoid rapid retry loops
                                tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
                            }
                        }
                    }
                }
            };

            service_futures.spawn(future);
        }
    }

    service_futures.join_all().await;

    Ok(())
}
