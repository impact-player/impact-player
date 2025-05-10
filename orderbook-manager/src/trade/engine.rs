use std::sync::{Arc, Mutex};

use anyhow::{Ok, Result};
use chrono::Utc;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde_json::json;
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    models::{
        AddTradePayload, Balance, CancelOrderPayload, CreateOrderPayload, MessageFromApi,
        MessageToApi, OpenOrdersPayload, Order, OrderCancelledPayload, OrderPlacedPayload,
        OrderSide, TradeData, User, UserBalancesPayload,
    },
    services::RedisManager,
};

use super::Orderbook;

pub struct Engine {
    pub orderbooks: Arc<Mutex<Vec<Orderbook>>>,
    pub users: Arc<Mutex<Vec<User>>>,
}

impl Engine {
    pub fn new() -> Self {
        let mut initial_users = Vec::new();

        initial_users.push(User {
            id: "1".to_string(),
            balances: vec![
                Balance {
                    ticker: "USDC".to_string(),
                    balance: dec!(10_000),
                    locked_balance: dec!(0),
                },
                Balance {
                    ticker: "SOL".to_string(),
                    balance: dec!(10),
                    locked_balance: dec!(0),
                },
            ],
        });

        initial_users.push(User {
            id: "1".to_string(),
            balances: vec![
                Balance {
                    ticker: "USDC".to_string(),
                    balance: dec!(10_000),
                    locked_balance: dec!(0),
                },
                Balance {
                    ticker: "SOL".to_string(),
                    balance: dec!(10),
                    locked_balance: dec!(0),
                },
            ],
        });

        Engine {
            orderbooks: Arc::new(Mutex::new(vec![Orderbook::new(
                "SOL".to_string(),
                "USDC".to_string(),
            )])),
            users: Arc::new(Mutex::new(initial_users)),
        }
    }

    pub fn process(&mut self, client_id: String, message: MessageFromApi) {
        match message {
            MessageFromApi::CreateOrder { data } => {
                let result = self.create_order(&data);

                match result {
                    Result::Ok((remaining_qty, filled_qty, order_id)) => {
                        info!(
                            order_id,
                            ?remaining_qty,
                            ?filled_qty,
                            "Order created successfully"
                        );
                        let redis_manager = RedisManager::instance();
                        let message = MessageToApi::OrderPlaced {
                            payload: OrderPlacedPayload {
                                order_id,
                                remaining_qty,
                                filled_qty,
                            },
                        };

                        let _ = redis_manager.send_to_api(&client_id, &message);
                        let trade_info = json!({
                            "price": data.price,
                            "quantity": filled_qty,
                            "side": data.side,
                            "timestamp": Utc::now().timestamp()
                        });

                        let db_info = AddTradePayload {
                            data: TradeData {
                                ticker: data.market.clone(),
                                time: Utc::now(),
                                price: data.price,
                                quantity: data.quantity,
                            },
                        };

                        let _ = redis_manager
                            .publish_message(&format!("trade@{}", data.market), &trade_info);
                        let _ = redis_manager.push_message_to_db(&db_info);
                    }
                    Err(e) => {
                        error!("Failed to create order: {}", e);
                        let redis_manager = RedisManager::instance();
                        let message = MessageToApi::OrderCancelled {
                            payload: OrderCancelledPayload {
                                message: Some(String::from("Order execution failed")),
                            },
                        };

                        let _ = redis_manager.send_to_api(&client_id, &message);
                    }
                }
            }
            MessageFromApi::CancelOrder { data } => {
                self.cancel_order(&data).unwrap();

                info!(order_id = ?data.order_id, "Order cancelled successfully");
                let redis_manager = RedisManager::instance();
                let message = MessageToApi::OrderCancelled {
                    payload: OrderCancelledPayload {
                        message: Some(String::from("ORDER CANCELLED")),
                    },
                };

                let _ = redis_manager.send_to_api(&client_id, &message);
            }
            MessageFromApi::GetDepth { data } => {
                let market: Vec<&str> = data.market.split('_').collect();
                let base_asset = market[0];
                let quote_asset = market[1];
                let mut orderbooks = self.orderbooks.lock().unwrap();

                let orderbook = orderbooks
                    .iter_mut()
                    .find(|ob| ob.base_asset == base_asset && ob.quote_asset == quote_asset)
                    .ok_or_else(|| anyhow::anyhow!("Market not found"))
                    .expect("Market not found");

                let depth = orderbook.get_depth();

                let redis_manager = RedisManager::instance();
                let message = MessageToApi::Depth { payload: depth };
                let _ = redis_manager.send_to_api(&client_id, &message);
            }
            MessageFromApi::GetOpenOrders { data } => {
                let market: Vec<&str> = data.market.split('_').collect();
                let base_asset = market[0];
                let quote_asset = market[1];
                let mut orderbooks = self.orderbooks.lock().unwrap();

                let orderbook = orderbooks
                    .iter_mut()
                    .find(|ob| ob.base_asset == base_asset && ob.quote_asset == quote_asset)
                    .ok_or_else(|| anyhow::anyhow!("Market not found"))
                    .expect("Market not found");

                let mut open_orders = Vec::new();

                for bid in orderbook.bids.iter() {
                    if bid.user_id == data.user_id {
                        open_orders.push(bid.clone());
                    }
                }

                for ask in orderbook.asks.iter() {
                    if ask.user_id == data.user_id {
                        open_orders.push(ask.clone());
                    }
                }

                let redis_manager = RedisManager::instance();
                let message = MessageToApi::OpenOrder {
                    payload: OpenOrdersPayload { open_orders },
                };

                let _ = redis_manager.send_to_api(&client_id, &message);
            }
            MessageFromApi::GetQuote { data } => {
                let market: Vec<&str> = data.market.split('_').collect();
                let base_asset = market[0];
                let quote_asset = market[1];
                let mut orderbooks = self.orderbooks.lock().unwrap();

                let orderbook = orderbooks
                    .iter_mut()
                    .find(|ob| ob.base_asset == base_asset && ob.quote_asset == quote_asset)
                    .ok_or_else(|| anyhow::anyhow!("Market not found"))
                    .expect("Market not found");

                let quote = orderbook.get_quote_detail(data.quantity, data.side);

                let redis_manager = RedisManager::instance();
                let message = MessageToApi::Quote { payload: quote };
                let _ = redis_manager.send_to_api(&client_id, &message);
            }
            MessageFromApi::GetUserBalances { data } => {
                let mut users = self.users.lock().unwrap();
                let user = users
                    .iter_mut()
                    .find(|u| u.id == data.user_id)
                    .expect("User not found");

                let redis_manager = RedisManager::instance();
                let message = MessageToApi::UserBalances {
                    payload: UserBalancesPayload {
                        balances: user.balances.clone(),
                    },
                };

                let _ = redis_manager.send_to_api(&client_id, &message);
            }
        }
    }

    fn create_order(&mut self, payload: &CreateOrderPayload) -> Result<(Decimal, Decimal, String)> {
        let order_id = Uuid::new_v4().to_string();

        if !self.validate_balance(&payload) {
            error!("Insufficient balance for trade");
            return Err(anyhow::anyhow!("Insufficient balance for trade"));
        }

        let market: Vec<&str> = payload.market.split('_').collect();
        let base_asset = market[0];
        let quote_asset = market[1];
        let mut orderbooks = self.orderbooks.lock().unwrap();

        let orderbook = orderbooks
            .iter_mut()
            .find(|ob| ob.base_asset == base_asset && ob.quote_asset == quote_asset)
            .ok_or_else(|| anyhow::anyhow!("Market not found"))?;

        let remaining_qty =
            orderbook.fill_orders(payload, &mut self.users, base_asset, quote_asset);

        if remaining_qty == Decimal::ZERO {
            return Ok((Decimal::ZERO, payload.quantity, order_id.clone()));
        }

        let new_order = Order {
            id: order_id.clone(),
            user_id: payload.user_id.clone(),
            price: payload.price,
            quantity: payload.quantity,
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

        let filled_qty = payload.quantity.checked_sub(remaining_qty).unwrap();

        Ok((remaining_qty, filled_qty, order_id))
    }

    fn cancel_order(&mut self, payload: &CancelOrderPayload) -> Result<()> {
        let market: Vec<&str> = payload.market.split('_').collect();
        let base_asset = market[0];
        let quote_asset = market[1];
        let mut orderbooks = self.orderbooks.lock().unwrap();

        let orderbook = orderbooks
            .iter_mut()
            .find(|ob| ob.base_asset == base_asset && ob.quote_asset == quote_asset)
            .ok_or_else(|| anyhow::anyhow!("Market not found"))?;

        if let Some(bid_index) = orderbook
            .bids
            .iter()
            .position(|order| order.id == payload.order_id)
        {
            let order = &orderbook.bids[bid_index];

            let mut users = self.users.lock().unwrap();
            if let Some(user) = users.iter_mut().find(|u| u.id == order.user_id) {
                if let Some(balance) = user.balances.iter_mut().find(|b| b.ticker == quote_asset) {
                    let locked_amount = order.price * order.quantity;
                    balance.locked_balance -= locked_amount;
                }
            }

            let _ = orderbook.bids.remove(bid_index);
            return Ok(());
        }

        if let Some(ask_index) = orderbook
            .asks
            .iter()
            .position(|order| order.id == payload.order_id)
        {
            let order = &orderbook.asks[ask_index];

            let mut users = self.users.lock().unwrap();
            if let Some(user) = users.iter_mut().find(|u| u.id == order.user_id) {
                if let Some(balance) = user.balances.iter_mut().find(|b| b.ticker == base_asset) {
                    balance.locked_balance -= order.quantity;
                }
            }

            let _ = &orderbook.asks.remove(ask_index);
            return Ok(());
        }

        Ok(())
    }

    fn validate_balance(&self, payload: &CreateOrderPayload) -> bool {
        let mut users = self.users.lock().unwrap();

        let user = users
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
                    .map(|b| b.balance)
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
    }
}
