pub mod api_client;
pub mod app_state;
pub mod tui;
pub mod websocket;

use crate::client::app_state::App;
use crate::client::app_state::APP_STATE;
use crate::client::tui::ui;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{error::Error, io, time::Duration};
use tokio::runtime::Runtime;

pub fn run_client() -> Result<(), Box<dyn Error>> {
    // Setup tracing for logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_writer(io::stderr)
        .init();

    // Initialize terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Application loop
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = std::time::Instant::now();

    // Run tokio runtime for async operations
    let rt = Runtime::new()?;

    // Connect to WebSocket on startup (NON-BLOCKING - using tokio::spawn - simplified)
    rt.spawn(async move {
        // Spawn a separate async task for connection
        let connect_result = App::default()
            .connect_websocket("ws://localhost:3000/ws".to_string())
            .await; // Try connection *without* initial lock

        // Now, *after* the connection attempt, update status using lock
        if let Err(e) = connect_result {
            let mut app = APP_STATE.lock().unwrap(); // Re-acquire lock to update status
            app.status = format!("WebSocket connection failed on startup: {}", e);
            drop(app); // Release lock after status update
        } else {
            let mut app = APP_STATE.lock().unwrap(); // Re-acquire lock to update status on success
            app.status = "WebSocket connected!".to_string(); // Update status on success
            drop(app);
        }
    }); // Connection is now attempted in a background task

    loop {
        terminal.draw(|f| ui(f))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                let mut app = APP_STATE.lock().unwrap();
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Enter => {
                        if !app.input.is_empty() {
                            let input_clone = app.input.clone();
                            if app.input.starts_with('/') {
                                rt.block_on(app.process_command(&input_clone));
                            } else if app.ws_tx.is_some() {
                                rt.block_on(app.send_message())?;
                            } else {
                                app.status =
                                    "Not connected to WebSocket. Cannot send message.".to_string();
                            }
                            app.input.clear();
                        }
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    _ => {}
                }
                if key.code == KeyCode::Char('u') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    // Example: Ctrl+U to refresh user list (now directly in main thread)
                    if let Err(e) = rt.block_on(app.fetch_user_list()) {
                        app.status = format!("Error fetching user list: {}", e);
                    }
                }
                drop(app);
            }
        }
        if last_tick.elapsed() >= tick_rate {
            let mut app = APP_STATE.lock().unwrap();
            if app.status == "Fetching user list..." {
                if let Err(e) = rt.block_on(app.fetch_user_list()) {
                    app.status = format!("Error fetching user list: {}", e);
                } else {
                    app.status = "User list fetched.".to_string();
                }
            }
            drop(app);
            last_tick = std::time::Instant::now();
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

impl App {
    pub async fn process_command(&mut self, command: &str) {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if let Some(&cmd) = parts.first() {
            match cmd {
                "/users" => {
                    // No tokio::spawn needed - fetch_user_list is now called directly in main thread
                    self.status = "Fetching user list...".to_string();
                }
                "/create_user" if parts.len() > 1 => {
                    let username = parts[1..].join(" ");
                    let username_clone = username.clone();
                    // No local runtime - call create_user and await directly
                    if let Err(e) = self.create_user(&username_clone).await {
                        self.status = format!("Error creating user: {}", e);
                    }
                    self.status = format!("Creating user '{}'...", username);
                }
                "/delete_user" if parts.len() == 2 => {
                    if let Ok(user_id) = parts[1].parse::<usize>() {
                        let user_id_clone = user_id;
                        // No local runtime - call delete_user and await directly
                        if let Err(e) = self.delete_user(user_id_clone).await {
                            self.status = format!("Error deleting user: {}", e);
                        }
                        self.status = format!("Deleting user ID '{}'...", user_id);
                    } else {
                        self.status = "Invalid user ID format.".to_string();
                    }
                }
                _ => {
                    self.status = format!("Unknown command: '{}'", command);
                }
            }
        }
    }
}
