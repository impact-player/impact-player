use core::fmt;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum MessageToEngine {
    #[serde(rename = "CREATE_ORDER")]
    CreateOrder { data: CreateOrderPayload },
    #[serde(rename = "CANCEL_ORDER")]
    CancelOrder { data: CancelOrderPayload },
    #[serde(rename = "GET_DEPTH")]
    GetDepth { data: GetDepthPayload },
    #[serde(rename = "GET_QUOTE")]
    GetQuote { data: GetQuotePayload },
    #[serde(rename = "GET_OPEN_ORDERS")]
    GetOpenOrders { data: GetOpenOrdersPayload },
    #[serde(rename = "GET_USER_BALANCES")]
    GetUserBalances { data: GetUserBalancesPayload },
    #[serde(rename = "CREATE_MARKET")]
    CreateMarket { data: CreateMarketPayload },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderPayload {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub market: String,
    pub price: Decimal,
    pub quantity: Decimal,
    pub side: OrderSide,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelOrderPayload {
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub market: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDepthPayload {
    pub market: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTradesPayload {
    pub market: String,
}

#[derive(Deserialize)]
pub struct GetKlinePayload {
    pub market: String,
    pub interval: String,

    #[serde(rename = "startTime")]
    pub start_time: String,

    #[serde(rename = "endTime")]
    pub end_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetQuotePayload {
    pub market: String,
    pub side: OrderSide,
    pub quantity: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOpenOrdersPayload {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub market: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserBalancesPayload {
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OrderSide {
    Bid,
    Ask,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateMarketPayload {
    pub name: String,
    pub description: Option<String>,
    pub base_asset: String,
    pub quote_asset: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Status {
    Incoming,
    Ongoing,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Status::Incoming => "incoming",
            Status::Ongoing => "ongoing",
        };
        write!(f, "{}", s)
    }
}
