use std::vec::ExtractIf;

use axum::{
    Form,
    extract::{Path, State},
};
use sqlx::SqlitePool;

use crate::{
    api_responses::{ApiError, ApiResponse, ApiResult},
    domain::models::user::CreateUser,
    infrastructure::repos,
};

pub async fn users_list(State(pool): State<SqlitePool>) -> ApiResult<ApiResponse> {
    match repos::users_repo::users_list(&pool).await {
        Ok(users) => Ok(ApiResponse::JsonResponse(serde_json::json!(users))),
        Err(err) => Err(ApiError::DataNotFound),
    }
}

pub async fn create_user_form(
    State(pool): State<SqlitePool>,
    Form(user_dto): Form<CreateUser>,
) -> ApiResult<ApiResponse> {
    println!("{:?}", user_dto);
    match repos::users_repo::create_user(&pool, user_dto).await {
        Ok(users) => Ok(ApiResponse::JsonResponse(serde_json::json!(users))),
        Err(err) => Err(ApiError::DataNotFound),
    }
}

pub async fn delete_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> ApiResult<ApiResponse> {
    match repos::users_repo::delete_user(&pool, id).await {
        Ok(user_id) => Ok(ApiResponse::JsonResponse(serde_json::json!(user_id))),
        Err(err) => Err(ApiError::DataNotFound),
    }
}

pub async fn users_list_with_name(
    State(pool): State<SqlitePool>,
    Path(user): Path<String>,
) -> ApiResult<ApiResponse> {
    match repos::users_repo::all_users_with_username(&pool, user).await {
        Ok(users) => Ok(ApiResponse::JsonResponse(serde_json::json!(users))),
        Err(err) => Err(ApiError::DataNotFound),
    }
}
