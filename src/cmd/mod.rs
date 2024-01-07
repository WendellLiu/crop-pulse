use crate::client::crop_transaction;
use crate::db::data::{crop_transactions, daily_crop_transactions};
use crate::db::pool;
use crate::logger;

use chrono::{Datelike, Duration, NaiveDate};

static STEP: u16 = 1000;

pub async fn fetch_and_save_crop_transaction_history(
    start_date: &str,
    end_date: &str,
    tc_type: &str,
) -> anyhow::Result<()> {
    logger::log(format!(
        "run with start_date: {}, end_date: {}, tc_type: {}",
        start_date, end_date, tc_type
    ));

    let pool = pool::POOL.get().await;

    let start = 0;
    let step = STEP;

    let iterator = std::iter::successors(Some(start), move |&n| Some(n + step));

    for skip in iterator {
        logger::log(format!("run with step: {}, skip: {}", STEP, skip));

        let response = crop_transaction::get_crop_transaction_history(
            STEP, skip, start_date, end_date, tc_type,
        )
        .await?;

        let response_size = response.len();

        logger::log(format!("size: {}", response_size));

        if response_size == 0 {
            logger::log("no available data");
            break;
        }

        let msg = crop_transactions::add_crop_transactions(pool, response).await?;
        logger::log(format!("Added new transaction with message {}", msg));
    }

    Ok(())
}

pub async fn aggregate_daily_crop_transactions(
    start_date_str: &str,
    end_date_str: &str,
) -> anyhow::Result<()> {
    logger::log(format!(
        "run with start_date: {}, end_date: {}",
        start_date_str, end_date_str
    ));

    let pool = pool::POOL.get().await;

    let (start_year, start_month, start_day) = parse_roc_date(start_date_str);
    let (end_year, end_month, end_day) = parse_roc_date(end_date_str);

    let start_date = NaiveDate::from_ymd_opt(start_year + 1911, start_month, start_day)
        .expect("invalid start date");
    let end_date =
        NaiveDate::from_ymd_opt(end_year + 1911, end_month, end_day).expect("invalid end date");

    let date_list: Vec<String> = (0..=(end_date - start_date).num_days())
        .map(|offset| start_date + Duration::days(offset))
        .map(|date| {
            let year = date.year();
            let month = date.month();
            let day = date.day();

            format!("{}.{:02}.{:02}", year - 1911, month, day)
        })
        .collect();

    let mut daily_crop_transaction_list = vec![];
    for date in date_list {
        println!("date: {}", date);
        let mut result =
            daily_crop_transactions::aggregate_daily_crop_transactions(pool, &date).await?;

        daily_crop_transaction_list.append(&mut result);
    }

    println!("size: {}", daily_crop_transaction_list.len());

    daily_crop_transactions::add_daily_crop_transactions(pool, daily_crop_transaction_list).await?;

    Ok(())
}

fn parse_roc_date(date_str: &str) -> (i32, u32, u32) {
    let parts: Vec<&str> = date_str.split('.').collect();
    let year = parts[0].parse::<i32>().unwrap();
    let month = parts[1].parse::<u32>().unwrap();
    let day = parts[2].parse::<u32>().unwrap();
    (year, month, day)
}
