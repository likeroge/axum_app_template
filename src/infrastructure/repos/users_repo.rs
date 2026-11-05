use sqlx::SqlitePool;

use crate::domain::models::user::{CreateUser, User};

pub async fn users_list(pool: SqlitePool) -> Result<Vec<User>, sqlx::Error> {
    match sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await
    {
        Ok(users) => Ok(users),
        Err(err) => Err(err),
    }
}

pub async fn create_user(pool: SqlitePool, user_dto: CreateUser) -> Result<String, sqlx::Error> {
    match sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
        .bind(user_dto.username)
        .bind(user_dto.email)
        .execute(&pool)
        .await
    {
        Ok(result) => {
            let id = result.last_insert_rowid();
            Ok(id.to_string())
        }
        Err(err) => Err(err),
    }
}
