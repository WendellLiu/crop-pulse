use crate::client::crop_transaction;
use crate::db::data::{crop_transactions, daily_crop_transactions};
use crate::db::pool;
use crate::helpers::date;
use crate::logger;

static STEP: u16 = 1000;

pub async fn fetch_and_save_crop_transaction_history(
    start_date_str: date::RocDateString,
    end_date_str: date::RocDateString,
    tc_type: &str,
) -> anyhow::Result<()> {
    logger::log(format!(
        "run with start_date: {}, end_date: {}, tc_type: {}",
        start_date_str, end_date_str, tc_type
    ));

    let pool = pool::POOL.get().await;

    let start = 0;
    let step = STEP;

    let date_iterator = date::RocDateStringRage(start_date_str, end_date_str);

    for date in date_iterator {
        let iterator = std::iter::successors(Some(start), move |&n| Some(n + step));

        for skip in iterator {
            logger::log(format!(
                "run with date: {}, tc_type: {}, step: {}, skip: {}",
                date, tc_type, STEP, skip
            ));

            let response =
                crop_transaction::get_crop_transaction_history(STEP, skip, &date, &date, tc_type)
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

    let date_iterator = date::RocDateStringRage(start_date_str, end_date_str);

    let mut daily_crop_transaction_list = vec![];
    for date in date_iterator {
        println!("date: {}", date);
        let mut result =
            daily_crop_transactions::aggregate_daily_crop_transactions(pool, &date).await?;

        daily_crop_transaction_list.append(&mut result);
    }

    println!("size: {}", daily_crop_transaction_list.len());

    daily_crop_transactions::add_daily_crop_transactions(pool, daily_crop_transaction_list).await?;

    Ok(())
}
