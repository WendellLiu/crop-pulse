use serde::Deserialize;
use sqlx::sqlite::SqlitePool;
use sqlx::Error::Database;
use sqlx::{QueryBuilder, Sqlite};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("input error: `{0}`")]
    InputError(String),
}

impl From<AggregatedCropsData> for CropsData {
    fn from(aggregate_data: AggregatedCropsData) -> Self {
        CropsData {
            type_code: aggregate_data.type_code.expect("Missing type code"),
            crop_code: aggregate_data.crop_code.expect("Missing crop code"),
            crop_name: aggregate_data.crop_name.expect("Missing crop name"),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct AggregatedCropsData {
    pub type_code: Option<String>,
    pub crop_code: Option<String>,
    pub crop_name: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CropsData {
    pub type_code: String,
    pub crop_code: String,
    pub crop_name: String,
}

pub async fn aggregate_crops(pool: &SqlitePool) -> anyhow::Result<Vec<CropsData>> {
    let daily_crop_transactions: Vec<AggregatedCropsData> = sqlx::query_as!(
        AggregatedCropsData,
        "
    SELECT
        crop_code,
        crop_name,
        type_code
    FROM
        daily_crop_transactions
    GROUP BY
        crop_code
        ",
    )
    .fetch_all(pool)
    .await?;

    Ok(daily_crop_transactions
        .into_iter()
        .map(CropsData::from)
        .collect())
}

fn build_insert_crops_query(crop_list: Vec<CropsData>) -> QueryBuilder<'static, Sqlite> {
    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        r#"
REPLACE INTO crops 
(id, type_code, crop_code, crop_name) "#,
    );

    query_builder.push_values(crop_list, |mut builder, crop| {
        builder
            .push_bind(crop.crop_code.clone())
            .push_bind(crop.type_code)
            .push_bind(crop.crop_code)
            .push_bind(crop.crop_name);
    });

    query_builder
}

pub async fn add_crops(pool: &SqlitePool, payload_list: &Vec<CropsData>) -> anyhow::Result<String> {
    if payload_list.len() == 0 {
        return Err(DbError::InputError("payload list is empty".to_string()).into());
    }

    let mut query_builder = build_insert_crops_query(payload_list.to_vec());

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
