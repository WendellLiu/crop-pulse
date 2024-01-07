use serde::Deserialize;
use sqlx::sqlite::SqlitePool;

#[derive(Deserialize, Debug)]
pub struct AggregateDailyCropData {
    pub transaction_date: Option<String>,
    pub type_code: Option<String>,
    pub crop_code: Option<String>,
    pub crop_name: Option<String>,
    pub high_price: Option<f64>,
    pub mid_price: Option<f64>,
    pub low_price: Option<f64>,
    pub average_price: Option<f64>,
    pub trading_volume: Option<f64>,
}

pub async fn aggregate_daily_crop_transactions(
    pool: &SqlitePool,
    transaction_date: &str,
) -> anyhow::Result<Vec<AggregateDailyCropData>> {
    let daily_crop_transactions = sqlx::query_as!(
        AggregateDailyCropData,
        "
        SELECT
            transaction_date,
            crop_code,
            crop_name,
            type_code,
            AVG(high_price) AS high_price,
            AVG(mid_price) AS mid_price,
            AVG(low_price) AS low_price,
            AVG(average_price) AS average_price,
            SUM(trading_volume) AS trading_volume
        FROM
            crop_transactions
        WHERE
            transaction_date = ?
        GROUP BY
            transaction_date,
            crop_code,
            crop_name,
            type_code
        ",
        transaction_date
    )
    .fetch_all(pool)
    .await?;

    Ok(daily_crop_transactions)
}
