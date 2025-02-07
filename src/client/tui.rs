use crate::client::app_state::APP_STATE;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn ui(f: &mut Frame) {
    let app_ref = APP_STATE.lock().unwrap();
    let app = &*app_ref;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
                Constraint::Length(5),
            ]
            .as_ref(),
        )
        .split(f.size());

    let status_bar = Paragraph::new(Line::from(vec![
        Span::styled("Status: ", Style::default().fg(Color::Yellow)),
        Span::raw(&app.status),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Status"));
    f.render_widget(status_bar, chunks[0]);

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .map(|m| ListItem::new(Line::from(Span::raw(m))))
        .collect();

    let history =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Chat History"));
    f.render_widget(history, chunks[1]);

    // Type hint added here: `Paragraph::new(app.input.as_str())` - using `.as_str()` to get &str
    let input_bar = Paragraph::new(app.input.as_str()) // Explicitly use .as_str() to get &str
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input_bar, chunks[2]);

    let user_list_items: Vec<ListItem> = app
        .user_list
        .iter()
        .map(|user| ListItem::new(Line::from(Span::raw(user))))
        .collect();
    let user_list_widget =
        List::new(user_list_items).block(Block::default().borders(Borders::ALL).title("Users"));
    f.render_widget(user_list_widget, chunks[3]);

    drop(app_ref);
}
