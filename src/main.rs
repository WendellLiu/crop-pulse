mod db;
mod logger;

use dotenvy::dotenv;

use crate::db::data::crop_transactions;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenv().expect(".env file not found");
    logger::log("hello world");

    let pool = db::pool::POOL.get().await;

    let transaction_id = crop_transactions::add_transaction(
        &pool,
        "Wheat".to_string(),
        100,
        5.99,
        "2024-01-05".to_string(),
    )
    .await?;
    println!("Added new transaction with id {transaction_id}");

    Ok(())
}
