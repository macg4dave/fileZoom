use ratatui::{layout::Rect, widgets::{Block, Paragraph, Borders}, Frame};
use crate::app::Panel;

pub fn draw_preview(f: &mut Frame, area: Rect, panel: &Panel) {
    let txt = if panel.preview.is_empty() { "(no preview)".to_string() } else { panel.preview.clone() };
    let p = Paragraph::new(txt).block(Block::default().borders(Borders::ALL).title("Preview"));
    f.render_widget(p, area);
}
