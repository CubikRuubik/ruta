use sqlx::{Pool, Postgres, query_as};

#[derive(Debug, sqlx::FromRow)]
pub struct EvmChains {
    pub id: i64,
    pub name: String,
    pub rpc_url: Option<String>,
    pub block_time: Option<i32>,
}

impl EvmChains {
    pub async fn fetch_by_id(id: u64, pool: &Pool<Postgres>) -> Result<Self, sqlx::Error> {
        query_as!(
            EvmChains,
            "SELECT id, name, rpc_url, block_time FROM evm_chains WHERE id = $1",
            id as i64
        )
        .fetch_one(pool)
        .await
    }

    pub async fn find_all(pool: &Pool<Postgres>) -> Result<Vec<Self>, sqlx::Error> {
        query_as!(
            EvmChains,
            "SELECT id, name, rpc_url, block_time FROM evm_chains"
        )
        .fetch_all(pool)
        .await
    }

    pub async fn create(
        id: i64,
        name: &str,
        rpc_url: Option<&str>,
        block_time: Option<i32>,
        pool: &Pool<Postgres>,
    ) -> Result<Self, sqlx::Error> {
        query_as!(
            EvmChains,
            "INSERT INTO evm_chains (id, name, rpc_url, block_time) VALUES ($1, $2, $3, $4) RETURNING id, name, rpc_url, block_time",
            id,
            name,
            rpc_url,
            block_time
        )
        .fetch_one(pool)
        .await
    }

    pub async fn upsert(
        id: i64,
        name: &str,
        rpc_url: Option<&str>,
        block_time: Option<i32>,
        pool: &Pool<Postgres>,
    ) -> Result<Self, sqlx::Error> {
        query_as!(
            EvmChains,
            "INSERT INTO evm_chains (id, name, rpc_url, block_time) VALUES ($1, $2, $3, $4)
             ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name, rpc_url = EXCLUDED.rpc_url, block_time = EXCLUDED.block_time
             RETURNING id, name, rpc_url, block_time",
            id,
            name,
            rpc_url,
            block_time
        )
        .fetch_one(pool)
        .await
    }
}
