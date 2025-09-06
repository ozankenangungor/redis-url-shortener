use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::error;

pub enum AppError {
    PoolError(deadpool_redis::PoolError),
    RedisError(redis::RedisError),
    NotFound,
    InvalidUrlInStorage(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "URL Not Found".to_string()),
            AppError::PoolError(e) => {
                error!("Redis pool error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            AppError::RedisError(e) => {
                error!("Redis command error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            AppError::InvalidUrlInStorage(invalid_url) => {
                error!(
                    "Data integrity error: Found invalid URL in storage: {}",
                    invalid_url
                );
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };
        (status, error_message).into_response()
    }
}

impl From<deadpool_redis::PoolError> for AppError {
    fn from(e: deadpool_redis::PoolError) -> Self {
        AppError::PoolError(e)
    }
}

impl From<redis::RedisError> for AppError {
    fn from(e: redis::RedisError) -> Self {
        AppError::RedisError(e)
    }
}
