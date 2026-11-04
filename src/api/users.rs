use axum::extract::State;
use sqlx::SqlitePool;

use crate::{
    api_responses::{ApiError, ApiResponse, ApiResult},
    infrastructure::repos,
};

pub async fn users_list(State(pool): State<SqlitePool>) -> ApiResult<ApiResponse> {
    match repos::users_repo::users_list(pool).await {
        Ok(users) => Ok(ApiResponse::JsonResponse(serde_json::json!(users))),
        Err(err) => Err(ApiError::DataNotFound),
    }
}
