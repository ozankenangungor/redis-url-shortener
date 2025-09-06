use crate::error::AppError;
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
};
use deadpool_redis::redis::AsyncCommands;
use tracing::{info, warn};
use url::Url;

pub async fn redirect(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = state.redis_pool.get().await?;
    info!("Looking up key: /{}", key);

    let redis_key = format!("/{}", key);

    let maybe_url: Option<String> = conn.get(&redis_key).await?;

    match maybe_url {
        Some(url) => match Url::parse(&url) {
            Ok(_) => {
                info!("Redirecting {} to {}", &redis_key, &url);
                Ok(Redirect::permanent(&url))
            }
            Err(e) => {
                warn!(
                    "Invalid URL found in Redis for key '{}': {}. Error: {}",
                    &redis_key, &url, e
                );
                Err(AppError::InvalidUrlInStorage(url))
            }
        },
        None => {
            warn!("Key not found: {}", &redis_key);
            Err(AppError::NotFound)
        }
    }
}
