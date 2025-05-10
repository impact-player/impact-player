use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::State,
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::stream::StreamExt;
use redis::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::{broadcast, Mutex};

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "UPPERCASE")]
enum ClientRequest {
    Subscribe { payload: SubscribePayload },
}

#[derive(Debug, Deserialize)]
struct SubscribePayload {
    room: String,
}

#[derive(Debug, Serialize)]
struct ServerMessage {
    room: String,
    data: String,
}

struct AppState {
    channels: Mutex<HashMap<String, broadcast::Sender<String>>>,
    redis_client: redis::Client,
}

type SharedState = Arc<AppState>;

#[tokio::main]
async fn main() {
    let redis_client = redis::Client::open("redis://127.0.0.1/").unwrap();

    let state = Arc::new(AppState {
        channels: Mutex::new(HashMap::new()),
        redis_client,
    });

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    println!("WebSocket server listening on ws://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<SharedState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: SharedState) {
    while let Some(Ok(msg)) = socket.next().await {
        if let Message::Text(text) = msg {
            let request: ClientRequest = serde_json::from_str(&text).unwrap();
            let ClientRequest::Subscribe { payload } = request;
            subscribe_client(socket, state.clone(), payload.room).await;
            return;
        }
    }
}

async fn subscribe_client(mut socket: WebSocket, state: SharedState, room: String) {
    let sender = {
        let mut channels = state.channels.lock().await;
        if let Some(s) = channels.get(&room) {
            s.clone()
        } else {
            let (s, _) = broadcast::channel(100);
            channels.insert(room.clone(), s.clone());

            let client = state.redis_client.clone();
            start_redis_listener(room.clone(), client, s.clone());

            s
        }
    };

    let mut rx = sender.subscribe();

    while let Ok(payload) = rx.recv().await {
        let msg = ServerMessage {
            room: room.clone(),
            data: payload,
        };
        if socket
            .send(Message::Text(serde_json::to_string(&msg).unwrap().into()))
            .await
            .is_err()
        {
            break;
        }
    }
}

fn start_redis_listener(room: String, client: Client, sender: broadcast::Sender<String>) {
    std::thread::spawn(move || {
        let mut conn = match client.get_connection() {
            Ok(c) => c,
            Err(err) => {
                eprintln!("Redis conn error for {}: {}", room, err);
                return;
            }
        };

        let mut pubsub = conn.as_pubsub();
        if let Err(err) = pubsub.subscribe(&room) {
            eprintln!("Redis subscribe error for {}: {}", room, err);
            return;
        }
        println!("Listening to Redis channel: {}", room);

        loop {
            let msg = match pubsub.get_message() {
                Ok(m) => m,
                Err(err) => {
                    eprintln!("PubSub get_message error on {}: {:?}", room, err.kind());
                    break;
                }
            };

            match msg.get_payload::<String>() {
                Ok(payload) => {
                    let _ = sender.send(payload);
                }
                Err(err) => {
                    eprintln!("Failed to parse payload on {}: {}", room, err);
                }
            }
        }
    });
}
