use async_once::AsyncOnce;
use lazy_static::lazy_static;
use sqlx::sqlite::SqlitePool;

use std::env;

lazy_static! {
    pub static ref POOL: AsyncOnce<SqlitePool> = AsyncOnce::new(async {
        let database_url = env::var("DATABASE_URL").unwrap();
        let pool = SqlitePool::connect(&database_url).await.unwrap();

        pool
    });
}
