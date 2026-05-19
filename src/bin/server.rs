use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::sync::broadcast::{Sender, channel};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

#[derive(Clone)]
struct User {
    id: SocketAddr,
    nick: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ChatMessage {
    id: String,
    from: String,
    message: String,
    time: u128,
    reactions: HashMap<String, u32>,
    reacted_by: HashMap<String, Vec<String>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClientMessage {
    message_type: String,
    data: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ServerMessage {
    message_type: String,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReactionPayload {
    message_id: String,
    emoji: String,
}

#[derive(Default)]
struct ChatState {
    users: Vec<User>,
    messages: Vec<ChatMessage>,
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

fn serialize_server_message(message: ServerMessage) -> String {
    serde_json::to_string(&message).expect("server message should serialize")
}

fn users_message(users: &[User]) -> String {
    serialize_server_message(ServerMessage {
        message_type: "users".to_string(),
        data_array: Some(users.iter().map(|user| user.nick.clone()).collect()),
        data: None,
    })
}

fn data_message(message_type: &str, data: String) -> String {
    serialize_server_message(ServerMessage {
        message_type: message_type.to_string(),
        data_array: None,
        data: Some(data),
    })
}

async fn broadcast_users(state: Arc<Mutex<ChatState>>, bcast_tx: &Sender<String>) {
    let users_message = {
        let state = state.lock().await;
        users_message(&state.users)
    };
    let _ = bcast_tx.send(users_message);
}

async fn register_user(
    addr: SocketAddr,
    nick: String,
    state: Arc<Mutex<ChatState>>,
    bcast_tx: &Sender<String>,
) {
    let users_message = {
        let mut state = state.lock().await;
        state
            .users
            .retain(|user| user.id != addr && user.nick != nick);
        state.users.push(User { id: addr, nick });
        users_message(&state.users)
    };

    let _ = bcast_tx.send(users_message);
}

async fn broadcast_chat_message(
    addr: SocketAddr,
    message: String,
    state: Arc<Mutex<ChatState>>,
    bcast_tx: &Sender<String>,
) {
    let outgoing = {
        let mut state = state.lock().await;
        let Some(sender) = state.users.iter().find(|user| user.id == addr) else {
            return;
        };

        let chat_message = ChatMessage {
            id: format!("{}-{}", now_ms(), addr.port()),
            from: sender.nick.clone(),
            message,
            time: now_ms(),
            reactions: HashMap::new(),
            reacted_by: HashMap::new(),
        };
        let data = serde_json::to_string(&chat_message).expect("chat message should serialize");
        state.messages.push(chat_message);
        data_message("message", data)
    };

    let _ = bcast_tx.send(outgoing);
}

async fn broadcast_reaction(
    addr: SocketAddr,
    payload: ReactionPayload,
    state: Arc<Mutex<ChatState>>,
    bcast_tx: &Sender<String>,
) {
    let outgoing = {
        let mut state = state.lock().await;
        let Some(reactor) = state.users.iter().find(|user| user.id == addr) else {
            return;
        };
        let reactor_nick = reactor.nick.clone();
        let Some(message) = state
            .messages
            .iter_mut()
            .find(|message| message.id == payload.message_id)
        else {
            return;
        };

        if message.from == reactor_nick {
            return;
        }

        let existing_reactors = message
            .reacted_by
            .entry(payload.emoji.clone())
            .or_insert_with(Vec::new);
        if existing_reactors.contains(&reactor_nick) {
            return;
        }

        existing_reactors.push(reactor_nick);
        let count = message.reactions.entry(payload.emoji.clone()).or_insert(0);
        *count += 1;

        let data = serde_json::json!({
            "messageId": message.id,
            "emoji": payload.emoji,
            "count": count,
        });
        data_message("reaction", data.to_string())
    };

    let _ = bcast_tx.send(outgoing);
}

async fn handle_client_message(
    addr: SocketAddr,
    text: &str,
    state: Arc<Mutex<ChatState>>,
    bcast_tx: &Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let client_message: ClientMessage = serde_json::from_str(text)?;

    match client_message.message_type.as_str() {
        "register" => {
            if let Some(nick) = client_message.data {
                register_user(addr, nick, state, bcast_tx).await;
            }
        }
        "message" => {
            if let Some(message) = client_message.data {
                if !message.trim().is_empty() {
                    broadcast_chat_message(addr, message, state, bcast_tx).await;
                }
            }
        }
        "reaction" => {
            if let Some(data) = client_message.data {
                let payload: ReactionPayload = serde_json::from_str(&data)?;
                broadcast_reaction(addr, payload, state, bcast_tx).await;
            }
        }
        _ => {}
    }

    Ok(())
}

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
    state: Arc<Mutex<ChatState>>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut bcast_rx = bcast_tx.subscribe();

    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            if let Err(err) = handle_client_message(addr, text, state.clone(), &bcast_tx).await {
                                eprintln!("Could not handle message from {}: {}", addr, err);
                            }
                        }
                    }
                    Some(Err(err)) => return Err(err.into()),
                    None => break,
                }
            }
            msg = bcast_rx.recv() => {
                if let Ok(text) = msg {
                    ws_stream.send(Message::text(text)).await?;
                }
            }
        }
    }

    {
        let mut state = state.lock().await;
        state.users.retain(|user| user.id != addr);
    }
    broadcast_users(state, &bcast_tx).await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(100);
    let state = Arc::new(Mutex::new(ChatState::default()));
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Rust YewChat websocket server listening on 127.0.0.1:8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {:?}", addr);
        let bcast_tx = bcast_tx.clone();
        let state = state.clone();

        tokio::spawn(async move {
            let Ok((_req, ws_stream)) = ServerBuilder::new().accept(socket).await else {
                eprintln!("Could not accept websocket connection from {}", addr);
                return;
            };

            if let Err(err) = handle_connection(addr, ws_stream, bcast_tx, state).await {
                eprintln!("Connection error from {}: {}", addr, err);
            }
        });
    }
}
