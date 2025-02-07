use crate::server::state::AppState;
// src/server/websocket.rs
use axum::extract::ws::{Utf8Bytes, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::IntoResponse;
use futures::{sink::SinkExt, stream::StreamExt};
// Import AppState

pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| websocket(socket, state))
}

async fn websocket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();

    tracing::debug!("New WebSocket connection established");

    let mut rx = state.tx.subscribe(); // Channel to receive broadcast messages

    // Spawn a task to handle sending messages to the client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender
                .send(axum::extract::ws::Message::Text(Utf8Bytes::from(msg)))
                .await
                .is_err()
            {
                break; // Connection closed
            }
        }
    });

    // Spawn a task to handle receiving messages from the client and broadcasting them
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let axum::extract::ws::Message::Text(text) = msg {
                tracing::debug!("Received message: {:?}", text);
                let _ = state.tx.send(text.parse().unwrap()); // Broadcast the message
            }
        }
    });

    // Wait for either task to complete (connection closed)
    tokio::select! {
        _ = (&mut send_task) => {
            tracing::debug!("Send task finished");
        },
        _ = (&mut recv_task) => {
            tracing::debug!("Receive task finished");
        },
    };

    tracing::debug!("WebSocket connection closed");
}
