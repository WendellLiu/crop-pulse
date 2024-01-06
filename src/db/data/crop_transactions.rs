use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use sqlx::sqlite::SqlitePool;
use sqlx::{QueryBuilder, Sqlite};

use crate::client::crop_transaction;

fn build_insert_crop_transactions_query(
    crop_transaction_list: Vec<crop_transaction::CropDataResponse>,
) -> QueryBuilder<'static, Sqlite> {
    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        r#"
INSERT INTO crop_transactions 
(id, transaction_date, type_code, crop_code, crop_name, market_code, market_name, high_price, mid_price, low_price, average_price, trading_volume) "#,
    );

    query_builder.push_values(crop_transaction_list, |mut builder, c_transaction| {
        let id_to_save = hash_string(format!(
            "{}:{}:{}",
            c_transaction.transaction_date, c_transaction.crop_code, c_transaction.market_code
        ));

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
    let mut query_builder = build_insert_crop_transactions_query(payload_list);

    let query = query_builder.build();

    query.execute(pool).await?.last_insert_rowid();
    Ok("succussful".to_string())
}

fn hash_string<T: Hash>(value: T) -> String {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    format!("{}", hasher.finish())
}
