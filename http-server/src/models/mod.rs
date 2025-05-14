pub mod message_from_engine;
pub use message_from_engine::*;

pub mod message_to_engine;
pub use message_to_engine::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Order {
    pub id: String,
    pub user_id: String,
    pub price: Decimal,
    pub quantity: Decimal,
    pub side: OrderSide,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Market {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub base_asset: String,
    pub quote_asset: String,
    pub start_time: OffsetDateTime,
    pub end_time: OffsetDateTime,
    pub status: Status,
}
