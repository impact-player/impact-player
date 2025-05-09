use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ClientMessage {
    #[serde(rename = "SUBSCRIBE")]
    Subscribe { payload: PayloadInfo },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PayloadInfo {
    pub room: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerMessage {
    #[serde(rename = "type")]
    pub message_type: String,
    pub room: String,
}

pub struct UserSubscription {
    pub rooms: Vec<String>,
}

impl UserSubscription {
    pub fn new() -> Self {
        Self { rooms: Vec::new() }
    }
}
