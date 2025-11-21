use ratatui::{layout::Rect, widgets::{Block, Paragraph, Borders}, Frame};
use crate::ui::{UIState, Theme};

pub fn render(f: &mut Frame, area: Rect, state: &UIState, _theme: &Theme) {
    let text = state.preview_text.clone().unwrap_or_else(|| "(no preview)".into());
    let p = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Preview"));
    f.render_widget(p, area);
}
