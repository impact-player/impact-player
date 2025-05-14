use std::sync::Arc;

use axum::{
    extract::{Query, State},
    Json,
};
use serde_json::{json, Value};
use sqlx::Row;

use crate::{models::GetTradesPayload, state::AppState};

fn sanitize_ticker(ticker: &str) -> String {
    ticker.replace(|c: char| !c.is_alphanumeric() && c != '_', "_") 
          .to_lowercase()
}

pub async fn get_trades(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GetTradesPayload>,
) -> Json<Value> {
    // Sanitize the market ticker for table name construction
    let sanitized_ticker = sanitize_ticker(&params.market);
    let table_name = format!("prices_{}", sanitized_ticker);

    // Use the prices_* table that's created dynamically
    let sql = format!(
        r#"SELECT "time", price, quantity, volume, market_ticker 
           FROM public.{} 
           ORDER BY "time" ASC"#,
        table_name
    );

    let rows = sqlx::query(&sql).fetch_all(&*state.db_pool).await;

    match rows {
        Ok(records) => {
            let data: Vec<Value> = records
                .into_iter()
                .map(|r| {
                    let time: chrono::DateTime<chrono::Utc> = r.get("time");
                    let volume: f64 = r.get::<Option<f64>, _>("volume").unwrap_or(0.0);

                    json!({
                        "time":           time.to_rfc3339(),
                        "price":          r.get::<f64, _>("price"),
                        "quantity":       r.get::<f64, _>("quantity"),
                        "volume":         volume,
                        "market_ticker":  r.get::<String, _>("market_ticker"),
                    })
                })
                .collect();

            Json(json!({
                "success": true,
                "data":    data
            }))
        }
        Err(e) => {
            tracing::error!("DB error fetching trades: {:?}", e);
            Json(json!({
                "success": false,
                "error":   format!("Failed to query trades table prices_{}", sanitized_ticker)
            }))
        }
    }
}