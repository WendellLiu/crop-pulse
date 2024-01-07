use crate::client::crop_transaction;
use crate::db::data::{crop_transactions, daily_crop_transactions};
use crate::db::pool;
use crate::logger;

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

pub async fn aggregate_daily_crop_transactions(date: &str) -> anyhow::Result<()> {
    logger::log(format!("run with date: {}", date));

    let pool = pool::POOL.get().await;

    let daily_crop_transaction_list =
        daily_crop_transactions::aggregate_daily_crop_transactions(pool, date).await?;
    let daily_crop_transaction = &daily_crop_transaction_list[0];

    println!("{:?}", daily_crop_transaction);

    Ok(())
}
