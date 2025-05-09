use anyhow::Result;
use redis::Client as RedisClient;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;

use crate::models::{ServerMessage, UserSubscription};

#[derive(Clone)]
pub struct AppState {
    pub inner: Arc<AppStateInner>,
}

pub struct AppStateInner {
    pub subscriptions: Mutex<HashMap<String, UserSubscription>>,
    pub redis_client: RedisClient,
    pub tx: broadcast::Sender<ServerMessage>,
}

impl AppState {
    pub async fn new(redis_client: RedisClient) -> Result<Self> {
        let (tx, _rx) = broadcast::channel::<ServerMessage>(100);

        let inner = Arc::new(AppStateInner {
            subscriptions: Mutex::new(HashMap::new()),
            redis_client: redis_client.clone(),
            tx: tx.clone(),
        });

        let app_state = Self { inner };

        let app_state_clone = app_state.clone();
        tokio::spawn(async move {
            let mut conn = app_state_clone.inner.redis_client.get_connection().unwrap();
            let mut pubsub = conn.as_pubsub();
            pubsub.psubscribe("*").unwrap();

            let pubsub_stream = pubsub.get_message();

            while let Some(msg) = pubsub_stream.iter().next() {
                let payload: String = msg.get_payload().unwrap();

                if let Ok(server_message) = serde_json::from_str::<ServerMessage>(&payload) {
                    let _ = app_state_clone.inner.tx.send(server_message);
                }
            }
        });

        Ok(app_state)
    }
}
