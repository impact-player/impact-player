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

                    let volume: f64 = r.get::<Option<f64>, _>("volume").unwrap_or(0.0);

                    json!([
                        ts_ms,
                        r.get::<f64, _>("open"),
                        r.get::<f64, _>("high"),
                        r.get::<f64, _>("low"),
                        r.get::<f64, _>("close"),
                        volume,
                    ])
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
