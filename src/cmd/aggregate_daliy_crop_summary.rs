use chrono::{Duration, NaiveDate};
use futures::future;

use crate::db::data::{crops, crops_summary_14_days, daily_crop_transactions};
use crate::db::pool;
use crate::helpers::date;
use crate::logger;
use crate::statistic::daily_summary::calculate_serial_beta_coefficient;

struct SummaryMaterial {
    high_price: Vec<f64>,
    mid_price: Vec<f64>,
    low_price: Vec<f64>,
    average_price: Vec<f64>,
    trading_volume: Vec<f64>,
}

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
                        return;
                    }
                };

                if crop_data.len() == 0 {
                    return;
                }

                println!("{:?}", crop_data);

                // TODO: migrate the following code to statistic module
                let crop_code = &crop_data.first().unwrap().crop_code;
                let end_date = internal_end_date_str;

                let summary_material = SummaryMaterial {
                    high_price: crop_data.iter().map(|data| data.high_price).collect(),
                    mid_price: crop_data.iter().map(|data| data.mid_price).collect(),
                    low_price: crop_data.iter().map(|data| data.low_price).collect(),
                    average_price: crop_data.iter().map(|data| data.average_price).collect(),
                    trading_volume: crop_data.iter().map(|data| data.trading_volume).collect(),
                };

                let crops_summary_14_days_data = crops_summary_14_days::CropsSummary14Days {
                    end_date: end_date.to_string(),
                    crop_code: crop_code.to_string(),
                    high_price_beta_coefficient: calculate_serial_beta_coefficient(
                        &summary_material.high_price,
                    ),
                    mid_price_beta_coefficient: calculate_serial_beta_coefficient(
                        &summary_material.mid_price,
                    ),
                    low_price_beta_coefficient: calculate_serial_beta_coefficient(
                        &summary_material.low_price,
                    ),
                    average_price_beta_coefficient: calculate_serial_beta_coefficient(
                        &summary_material.average_price,
                    ),
                    trading_volume_beta_coefficient: calculate_serial_beta_coefficient(
                        &summary_material.trading_volume,
                    ),
                    trading_volume_sum: summary_material.trading_volume.iter().sum(),
                };

                println!("{:?}", crops_summary_14_days_data);
            })
        })
        .collect();

    future::join_all(tasks).await;

    Ok(())
}
