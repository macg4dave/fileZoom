// Use the library crate modules (exposed from `lib.rs`)
use app::{App, Mode, InputKind, Action};
use app::ui;

use std::io;
use std::path::PathBuf;
use std::time::Duration;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Terminal;
use crossterm::event::{self, Event, KeyCode, MouseEventKind, MouseButton};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};

// (moved above)

fn run_app() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new()?;

    loop {
        terminal.draw(|f| ui::ui(f, &app))?;

        if event::poll(Duration::from_millis(100))? {
            let ev = event::read()?;
            match ev {
                Event::Key(key) => {
                    let code = key.code;
                    match &mut app.mode {
                        Mode::Normal => match code {
                            KeyCode::Char('q') => break,
                            KeyCode::Down => app.next((terminal.size()?.height as usize).saturating_sub(4)),
                            KeyCode::Up => app.previous((terminal.size()?.height as usize).saturating_sub(4)),
                            KeyCode::PageDown => app.page_down((terminal.size()?.height as usize).saturating_sub(4)),
                            KeyCode::PageUp => app.page_up((terminal.size()?.height as usize).saturating_sub(4)),
                            KeyCode::Enter => { let _ = app.enter(); },
                            KeyCode::Backspace => { let _ = app.go_up(); },
                            KeyCode::Char('r') => { let _ = app.refresh(); },
                            KeyCode::Char('d') => {
                                if let Some(e) = app.entries.get(app.selected) {
                                    let msg = format!("Delete {}? (y/n)", e.name);
                                    app.mode = Mode::Confirm { msg, on_yes: Action::DeleteSelected };
                                }
                            }
                            KeyCode::Char('c') => {
                                if let Some(e) = app.entries.get(app.selected) {
                                    let prompt = format!("Copy {} to:", e.name);
                                    app.mode = Mode::Input { prompt, buffer: String::new(), kind: InputKind::Copy };
                                }
                            }
                            KeyCode::Char('m') => {
                                if let Some(e) = app.entries.get(app.selected) {
                                    let prompt = format!("Move {} to:", e.name);
                                    app.mode = Mode::Input { prompt, buffer: String::new(), kind: InputKind::Move };
                                }
                            }
                            KeyCode::Char('n') => {
                                app.mode = Mode::Input { prompt: "New file name:".to_string(), buffer: String::new(), kind: InputKind::NewFile };
                            }
                            KeyCode::Char('N') => {
                                app.mode = Mode::Input { prompt: "New dir name:".to_string(), buffer: String::new(), kind: InputKind::NewDir };
                            }
                            KeyCode::Char('R') => {
                                if let Some(e) = app.entries.get(app.selected) {
                                    let prompt = format!("Rename {} to:", e.name);
                                    app.mode = Mode::Input { prompt, buffer: String::new(), kind: InputKind::Rename };
                                }
                            }
                            KeyCode::Char('s') => {
                                app.sort = match app.sort {
                                    app::SortKey::Name => app::SortKey::Size,
                                    app::SortKey::Size => app::SortKey::Modified,
                                    app::SortKey::Modified => app::SortKey::Name,
                                };
                                app.refresh()?;
                            }
                            KeyCode::Char('S') => {
                                app.sort_desc = !app.sort_desc;
                                app.refresh()?;
                            }
                            KeyCode::Char(' ') => { /* reserved */ }
                            KeyCode::Home => { app.selected = 0; }
                            KeyCode::End => { if !app.entries.is_empty() { app.selected = app.entries.len() - 1; } }
                            KeyCode::Char('p') => { /* toggle preview behavior */ }
                            KeyCode::Char('>') => { app.preview_offset = app.preview_offset.saturating_add(5); }
                            KeyCode::Char('<') => { app.preview_offset = app.preview_offset.saturating_sub(5); }
                            _ => {}
                        },
                        Mode::Confirm { msg: _, on_yes } => match code {
                            KeyCode::Char('y') => {
                                match on_yes {
                                    Action::DeleteSelected => { let _ = app.delete_selected(); }
                                    _ => {}
                                }
                                app.mode = Mode::Normal;
                            }
                            KeyCode::Char('n') | KeyCode::Esc => { app.mode = Mode::Normal; }
                            _ => {}
                        },
                        Mode::Input { prompt: _, buffer, kind } => match code {
                            KeyCode::Enter => {
                                let input = buffer.trim().to_string();
                                match kind {
                                    InputKind::Copy => { let dst = if input.starts_with('/') { PathBuf::from(&input) } else { app.cwd.join(&input) }; let _ = app.copy_selected_to(dst); }
                                    InputKind::Move => { let dst = if input.starts_with('/') { PathBuf::from(&input) } else { app.cwd.join(&input) }; let _ = app.move_selected_to(dst); }
                                    InputKind::Rename => { let _ = app.rename_selected_to(input); }
                                    InputKind::NewFile => { let _ = app.new_file(input); }
                                    InputKind::NewDir => { let _ = app.new_dir(input); }
                                }
                                app.mode = Mode::Normal;
                            }
                            KeyCode::Char(c) => { buffer.push(c); }
                            KeyCode::Backspace => { buffer.pop(); }
                            KeyCode::Esc => { app.mode = Mode::Normal; }
                            _ => {}
                        }
                    }
                }
                Event::Mouse(mouse) => {
                    // compute layout areas the same way UI does
                    let size = terminal.size()?;
                    let vchunks = Layout::default().direction(Direction::Vertical).constraints([Constraint::Min(0), Constraint::Length(1)].as_ref()).split(size);
                    let main_chunks = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref()).split(vchunks[0]);
                    let list_area = main_chunks[0];
                    // mouse.column/row are u16
                    let col = mouse.column as u16;
                    let row = mouse.row as u16;
                    match mouse.kind {
                        MouseEventKind::Down(MouseButton::Left) => {
                            if col >= list_area.x && col < list_area.x + list_area.width && row >= list_area.y && row < list_area.y + list_area.height {
                                // compute index within list (account for border/title)
                                let rel_y = (row as i32) - (list_area.y as i32) - 1;
                                if rel_y >= 0 {
                                    let list_height = (list_area.height as usize).saturating_sub(2);
                                    let idx = app.offset.saturating_add(rel_y as usize);
                                    if idx < app.entries.len() {
                                        app.selected = idx;
                                        app.ensure_selection_visible(list_height);
                                        app.update_preview();
                                        // if double click semantics desired, could track timestamp here
                                    }
                                }
                            }
                        }
                        MouseEventKind::ScrollUp => {
                            let list_height = (list_area.height as usize).saturating_sub(2);
                            app.page_up(list_height);
                        }
                        MouseEventKind::ScrollDown => {
                            let list_height = (list_area.height as usize).saturating_sub(2);
                            app.page_down(list_height);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

    }
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    run_app()
}

