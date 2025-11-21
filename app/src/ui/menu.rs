use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};
use ratatui::Frame;

use crate::app::App;
use crate::ui::colors::current as theme_current;
use crate::ui::menu_model::MenuModel;

/// Return the ordered labels used for the top menu.
pub fn menu_labels() -> Vec<&'static str> {
    MenuModel::default_model().labels()
}

/// Draw the top menu bar. The menu is currently static and non-interactive.
/// Draw a combined header containing a small logo area, the menu tabs,
/// and a right-aligned status area. The header is responsive and will
/// allocate space from `area` using a horizontal layout.
pub fn draw_menu(f: &mut Frame, area: Rect, status: &str, app: &App) {
    let labels = menu_labels();
    let theme = theme_current();

    // Optionally render a small single-line header above the menu when there's room
    let menu_area = if area.height >= 2 {
        let v = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)].as_ref())
            .split(area);
        // render a compact header line with application name + status
        let header_para = Paragraph::new(format!("fileZoom — {}", status))
            .block(Block::default())
            .style(theme.help_block_style);
        f.render_widget(header_para, v[0]);
        v[1]
    } else {
        area
    };

    // Split the menu area into three: logo, tabs, status.
    // The status region is fixed so the tabs area remains responsive.
    let h = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(14),
                Constraint::Min(0),
                Constraint::Length(30),
            ]
            .as_ref(),
        )
        .split(menu_area);

    // Left: small app label/logo
    let logo = Paragraph::new("fileZoom").block(
        Block::default()
            .borders(Borders::NONE)
            .style(theme.help_block_style),
    );
    f.render_widget(logo, h[0]);

    // Use Tabs to render a top menu; selection is driven by app.menu_index
    // Build display labels with small icons so tests can still assert
    // on the canonical `menu_labels()` values while the UI renders
    // a slightly richer label for the user.
    let display_labels: Vec<String> = labels
        .iter()
        .map(|t| match *t {
            "File" => "📁 File".to_string(),
            "Copy" => "📄 Copy".to_string(),
            "Move" => "✂️ Move".to_string(),
            "New" => "➕ New".to_string(),
            "Sort" => "↕ Sort".to_string(),
            "Help" => "❓ Help".to_string(),
            other => other.to_string(),
        })
        .collect();

    // Style each title so the currently selected tab is visibly highlighted
    let titles: Vec<Line> = display_labels
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let s = if i == app.menu_index {
                theme.highlight_style
            } else {
                theme.help_block_style
            };
            Line::from(Span::styled(t.as_str(), s))
        })
        .collect();
    let block = Block::default()
        .borders(Borders::BOTTOM)
        .style(theme.help_block_style);
    let mut tabs = Tabs::new(titles).select(app.menu_index).block(block);
    // Apply highlight style when the menu is focused so selection is obvious
    if app.menu_focused {
        tabs = tabs.highlight_style(theme.highlight_style);
    } else {
        tabs = tabs.highlight_style(theme.help_block_style);
    }
    // Center: render the tabs into the main chunk
    f.render_widget(tabs, h[1]);

    // If a submenu is open render a small dropdown under the menu
    if app.menu_state.open {
        if let Some(top) = MenuModel::default_model().0.get(app.menu_state.top_index) {
            if let Some(sub) = &top.submenu {
                // compute width per label inside the tabs area
                let total = labels.len() as u16;
                let cell_w = if total > 0 { h[1].width / total } else { h[1].width };
                let sx = h[1].x + cell_w.saturating_mul(app.menu_index as u16);
                // dropdown height = items + padding
                let height = (sub.len() as u16).saturating_add(2).max(1);
                let rect = Rect::new(sx, menu_area.y.saturating_add(1), cell_w, height);
                use ratatui::widgets::{List, ListItem};
                use ratatui::text::Span as RSpan;
                let items: Vec<ListItem> = sub
                    .iter()
                    .enumerate()
                    .map(|(i, it)| {
                        let s = if Some(i) == app.menu_state.submenu_index { RSpan::styled(it.label, theme.highlight_style) } else { RSpan::raw(it.label) };
                        ListItem::new(s)
                    })
                    .collect();
                let list = List::new(items).block(Block::default().borders(Borders::ALL));
                f.render_widget(list, rect);
            }
        }
    }

    // Right: render status text (no borders to keep it compact)
    let status_p = Paragraph::new(status.to_string()).block(
        Block::default()
            .borders(Borders::NONE)
            .style(theme.help_block_style),
    );
    f.render_widget(status_p, h[2]);
}
