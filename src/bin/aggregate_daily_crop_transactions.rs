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
    date: Option<String>,
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

    let date = args.date.unwrap_or(default_date);

    cmd::aggregate_daily_crop_transactions(&date).await?;

    Ok(())
}
