mod client;
mod cmd;
mod db;
mod logger;

use dotenvy::dotenv;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenv().expect(".env file not found");

    let tc_types = vec!["N04", "N05"];

    for tc_type in tc_types {
        cmd::fetch_and_save_crop_transaction_history("113.01.01", "113.01.02", tc_type).await?;
    }

    Ok(())
}
