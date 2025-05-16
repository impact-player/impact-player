use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessageFromApi {
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
    #[serde(rename = "ON_RAMP_USER")]
    OnRampUser { data: OnRampPayload },
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
pub struct GetQuotePayload {
    pub market: String,
    pub side: OrderSide,
    pub quantity: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
pub struct OnRampPayload {
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
