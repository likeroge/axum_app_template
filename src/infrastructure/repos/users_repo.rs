use sqlx::SqlitePool;

use crate::domain::models::user::{CreateUser, User};

pub async fn users_list(pool: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
    match sqlx::query_as("SELECT * FROM users")
        // match sqlx::query_as::<_, User>("SELECT * FROM users")
        // match sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(pool)
        .await
    {
        Ok(users) => Ok(users),
        Err(err) => Err(err),
    }
}

pub async fn delete_user(pool: &SqlitePool, user_id: i64) -> Result<i64, sqlx::Error> {
    match sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id)
        .execute(pool)
        .await
    {
        Ok(result) => Ok(result.last_insert_rowid()),
        Err(err) => Err(err),
    }
}

pub async fn all_users_with_username(
    pool: &SqlitePool,
    user_name: String,
) -> Result<Vec<User>, sqlx::Error> {
    match sqlx::query_as!(User, "SELECT * FROM users WHERE name=?", user_name)
        .fetch_all(pool)
        .await
    {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

pub async fn create_user(pool: &SqlitePool, user_dto: CreateUser) -> Result<String, sqlx::Error> {
    match sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
        .bind(user_dto.username)
        .bind(user_dto.email)
        .execute(pool)
        .await
    {
        Ok(result) => {
            let id = result.last_insert_rowid();
            Ok(id.to_string())
        }
        Err(err) => Err(err),
    }
}
