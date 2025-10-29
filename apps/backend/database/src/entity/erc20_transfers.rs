use alloy::primitives::Address;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, postgres::PgConnection, query, query_as, types::chrono};

#[derive(Debug, sqlx::FromRow)]
pub struct Erc20Transfers {
    pub id: i64,
    pub block_number: i64,
    pub transaction_hash: Vec<u8>,
    pub log_index: i32,
    pub from_address: Vec<u8>,
    pub to_address: Vec<u8>,
    pub amount: BigDecimal, // DECIMAL(78,0)
    pub contract_address: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Erc20Transfers {
    pub async fn create(
        block_number: u64,
        transaction_hash: [u8; 32],
        from_address: Vec<u8>,
        to_address: Vec<u8>,
        amount: alloy::primitives::U256,
        contract_address: Address,
        tx: &mut PgConnection,
    ) -> Result<(), sqlx::Error> {
        use std::str::FromStr;

        let amount_decimal = BigDecimal::from_str(&amount.to_string())
            .map_err(|_| sqlx::Error::Decode("Invalid amount".into()))?;

        sqlx::query(
            "INSERT INTO token_transfers (block_number, transaction_hash, log_index, from_address, to_address, amount, contract_address) VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT (transaction_hash, log_index) DO NOTHING"
        )
        .bind(block_number as i64)
        .bind(&transaction_hash[..])
        .bind(0i32) // log_index, assuming single log for simplicity
        .bind(&from_address)
        .bind(&to_address)
        .bind(amount_decimal)
        .bind(contract_address.to_string())
        .execute(tx)
        .await?;
        Ok(())
    }

    pub async fn find_all(limit: i64, pool: &Pool<Postgres>) -> Result<Vec<Self>, sqlx::Error> {
        query_as!(
            Erc20Transfers,
            "SELECT id, block_number, transaction_hash, log_index, from_address, to_address, amount, contract_address, created_at FROM token_transfers ORDER BY id DESC LIMIT $1",
            limit
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_contract_address(
        contract_address: &str,
        limit: i64,
        pool: &Pool<Postgres>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        query_as!(
            Erc20Transfers,
            "SELECT id, block_number, transaction_hash, log_index, from_address, to_address, amount, contract_address, created_at FROM token_transfers WHERE contract_address = $1 ORDER BY block_number DESC LIMIT $2",
            contract_address,
            limit
        )
        .fetch_all(pool)
        .await
    }

    pub async fn sum_amounts_by_contract_address(
        contract_address: &str,
        pool: &Pool<Postgres>,
    ) -> Result<BigDecimal, sqlx::Error> {
        let result: Option<BigDecimal> = sqlx::query_scalar!(
            "SELECT COALESCE(SUM(amount), 0) FROM token_transfers WHERE contract_address = $1",
            contract_address
        )
        .fetch_one(pool)
        .await?;

        Ok(result.unwrap_or_else(|| BigDecimal::from(0)))
    }
}
