use std::{sync::{mpsc, Arc, Mutex}, thread};

use crate::models::{OrderbookMessage, User};

use super::Orderbook;

struct OrderbookWorker {
    market: String,
    orderbook: Orderbook,
    users: Arc<Mutex<Vec<User>>>,
    sender: mpsc::Sender<OrderbookMessage>,
    thread_handle: Option<thread::JoinHandle<()>>
}

impl OrderbookWorker {
    fn new(
        market: String,
        base_asset: String,
        quote_asset: String,
        users: Arc<Mutex<Vec<User>>>
    ) -> Self {
        OrderbookWorker { market, orderbook: (), users: (), sender: (), thread_handle: () }
    }
}