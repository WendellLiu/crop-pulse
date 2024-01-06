mod client;
mod cmd;
mod db;
mod logger;

use clap::Parser;
use dotenvy::dotenv;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("113.01.01"))]
    start: String,

    #[arg(short, long, default_value_t = String::from("113.01.02"))]
    end: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenv().expect(".env file not found");

    let args = Args::parse();

    let tc_types = vec!["N04", "N05"];

    for tc_type in tc_types {
        cmd::fetch_and_save_crop_transaction_history(&args.start, &args.end, tc_type).await?;
    }

    Ok(())
}
