use ratatui::layout::Rect;
use ratatui::text::{Span, Line};
use ratatui::widgets::{Block, Tabs};
use ratatui::Frame;

use crate::app::App;
use crate::ui::colors::current as theme_current;

/// Return the ordered labels used for the top menu.
pub fn menu_labels() -> Vec<&'static str> {
    vec!["File", "Copy", "Move", "New", "Sort", "Help"]
}

/// Draw the top menu bar. The menu is currently static and non-interactive.
pub fn draw_menu(f: &mut Frame, area: Rect, _app: &App) {
    let labels = menu_labels();
    let theme = theme_current();

    // Use Tabs to render a top menu with a highlighted first tab (static)
    let titles: Vec<Line> = labels
        .iter()
        .map(|t| Line::from(Span::styled(*t, theme.help_block_style)))
        .collect();
    let tabs = Tabs::new(titles).select(0).block(Block::default());
    f.render_widget(tabs, area);
}
