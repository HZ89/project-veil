use serde::{Deserialize, Serialize};
use std::{
    // Changed import here
    collections::HashMap,
    sync::{Arc, Mutex}, // Use std::sync::Mutex
};
use tokio::sync::broadcast;

// Shared state for managing users and WebSocket connections
#[derive(Debug, Clone)]
pub struct AppState {
    pub user_state: Arc<Mutex<UserState>>,  // Use std::sync::Mutex
    pub tx: Arc<broadcast::Sender<String>>, // Broadcast channel for chat messages
}

#[derive(Debug, Default, Clone)]
pub struct UserState {
    pub users: HashMap<usize, String>, // In-memory user storage (UserId -> Username)
    pub next_user_id: usize,
    pub clients: HashMap<usize, broadcast::Sender<String>>, // User ID to sender for private messages (future use)
}

// User representation for API requests
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: usize,
    pub username: String,
}

#[derive(Deserialize)]
pub struct CreateUserPayload {
    pub username: String,
}
