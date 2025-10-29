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
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::{request::TransactionRequest, Filter},
    sol_types::SolCall,
};
use database::entity::{
    erc20_transfers::Erc20Transfers, evm_chains::EvmChains, evm_sync_logs::EvmSyncLogs,
};
use sqlx::{Pool, Postgres};
use tokio::sync::broadcast;
use tokio::time::{sleep, Duration};
use tower::Service;

use crate::erc20::Erc20Transfer;
use crate::error::AppError;

pub struct ListenerService {
    pub chain_id: u64,
    pub address: String,
    pub db_pool: Pool<Postgres>,
    pub transfer_tx: broadcast::Sender<Erc20Transfer>,
}

impl Service<()> for ListenerService {
    type Response = ();
    type Error = Box<dyn Error + Send + Sync>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: ()) -> Self::Future {
        let db_pool = self.db_pool.clone();
        let chain_id = self.chain_id;
        let address = self.address.clone();
        let transfer_tx = self.transfer_tx.clone();

        Box::pin(async move { fetch_and_save_logs(chain_id, db_pool, address, transfer_tx).await })
    }
}

alloy::sol! {
    #[sol(rpc)]
    interface IERC20 {
        function symbol() external view returns (string);
        function decimals() external view returns (uint8);
    }
}

pub async fn get_token_symbol(
    chain_id: u64,
    contract_address: &str,
    db_pool: &Pool<Postgres>,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let chain = EvmChains::fetch_by_id(chain_id, db_pool).await?;
    let rpc_url = chain
        .rpc_url
        .ok_or_else(|| AppError::MissingEnvVar("RPC_URL for chain".into()))?;

    let provider = ProviderBuilder::new().on_builtin(&rpc_url).await?;
    let contract = Address::from_str(contract_address)?;

    let symbol_call = IERC20::symbolCall {};
    let tx = TransactionRequest::default()
        .to(contract)
        .input(symbol_call.abi_encode().into());

    let result = provider.call(&tx).await?;
    let decoded = IERC20::symbolCall::abi_decode_returns(&result, true)?;
    Ok(decoded._0)
}

pub async fn get_token_decimals(
    chain_id: u64,
    contract_address: &str,
    db_pool: &Pool<Postgres>,
) -> Result<u8, Box<dyn Error + Send + Sync>> {
    let chain = EvmChains::fetch_by_id(chain_id, db_pool).await?;
    let rpc_url = chain
        .rpc_url
        .ok_or_else(|| AppError::MissingEnvVar("RPC_URL for chain".into()))?;

    let provider = ProviderBuilder::new().on_builtin(&rpc_url).await?;
    let contract = Address::from_str(contract_address)?;

    let decimals_call = IERC20::decimalsCall {};
    let tx = TransactionRequest::default()
        .to(contract)
        .input(decimals_call.abi_encode().into());

    let result = provider.call(&tx).await?;
    let decoded = IERC20::decimalsCall::abi_decode_returns(&result, true)?;
    Ok(decoded._0)
}

pub async fn fetch_and_save_logs(
    chain_id: u64,
    db_pool: Pool<Postgres>,
    address: String,
    transfer_tx: broadcast::Sender<Erc20Transfer>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    loop {
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
            0 => latest_block - 9,
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

        let decimals = get_token_decimals(chain_id, &address, &db_pool)
            .await
            .unwrap_or(18);

        for log in logs {
            if let Some(transfer) = Erc20Transfer::from_log(&log) {
                let block_number = log.block_number.ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "Missing block number")
                })?;

                let transaction_hash = log.transaction_hash.ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "Missing transaction hash")
                })?;

                let adjusted_amount = transfer.amount / U256::from(10u64.pow(decimals as u32)); // U256::from(6);

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
                    adjusted_amount,
                    contract_address,
                    &mut tx,
                )
                .await
                .inspect_err(|error| eprintln!("Error saving ERC-20 transfer {error}"));

                let _ = transfer_tx.send(transfer.clone());
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

        sleep(Duration::from_secs(10)).await;
    }
}
