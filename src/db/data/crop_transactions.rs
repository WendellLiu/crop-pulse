use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use sqlx::sqlite::SqlitePool;
use sqlx::Error::Database;
use sqlx::{QueryBuilder, Sqlite};
use thiserror::Error;

use crate::client::crop_transaction;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("input error: `{0}`")]
    InputError(String),
}

fn build_insert_crop_transactions_query(
    crop_transaction_list: Vec<crop_transaction::CropDataResponse>,
) -> QueryBuilder<'static, Sqlite> {
    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        r#"
REPLACE INTO crop_transactions 
(id, transaction_date, type_code, crop_code, crop_name, market_code, market_name, high_price, mid_price, low_price, average_price, trading_volume) "#,
    );

    query_builder.push_values(crop_transaction_list, |mut builder, c_transaction| {
        let id_to_save = generate_id(&c_transaction);

        builder
            .push_bind(id_to_save)
            .push_bind(c_transaction.transaction_date)
            .push_bind(c_transaction.type_code)
            .push_bind(c_transaction.crop_code)
            .push_bind(c_transaction.crop_name)
            .push_bind(c_transaction.market_code)
            .push_bind(c_transaction.market_name)
            .push_bind(c_transaction.high_price)
            .push_bind(c_transaction.mid_price)
            .push_bind(c_transaction.low_price)
            .push_bind(c_transaction.average_price)
            .push_bind(c_transaction.trading_volume);
    });

    query_builder
}

pub async fn add_crop_transactions(
    pool: &SqlitePool,
    payload_list: Vec<crop_transaction::CropDataResponse>,
) -> anyhow::Result<String> {
    if payload_list.len() == 0 {
        return Err(DbError::InputError("payload list is empty".to_string()).into());
    }

    let mut query_builder = build_insert_crop_transactions_query(payload_list);

    let query = query_builder.build();
    let query_result = query.execute(pool).await;

    match query_result {
        Ok(_) => Ok("success".to_string()),
        Err(err) => match err {
            Database(ref db_error) => {
                if db_error.is_unique_violation() {
                    return Ok("success with duplicated data".to_string());
                }
                Err(err.into())
            }
            _ => Err(err.into()),
        },
    }
}

fn generate_id(response: &crop_transaction::CropDataResponse) -> String {
    let transaction_date = response.transaction_date.clone().unwrap_or("".to_string());
    let crop_code = response.crop_code.clone().unwrap_or("".to_string());
    let market_code = response.market_code.clone().unwrap_or("".to_string());

    format!("{}:{}:{}", transaction_date, crop_code, market_code)
}

fn hash_string<T: Hash>(value: T) -> String {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    format!("{}", hasher.finish())
}
