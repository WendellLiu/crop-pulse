mod client;
mod db;
mod logger;

use dotenvy::dotenv;

use crate::client::crop_transaction;
use crate::db::data::crop_transactions;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenv().expect(".env file not found");

    let pool = db::pool::POOL.get().await;

    let transaction_id = crop_transactions::add_transaction(
        &pool,
        "Wheat".to_string(),
        100,
        5.99,
        "2024-01-05".to_string(),
    )
    .await?;

    logger::log(format!("Added new transaction with id {transaction_id}"));

    let response = crop_transaction::get_crop_transaction_history().await?;
    logger::log(&response[0]);

    Ok(())
}
