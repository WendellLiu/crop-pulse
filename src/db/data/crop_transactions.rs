use sqlx::sqlite::SqlitePool;

pub async fn add_transaction(
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
