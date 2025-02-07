use crate::client::app_state::{App, UserResponse};
use serde_json::json;
// Import App and APP_STATE
use std::error::Error;

impl App {
    pub async fn fetch_user_list(&mut self) -> Result<(), Box<dyn Error>> {
        let response = self
            .http_client
            .get("http://localhost:3000/users")
            .send()
            .await?;
        if response.status().is_success() {
            let users: Vec<UserResponse> = response.json().await?;
            self.user_list = users.iter().map(|u| u.username.clone()).collect();
            self.status = "User list updated.".to_string();
        } else {
            self.status = format!("Failed to fetch user list: {}", response.status());
        }
        Ok(())
    }

    pub async fn create_user(&mut self, username: &str) -> Result<(), Box<dyn Error>> {
        let response = self
            .http_client
            .post("http://localhost:3000/users")
            .json(&json!({"username": username}))
            .send()
            .await?;
        if response.status().is_success() {
            self.status = format!("User '{}' created successfully.", username);
            self.fetch_user_list().await?; // Refresh user list after creating user
        } else {
            self.status = format!(
                "Failed to create user '{}': {}",
                username,
                response.status()
            );
        }
        Ok(())
    }

    pub async fn delete_user(&mut self, user_id: usize) -> Result<(), Box<dyn Error>> {
        let response = self
            .http_client
            .delete(&format!("http://localhost:3000/users/{}", user_id))
            .send()
            .await?;
        if response.status().is_success() {
            self.status = format!("User ID '{}' deleted successfully.", user_id);
            self.fetch_user_list().await?; // Refresh user list after deleting user
        } else {
            self.status = format!(
                "Failed to delete user ID '{}': {}",
                user_id,
                response.status()
            );
        }
        Ok(())
    }
}
