use sqlx::sqlite::SqlitePool;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::client::crop_transaction;

pub async fn add_crop_transaction(
    pool: &SqlitePool,
    payload: &crop_transaction::CropDataResponse,
) -> anyhow::Result<String> {
    let mut conn = pool.acquire().await?;

    let id_to_save = hash_string(format!(
        "{}:{}:{}",
        payload.transaction_date, payload.crop_code, payload.market_code
    ));

    let id = sqlx::query!(
        r#"
INSERT INTO crop_transactions (transaction_date, type_code, crop_code, crop_name, market_code, market_name, high_price, mid_price, low_price, average_price, trading_volume, id)
VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12);
        "#,
        payload.transaction_date,
        payload.type_code,
        payload.crop_code,
        payload.crop_name,
        payload.market_code,
        payload.market_name,
        payload.high_price,
        payload.mid_price,
        payload.low_price,
        payload.average_price,
        payload.trading_volume,
        id_to_save,
    )
    .execute(&mut *conn)
    .await?
    .last_insert_rowid();

    Ok(id.to_string())
}

fn hash_string<T: Hash>(value: T) -> String {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    format!("{}", hasher.finish())
}
