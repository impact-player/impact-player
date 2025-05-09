use anyhow::Result;
use constant::MESSAGE_FROM_API_CHANNEL;
use models::IncomingMessage;
use redis::Commands;
use services::RedisManager;
use trade::Engine;

mod constant;
mod models;
mod services;
mod trade;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mut engine = Engine::new();

    let redis_manager = RedisManager::new();
    let mut conn = redis_manager.get_connection()?;

    loop {
        let response: Option<(String, String)> = conn.brpop(MESSAGE_FROM_API_CHANNEL, 0.0)?;

        if let Some((_, message)) = response {
            let parsed_message: IncomingMessage = serde_json::from_str(&message)?;

            engine.process(parsed_message.client_id, parsed_message.message);
        }
    }
}
