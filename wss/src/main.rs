use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::State,
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{stream::StreamExt, sink::SinkExt};
use redis::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::{broadcast, Mutex};

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "UPPERCASE")]
enum ClientRequest {
    Subscribe { payload: SubscribePayload },
    Unsubscribe { payload: UnsubscribePayload },
}

#[derive(Debug, Deserialize)]
struct SubscribePayload {
    room: String,
}

#[derive(Debug, Deserialize)]
struct UnsubscribePayload {
    room: String,
}

#[derive(Debug, Serialize)]
struct ServerMessage {
    room: String,
    data: String,
}

struct AppState {
    channels: Mutex<HashMap<String, ChannelInfo>>,
    redis_client: redis::Client,
}

struct ChannelInfo {
    sender: broadcast::Sender<String>,
    subscribers: usize,
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

async fn handle_socket(socket: WebSocket, state: SharedState) {
    let (tx, mut rx) = socket.split();
    let tx = Arc::new(Mutex::new(tx));
    
    let mut subscribed_rooms = Vec::new();
    
    while let Some(Ok(msg)) = rx.next().await {
        if let Message::Text(text) = msg {
            match serde_json::from_str(&text) {
                Ok(request) => {
                    match request {
                        ClientRequest::Subscribe { payload } => {
                            let room = payload.room.clone();
                            subscribe_client(tx.clone(), state.clone(), payload.room.clone(), &mut subscribed_rooms).await;
                            println!("Client subscribed to room: {}", room);
                        },
                        ClientRequest::Unsubscribe { payload } => {
                            let room = payload.room.clone();
                            unsubscribe_client(state.clone(), payload.room, &mut subscribed_rooms).await;
                            println!("Client unsubscribed from room: {}", room);
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Failed to parse client message: {}", e);
                }
            }
        } else if let Message::Close(_) = msg {
            break;
        }
    }
    
    for room in subscribed_rooms {
        unsubscribe_from_room(state.clone(), room).await;
    }
}

async fn subscribe_client(
    tx: Arc<Mutex<futures::stream::SplitSink<WebSocket, Message>>>, 
    state: SharedState, 
    room: String,
    subscribed_rooms: &mut Vec<String>
) {
    let sender = {
        let mut channels = state.channels.lock().await;
        if let Some(info) = channels.get_mut(&room) {
            info.subscribers += 1;
            info.sender.clone()
        } else {
            let (s, _) = broadcast::channel(100);
            channels.insert(room.clone(), ChannelInfo {
                sender: s.clone(),
                subscribers: 1,
            });

            let client = state.redis_client.clone();
            start_redis_listener(room.clone(), client, s.clone());

            s
        }
    };

    let mut rx = sender.subscribe();
    
    if !subscribed_rooms.contains(&room) {
        subscribed_rooms.push(room.clone());
    }

    let room_clone = room.clone();
    tokio::spawn(async move {
        while let Ok(payload) = rx.recv().await {
            let msg = ServerMessage {
                room: room_clone.clone(),
                data: payload,
            };
            
            let mut tx_guard = tx.lock().await;
            if tx_guard
                .send(Message::Text(serde_json::to_string(&msg).unwrap().into()))
                .await
                .is_err()
            {
                break;
            }
        }
    });
}

async fn unsubscribe_client(state: SharedState, room: String, subscribed_rooms: &mut Vec<String>) {
    if let Some(pos) = subscribed_rooms.iter().position(|r| r == &room) {
        subscribed_rooms.remove(pos);
    }
    
    unsubscribe_from_room(state, room).await;
}

async fn unsubscribe_from_room(state: SharedState, room: String) {
    let mut should_cleanup = false;
    
    {
        let mut channels = state.channels.lock().await;
        if let Some(info) = channels.get_mut(&room) {
            info.subscribers -= 1;
            if info.subscribers == 0 {
                should_cleanup = true;
            }
        }
    }

    if should_cleanup {
        let mut channels = state.channels.lock().await;
        channels.remove(&room);
        println!("Removed channel for room: {} (no subscribers left)", room);
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
                    
                    if sender.receiver_count() == 0 {
                        println!("No subscribers left for {}, stopping listener", room);
                        break;
                    }
                    
                    let _ = sender.send(payload);
                }
                Err(err) => {
                    eprintln!("Failed to parse payload on {}: {}", room, err);
                }
            }
        }
        
        if let Err(err) = pubsub.unsubscribe(&room) {
            eprintln!("Redis unsubscribe error for {}: {}", room, err);
        } else {
            println!("Unsubscribed from Redis channel: {}", room);
        }
    });
}