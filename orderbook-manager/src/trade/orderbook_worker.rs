#![allow(unused_variables)]

use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use chrono::Utc;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde_json::json;
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    models::{
        AddTradePayload, CancelOrderPayload, CreateOrderPayload, GetOpenOrdersPayload,
        MessageToApi, OpenOrders, Order, OrderCancelledPayload, OrderPlacedPayload, OrderSide,
        OrderbookMessage, TradeData, User,
    },
    services::RedisManager,
};
use std::str::FromStr;

use super::Orderbook;

#[allow(unused)]
pub struct OrderbookWorker {
    pub market: String,
    pub orderbook: Orderbook,
    pub users: Arc<Mutex<Vec<User>>>,
    pub sender: mpsc::Sender<OrderbookMessage>,
    pub thread_handle: Option<thread::JoinHandle<()>>,
}

impl OrderbookWorker {
    pub fn new(
        market: String,
        base_asset: String,
        quote_asset: String,
        users: Arc<Mutex<Vec<User>>>,
    ) -> Self {
        let (sender, receiver) = mpsc::channel();
        let orderbook = Orderbook::new(base_asset.clone(), quote_asset.clone());
        let orderbook_clone = orderbook.clone();
        let users_clone = users.clone();
        let market_clone = market.clone();

        let thread_handle = thread::spawn(move || {
            info!("Started orderbook thread for market: {}", market_clone);
            let mut orderbook = orderbook;

            loop {
                match receiver.recv() {
                    Ok(message) => match message {
                        OrderbookMessage::CreateOrder { client_id, payload } => {
                            info!("Processing create order for market: {}", market_clone);
                            Self::handle_create_order(&mut orderbook, &users, client_id, payload);
                        }
                        OrderbookMessage::CancelOrder { client_id, payload } => {
                            info!("Processing cancel order for market: {}", market_clone);
                            Self::handle_cancel_order(&mut orderbook, &users, client_id, payload);
                        }
                        OrderbookMessage::GetDepth { client_id, market } => {
                            info!("Processing get depth for market: {}", market_clone);
                            Self::handle_get_depth(&orderbook, client_id);
                        }
                        OrderbookMessage::GetOpenOrders { client_id, payload } => {
                            info!("Processing get open orders for market: {}", market_clone);
                            Self::handle_get_open_orders(&orderbook, client_id, payload);
                        }
                        OrderbookMessage::GetQuote {
                            client_id,
                            market,
                            quantity,
                            side,
                        } => {
                            info!("Processing get quote for market: {}", market_clone);
                            Self::handle_get_quote(&orderbook, client_id, quantity, side);
                        }
                        OrderbookMessage::ShutDown => {
                            info!("Processing shutdown for market: {}", market_clone);
                        }
                    },
                    Err(e) => {
                        error!("Error receiving message in orderbook thread: {}", e);
                        break;
                    }
                }
            }
        });

        OrderbookWorker {
            market,
            orderbook: orderbook_clone,
            users: users_clone,
            sender,
            thread_handle: Some(thread_handle),
        }
    }

    fn handle_create_order(
        orderbook: &mut Orderbook,
        users: &Arc<Mutex<Vec<User>>>,
        client_id: String,
        payload: CreateOrderPayload,
    ) {
        let order_id = Uuid::new_v4().to_string();
        let redis_manager = RedisManager::instance();

        let is_valid = {
            let mut users_guard = users.lock().unwrap();

            let user = users_guard
                .iter_mut()
                .find(|u| u.id == payload.user_id)
                .expect("User not found");

            match payload.side {
                OrderSide::Bid => {
                    let usdc_balance = user
                        .balances
                        .iter()
                        .find(|b| b.ticker == "USDC".to_string())
                        .map(|b| b.balance - b.locked_balance)
                        .unwrap_or(dec!(0));

                    let required_amount = payload.price * payload.quantity;
                    if usdc_balance >= required_amount {
                        if let Some(balance) = user
                            .balances
                            .iter_mut()
                            .find(|b| b.ticker == "USDC".to_string())
                        {
                            balance.locked_balance += required_amount;
                        }
                        true
                    } else {
                        false
                    }
                }
                OrderSide::Ask => {
                    let sol_balance = user
                        .balances
                        .iter()
                        .find(|b| b.ticker == "SOL".to_string())
                        .map(|b| b.balance - b.locked_balance)
                        .unwrap_or(dec!(0));

                    if sol_balance >= payload.quantity {
                        if let Some(balance) = user
                            .balances
                            .iter_mut()
                            .find(|b| b.ticker == "SOL".to_string())
                        {
                            balance.locked_balance += payload.quantity;
                        }
                        true
                    } else {
                        false
                    }
                }
            }
        };

        if !is_valid {
            error!("Insufficient balance for trade");
            let message = MessageToApi::OrderCancelled {
                payload: OrderCancelledPayload {
                    message: Some(String::from("Insufficient balance for trade")),
                },
            };
            let _ = redis_manager.send_to_api(&client_id, &message);
            return;
        }

        let market: Vec<&str> = payload.market.split('_').collect();
        let base_asset = market[0];
        let quote_asset = market[1];

        let remaining_qty = orderbook.fill_orders(&payload, users, base_asset, quote_asset);
        let filled_qty = payload.quantity.checked_sub(remaining_qty).unwrap();

        if remaining_qty > Decimal::ZERO {
            let new_order = Order {
                id: order_id.clone(),
                user_id: payload.user_id.clone(),
                price: payload.price,
                quantity: remaining_qty,
                side: payload.side.clone(),
                timestamp: Utc::now().timestamp(),
            };

            match payload.side {
                OrderSide::Bid => {
                    orderbook.bids.push(new_order);
                    orderbook.bids.sort_by(|a, b| {
                        b.price
                            .cmp(&a.price)
                            .then_with(|| a.timestamp.cmp(&b.timestamp))
                    });
                }
                OrderSide::Ask => {
                    orderbook.asks.push(new_order);
                    orderbook.asks.sort_by(|a, b| {
                        a.price
                            .cmp(&b.price)
                            .then_with(|| a.timestamp.cmp(&b.timestamp))
                    });
                }
            }
        }

        info!(
            order_id,
            ?remaining_qty,
            ?filled_qty,
            "Order created successfully"
        );

        let message = MessageToApi::OrderPlaced {
            payload: OrderPlacedPayload {
                order_id: order_id.clone(),
                remaining_qty,
                filled_qty,
            },
        };

        let _ = redis_manager.send_to_api(&client_id, &message);

        let depth = orderbook.get_depth();

        match payload.side {
            OrderSide::Bid => {
                let matching_ask = depth
                    .asks
                    .iter()
                    .find(|x| {
                        Decimal::from_str(&x[0].to_string()).unwrap_or(dec!(0)) <= payload.price
                    })
                    .cloned();

                info!("publish ws depth updates for bid");

                let message = json!({
                    "stream": format!("depth@{}", payload.market),
                    "data": {
                        "a": if let Some(ask) = matching_ask { vec![ask] } else { Vec::new() },
                        "b": depth.bids,
                        "e": "depth"
                    }
                });

                let _ =
                    redis_manager.publish_message(&format!("depth@{}", payload.market), &message);
            }
            OrderSide::Ask => {
                let matching_bid = depth
                    .bids
                    .iter()
                    .find(|x| {
                        Decimal::from_str(&x[0].to_string()).unwrap_or(dec!(0)) >= payload.price
                    })
                    .cloned();

                let message = json!({
                    "stream": format!("depth@{}", payload.market),
                    "data": {
                        "a": depth.asks,
                        "b": if let Some(bid) = matching_bid { vec![bid] } else { Vec::new() },
                        "e": "depth"
                    }
                });

                let _ =
                    redis_manager.publish_message(&format!("depth@{}", payload.market), &message);
            }
        }

        let trade_info = json!({
            "price": payload.price,
            "quantity": filled_qty,
            "side": payload.side,
            "timestamp": Utc::now().timestamp()
        });

        let db_info = AddTradePayload {
            data: TradeData {
                ticker: payload.market.clone(),
                time: Utc::now(),
                price: payload.price,
                quantity: payload.quantity,
            },
        };

        let _ = redis_manager.publish_message(&format!("trade@{}", payload.market), &trade_info);
        let _ = redis_manager.push_message_to_db(&db_info);
    }

    fn handle_cancel_order(
        orderbook: &mut Orderbook,
        users: &Arc<Mutex<Vec<User>>>,
        client_id: String,
        payload: CancelOrderPayload,
    ) {
        let market: Vec<&str> = payload.market.split('_').collect();
        let base_asset = market[0];
        let quote_asset = market[1];
        let redis_manager = RedisManager::instance();

        let mut order_found = false;

        if let Some(bid_index) = orderbook
            .bids
            .iter()
            .position(|order| order.id == payload.order_id)
        {
            let order = &orderbook.bids[bid_index];

            let mut users_guard = users.lock().unwrap();
            if let Some(user) = users_guard.iter_mut().find(|u| u.id == order.user_id) {
                if let Some(balance) = user.balances.iter_mut().find(|b| b.ticker == quote_asset) {
                    let locked_amount = order.price * order.quantity;
                    balance.locked_balance -= locked_amount;
                }
            }

            orderbook.bids.remove(bid_index);
            order_found = true;
        }

        if !order_found {
            if let Some(ask_index) = orderbook
                .asks
                .iter()
                .position(|order| order.id == payload.order_id)
            {
                let order = &orderbook.asks[ask_index];

                let mut users_guard = users.lock().unwrap();
                if let Some(user) = users_guard.iter_mut().find(|u| u.id == order.user_id) {
                    if let Some(balance) = user.balances.iter_mut().find(|b| b.ticker == base_asset)
                    {
                        balance.locked_balance -= order.quantity;
                    }
                }

                orderbook.asks.remove(ask_index);
                order_found = true;
            }
        }

        info!(order_id = ?payload.order_id, "Order cancelled successfully");
        let message = MessageToApi::OrderCancelled {
            payload: OrderCancelledPayload {
                message: Some(String::from("ORDER CANCELLED")),
            },
        };

        let _ = redis_manager.send_to_api(&client_id, &message);
    }

    fn handle_get_depth(orderbook: &Orderbook, client_id: String) {
        let depth = orderbook.get_depth();

        let redis_manager = RedisManager::instance();
        let message = MessageToApi::Depth { payload: depth };
        let _ = redis_manager.send_to_api(&client_id, &message);
    }

    fn handle_get_open_orders(
        orderbook: &Orderbook,
        client_id: String,
        payload: GetOpenOrdersPayload,
    ) {
        let mut open_orders = Vec::new();

        for bid in orderbook.bids.iter() {
            if bid.user_id == payload.user_id {
                open_orders.push(bid.clone());
            }
        }

        for ask in orderbook.asks.iter() {
            if ask.user_id == payload.user_id {
                open_orders.push(ask.clone());
            }
        }

        let redis_manager = RedisManager::instance();
        let message = MessageToApi::OpenOrders {
            payload: OpenOrders {
                user_id: payload.user_id,
                market: payload.market,
            },
        };

        let _ = redis_manager.send_to_api(&client_id, &message);
    }

    fn handle_get_quote(
        orderbook: &Orderbook,
        client_id: String,
        quantity: Decimal,
        side: OrderSide,
    ) {
        let quote = orderbook.get_quote_detail(quantity, side);

        let redis_manager = RedisManager::instance();
        let message = MessageToApi::Quote { payload: quote };
        let _ = redis_manager.send_to_api(&client_id, &message);
    }
}
