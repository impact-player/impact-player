use std::sync::Arc;

use anyhow::Result;
use axum::{
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    routing::{delete, get, post},
    Router,
};
use dotenv::dotenv;
use routes::{
    cancel_order, create_market, create_order, get_all_markets, get_balances, get_depth,
    get_klines, get_market_by_id, get_quote, get_trades, on_ramp, open_orders,
};
use state::AppState;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{error, info};

mod models;
mod routes;
mod services;
mod state;

#[tokio::main]
async fn main() -> Result<()> {
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
                        .route("/create", post(create_order))
                        .route("/cancel", delete(cancel_order))
                        .route("/open", get(open_orders))
                        .route("/quote", post(get_quote)),
                )
                .nest(
                    "/market",
                    Router::new()
                        .route("/markets", get(get_all_markets))
                        .route("/{id}", get(get_market_by_id))
                        .route("/create", post(create_market)),
                )
                .route("/depth", get(get_depth))
                .route("/create", post(create_market))
                .route("/klines", get(get_klines))
                .route("/trades", get(get_trades))
                .nest(
                    "/user",
                    Router::new()
                        .route("/balances", get(get_balances))
                        .route("/onramp", post(on_ramp)),
                ),
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

    info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await.unwrap_or_else(|e| {
        error!("Server error: {}", e);
        std::process::exit(1);
    });

    Ok(())
}
