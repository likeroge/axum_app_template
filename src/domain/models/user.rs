use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Default, FromRow, Serialize, Deserialize)]
pub struct User {
    id: u64,
    name: String,
    email: String,
    created_at: String,
}
