use anyhow::Result;
use axum::{routing::get, Router};
use state::AppState;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

mod models;
mod state;

#[tokio::main]
async fn main() -> Result<()> {
    let redis_url = "redis://127.0.0.1/";
    let redis_client = redis::Client::open(redis_url)?;

    let app_state = AppState::new(redis_client).await?;

    let app = Router::new()
        .route("/ws", get(|| async {}))
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
