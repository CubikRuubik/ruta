use std::{
    env,
    error::Error,
    future::Future,
    pin::Pin,
    str::FromStr,
    task::{Context, Poll},
};

use alloy::{
    eips::BlockNumberOrTag,
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    rpc::types::Filter,
};
use database::entity::{
    erc20_transfers::Erc20Transfers, evm_chains::EvmChains, evm_sync_logs::EvmSyncLogs,
};
use sqlx::{Pool, Postgres};
use tokio::time::{sleep, Duration};
use tower::Service;

use crate::{erc20::Erc20Transfer, error::AppError};

pub struct ListenerService {
    pub chain_id: u64,
    pub address: String,
    pub db_pool: Pool<Postgres>,
}

impl Service<()> for ListenerService {
    type Response = ();
    type Error = Box<dyn Error>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: ()) -> Self::Future {
        let db_pool = self.db_pool.clone();
        let chain_id = self.chain_id;
        let address = self.address.clone();

        Box::pin(async move { fetch_and_save_logs(chain_id, db_pool, address).await })
    }
}

pub async fn fetch_and_save_logs(
    chain_id: u64,
    db_pool: Pool<Postgres>,
    address: String,
) -> Result<(), Box<dyn Error>> {
    loop {
        // Get RPC URL from database for the specified chain
        let chain = EvmChains::fetch_by_id(chain_id, &db_pool).await?;
        let rpc_url = chain
            .rpc_url
            .ok_or_else(|| AppError::MissingEnvVar("RPC_URL for chain".into()))?;

        let provider = ProviderBuilder::new().on_builtin(&rpc_url).await?;
        let sync_log = EvmSyncLogs::find_or_create_by_address(&address, chain_id, &db_pool).await?;

        let latest_block = provider.get_block_number().await?;
        if latest_block == sync_log.last_synced_block_number as u64 {
            println!("Fully indexed address: {address}, sleeping for 60 seconds");
            sleep(Duration::from_secs(60)).await;
            continue;
        }

        let from_block_number = match sync_log.last_synced_block_number as u64 {
            0 => latest_block - 9, // TODO: consider other value
            block_number => block_number + 1_u64,
        };

        let to_block_number = match sync_log.last_synced_block_number as u64 {
            0 => latest_block,
            block_number => std::cmp::min(block_number + 10_u64, latest_block), // get the smallest value
        };
        println!(
            "Indexing address: {address}, from {} block to {}",
            from_block_number, to_block_number
        );
        let filter = Filter::new()
            .address(Address::from_str(&address)?)
            .from_block(BlockNumberOrTag::Number(from_block_number))
            .to_block(BlockNumberOrTag::Number(to_block_number));

        let logs = provider.get_logs(&filter).await?;

        let mut tx = db_pool.begin().await?;
        let contract_address = Address::from_str(&address)?;

        for log in logs {
            // Try to parse as ERC-20 Transfer event
            if let Some(transfer) = Erc20Transfer::from_log(&log) {
                let block_number = log.block_number.ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "Missing block number")
                })?;

                let transaction_hash = log.transaction_hash.ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "Missing transaction hash")
                })?;

                let _ = Erc20Transfers::create(
                    block_number,
                    transaction_hash.to_vec().try_into().map_err(|_| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Invalid transaction hash",
                        )
                    })?,
                    transfer.from.to_vec(),
                    transfer.to.to_vec(),
                    transfer.amount,
                    contract_address,
                    &mut tx,
                )
                .await
                .inspect_err(|error| eprintln!("Error saving ERC-20 transfer {error}"));
            }
        }

        let _ = sync_log
            .update_last_synced_block_number(to_block_number, &mut *tx)
            .await
            .inspect_err(|error| eprintln!("Error updating last_synced_block_number {error}"));

        match tx.commit().await {
            Ok(_) => {
                println!(
                    "Saved logs for {address}, blocks: {from_block_number} to {to_block_number}",
                )
            }
            Err(err) => eprintln!("{err}"),
        }

        // Sleep for a short time before checking again
        sleep(Duration::from_secs(10)).await;
    }
}
