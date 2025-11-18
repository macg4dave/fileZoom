use std::io;
use anyhow::Context;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use tui::backend::CrosstermBackend;
use tui::Terminal;

/// Initialize terminal (enter alternate screen + enable raw mode) and return a TUI Terminal.
pub fn init_terminal() -> anyhow::Result<Terminal<CrosstermBackend<std::io::Stdout>>> {
    enable_raw_mode().context("enable_raw_mode failed")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).context("enter alternate screen failed")?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).context("create terminal backend failed")?;
    Ok(terminal)
}

/// Restore terminal state (leave alternate screen + disable raw mode) and show cursor.
pub fn restore_terminal(mut terminal: Terminal<CrosstermBackend<std::io::Stdout>>) -> anyhow::Result<()> {
    disable_raw_mode().context("disable_raw_mode failed")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen).context("leave alternate screen failed")?;
    terminal.show_cursor().context("show cursor failed")?;
    Ok(())
}
