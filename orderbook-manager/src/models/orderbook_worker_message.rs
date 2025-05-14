use super::{CancelOrderPayload, CreateOrderPayload, GetOpenOrdersPayload, OrderSide};
use rust_decimal::Decimal;

pub enum OrderbookMessage {
    CreateOrder {
        client_id: String,
        payload: CreateOrderPayload,
    },
    CancelOrder {
        client_id: String,
        payload: CancelOrderPayload,
    },
    GetDepth {
        client_id: String,
        market: String,
    },
    GetOpenOrders {
        client_id: String,
        payload: GetOpenOrdersPayload,
    },
    GetQuote {
        client_id: String,
        market: String,
        quantity: Decimal,
        side: OrderSide,
    },
    ShutDown,
}
