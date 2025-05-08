use axum::{
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    routing::{delete, get, post},
    Router,
};
use dotenv::dotenv;
use std::sync::Arc;

use state::AppState;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{error, info};

mod config;
mod models;
mod routes;
mod services;
mod state;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let app_state = Arc::new(AppState::new().await);

    let app = Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .route("/healthcheck", get(|| async { "success: true" }))
                .nest(
                    "/order",
                    Router::new()
                        .route("/create", post(routes::create_order))
                        .route("/cancel", delete(routes::cancel_order))
                        .route("/open", get(routes::open_orders))
                        .route("/quote", post(routes::get_quote))
                        .route("/margin-positions", get(routes::margin_positions)),
                )
                .route("/depth", get(routes::get_depth))
                .route("/ticker", get(routes::get_ticker)),
        )
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_headers([CONTENT_TYPE])
                .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE]),
        )
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap_or_else(|e| {
        error!("Server error: {}", e);
        std::process::exit(1);
    });
}
