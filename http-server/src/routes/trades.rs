use std::sync::Arc;

use axum::{
    extract::{Query, State},
    Json,
};
use serde_json::{json, Value};
use sqlx::Row;

use crate::{models::GetTradesPayload, state::AppState};

pub async fn get_trades(
    State(state): State<Arc<AppState>>,
    Query(_params): Query<GetTradesPayload>,
) -> Json<Value> {
    let sql = r#"
        SELECT time, price, quantity, volume, currency_code
        FROM sol_prices
        ORDER BY time ASC
    "#;

    let rows = sqlx::query(sql).fetch_all(&*state.db_pool).await;

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
                        "currency_code":  r.get::<String, _>("currency_code"),
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
                "error":   "Failed to query sol_prices"
            }))
        }
    }
}
