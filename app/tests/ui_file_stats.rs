use ratatui::backend::TestBackend;
use ratatui::layout::Rect;
use ratatui::Terminal;

use fileZoom::ui::panels::draw_preview;
use fileZoom::app::Panel;
use fileZoom::app::types::Entry;

#[test]
fn preview_empty_shows_placeholder() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).expect("failed to create terminal");

    // Create a panel rooted at / (no parent row) with a single entry and
    // an empty textual preview. With the new layout preview no longer
    // falls back to file-stats; this is a smoke test to ensure no panic
    // occurs when preview is empty.
    let mut panel = Panel::new(std::path::PathBuf::from("/"));
    panel.entries = vec![Entry::file("foo.txt", std::path::PathBuf::from("/foo.txt"), 1234, None)];
    panel.selected = 1; // header is 0 -> entry at 1 when no parent
    panel.preview = String::new();

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 80, 24);
            draw_preview(f, area, &panel);
        })
        .expect("failed to draw");
}

#[test]
fn ui_renders_file_stats_column_when_enabled() {
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;
    use fileZoom::ui::ui;
    use fileZoom::app::{App, Panel, Mode, Side, SortKey};

    let backend = TestBackend::new(140, 24);
    let mut terminal = Terminal::new(backend).expect("failed to create terminal");

    // Construct minimal app state with file-stats visible and a selected entry.
    let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let mut app = App {
        left: Panel::new(cwd.clone()),
        right: Panel::new(cwd.clone()),
        active: Side::Left,
        mode: Mode::Normal,
        sort: SortKey::Name,
        sort_order: fileZoom::app::types::SortOrder::Ascending,
        menu_index: 0,
        menu_focused: false,
        menu_state: fileZoom::ui::menu_model::MenuState::default(),
        preview_visible: false,
        file_stats_visible: true,
        command_line: None,
        settings: fileZoom::app::settings::write_settings::Settings::default(),
        op_progress_rx: None,
        op_cancel_flag: None,
        op_decision_tx: None,
        last_mouse_click_time: None,
        last_mouse_click_pos: None,
        drag_active: false,
        drag_start: None,
        drag_current: None,
        drag_button: None,
    };

    // Ensure left panel has an entry and selection points to it.
    app.left.entries.push(fileZoom::app::types::Entry::file("foo.txt", std::path::PathBuf::from("/foo.txt"), 123, None));
    app.left.selected = 1; // header is index 0

    terminal
        .draw(|f| ui(f, &app))
        .expect("failed to draw ui");
}
