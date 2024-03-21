use serde::Deserialize;
use sqlx::sqlite::SqlitePool;
use sqlx::{QueryBuilder, Sqlite};
use thiserror::Error;

#[derive(Deserialize, Debug, Clone)]
pub struct CropsSummary14Days {
    pub end_date: String,
    pub crop_code: String,
    pub high_price_beta_coefficient: f64,
    pub mid_price_beta_coefficient: f64,
    pub low_price_beta_coefficient: f64,
    pub average_price_beta_coefficient: f64,
    pub trading_volume_beta_coefficient: f64,
    pub trading_volume_sum: f64,
}

#[derive(Error, Debug)]
pub enum DbError {
    #[error("input error: `{0}`")]
    InputError(String),
}

pub async fn add_crops_summary_14_days(
    pool: &SqlitePool,
    payload_list: Vec<CropsSummary14Days>,
) -> anyhow::Result<String> {
    if payload_list.len() == 0 {
        return Err(DbError::InputError("payload list is empty".to_string()).into());
    }

    for chunk in payload_list.chunks(1000) {
        let mut query_builder = build_insert_crops_summary_14_days_query(chunk.to_vec());

        let query = query_builder.build();
        query.execute(pool).await?;
    }

    Ok("success".to_string())
}

fn build_insert_crops_summary_14_days_query(
    crops_summary_14_days_list: Vec<CropsSummary14Days>,
) -> QueryBuilder<'static, Sqlite> {
    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        r#"
REPLACE INTO crops_summary_14_days 
(id, end_date, crop_code, high_price_beta_coefficient, mid_price_beta_coefficient, low_price_beta_coefficient, average_price_beta_coefficient, trading_volume_beta_coefficient, trading_volume_sum) "#,
    );

    query_builder.push_values(
        crops_summary_14_days_list,
        |mut builder, cs_14_days_transaction| {
            let id_to_save = generate_id(&cs_14_days_transaction);

            builder
                .push_bind(id_to_save)
                .push_bind(cs_14_days_transaction.end_date)
                .push_bind(cs_14_days_transaction.crop_code)
                .push_bind(cs_14_days_transaction.high_price_beta_coefficient)
                .push_bind(cs_14_days_transaction.mid_price_beta_coefficient)
                .push_bind(cs_14_days_transaction.low_price_beta_coefficient)
                .push_bind(cs_14_days_transaction.average_price_beta_coefficient)
                .push_bind(cs_14_days_transaction.trading_volume_beta_coefficient)
                .push_bind(cs_14_days_transaction.trading_volume_sum);
        },
    );

    query_builder
}
fn generate_id(data: &CropsSummary14Days) -> String {
    format!("{}:{}", data.end_date, data.crop_code)
}
