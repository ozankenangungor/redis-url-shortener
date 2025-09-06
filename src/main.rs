mod error;
mod handlers;
mod state;

use crate::{handlers::redirect, state::AppState};
use axum::{routing::get, Router};
use deadpool_redis::{Config, Runtime};
use std::env;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "url_shortener=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let server_addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());

    let cfg = Config::from_url(redis_url);
    let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
    info!("Redis connection pool created.");

    let app_state = AppState { redis_pool: pool };

    let app = Router::new()
        .route("/:key", get(redirect))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&server_addr).await?;
    info!("Listening on http://{}", &server_addr);

    axum::serve(listener, app).await?;

    Ok(())
}
