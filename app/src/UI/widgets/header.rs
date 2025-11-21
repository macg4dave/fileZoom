use ratatui::{layout::Rect, widgets::{Paragraph, Block, Borders}, text::Span};
use ratatui::Frame;
use crate::ui::{UIState, Theme};

pub fn render(f: &mut Frame, area: Rect, _state: &UIState, theme: &Theme) {
    let p = Paragraph::new(Span::raw(" fileZoom — left/right panels | adaptive UI "))
        .block(Block::default().borders(Borders::ALL).title(" header "))
        .style(theme.style_fg());
    f.render_widget(p, area);
}
