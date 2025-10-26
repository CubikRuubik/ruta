use std::env;

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub mod entity;

mod defaults {
    pub const DATABASE_MAX_CONNECTIONS: &str = "5";
}

async fn create_pool(max_connections: u32) -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");

    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&database_url)
        .await
}

pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let db_max_connections = env::var("DATABASE_MAX_CONNECTIONS")
            .unwrap_or(String::from(defaults::DATABASE_MAX_CONNECTIONS))
            .parse::<u32>()
            .unwrap();

        let pool = create_pool(db_max_connections).await.unwrap();

        // Seed the database with initial data
        seed_database(&pool).await?;

        Ok(Database { pool })
    }

    pub fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}

pub async fn initialize_database() -> Result<Pool<Postgres>, sqlx::Error> {
    let db_max_connections = env::var("DATABASE_MAX_CONNECTIONS")
        .unwrap_or(String::from(defaults::DATABASE_MAX_CONNECTIONS))
        .parse::<u32>()
        .unwrap();

    let pool = create_pool(db_max_connections).await.unwrap();

    Ok(pool)
}

async fn seed_database(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    use entity::evm_chains::EvmChains;

    let chains = vec![(
        1i64,
        "Ethereum Mainnet",
        Some("https://eth-mainnet.g.alchemy.com/v2/qqcsONEuBfdAuvm-OclpT"),
        Some(12),
    )];

    for (id, name, rpc_url, block_time) in chains {
        EvmChains::upsert(id, name, rpc_url, block_time, pool).await?;
    }

    Ok(())
}

// psql -U ppp -d indexer_db
// SELECT * FROM evm_chains;
// SELECT * FROM token_transfers;
// SELECT * FROM evm_sync_logs;
// psql -U ppp -d indexer_db -c "TRUNCATE TABLE evm_chains, evm_sync_logs, token_transfers RESTART IDENTITY CASCADE;"
// export DATABASE_URL="postgresql://ppp@localhost/indexer_db"
