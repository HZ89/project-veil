pub mod api;
pub mod state;
pub mod websocket;

use axum::{
    routing::{any, delete, get, post},
    serve, Router,
};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use self::api::user as user_api;
use self::state::{AppState, UserState};
use self::websocket::ws_handler;

pub async fn run_server() {
    tracing_subscriber::registry()
        .with(EnvFilter::new(
            // Use imported EnvFilter directly
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "veil_server=debug,tower_http=debug".into()),
        ))
        .with(fmt::layer()) // Use imported fmt
        .init();

    let (tx, _rx) = broadcast::channel(100);

    let app_state = AppState {
        user_state: Arc::new(Mutex::new(UserState::default())), // Use imported Mutex
        tx: Arc::new(tx),
    };

    let app = Router::new()
        .route("/ws", any(ws_handler).with_state(app_state.clone()))
        .route(
            "/users",
            post(user_api::create_user)
                .get(user_api::list_users)
                .with_state(app_state.clone()),
        )
        .route(
            "/users/:id",
            delete(user_api::delete_user).with_state(app_state.clone()),
        )
        .with_state(app_state.clone())
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("Server listening on {}", addr);

    serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}
