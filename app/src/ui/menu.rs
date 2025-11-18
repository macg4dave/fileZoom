use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Paragraph};
use tui::Frame;

use crate::app::App;

/// Return the ordered labels used for the top menu.
pub fn menu_labels() -> Vec<&'static str> {
    vec!["File", "Copy", "Move", "New", "Sort", "Help"]
}

/// Draw the top menu bar. The menu is currently static and non-interactive.
pub fn draw_menu<B: Backend>(f: &mut Frame<B>, area: Rect, _app: &App) {
    let menu_items = menu_labels();
    let mut parts: Vec<Span> = Vec::new();
    for (i, it) in menu_items.iter().enumerate() {
        if i > 0 {
            parts.push(Span::raw("  "));
        }
        parts.push(Span::styled(
            *it,
            Style::default().fg(Color::Black).bg(Color::White),
        ));
    }
    let spans = vec![Spans::from(parts)];
    let menu = Paragraph::new(spans).block(Block::default());
    f.render_widget(menu, area);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_labels_expected() {
        let labels = menu_labels();
        assert_eq!(labels, vec!["File", "Copy", "Move", "New", "Sort", "Help"]);
    }
}
