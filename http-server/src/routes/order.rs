use std::sync::Arc;

use axum::{
    extract::{Query, State},
    Json,
};
use serde_json::{json, Value};

use crate::{
    models::{
        CancelOrderPayload, CreateOrderPayload, GetOpenOrdersPayload, GetQuotePayload,
        MessageToEngine,
    },
    state::AppState,
};

pub async fn create_order(
    State(state): State<Arc<AppState>>,
    Json(order_data): Json<CreateOrderPayload>,
) -> Json<Value> {
    let message = MessageToEngine::CreateOrder { data: order_data };

    match state.redis_manager.send_and_wait(message) {
        Ok(response) => Json(json!(response)),
        Err(e) => Json(json!({
            "error": format!("Redis error: {}", e)
        })),
    }
}

pub async fn cancel_order(
    State(state): State<Arc<AppState>>,
    Json(order_data): Json<CancelOrderPayload>,
) -> Json<Value> {
    let message = MessageToEngine::CancelOrder { data: order_data };

    match state.redis_manager.send_and_wait(message) {
        Ok(response) => Json(json!(response)),
        Err(e) => Json(json!({
            "error": format!("Redis error: {}", e)
        })),
    }
}

pub async fn get_quote(
    State(state): State<Arc<AppState>>,
    Json(order_data): Json<GetQuotePayload>,
) -> Json<Value> {
    let message = MessageToEngine::GetQuote { data: order_data };

    match state.redis_manager.send_and_wait(message) {
        Ok(response) => Json(json!(response)),
        Err(e) => Json(json!({
            "error": format!("Redis error: {}", e)
        })),
    }
}

pub async fn open_orders(
    State(state): State<Arc<AppState>>,
    Query(order_data): Query<GetOpenOrdersPayload>,
) -> Json<Value> {
    let message = MessageToEngine::GetOpenOrders { data: order_data };

    match state.redis_manager.send_and_wait(message) {
        Ok(response) => Json(json!(response)),
        Err(e) => Json(json!({
            "error": format!("Redis error: {}", e)
        })),
    }
}
