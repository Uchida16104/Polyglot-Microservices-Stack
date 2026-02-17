use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Internal error: {0}")]
    Internal(String),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::NotFound(msg)   => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::Internal(msg)   => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::Database(e)     => {
                tracing::error!("Database error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
            }
            AppError::Redis(e) => {
                tracing::error!("Redis error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Cache error".to_string())
            }
        };

        (status, Json(serde_json::json!({ "message": message }))).into_response()
    }
}
