use std::time::Duration;

use anyhow::Result;
use dotenv::dotenv;
use redis::Commands;
use services::RedisManager;
use sqlx::PgPool;
use time::OffsetDateTime;
use types::IncomingMessage;

mod services;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let redis_manager = RedisManager::new();
    let mut conn = redis_manager.get_connection()?;

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;

    let refresh_pool = pool.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(10));
        loop {
            interval.tick().await;
            if let Err(e) = refresh_views(&refresh_pool).await {
                eprintln!("Error refreshing views: {:?}", e);
            } else {
                println!("Materialized views refreshed successfully");
            }
        }
    });

    loop {
        let response: Option<(String, String)> = conn.brpop("db_processor", 0.0)?;

        if let Some((_, message)) = response {
            let parsed_message: IncomingMessage = serde_json::from_str(&message)?;

            if parsed_message.message_type == "TRADE_ADDED" {
                let data = parsed_message.data;

                match data.ticker.as_str() {
                    "SOL_USDC" => {
                        sqlx::query!(
                                "INSERT INTO sol_prices (time, price, quantity, currency_code) VALUES ($1, $2, $3, $4)", 
                                OffsetDateTime::from_unix_timestamp(data.time.timestamp()).unwrap(),
                                data.price.to_string().parse::<f64>().unwrap(),
                                data.quantity.to_string().parse::<f64>().unwrap(),
                                "SOL"
                            ).execute(&pool).await?;
                    }
                    _ => {}
                }
            }
        }
    }
}

async fn refresh_views(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!("REFRESH MATERIALIZED VIEW klines_1m").execute(pool).await?;
    sqlx::query!("REFRESH MATERIALIZED VIEW klines_1h").execute(pool).await?;
    sqlx::query!("REFRESH MATERIALIZED VIEW klines_1d").execute(pool).await?;
    sqlx::query!("REFRESH MATERIALIZED VIEW klines_1w").execute(pool).await?;
    Ok(())
}