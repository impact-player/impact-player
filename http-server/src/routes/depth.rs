use std::sync::Arc;

use axum::{
    extract::{Query, State},
    Json,
};
use serde_json::{json, Value};

use crate::{
    models::{GetDepthPayload, MessageToEngine},
    state::AppState,
};

pub async fn get_depth(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GetDepthPayload>,
) -> Json<Value> {
    let message = MessageToEngine::GetDepth {
        data: GetDepthPayload {
            market: params.market,
        },
    };

    match state.redis_manager.send_and_wait(message) {
        Ok(response) => Json(json!(response)),
        Err(e) => Json(json!({
            "error": format!("Redis error: {}", e)
        })),
    }
}
