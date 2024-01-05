mod logger;

use std::env;

use dotenvy::dotenv;
use sqlx::sqlite::SqlitePool;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenv().expect(".env file not found");
    logger::log("hello world");

    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    let transaction_id = add_transaction(
        &pool,
        "Wheat".to_string(),
        100,
        5.99,
        "2024-01-05".to_string(),
    )
    .await?;
    println!("Added new transaction with id {transaction_id}");

    Ok(())
}

async fn add_transaction(
    pool: &SqlitePool,
    name: String,
    quantity: i32,
    price: f64,
    date: String,
) -> anyhow::Result<i64> {
    let mut conn = pool.acquire().await?;

    // Insert the task, then obtain the ID of this row
    let id = sqlx::query!(
        r#"
INSERT INTO crop_transactions (crop_name, quantity, price, transaction_date)
VALUES (?1, ?2, ?3, ?4)
        "#,
        name,
        quantity,
        price,
        date
    )
    .execute(&mut *conn)
    .await?
    .last_insert_rowid();

    Ok(id)
}
