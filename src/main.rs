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

    let response =
        crop_transaction::get_crop_transaction_history("113.01.01", "113.01.02", "N05").await?;
    let item = &response[0];
    logger::log(item);

    let transaction_id = crop_transactions::add_transaction(pool, item).await?;
    logger::log(format!("Added new transaction with id {transaction_id}"));

    Ok(())
}
