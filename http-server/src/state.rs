use std::sync::Arc;

use crate::services::RedisManager;

#[derive(Clone)]
pub struct AppState {
    pub redis_manager: Arc<RedisManager>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            redis_manager: Arc::new(RedisManager::new()),
        }
    }
}
