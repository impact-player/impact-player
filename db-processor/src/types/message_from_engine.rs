use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum MessageFromEngine {
    #[serde(rename = "TRADE_ADDED")]
    AddTrade { data: AddTradePayload },
}

#[derive(Debug, Deserialize, Clone)]
pub struct AddTradePayload {
    pub ticker: String,
    pub time: DateTime<Utc>,
    pub quantity: Decimal,
    pub price: Decimal,
}
