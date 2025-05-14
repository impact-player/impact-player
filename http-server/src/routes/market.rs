use std::sync::Arc;

use axum::{
    extract::State,
    Json,
};
use serde_json::{json, Value};
use time::OffsetDateTime;

use crate::{
    models::{CreateMarketPayload, MessageToEngine},
    state::AppState,
};

pub async fn create_market(
    State(state): State<Arc<AppState>>,
    Json(market_data): Json<CreateMarketPayload>,
) -> Json<Value> {
    let redis_msg = MessageToEngine::CreateMarket { data: market_data.clone() };
    match state.redis_manager.send_and_wait(redis_msg) {
        Ok(_) => { /* continue */ }
        Err(e) => return Json(json!({ "error": format!("Redis error: {}", e) })),
    }
    
    let db_msg = sqlx::query!(
        r#"
        INSERT INTO markets
          (name, description, base_asset, quote_asset, start_time, end_time, status)
        VALUES
          ($1, $2, $3, $4, $5, $6, $7)
        "#,
        market_data.name,
        market_data.description,
        market_data.base_asset,
        market_data.quote_asset,
        OffsetDateTime::from_unix_timestamp(market_data.start_time.timestamp()).unwrap(),
        OffsetDateTime::from_unix_timestamp(market_data.end_time.timestamp()).unwrap(),
        market_data.status.to_string(),
    )
    .execute(&*state.db_pool)
    .await;

    match db_msg {
        Ok(_) => Json(json!({
            "success": true,
            "message": "Market created successfully"
        })),
        Err(e) => Json(json!({ "error": format!("DB error: {}", e) })),
    }
}
