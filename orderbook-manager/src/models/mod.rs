use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

pub mod message_from_api;
pub use message_from_api::*;

pub mod message_to_api;
pub use message_to_api::*;

pub mod message_to_db;
pub use message_to_db::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct IncomingMessage {
    pub client_id: String,
    pub message: MessageFromApi,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Order {
    pub id: String,
    pub user_id: String,
    pub price: Decimal,
    pub quantity: Decimal,
    pub side: OrderSide,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub balances: Vec<Balance>,
}
