use crate::app::{Action, App, InputKind, Mode, Side, SortKey};
use crate::errors_logs;
use crate::input::KeyCode;
use std::path::PathBuf;

pub fn handle_key(app: &mut App, code: KeyCode, page_size: usize) -> anyhow::Result<bool> {
    match &mut app.mode {
        Mode::Normal => handle_normal(app, code, page_size),
        Mode::Message { .. } => {
            // Dismiss message on Enter, Esc, or any key
            match code {
                KeyCode::Enter | KeyCode::Esc | KeyCode::Char(_) => app.mode = Mode::Normal,
                _ => {}
            }
            Ok(false)
        }
        Mode::Confirm { .. } => handle_confirm(app, code),
        Mode::Input { .. } => handle_input(app, code),
    }
}

fn handle_normal(app: &mut App, code: KeyCode, page_size: usize) -> anyhow::Result<bool> {
    match code {
        KeyCode::Char('q') => return Ok(true),
        KeyCode::Down => app.next(page_size),
        KeyCode::Up => app.previous(page_size),
        KeyCode::PageDown => app.page_down(page_size),
        KeyCode::PageUp => app.page_up(page_size),
        KeyCode::Enter => {
            // clone the selected entry so we don't hold immutable borrows on `app`
            let e_opt = match app.active {
                Side::Left => app.left.entries.get(app.left.selected).cloned(),
                Side::Right => app.right.entries.get(app.right.selected).cloned(),
            };

            if let Some(e) = e_opt {
                let cwd_display = match app.active {
                    Side::Left => app.left.cwd.display().to_string(),
                    Side::Right => app.right.cwd.display().to_string(),
                };

                if e.name == cwd_display {
                    let prompt = format!("Change path (current: {}):", e.path.display());
                    app.mode = Mode::Input {
                        prompt,
                        buffer: String::new(),
                        kind: InputKind::ChangePath,
                    };
                } else {
                    if let Err(err) = app.enter() {
                        let path_s = e.path.display().to_string();
                        let msg = errors_logs::render_io_error(&err, Some(&path_s), None, None);
                        app.mode = Mode::Message {
                            title: "Error".to_string(),
                            content: msg,
                            buttons: vec!["OK".to_string()],
                            selected: 0,
                        };
                    }
                }
            } else {
                if let Err(err) = app.enter() {
                    let msg = errors_logs::render_io_error(&err, None, None, None);
                    app.mode = Mode::Message {
                        title: "Error".to_string(),
                        content: msg,
                        buttons: vec!["OK".to_string()],
                        selected: 0,
                    };
                }
            }
        }
        KeyCode::Backspace => {
            if let Err(err) = app.go_up() {
                let msg = errors_logs::render_io_error(&err, None, None, None);
                app.mode = Mode::Message {
                    title: "Error".to_string(),
                    content: msg,
                    buttons: vec!["OK".to_string()],
                    selected: 0,
                };
            }
        }
        KeyCode::Char('r') => {
            if let Err(err) = app.refresh() {
                let msg = errors_logs::render_io_error(&err, None, None, None);
                app.mode = Mode::Message {
                    title: "Error".to_string(),
                    content: msg,
                    buttons: vec!["OK".to_string()],
                    selected: 0,
                };
            }
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
                    selected: 0,
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
            crate::ui::colors::toggle();
        }
        KeyCode::Char('>') => match app.active {
            Side::Left => app.left.preview_offset = app.left.preview_offset.saturating_add(5),
            Side::Right => app.right.preview_offset = app.right.preview_offset.saturating_add(5),
        },
        KeyCode::Char('<') => match app.active {
            Side::Left => app.left.preview_offset = app.left.preview_offset.saturating_sub(5),
            Side::Right => app.right.preview_offset = app.right.preview_offset.saturating_sub(5),
        },
        _ => {}
    }
    Ok(false)
}

fn handle_confirm(app: &mut App, code: KeyCode) -> anyhow::Result<bool> {
    match &mut app.mode {
        Mode::Confirm { on_yes, .. } => match code {
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
                    Action::DeleteSelected => {
                        if let Err(err) = app.delete_selected() {
                            let msg = errors_logs::render_io_error(&err, None, None, None);
                            app.mode = Mode::Message {
                                title: "Error".to_string(),
                                content: msg,
                                buttons: vec!["OK".to_string()],
                                selected: 0,
                            };
                        }
                    }
                    Action::CopyTo(p) => {
                        if let Err(err) = app.copy_selected_to(p) {
                            let msg = errors_logs::render_io_error(&err, None, None, None);
                            app.mode = Mode::Message {
                                title: "Error".to_string(),
                                content: msg,
                                buttons: vec!["OK".to_string()],
                                selected: 0,
                            };
                        }
                    }
                    Action::MoveTo(p) => {
                        if let Err(err) = app.move_selected_to(p) {
                            let msg = errors_logs::render_io_error(&err, None, None, None);
                            app.mode = Mode::Message {
                                title: "Error".to_string(),
                                content: msg,
                                buttons: vec!["OK".to_string()],
                                selected: 0,
                            };
                        }
                    }
                    Action::RenameTo(name) => {
                        if let Err(err) = app.rename_selected_to(name) {
                            let msg = errors_logs::render_io_error(&err, None, None, None);
                            app.mode = Mode::Message {
                                title: "Error".to_string(),
                                content: msg,
                                buttons: vec!["OK".to_string()],
                                selected: 0,
                            };
                        }
                    }
                    Action::NewFile(name) => {
                        if let Err(err) = app.new_file(name) {
                            let msg = errors_logs::render_io_error(&err, None, None, None);
                            app.mode = Mode::Message {
                                title: "Error".to_string(),
                                content: msg,
                                buttons: vec!["OK".to_string()],
                                selected: 0,
                            };
                        }
                    }
                    Action::NewDir(name) => {
                        if let Err(err) = app.new_dir(name) {
                            let msg = errors_logs::render_io_error(&err, None, None, None);
                            app.mode = Mode::Message {
                                title: "Error".to_string(),
                                content: msg,
                                buttons: vec!["OK".to_string()],
                                selected: 0,
                            };
                        }
                    }
                }
            }
            KeyCode::Char('n') | KeyCode::Esc => {
                app.mode = Mode::Normal;
            }
            _ => {}
        },
        _ => {}
    }
    Ok(false)
}

fn handle_input(app: &mut App, code: KeyCode) -> anyhow::Result<bool> {
    match &mut app.mode {
        Mode::Input {
            prompt: _,
            buffer,
            kind,
        } => match code {
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
                        if let Err(err) = app.copy_selected_to(dst) {
                            let msg = errors_logs::render_io_error(&err, None, None, None);
                            app.mode = Mode::Message {
                                title: "Error".to_string(),
                                content: msg,
                                buttons: vec!["OK".to_string()],
                                selected: 0,
                            };
                        }
                    }
                    InputKind::Move => {
                        let dst = PathBuf::from(input);
                        if let Err(err) = app.move_selected_to(dst) {
                            let msg = errors_logs::render_io_error(&err, None, None, None);
                            app.mode = Mode::Message {
                                title: "Error".to_string(),
                                content: msg,
                                buttons: vec!["OK".to_string()],
                                selected: 0,
                            };
                        }
                    }
                    InputKind::Rename => {
                        if let Err(err) = app.rename_selected_to(input) {
                            let msg = errors_logs::render_io_error(&err, None, None, None);
                            app.mode = Mode::Message {
                                title: "Error".to_string(),
                                content: msg,
                                buttons: vec!["OK".to_string()],
                                selected: 0,
                            };
                        }
                    }
                    InputKind::NewFile => {
                        if let Err(err) = app.new_file(input) {
                            let msg = errors_logs::render_io_error(&err, None, None, None);
                            app.mode = Mode::Message {
                                title: "Error".to_string(),
                                content: msg,
                                buttons: vec!["OK".to_string()],
                                selected: 0,
                            };
                        }
                    }
                    InputKind::NewDir => {
                        if let Err(err) = app.new_dir(input) {
                            let msg = errors_logs::render_io_error(&err, None, None, None);
                            app.mode = Mode::Message {
                                title: "Error".to_string(),
                                content: msg,
                                buttons: vec!["OK".to_string()],
                                selected: 0,
                            };
                        }
                    }
                    InputKind::ChangePath => {
                        let p = PathBuf::from(input);
                        match app.active {
                            Side::Left => {
                                app.left.cwd = p;
                                if let Err(err) = app.refresh() {
                                    let msg = errors_logs::render_io_error(&err, None, None, None);
                                    app.mode = Mode::Message {
                                        title: "Error".to_string(),
                                        content: msg,
                                        buttons: vec!["OK".to_string()],
                                        selected: 0,
                                    };
                                }
                            }
                            Side::Right => {
                                app.right.cwd = p;
                                if let Err(err) = app.refresh() {
                                    let msg = errors_logs::render_io_error(&err, None, None, None);
                                    app.mode = Mode::Message {
                                        title: "Error".to_string(),
                                        content: msg,
                                        buttons: vec!["OK".to_string()],
                                        selected: 0,
                                    };
                                }
                            }
                        }
                    }
                }
            }
            KeyCode::Backspace => {
                buffer.pop();
            }
            KeyCode::Esc => {
                app.mode = Mode::Normal;
            }
            KeyCode::Char(c) => {
                buffer.push(c);
            }
            _ => {}
        },
        _ => {}
    }
    Ok(false)
}
