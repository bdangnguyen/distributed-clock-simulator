use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
};

use crate::app::App;

pub fn setup_ui(frame: &mut Frame, _app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .split(frame.area());

    for chunk in chunks.iter() {
        let block = Block::default().borders(Borders::ALL);
        frame.render_widget(block, *chunk);
    }
}
