#[path = "../client/mod.rs"]
mod client;
#[path = "../cmd/mod.rs"]
mod cmd;
#[path = "../db/mod.rs"]
mod db;
#[path = "../logger/mod.rs"]
mod logger;

use chrono::{Datelike, Duration, Utc};
use clap::Parser;
use dotenvy::dotenv;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    start: Option<String>,

    #[arg(short, long)]
    end: Option<String>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenv().expect(".env file not found");

    let args = Args::parse();

    let taiwan_offset = chrono::FixedOffset::east_opt(8 * 3600).unwrap();
    let current = Utc::now().with_timezone(&taiwan_offset);
    let yesterday = current - Duration::days(1);

    // for ROC calendar
    let year = yesterday.year() - 1911;
    let month = format!("{:02}", yesterday.month());
    let day = format!("{:02}", yesterday.day());
    let default_date = format!("{}.{}.{}", year, month, day);

    let start_date = args.start.unwrap_or(default_date.clone());
    let end_date = args.end.unwrap_or(default_date);

    let tc_types = vec!["N04", "N05"];

    for tc_type in tc_types {
        cmd::fetch_and_save_crop_transaction_history(&start_date, &end_date, tc_type).await?;
    }

    Ok(())
}
