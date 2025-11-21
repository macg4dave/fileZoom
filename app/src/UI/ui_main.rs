use ratatui::{layout::{Constraint, Direction, Layout}, Terminal};
use ratatui::backend::Backend;
use crate::ui::{UIState, Theme};
use ratatui::Frame;
use crate::app::core::App as CoreApp;

/// Draw one frame using the provided Terminal and view model.
pub fn draw_frame<B: Backend>(terminal: &mut Terminal<B>, state: &UIState, theme: &Theme) -> std::io::Result<()> {
    terminal.draw(|f| {
        let size = f.area();
        // header (3), main (min), footer (2)
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(2)])
            .split(size);

        let main = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
            .split(chunks[1]);

        crate::ui::widgets::header::render(f, chunks[0], state, theme);
        crate::ui::widgets::file_list::render(f, main[0], state, theme);
        crate::ui::widgets::preview::render(f, main[1], state, theme);
        crate::ui::widgets::footer::render(f, chunks[2], state, theme);
    }).map(|_| ())
}

/// Legacy UI entrypoint used by the runner: draw directly into a Frame
pub fn ui(f: &mut Frame, _app: &CoreApp) {
    // Construct a tiny UIState view-model for scaffold draws.
    let state = UIState::sample();
    let theme = Theme::dark();

    let size = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(2)])
        .split(size);
    let main = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(chunks[1]);

    crate::ui::widgets::header::render(f, chunks[0], &state, &theme);
    crate::ui::widgets::file_list::render(f, main[0], &state, &theme);
    crate::ui::widgets::preview::render(f, main[1], &state, &theme);
    crate::ui::widgets::footer::render(f, chunks[2], &state, &theme);
}
