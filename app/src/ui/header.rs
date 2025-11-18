use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::ListItem;

/// Render a header row displaying the full path for a panel.
pub fn render_header(path_display: &str) -> ListItem {
    let text = format!("{}", path_display);
    let style = Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD);
    ListItem::new(Spans::from(vec![Span::styled(text, style)]))
}
