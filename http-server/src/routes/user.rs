use std::sync::Arc;

use axum::{
    extract::{Query, State},
    Json,
};
use serde_json::{json, Value};

use crate::{
    models::{GetUserBalancesPayload, MessageToEngine, OnRampPayload},
    state::AppState,
};

pub async fn get_balances(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GetUserBalancesPayload>,
) -> Json<Value> {
    let message = MessageToEngine::GetUserBalances {
        data: GetUserBalancesPayload {
            user_id: params.user_id,
        },
    };

    match state.redis_manager.send_and_wait(message) {
        Ok(response) => Json(json!(response)),
        Err(e) => Json(json!({
            "error": format!("Redis error: {}", e)
        })),
    }
}

pub async fn on_ramp(
    State(state): State<Arc<AppState>>,
    Json(params): Json<OnRampPayload>,
) -> Json<Value> {
    let message = MessageToEngine::OnRampUser {
        data: OnRampPayload {
            user_id: params.user_id,
        },
    };

    match state.redis_manager.send_and_wait(message) {
        Ok(response) => Json(json!(response)),
        Err(e) => Json(json!({
            "error": format!("Redis error: {}", e)
        })),
    }
}
