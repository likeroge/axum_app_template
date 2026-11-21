use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::str::FromStr;

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("{}", db_url);

    let connect_options = SqliteConnectOptions::from_str(&db_url)
        .expect("Invalid sqlite address")
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await
        .expect("Failed to create pool");

    // Запуск миграций
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    Ok(pool)
}
