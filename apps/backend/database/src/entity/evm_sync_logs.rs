use sqlx::{Pool, Postgres, postgres::PgConnection, query, query_as};

#[derive(Debug, sqlx::FromRow)]
pub struct EvmSyncLogs {
    pub contract_address: String,
    pub last_synced_block_number: i64,
    pub chain_id: i64,
}

impl EvmSyncLogs {
    pub async fn find_or_create_by_address(
        address: &str,
        chain_id: u64,
        pool: &Pool<Postgres>,
    ) -> Result<Self, sqlx::Error> {
        let result = query_as!(
            EvmSyncLogs,
            "SELECT contract_address, last_synced_block_number, chain_id FROM evm_sync_logs WHERE contract_address = $1 AND chain_id = $2",
            address,
            chain_id as i64
        )
        .fetch_optional(pool)
        .await?;

        match result {
            Some(sync_log) => Ok(sync_log),
            None => {
                query_as!(
                    EvmSyncLogs,
                    "INSERT INTO evm_sync_logs (contract_address, chain_id) VALUES ($1, $2) RETURNING contract_address, last_synced_block_number, chain_id",
                    address,
                    chain_id as i64
                )
                .fetch_one(pool)
                .await
            }
        }
    }

    pub async fn update_last_synced_block_number(
        &self,
        block_number: u64,
        tx: &mut PgConnection,
    ) -> Result<(), sqlx::Error> {
        query!(
            "UPDATE evm_sync_logs SET last_synced_block_number = $1 WHERE contract_address = $2 AND chain_id = $3",
            block_number as i64,
            self.contract_address,
            self.chain_id
        )
        .execute(tx)
        .await?;
        Ok(())
    }

    pub async fn find_all_by_chain_id(
        chain_id: u64,
        pool: &Pool<Postgres>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        query_as!(
            EvmSyncLogs,
            "SELECT contract_address, last_synced_block_number, chain_id FROM evm_sync_logs WHERE chain_id = $1",
            chain_id as i64
        )
        .fetch_all(pool)
        .await
    }

    pub async fn create(
        contract_address: &str,
        chain_id: i64,
        pool: &Pool<Postgres>,
    ) -> Result<Self, sqlx::Error> {
        query_as!(
            EvmSyncLogs,
            "INSERT INTO evm_sync_logs (contract_address, chain_id) VALUES ($1, $2) RETURNING contract_address, last_synced_block_number, chain_id",
            contract_address,
            chain_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn find_all_addresses(pool: &Pool<Postgres>) -> Result<Vec<String>, sqlx::Error> {
        sqlx::query_scalar!("SELECT DISTINCT contract_address FROM evm_sync_logs")
            .fetch_all(pool)
            .await
    }
}
