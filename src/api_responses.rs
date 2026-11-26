use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::Value;

pub enum ApiResponse {
    JsonResponse(Value),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            ApiResponse::JsonResponse(data) => (StatusCode::OK, Json(data)).into_response(),
        }
    }
}

pub enum ApiError {
    DataNotFound,
    CustomError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::DataNotFound => (StatusCode::NOT_FOUND, "Data Not Found").into_response(),
            ApiError::CustomError(err) => (StatusCode::BAD_REQUEST, err).into_response(),
        }
    }
}

pub type ApiResult<ApiResponse> = Result<ApiResponse, ApiError>;
