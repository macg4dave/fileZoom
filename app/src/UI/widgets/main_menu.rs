use ratatui::{layout::Rect, widgets::{Block, Paragraph, Borders}, Frame};
use crate::ui::{UIState, Theme};

pub fn render(f: &mut Frame, area: Rect, _state: &UIState, _theme: &Theme) {
    let p = Paragraph::new("Main menu — (placeholder)").block(Block::default().borders(Borders::ALL).title("Menu"));
    f.render_widget(p, area);
}
