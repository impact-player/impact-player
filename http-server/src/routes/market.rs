use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::{json, Value};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    models::{CreateMarketPayload, Market, MessageToEngine, Status},
    state::AppState,
};

pub async fn create_market(
    State(state): State<Arc<AppState>>,
    Json(market_data): Json<CreateMarketPayload>,
) -> Json<Value> {
    let redis_msg = MessageToEngine::CreateMarket {
        data: market_data.clone(),
    };
    if let Err(e) = state.redis_manager.send_and_wait(redis_msg) {
        return Json(json!({ "error": format!("Redis error: {}", e) }));
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

pub async fn get_all_markets(State(state): State<Arc<AppState>>) -> Json<Value> {
    let rows = sqlx::query!(
        r#"
        SELECT id, name, description, base_asset, quote_asset,
            start_time, end_time, status, created_at, updated_at
        FROM markets
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(&*state.db_pool)
    .await;

    match rows {
        Ok(records) => {
            let markets: Vec<Market> = records
                .into_iter()
                .map(|r| Market {
                    id: r.id,
                    name: r.name,
                    description: r.description,
                    base_asset: r.base_asset,
                    quote_asset: r.quote_asset,
                    start_time: r.start_time.into(),
                    end_time: r.end_time.into(),
                    status: string_to_status(&r.status),
                })
                .collect();

            Json(json!(markets))
        }
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

pub async fn get_market_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Json<Value> {
    let row = sqlx::query!(
        r#"
        SELECT id, name, description, base_asset, quote_asset,
               start_time, end_time, status, created_at, updated_at
        FROM markets
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(&*state.db_pool)
    .await;

    match row {
        Ok(r) => {
            let market = Market {
                id: r.id,
                name: r.name,
                description: r.description,
                base_asset: r.base_asset,
                quote_asset: r.quote_asset,
                start_time: r.start_time.into(),
                end_time: r.end_time.into(),
                status: string_to_status(&r.status),
            };
            Json(json!(market))
        }
        Err(sqlx::Error::RowNotFound) => Json(json!({ "error": "Market not found" })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

fn string_to_status(status_str: &Option<String>) -> Status {
    match status_str {
        Some(s) if s == "Ongoing" => Status::Ongoing,
        _ => Status::Incoming, // Default to Incoming for any other value or None
    }
}
