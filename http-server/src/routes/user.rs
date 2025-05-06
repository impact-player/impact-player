use axum::{extract::State, Json};
use serde_json::{json, Value};
use std::sync::Arc;
use tracing::info;

use crate::{models::OnRampPayload, state::AppState};

pub async fn onramp(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<OnRampPayload>,
) -> Json<Value> {
    info!("hello");
    let response = state.redis_manager.onramp_and_wait(payload);

    info!("Onramp response: {:?}", response);

    match response {
        Ok(response) => Json(json!(response)),
        Err(e) => Json(json!({
            "error": format!("Redis error: {}", e)
        })),
    }
}
