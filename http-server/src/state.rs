use std::sync::Arc;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::{config::Config, services::redis_manager::RedisManager};

#[derive(Clone)]
pub struct AppState {
    pub env: Config,
    pub redis_manager: Arc<RedisManager>,
}

impl AppState {
    pub async fn new() -> Self {
        let config = Config::init();

        Self {
            env: config,
            redis_manager: Arc::new(RedisManager::new()),
        }
    }
}
