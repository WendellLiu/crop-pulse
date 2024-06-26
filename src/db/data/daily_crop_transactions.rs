use serde::Deserialize;
use sqlx::sqlite::SqlitePool;
use sqlx::{QueryBuilder, Sqlite};
use thiserror::Error;

use crate::helpers::date;

#[derive(Deserialize, Debug, sqlx::FromRow)]
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

impl From<AggregateDailyCropData> for DailyCropData {
    fn from(aggregate_data: AggregateDailyCropData) -> Self {
        DailyCropData {
            transaction_date: aggregate_data
                .transaction_date
                .expect("Missing transaction date"),
            type_code: aggregate_data.type_code.expect("Missing type code"),
            crop_code: aggregate_data.crop_code.expect("Missing crop code"),
            crop_name: aggregate_data.crop_name.expect("Missing crop name"),
            high_price: aggregate_data.high_price.expect("Missing high price"),
            mid_price: aggregate_data.mid_price.expect("Missing mid price"),
            low_price: aggregate_data.low_price.expect("Missing low price"),
            average_price: aggregate_data.average_price.expect("Missing average price"),
            trading_volume: aggregate_data
                .trading_volume
                .expect("Missing trading volume"),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct DailyCropData {
    pub transaction_date: String,
    pub type_code: String,
    pub crop_code: String,
    pub crop_name: String,
    pub high_price: f64,
    pub mid_price: f64,
    pub low_price: f64,
    pub average_price: f64,
    pub trading_volume: f64,
}

pub async fn aggregate_daily_crop_transactions(
    pool: &SqlitePool,
    transaction_date: &date::RocDateString,
) -> anyhow::Result<Vec<DailyCropData>> {
    let daily_crop_transactions: Vec<AggregateDailyCropData> = sqlx::query_as!(
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
        transaction_date.0
    )
    .fetch_all(pool)
    .await?;

    Ok(daily_crop_transactions
        .into_iter()
        .map(DailyCropData::from)
        .collect())
}

fn build_insert_daily_crop_transactions_query(
    daily_crop_transaction_list: Vec<DailyCropData>,
) -> QueryBuilder<'static, Sqlite> {
    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        r#"
REPLACE INTO daily_crop_transactions 
(id, transaction_date, type_code, crop_code, crop_name, high_price, mid_price, low_price, average_price, trading_volume) "#,
    );

    query_builder.push_values(
        daily_crop_transaction_list,
        |mut builder, dc_transaction| {
            let id_to_save = generate_id(&dc_transaction);

            builder
                .push_bind(id_to_save)
                .push_bind(dc_transaction.transaction_date)
                .push_bind(dc_transaction.type_code)
                .push_bind(dc_transaction.crop_code)
                .push_bind(dc_transaction.crop_name)
                .push_bind(dc_transaction.high_price)
                .push_bind(dc_transaction.mid_price)
                .push_bind(dc_transaction.low_price)
                .push_bind(dc_transaction.average_price)
                .push_bind(dc_transaction.trading_volume);
        },
    );

    query_builder
}

#[derive(Error, Debug)]
pub enum DbError {
    #[error("input error: `{0}`")]
    InputError(String),
}

pub async fn add_daily_crop_transactions(
    pool: &SqlitePool,
    payload_list: Vec<DailyCropData>,
) -> anyhow::Result<String> {
    if payload_list.len() == 0 {
        return Err(DbError::InputError("payload list is empty".to_string()).into());
    }

    for chunk in payload_list.chunks(1000) {
        let mut query_builder = build_insert_daily_crop_transactions_query(chunk.to_vec());

        let query = query_builder.build();
        query.execute(pool).await?;
    }

    Ok("success".to_string())
}

fn generate_id(data: &DailyCropData) -> String {
    format!("{}:{}", data.transaction_date, data.crop_code)
}

pub async fn fetch_all_daily_crop_transactions(
    pool: &SqlitePool,
    start_date_str: &date::RocDateString,
    end_date_str: &date::RocDateString,
    crop_code: &String,
) -> anyhow::Result<Vec<DailyCropData>> {
    let date_iterator = date::RocDateStringRange(start_date_str.clone(), end_date_str.clone());

    let parameters = date_iterator
        .clone()
        .into_iter()
        .map(|_| "?")
        .collect::<Vec<&str>>()
        .join(", ");

    let sql = format!(
        "
        SELECT
            transaction_date,
            crop_code,
            crop_name,
            type_code,
            high_price,
            mid_price,
            low_price,
            average_price,
            trading_volume
        FROM
            daily_crop_transactions
        WHERE
            transaction_date IN ({}) AND crop_code = ? 
        ",
        parameters
    );

    let mut query = sqlx::query_as::<_, AggregateDailyCropData>(&sql);

    for date in date_iterator {
        query = query.bind(format!("{}", date));
    }

    query = query.bind(crop_code);

    let daily_crop_transactions: Vec<AggregateDailyCropData> = query.fetch_all(pool).await?;

    Ok(daily_crop_transactions
        .into_iter()
        .map(DailyCropData::from)
        .collect())
}
