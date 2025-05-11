use std::sync::Arc;

use sqlx::PgPool;

use crate::services::RedisManager;

#[derive(Clone)]
pub struct AppState {
    pub redis_manager: Arc<RedisManager>,
    pub db_pool: Arc<PgPool>,
}

impl AppState {
    pub async fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect(&database_url).await.unwrap();
        Self {
            redis_manager: Arc::new(RedisManager::new()),
            db_pool: Arc::new(pool),
        }
    }
}
