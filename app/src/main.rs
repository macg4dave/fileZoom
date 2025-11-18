// Use the library crate modules (exposed from `lib.rs`)
use app::{App, Mode, InputKind, Action, Side};
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
                                let e_opt = match app.active {
                                    Side::Left => app.left.entries.get(app.left.selected),
                                    Side::Right => app.right.entries.get(app.right.selected),
                                };
                                if let Some(e) = e_opt {
                                    let msg = format!("Delete {}? (y/n)", e.name);
                                    app.mode = Mode::Confirm { msg, on_yes: Action::DeleteSelected };
                                }
                            }
                            KeyCode::Char('c') => {
                                let e_opt = match app.active {
                                    Side::Left => app.left.entries.get(app.left.selected),
                                    Side::Right => app.right.entries.get(app.right.selected),
                                };
                                if let Some(e) = e_opt {
                                    let prompt = format!("Copy {} to:", e.name);
                                    app.mode = Mode::Input { prompt, buffer: String::new(), kind: InputKind::Copy };
                                }
                            }
                            KeyCode::Char('m') => {
                                let e_opt = match app.active {
                                    Side::Left => app.left.entries.get(app.left.selected),
                                    Side::Right => app.right.entries.get(app.right.selected),
                                };
                                if let Some(e) = e_opt {
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
                                let e_opt = match app.active {
                                    Side::Left => app.left.entries.get(app.left.selected),
                                    Side::Right => app.right.entries.get(app.right.selected),
                                };
                                if let Some(e) = e_opt {
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
                            KeyCode::Tab => { app.active = match app.active { Side::Left => Side::Right, Side::Right => Side::Left }; }
                            KeyCode::Home => { match app.active { Side::Left => app.left.selected = 0, Side::Right => app.right.selected = 0 } }
                            KeyCode::End => { match app.active { Side::Left => if !app.left.entries.is_empty() { app.left.selected = app.left.entries.len() - 1 }, Side::Right => if !app.right.entries.is_empty() { app.right.selected = app.right.entries.len() - 1 } } }
                            KeyCode::Char('p') => { /* toggle preview behavior */ }
                            KeyCode::Char('>') => { match app.active { Side::Left => app.left.preview_offset = app.left.preview_offset.saturating_add(5), Side::Right => app.right.preview_offset = app.right.preview_offset.saturating_add(5) } }
                            KeyCode::Char('<') => { match app.active { Side::Left => app.left.preview_offset = app.left.preview_offset.saturating_sub(5), Side::Right => app.right.preview_offset = app.right.preview_offset.saturating_sub(5) } }
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
                                    InputKind::Copy => { let dst = if input.starts_with('/') { PathBuf::from(&input) } else { match app.active { Side::Left => app.left.cwd.join(&input), Side::Right => app.right.cwd.join(&input) } }; let _ = app.copy_selected_to(dst); }
                                    InputKind::Move => { let dst = if input.starts_with('/') { PathBuf::from(&input) } else { match app.active { Side::Left => app.left.cwd.join(&input), Side::Right => app.right.cwd.join(&input) } }; let _ = app.move_selected_to(dst); }
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
                    let vchunks = Layout::default().direction(Direction::Vertical).constraints([Constraint::Length(1), Constraint::Min(0), Constraint::Length(1)].as_ref()).split(size);
                    let main_chunks = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref()).split(vchunks[1]);
                    let left_area = main_chunks[0];
                    let right_area = main_chunks[1];
                    // mouse.column/row are u16
                    let col = mouse.column as u16;
                    let row = mouse.row as u16;
                    match mouse.kind {
                        MouseEventKind::Down(MouseButton::Left) => {
                            if col >= left_area.x && col < left_area.x + left_area.width && row >= left_area.y && row < left_area.y + left_area.height {
                                app.active = Side::Left;
                                // compute index within left list (account for border/title)
                                let rel_y = (row as i32) - (left_area.y as i32) - 1;
                                if rel_y >= 0 {
                                    let list_height = (left_area.height as usize).saturating_sub(2);
                                    let idx = app.left.offset.saturating_add(rel_y as usize);
                                    if idx < app.left.entries.len() {
                                        app.left.selected = idx;
                                        app.ensure_selection_visible(list_height);
                                        app.update_preview_for(Side::Left);
                                    }
                                }
                            }
                            if col >= right_area.x && col < right_area.x + right_area.width && row >= right_area.y && row < right_area.y + right_area.height {
                                app.active = Side::Right;
                                // compute index within right list (account for border/title)
                                let rel_y = (row as i32) - (right_area.y as i32) - 1;
                                if rel_y >= 0 {
                                    let list_height = (right_area.height as usize).saturating_sub(2);
                                    let idx = app.right.offset.saturating_add(rel_y as usize);
                                    if idx < app.right.entries.len() {
                                        app.right.selected = idx;
                                        app.ensure_selection_visible(list_height);
                                        app.update_preview_for(Side::Right);
                                    }
                                }
                            }
                        }
                        MouseEventKind::ScrollUp => {
                            // scroll the active panel
                            let list_height = (left_area.height as usize).saturating_sub(2);
                            app.page_up(list_height);
                        }
                        MouseEventKind::ScrollDown => {
                            let list_height = (left_area.height as usize).saturating_sub(2);
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

