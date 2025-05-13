use std::{collections::HashMap, sync::{Arc, Mutex}};

use anyhow::{Ok, Result};
use rust_decimal_macros::dec;
use tracing::{error, info};
use crate::{models::{Balance, MessageFromApi, MessageToApi, OrderCancelledPayload, OrderbookMessage, User, UserBalancesPayload}, services::RedisManager};

use super::{OrderbookWorker};

pub struct Engine {
    pub orderbook_workers: HashMap<String, OrderbookWorker>,
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
                    balance: dec!(100_000_000),
                    locked_balance: dec!(0),
                },
                Balance {
                    ticker: "SOL".to_string(),
                    balance: dec!(100_000_000),
                    locked_balance: dec!(0),
                },
            ],
        });

        initial_users.push(User {
            id: "2".to_string(),
            balances: vec![
                Balance {
                    ticker: "USDC".to_string(),
                    balance: dec!(100_000_000),
                    locked_balance: dec!(0),
                },
                Balance {
                    ticker: "SOL".to_string(),
                    balance: dec!(100_000_000),
                    locked_balance: dec!(0),
                },
            ],
        });

        let users = Arc::new(Mutex::new(initial_users));

        let mut engine = Engine {
            orderbook_workers: HashMap::new(),
            users,
        };

        engine.create_market("SOL".to_string(), "USDC".to_string()).unwrap();

        engine
    }

    pub fn create_market(&mut self, base_asset: String, quote_asset: String) -> Result<()> {
        let market = format!("{}_{}", base_asset, quote_asset);

        if self.orderbook_workers.contains_key(&market) {
            return Err(anyhow::anyhow!("Market already exists"));
        }

        let worker = OrderbookWorker::new(market.clone(), base_asset, quote_asset, Arc::clone(&self.users));

        self.orderbook_workers.insert(market, worker);

        Ok(())
    }

    pub fn process(&mut self, client_id: String, message: MessageFromApi) {
        match message {
            MessageFromApi::CreateOrder { data } => {
                if let Some(worker) = self.orderbook_workers.get(&data.market) {
                    if let Err(e) = worker.sender.send(OrderbookMessage::CreateOrder {
                        client_id,
                        payload: data,
                    }) {
                        error!("Failed to send create order message to worker: {}", e);
                    }
                } else {
                    error!("Market not found: {}", data.market);
                    let redis_manager = RedisManager::instance();
                    let message = MessageToApi::OrderCancelled {
                        payload: OrderCancelledPayload {
                            message: Some(String::from("Market not found")),
                        },
                    };
                    let _ = redis_manager.send_to_api(&client_id, &message);
                }
            },
            MessageFromApi::CancelOrder { data } => {
                if let Some(worker) = self.orderbook_workers.get(&data.market) {
                    if let Err(e) = worker.sender.send(OrderbookMessage::CancelOrder {
                        client_id,
                        payload: data,
                    }) {
                        error!("Failed to send cancel order message to worker: {}", e);
                    }
                } else {
                    error!("Market not found: {}", data.market);
                }
            },
            MessageFromApi::GetDepth { data } => {
                if let Some(worker) = self.orderbook_workers.get(&data.market) {
                    if let Err(e) = worker.sender.send(OrderbookMessage::GetDepth {
                        client_id,
                        market: data.market,
                    }) {
                        error!("Failed to send get depth message to worker: {}", e);
                    }
                } else {
                    error!("Market not found: {}", data.market);
                }
            },
            MessageFromApi::GetOpenOrders { data } => {
                if let Some(worker) = self.orderbook_workers.get(&data.market) {
                    if let Err(e) = worker.sender.send(OrderbookMessage::GetOpenOrders {
                        client_id,
                        payload: data,
                    }) {
                        error!("Failed to send get open orders message to worker: {}", e);
                    }
                } else {
                    error!("Market not found: {}", data.market);
                }
            },
            MessageFromApi::GetQuote { data } => {
                if let Some(worker) = self.orderbook_workers.get(&data.market) {
                    if let Err(e) = worker.sender.send(OrderbookMessage::GetQuote {
                        client_id,
                        market: data.market,
                        quantity: data.quantity,
                        side: data.side,
                    }) {
                        error!("Failed to send get quote message to worker: {}", e);
                    }
                } else {
                    error!("Market not found: {}", data.market);
                }
            },
            MessageFromApi::GetUserBalances { data } => {
                // User balances are not tied to a specific orderbook, so we handle this here
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
            },
        }
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        for (market, worker) in self.orderbook_workers.iter_mut() {
            info!("Shutting down orderbook worker for market: {}", market);
            if let Err(e) = worker.sender.send(OrderbookMessage::ShutDown) {
                error!("Failed to send shutdown message to worker: {}", e);
            }
            
            // Join the thread
            if let Some(thread_handle) = worker.thread_handle.take() {
                if let Err(e) = thread_handle.join() {
                    error!("Failed to join orderbook worker thread: {:?}", e);
                }
            }
        }
    }
}