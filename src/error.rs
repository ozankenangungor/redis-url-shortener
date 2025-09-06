use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("URL not found")]
    NotFound,

    #[error("An unexpected Redis pool error occurred")]
    PoolError(#[from] deadpool_redis::PoolError),

    #[error("An unexpected Redis command error occurred")]
    RedisError(#[from] redis::RedisError),

    #[error("Data integrity error: Found invalid URL in storage: {0}")]
    InvalidUrlInStorage(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        error!("{self}");

        let (status, user_message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),

            AppError::PoolError(_) | AppError::RedisError(_) | AppError::InvalidUrlInStorage(_) => {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal server error occurred".to_string(),
                )
            }
        };

        (status, user_message).into_response()
    }
}
