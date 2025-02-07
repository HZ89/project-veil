use futures::stream::SplitSink;
use lazy_static::lazy_static;
use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
// Import MaybeTlsStream
use tokio_tungstenite::tungstenite::Message;

// --- App State and Data Structures --

pub struct App {
    pub input: String,
    pub messages: Vec<String>,
    pub status: String,
    pub user_list: Vec<String>,
    pub ws_tx: Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
    pub http_client: Client,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            messages: Vec::new(),
            status: "Not connected".to_string(),
            user_list: Vec::new(),
            ws_tx: None,
            http_client: Client::new(),
        }
    }
}

// --- Data Structures for HTTP Responses ---

#[derive(Deserialize, Debug)]
pub struct UserResponse {
    pub id: usize,
    pub username: String,
}

// --- Global App State (Mutex for thread safety) ---
lazy_static! {
    pub static ref APP_STATE: Arc<Mutex<App>> = Arc::new(Mutex::new(App::default()));
}
