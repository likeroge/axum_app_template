use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Default, FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Default, Serialize, Deserialize, FromRow, Debug)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
}
