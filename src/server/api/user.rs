use crate::server::state::{AppState, CreateUserPayload, User};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
// Import state structs

// --- User CRUD Handlers ---

// Create a new user
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserPayload>,
) -> impl IntoResponse {
    let mut user_state = state.user_state.lock().unwrap();
    let user_id = user_state.next_user_id;
    user_state.next_user_id += 1;
    user_state.users.insert(user_id, payload.username.clone());

    let new_user = User {
        id: user_id,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(new_user))
}

// List all users
pub async fn list_users(State(state): State<AppState>) -> impl IntoResponse {
    let user_state = state.user_state.lock().unwrap();
    let users: Vec<User> = user_state
        .users
        .iter()
        .map(|(id, username)| User {
            id: *id,
            username: username.clone(),
        })
        .collect();
    Json(users)
}

// Delete a user
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<usize>,
) -> impl IntoResponse {
    let mut user_state = state.user_state.lock().unwrap();
    if user_state.users.remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
