use futures::future;

use crate::client::crop_transaction;
use crate::db::data::{crop_transactions, crops, daily_crop_transactions};
use crate::db::pool;
use crate::helpers::date;
use crate::logger;
use chrono::{Duration, NaiveDate};

static STEP: u16 = 1000;

pub async fn fetch_and_save_crop_transaction_history(
    start_date_str: date::RocDateString,
    end_date_str: date::RocDateString,
) -> anyhow::Result<()> {
    logger::log(format!(
        "run with start_date: {}, end_date: {}",
        start_date_str, end_date_str,
    ));

    let pool = pool::POOL.get().await;

    let start = 0;
    let step = STEP;

    let date_iterator = date::RocDateStringRange(start_date_str, end_date_str);

    for date in date_iterator {
        let iterator = std::iter::successors(Some(start), move |&n| Some(n + step));

        for skip in iterator {
            logger::log(format!(
                "run with date: {}, step: {}, skip: {}",
                date, STEP, skip
            ));

            let response =
                crop_transaction::get_crop_transaction_history(STEP, skip, &date, &date).await?;

            let response_size = response.len();

            logger::log(format!("size: {}", response_size));

            if response_size == 0 {
                logger::log("no available data");
                break;
            }

            let msg = crop_transactions::add_crop_transactions(pool, response).await?;
            logger::log(format!("Added new transaction with message {}", msg));
        }
    }

    Ok(())
}

pub async fn aggregate_daily_crop_transactions(
    start_date_str: date::RocDateString,
    end_date_str: date::RocDateString,
) -> anyhow::Result<()> {
    logger::log(format!(
        "run with start_date: {}, end_date: {}",
        start_date_str, end_date_str
    ));

    let pool = pool::POOL.get().await;

    let date_iterator = date::RocDateStringRange(start_date_str, end_date_str);

    let mut daily_crop_transaction_list = vec![];
    for date in date_iterator {
        println!("date: {}", date);
        let mut result =
            daily_crop_transactions::aggregate_daily_crop_transactions(pool, &date).await?;

        daily_crop_transaction_list.append(&mut result);

        // append crops data
        let crop_data = crops::aggregate_crops(pool).await?;
        crops::add_crops(pool, &crop_data).await?;

        let data_size = crop_data.len();
        logger::log(format!("Upsert {} crops", data_size));
    }

    println!(
        "total daily crop transaction size: {}",
        daily_crop_transaction_list.len()
    );

    daily_crop_transactions::add_daily_crop_transactions(pool, daily_crop_transaction_list).await?;

    Ok(())
}

pub async fn aggregate_daliy_crop_summary(end_date_str: date::RocDateString) -> anyhow::Result<()> {
    logger::log(format!("run with end_date: {}", end_date_str));

    let pool = pool::POOL.get().await;

    let start_date = Option::<NaiveDate>::from(end_date_str.clone()).expect("invalid end date")
        - Duration::days(13);
    let start_date_str: date::RocDateString = start_date.into();

    let crop_list = crops::fetch_all_crops(pool).await?;

    let tasks: Vec<_> = crop_list
        .into_iter()
        .map(|crop_data| {
            let internal_start_date_str = start_date_str.clone();
            let internal_end_date_str = end_date_str.clone();

            tokio::spawn(async move {
                let data = daily_crop_transactions::fetch_all_daily_crop_transactions(
                    pool,
                    &internal_start_date_str,
                    &internal_end_date_str,
                    &crop_data.crop_code.to_string(),
                )
                .await;

                logger::log(format!("{:?}", data));
            })
        })
        .collect();

    future::join_all(tasks).await;

    Ok(())
}
