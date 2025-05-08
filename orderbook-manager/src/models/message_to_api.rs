use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::{Order, OrderSide};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessageToApi {
    #[serde(rename = "ORDER_PLACES")]
    OrderPlaced { payload: OrderPlacedPayload },
    #[serde(rename = "ORDER_CANCELLED")]
    OrderCancelled { payload: OrderCancelledPayload },
    #[serde(rename = "OPEN_ORDER")]
    OpenOrder { payload: OpenOrdersPayload },
    #[serde(rename = "DEPTH")]
    Depth { payload: DepthPayload },
    #[serde(rename = "USER_BALANCES")]
    UserBalances { payload: UserBalancesPayload },
    #[serde(rename = "QUOTE")]
    Quote { payload: QuotePayload },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderPlacedPayload {
    pub order_id: String,
    pub remaining_qty: Decimal,
    pub filled_qty: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderCancelledPayload {
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenOrdersPayload {
    pub open_orders: Vec<Order>,
}

// Depth Payload
#[derive(Debug, Serialize, Deserialize)]
pub struct DepthPayload {
    pub orders: HashMap<Decimal, OrderDetails>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderDetails {
    pub type_: OrderSide,
    pub quantity: Decimal,
}

// User Balances
#[derive(Debug, Serialize, Deserialize)]
pub struct UserBalancesPayload {
    pub balances: Vec<Balance>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    pub ticker: String,
    pub balance: Decimal,
    pub locked_balance: Decimal,
}

// Quote
#[derive(Debug, Serialize, Deserialize)]
pub struct QuotePayload {
    pub avg_price: Decimal,
    pub quantity: Decimal,
    pub total_cost: Decimal,
}
