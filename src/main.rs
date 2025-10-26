use database::Database;
use indexer::service::fetch_and_save_logs;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database
    let db = Database::new().await?;
    println!("Database initialized and migrations run");

    let chain_id = 1;
    let address = env::var("CONTRACT_ADDRESSES")
        .unwrap_or_else(|_| "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string());

    println!("Starting ERC-20 indexer test for contract: {}", address);

    match fetch_and_save_logs(chain_id, db.pool().clone(), address.clone()).await {
        Ok(_) => println!(
            "Successfully indexed ERC-20 transfers for contract: {}",
            address
        ),
        Err(e) => println!("Error indexing contract {}: {:?}", address, e),
    }

    Ok(())
}
