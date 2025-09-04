use anyhow::Result;
use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
};
use redis::{Client, Commands};
use std::env;

#[derive(Clone)]
struct AppState {
    redis_client: Client,
}

#[tokio::main]
async fn main() -> Result<()> {
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    let client = Client::open(redis_url)?;

    let app_state = AppState {
        redis_client: client,
    };

    let app = Router::new()
        .route("/{key}", get(redirect))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    println!("Listening on http://0.0.0.0:8080");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn redirect(State(state): State<AppState>, Path(key): Path<String>) -> impl IntoResponse {
    let mut conn = match state.redis_client.get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to get Redis connection: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not connect to the database",
            ));
        }
    };

    let redis_key = format!("/{}", key);

    match conn.get::<&str, String>(&redis_key) {
        Ok(url) => {
            println!("Redirecting {} to {}", &redis_key, &url);
            Ok(Redirect::permanent(&url))
        }
        Err(_) => {
            println!("Key {} not found", &redis_key);
            Err((StatusCode::NOT_FOUND, "URL Not Found"))
        }
    }
}
