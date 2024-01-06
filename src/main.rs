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
        crop_transaction::get_crop_transaction_history(1000, 0, "113.01.01", "113.01.02", "N05")
            .await?;

    let msg = crop_transactions::add_crop_transactions(pool, response).await?;
    logger::log(format!("Added new transaction with message {}", msg));

    Ok(())
}
