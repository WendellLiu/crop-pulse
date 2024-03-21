use chrono::{Duration, NaiveDate};
use futures::future;
use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};

use crate::db::data::{crops, daily_crop_transactions};
use crate::db::pool;
use crate::helpers::date;
use crate::logger;
use crate::statistic::basic::normalize;

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

                let x = (1..crop_data.len() + 1).map(|x| x as f64).collect();

                let summary_material = SummaryMaterial {
                    high_price: crop_data.iter().map(|data| data.high_price).collect(),
                    mid_price: crop_data.iter().map(|data| data.mid_price).collect(),
                    low_price: crop_data.iter().map(|data| data.low_price).collect(),
                    average_price: crop_data.iter().map(|data| data.average_price).collect(),
                    trading_volume: crop_data.iter().map(|data| data.trading_volume).collect(),
                };

                let y = normalize(&summary_material.average_price);
                println!("average_price y: {:?}", y);

                let data = vec![("Y", y), ("X", x)];
                let data = RegressionDataBuilder::new().build_from(data).unwrap();
                let formula = "Y ~ X";
                let model = FormulaRegressionBuilder::new()
                    .data(&data)
                    .formula(formula)
                    .fit()
                    .unwrap();
                let parameters: Vec<_> = model.iter_parameter_pairs().collect();

                println!("{:?}", parameters);
            })
        })
        .collect();

    future::join_all(tasks).await;

    Ok(())
}
