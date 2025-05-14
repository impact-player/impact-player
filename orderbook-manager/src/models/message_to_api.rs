use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessageToApi {
    #[serde(rename = "ORDER_PLACES")]
    OrderPlaced { payload: OrderPlacedPayload },
    #[serde(rename = "ORDER_CANCELLED")]
    OrderCancelled { payload: OrderCancelledPayload },
    #[serde(rename = "OPEN_ORDERS")]
    OpenOrders { payload: OpenOrders },
    #[serde(rename = "DEPTH")]
    Depth { payload: DepthPayload },
    #[serde(rename = "USER_BALANCES")]
    UserBalances { payload: UserBalancesPayload },
    #[serde(rename = "QUOTE")]
    Quote { payload: QuotePayload },
    #[serde(rename = "MARKET_CREATED")]
    MarketCreated {payload: MarketCreated }
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
pub struct MarketCreated {
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenOrders {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub market: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DepthPayload {
    pub bids: Vec<[String; 2]>,
    pub asks: Vec<[String; 2]>,
}

// User Balances
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserBalancesPayload {
    pub balances: Vec<Balance>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
