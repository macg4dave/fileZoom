use ratatui::{layout::Rect, widgets::{Block, Paragraph, Borders}, Frame};
use crate::ui::{UIState, Theme};

pub fn render(f: &mut Frame, area: Rect, state: &UIState, _theme: &Theme) {
    let content = format!("Progress: {}% | {} items", state.progress, state.left_list.len());
    let p = Paragraph::new(content).block(Block::default().borders(Borders::ALL));
    f.render_widget(p, area);
}
