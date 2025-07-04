use sqlx::SqlitePool;

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePool::connect("sqlite:anon_shop.db").await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}