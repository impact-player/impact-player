use crate::{models::GetKlinePayload, state::AppState};
use axum::{
    debug_handler,
    extract::{Query, State},
    Json,
};
use serde_json::{json, Value};
use sqlx::Row;
use std::sync::Arc;
use time::OffsetDateTime;

#[debug_handler]
pub async fn get_klines(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GetKlinePayload>,
) -> Json<Value> {
    let table_name = match params.interval.as_str() {
        "1m" => "klines_1m",
        "1h" => "klines_1h",
        "1d" => "klines_1d",
        "1w" => "klines_1w",
        _ => {
            return Json(json!({
                "success": false,
                "error": "Invalid interval (must be one of 1m|1h|1d|1w)"
            }))
        }
    };

    let start_ms = match params.start_time.parse::<i64>() {
        Ok(ms) => ms,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid startTime; must be integer milliseconds"
            }))
        }
    };
    let end_ms = match params.end_time.parse::<i64>() {
        Ok(ms) => ms,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid endTime; must be integer milliseconds"
            }))
        }
    };

    let start_dt = OffsetDateTime::from_unix_timestamp(start_ms / 1_000)
        .unwrap()
        .replace_nanosecond(((start_ms % 1_000) * 1_000_000) as u32)
        .unwrap();
    let end_dt = OffsetDateTime::from_unix_timestamp(end_ms / 1_000)
        .unwrap()
        .replace_nanosecond(((end_ms % 1_000) * 1_000_000) as u32)
        .unwrap();

    let sql = format!(
        r#"
        SELECT bucket, open, high, low, close, volume
          FROM {table}
         WHERE currency_code = $1
           AND bucket >= $2
           AND bucket <= $3
         ORDER BY bucket ASC
        "#,
        table = table_name
    );

    let rows = sqlx::query(&sql)
        .bind(&params.market)
        .bind(start_dt)
        .bind(end_dt)
        .fetch_all(&*state.db_pool)
        .await;

    match rows {
        Ok(results) => {
            let data: Vec<Value> = results
                .into_iter()
                .map(|r| {
                    let bucket: OffsetDateTime = r.get("bucket");
                    let ts_ms = bucket.unix_timestamp() * 1_000 + bucket.millisecond() as i64;
                    
                    // For interval-based data, calculate the start time based on the interval
                    let start_ts_ms = match params.interval.as_str() {
                        "1m" => ts_ms - 60 * 1000,         // 1 minute in ms
                        "1h" => ts_ms - 60 * 60 * 1000,    // 1 hour in ms
                        "1d" => ts_ms - 24 * 60 * 60 * 1000, // 1 day in ms
                        "1w" => ts_ms - 7 * 24 * 60 * 60 * 1000, // 1 week in ms
                        _ => ts_ms - 60 * 1000,           // default to 1 minute
                    };

                    let open: f64 = r.get::<f64, _>("open");
                    let high: f64 = r.get::<f64, _>("high");
                    let low: f64 = r.get::<f64, _>("low");
                    let close: f64 = r.get::<f64, _>("close");
                    let volume: f64 = r.get::<Option<f64>, _>("volume").unwrap_or(0.0);
                    
                    // Return in the format expected by the client
                    json!({
                        "open": open.to_string(),
                        "high": high.to_string(),
                        "low": low.to_string(),
                        "close": close.to_string(),
                        "volume": volume.to_string(),
                        "quoteVolume": "0", // Default since we don't have this data
                        "trades": "0",      // Default since we don't have this data
                        "start": start_ts_ms.to_string(),
                        "end": ts_ms.to_string()
                    })
                })
                .collect();

            Json(json!({
                "success": true,
                "data": data
            }))
        }
        Err(e) => {
            tracing::error!("DB error fetching klines: {:?}", e);
            Json(json!({
                "success": false,
                "error": "Database query failed"
            }))
        }
    }
}