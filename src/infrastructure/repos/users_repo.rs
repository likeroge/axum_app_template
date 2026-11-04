use sqlx::SqlitePool;

use crate::domain::models::user::User;

pub async fn users_list(pool: SqlitePool) -> Result<Vec<User>, sqlx::Error> {
    match sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await
    {
        Ok(users) => Ok(users),
        Err(err) => Err(err),
    }
}
