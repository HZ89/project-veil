use crate::client::app_state::{App, APP_STATE};
// src/client/websocket.rs
use futures::stream::{SplitSink, SplitStream, StreamExt};
use std::{io::Error as IoError, sync::Arc, sync::Mutex, thread, time::Duration};
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use tokio::time::sleep;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{error::Error as WsError, Message},
    MaybeTlsStream,
    WebSocketStream, // Make sure MaybeTlsStream is imported
};
// Import App and APP_STATE
use futures::sink::SinkExt;
// Import SinkExt trait for .send()
// Removed incorrect import:
// use tokio_stream::wrappers::TcpStream as TokioTcpStream; // Import Tokio's TcpStream wrapper

impl App {
    pub async fn connect_websocket(&mut self, server_address: String) -> Result<(), WsError> {
        match connect_async(&server_address).await {
            Ok((ws_stream, _response)) => {
                self.status = format!("Connected to {}", server_address);
                tracing::info!("WebSocket handshake has been successfully completed");
                // Corrected type is already in place:
                let (ws_sender, ws_receiver) = ws_stream.split();
                self.ws_tx = Some(ws_sender);
                self.start_receiving_messages(ws_receiver);
                Ok(())
            }
            Err(err) => {
                self.status = format!("Connection error: {}", err);
                tracing::error!("Error during WebSocket handshake: {}", err);
                Err(err)
            }
        }
    }

    fn start_receiving_messages(
        &mut self,
        ws_receiver: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ) {
        let messages_clone = Arc::new(Mutex::new(self.messages.clone()));
        let status_clone = Arc::new(Mutex::new(self.status.clone()));
        let user_list_clone = Arc::new(Mutex::new(self.user_list.clone()));
        let app_messages_weak = Arc::downgrade(&messages_clone);
        let app_status_weak = Arc::downgrade(&status_clone);
        let app_user_list_weak = Arc::downgrade(&user_list_clone);

        thread::spawn(move || {
            let runtime = Runtime::new().unwrap();
            runtime.block_on(async move {
                let mut receiver = ws_receiver;
                while let Some(result_msg) = receiver.next().await {
                    match result_msg {
                        Ok(msg) => {
                            if let Message::Text(text_bytes) = msg {
                                if let Some(messages_arc) = app_messages_weak.upgrade() {
                                    let mut messages = messages_arc.lock().unwrap();
                                    // Corrected: Dereference &text_bytes to get &[u8]
                                    messages.push(
                                        String::from_utf8_lossy(text_bytes.as_ref()).into_owned(),
                                    );
                                    if messages.len() > 10 {
                                        messages.remove(0);
                                    }
                                    drop(messages);
                                }
                            }
                        }
                        Err(e) => {
                            tracing::error!("Error receiving message: {}", e);
                            if let Some(status_arc) = app_status_weak.upgrade() {
                                let mut status = status_arc.lock().unwrap();
                                *status = format!("WebSocket receive error: {}", e);
                                drop(status);
                            }
                            break;
                        }
                    }
                    sleep(Duration::from_millis(10)).await;
                }
                if let Some(status_arc) = app_status_weak.upgrade() {
                    let mut status = status_arc.lock().unwrap();
                    *status = "Disconnected from server.".to_string();
                    drop(status);
                }
                tracing::warn!("WebSocket receive task ended.");
            });
        });
    }

    pub async fn send_message(&mut self) -> Result<(), WsError> {
        if let Some(sender) = &mut self.ws_tx {
            let msg_text: String = self.input.drain(..).collect();
            // Corrected: Convert String to Utf8Bytes using .into()
            let msg = Message::Text(msg_text.into());
            sender.send(msg).await?;
        }
        Ok(())
    }
}
