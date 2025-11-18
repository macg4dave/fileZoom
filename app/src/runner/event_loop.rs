use crate::ui;
use crate::app::{Action, App, InputKind, Mode, Side, SortKey};
use crate::ui::colors;

use std::path::PathBuf;
use std::time::Duration;

use crate::input::{poll, read_event, InputEvent, KeyCode};
use crate::runner::terminal::{init_terminal, restore_terminal};

pub fn run_app() -> anyhow::Result<()> {
    let mut terminal = init_terminal()?;

    // Initialize app using the current working directory.
    let mut app = App::new()?;

    // Main event loop
    loop {
        terminal.draw(|f| ui::ui(f, &app))?;

        if poll(Duration::from_millis(100))? {
            let iev = read_event()?;
            match iev {
                InputEvent::Key(key) => {
                    let code = key.code;
                    match &mut app.mode {
                        Mode::Normal => match code {
                            KeyCode::Char('q') => break,
                            KeyCode::Down => {
                                app.next((terminal.size()?.height as usize).saturating_sub(4))
                            }
                            KeyCode::Up => app
                                .previous((terminal.size()?.height as usize).saturating_sub(4)),
                            KeyCode::PageDown => app.page_down(
                                (terminal.size()?.height as usize).saturating_sub(4),
                            ),
                            KeyCode::PageUp => app
                                .page_up((terminal.size()?.height as usize).saturating_sub(4)),
                            KeyCode::Enter => {
                                let e_opt = match app.active {
                                    Side::Left => app.left.entries.get(app.left.selected),
                                    Side::Right => app.right.entries.get(app.right.selected),
                                };
                                if let Some(e) = e_opt {
                                    let cwd_display = match app.active {
                                        Side::Left => app.left.cwd.display().to_string(),
                                        Side::Right => app.right.cwd.display().to_string(),
                                    };
                                    if e.name == cwd_display {
                                        let prompt = format!(
                                            "Change path (current: {}):",
                                            e.path.display()
                                        );
                                        app.mode = Mode::Input {
                                            prompt,
                                            buffer: String::new(),
                                            kind: InputKind::ChangePath,
                                        };
                                    } else {
                                        let _ = app.enter();
                                    }
                                } else {
                                    let _ = app.enter();
                                }
                            }
                            KeyCode::Backspace => {
                                let _ = app.go_up();
                            }
                            KeyCode::Char('r') => {
                                let _ = app.refresh();
                            }
                            KeyCode::Char('d') => {
                                let e_opt = match app.active {
                                    Side::Left => app.left.entries.get(app.left.selected),
                                    Side::Right => app.right.entries.get(app.right.selected),
                                };
                                if let Some(e) = e_opt {
                                    let msg = format!("Delete {}? (y/n)", e.name);
                                    app.mode = Mode::Confirm {
                                        msg,
                                        on_yes: Action::DeleteSelected,
                                    };
                                }
                            }
                            KeyCode::Char('c') => {
                                let e_opt = match app.active {
                                    Side::Left => app.left.entries.get(app.left.selected),
                                    Side::Right => app.right.entries.get(app.right.selected),
                                };
                                if let Some(e) = e_opt {
                                    let prompt = format!("Copy {} to:", e.name);
                                    app.mode = Mode::Input {
                                        prompt,
                                        buffer: String::new(),
                                        kind: InputKind::Copy,
                                    };
                                }
                            }
                            KeyCode::Char('m') => {
                                let e_opt = match app.active {
                                    Side::Left => app.left.entries.get(app.left.selected),
                                    Side::Right => app.right.entries.get(app.right.selected),
                                };
                                if let Some(e) = e_opt {
                                    let prompt = format!("Move {} to:", e.name);
                                    app.mode = Mode::Input {
                                        prompt,
                                        buffer: String::new(),
                                        kind: InputKind::Move,
                                    };
                                }
                            }
                            KeyCode::Char('n') => {
                                app.mode = Mode::Input {
                                    prompt: "New file name:".to_string(),
                                    buffer: String::new(),
                                    kind: InputKind::NewFile,
                                };
                            }
                            KeyCode::Char('N') => {
                                app.mode = Mode::Input {
                                    prompt: "New dir name:".to_string(),
                                    buffer: String::new(),
                                    kind: InputKind::NewDir,
                                };
                            }
                            KeyCode::Char('R') => {
                                let e_opt = match app.active {
                                    Side::Left => app.left.entries.get(app.left.selected),
                                    Side::Right => app.right.entries.get(app.right.selected),
                                };
                                if let Some(e) = e_opt {
                                    let prompt = format!("Rename {} to:", e.name);
                                    app.mode = Mode::Input {
                                        prompt,
                                        buffer: String::new(),
                                        kind: InputKind::Rename,
                                    };
                                }
                            }
                            KeyCode::Char('s') => {
                                app.sort = match app.sort {
                                    SortKey::Name => SortKey::Size,
                                    SortKey::Size => SortKey::Modified,
                                    SortKey::Modified => SortKey::Name,
                                };
                                app.refresh()?;
                            }
                            KeyCode::Char('S') => {
                                app.sort_desc = !app.sort_desc;
                                app.refresh()?;
                            }
                            KeyCode::Char(' ') => { /* reserved */ }
                            KeyCode::Tab => {
                                app.active = match app.active {
                                    Side::Left => Side::Right,
                                    Side::Right => Side::Left,
                                };
                            }
                            KeyCode::Home => match app.active {
                                Side::Left => app.left.selected = 0,
                                Side::Right => app.right.selected = 0,
                            },
                            KeyCode::End => match app.active {
                                Side::Left => {
                                    if !app.left.entries.is_empty() {
                                        app.left.selected = app.left.entries.len() - 1
                                    }
                                }
                                Side::Right => {
                                    if !app.right.entries.is_empty() {
                                        app.right.selected = app.right.entries.len() - 1
                                    }
                                }
                            },
                            KeyCode::Char('p') => { /* toggle preview behavior */ }
                            KeyCode::Char('t') => {
                                // Toggle theme between default and dark
                                colors::toggle();
                            }
                            KeyCode::Char('>') => match app.active {
                                Side::Left => {
                                    app.left.preview_offset =
                                        app.left.preview_offset.saturating_add(5)
                                }
                                Side::Right => {
                                    app.right.preview_offset =
                                        app.right.preview_offset.saturating_add(5)
                                }
                            },
                            KeyCode::Char('<') => match app.active {
                                Side::Left => {
                                    app.left.preview_offset =
                                        app.left.preview_offset.saturating_sub(5)
                                }
                                Side::Right => {
                                    app.right.preview_offset =
                                        app.right.preview_offset.saturating_sub(5)
                                }
                            },
                            _ => {}
                        },

                        Mode::Confirm { msg: _, on_yes } => match code {
                            KeyCode::Char('y') | KeyCode::Char('Y') => {
                                // Clone the action out so we drop the borrow on app.mode
                                let action = match on_yes {
                                    Action::DeleteSelected => Action::DeleteSelected,
                                    Action::CopyTo(p) => Action::CopyTo(p.clone()),
                                    Action::MoveTo(p) => Action::MoveTo(p.clone()),
                                    Action::RenameTo(name) => Action::RenameTo(name.clone()),
                                    Action::NewFile(name) => Action::NewFile(name.clone()),
                                    Action::NewDir(name) => Action::NewDir(name.clone()),
                                };
                                // leave normal mode before performing file operations
                                app.mode = Mode::Normal;
                                match action {
                                    Action::DeleteSelected => { let _ = app.delete_selected(); }
                                    Action::CopyTo(p) => { let _ = app.copy_selected_to(p); }
                                    Action::MoveTo(p) => { let _ = app.move_selected_to(p); }
                                    Action::RenameTo(name) => { let _ = app.rename_selected_to(name); }
                                    Action::NewFile(name) => { let _ = app.new_file(name); }
                                    Action::NewDir(name) => { let _ = app.new_dir(name); }
                                }
                            }
                            KeyCode::Char('n') | KeyCode::Esc => {
                                app.mode = Mode::Normal;
                            }
                            _ => {}
                        },

                        Mode::Input { prompt: _, buffer, kind } => match code {
                            KeyCode::Enter => {
                                // Snapshot buffer and kind, then leave Input mode so we
                                // can perform mutable operations on `app`.
                                let input = buffer.clone();
                                let kind_snapshot = match &*kind {
                                    InputKind::Copy => InputKind::Copy,
                                    InputKind::Move => InputKind::Move,
                                    InputKind::Rename => InputKind::Rename,
                                    InputKind::NewFile => InputKind::NewFile,
                                    InputKind::NewDir => InputKind::NewDir,
                                    InputKind::ChangePath => InputKind::ChangePath,
                                };
                                app.mode = Mode::Normal;
                                match kind_snapshot {
                                    InputKind::Copy => {
                                        let dst = PathBuf::from(input);
                                        let _ = app.copy_selected_to(dst);
                                    }
                                    InputKind::Move => {
                                        let dst = PathBuf::from(input);
                                        let _ = app.move_selected_to(dst);
                                    }
                                    InputKind::Rename => {
                                        let _ = app.rename_selected_to(input);
                                    }
                                    InputKind::NewFile => {
                                        let _ = app.new_file(input);
                                    }
                                    InputKind::NewDir => {
                                        let _ = app.new_dir(input);
                                    }
                                    InputKind::ChangePath => {
                                        let p = PathBuf::from(input);
                                        match app.active {
                                            Side::Left => { app.left.cwd = p; let _ = app.refresh(); }
                                            Side::Right => { app.right.cwd = p; let _ = app.refresh(); }
                                        }
                                    }
                                }
                            }
                            KeyCode::Backspace => { buffer.pop(); }
                            KeyCode::Esc => { app.mode = Mode::Normal; }
                            KeyCode::Char(c) => { buffer.push(c); }
                            _ => {}
                        },
                    }
                }
                InputEvent::Mouse(_) => {
                    // Mouse events are ignored at runtime; kept for future use.
                }
                InputEvent::Resize(_, _) => {
                    // Let the next loop iteration redraw with the new size.
                }
                InputEvent::Other => {}
            }
        }
    }

    // Restore terminal state before exiting.
    restore_terminal(terminal)?;
    Ok(())
}