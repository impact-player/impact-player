use std::sync::{Arc, Mutex};

use rust_decimal_macros::dec;

use crate::models::{Balance, User};

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
}
