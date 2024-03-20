use futures::future;

use crate::db::data::{crops, daily_crop_transactions};
use crate::db::pool;
use crate::helpers::date;
use crate::logger;
use chrono::{Duration, NaiveDate};

pub async fn main(end_date_str: date::RocDateString) -> anyhow::Result<()> {
    logger::log(format!("run with end_date: {}", end_date_str));

    let pool = pool::POOL.get().await;

    let start_date = Option::<NaiveDate>::from(end_date_str.clone()).expect("invalid end date")
        - Duration::days(13);
    let start_date_str: date::RocDateString = start_date.into();

    let crop_list = crops::fetch_all_crops(pool).await?;

    let tasks: Vec<_> = crop_list
        .into_iter()
        .take(1) // TODO: only for debug
        .map(|crop_data| {
            let internal_start_date_str = start_date_str.clone();
            let internal_end_date_str = end_date_str.clone();

            tokio::spawn(async move {
                let result = daily_crop_transactions::fetch_all_daily_crop_transactions(
                    pool,
                    &internal_start_date_str,
                    &internal_end_date_str,
                    &crop_data.crop_code.to_string(),
                )
                .await;

                let crop_data = match result {
                    Ok(d) => d,
                    Err(e) => {
                        logger::error(format!("fetch daily crop transaction data: {:?}", e));
                        vec![]
                    }
                };

                println!("{:?}", crop_data);
            })
        })
        .collect();

    future::join_all(tasks).await;

    Ok(())
}
