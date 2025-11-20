use ratatui::text::{Span, Line};
use ratatui::text::Text;
use ratatui::widgets::ListItem;

use crate::ui::colors::current as theme_current;

/// Render a header row displaying the full path for a panel.
pub fn render_header(path_display: &str) -> ListItem<'_> {
    let theme = theme_current();
    let text = format!("> {}", path_display);
    let style = theme.header_style;
    ListItem::new(Text::from(Line::from(vec![Span::styled(text, style)])))
}
