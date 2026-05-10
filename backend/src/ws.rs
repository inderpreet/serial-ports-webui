use axum::{
    extract::{State, WebSocketUpgrade},
    response::Response,
};
use axum::extract::ws::{WebSocket, Message};
use futures::{SinkExt, StreamExt};
use crate::AppState;

pub async fn handler(State(state): State<AppState>, ws: WebSocketUpgrade) -> Response {
    let rx = state.ws_tx.subscribe();
    ws.on_upgrade(move |socket| handle_socket(socket, rx))
}

async fn handle_socket(socket: WebSocket, mut rx: tokio::sync::broadcast::Receiver<String>) {
    let (mut sender, mut receiver) = socket.split();

    let send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Close(_) = msg {
            break;
        }
    }

    send_task.abort();
}
