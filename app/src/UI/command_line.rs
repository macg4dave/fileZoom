use crate::app::core::App;
use crate::input::KeyCode;
use ratatui::{layout::Rect, widgets::{Block, Borders}, Frame};
use tui_textarea::{CursorMove, Input as TextInput, Key as TextKey, TextArea};
use std::path::{Path, PathBuf};
use std::fs;

/// Maximum number of historical commands remembered in the inline prompt.
const MAX_HISTORY: usize = 50;

/// Inline command-line state backed by `tui-textarea`.
#[derive(Clone, Debug)]
pub struct CommandLineState {
    /// Whether the command line should be rendered and capture input.
    pub visible: bool,
    /// Text editing widget storing the current command buffer and cursor.
    pub textarea: TextArea<'static>,
    /// Rolling history of submitted commands (most recent at the end).
    history: Vec<String>,
    /// Optional index into `history` when navigating with Up/Down.
    history_index: Option<usize>,
    /// Draft text captured before entering history navigation.
    draft: String,
}

impl Default for CommandLineState {
    fn default() -> Self {
        Self {
            visible: true,
            textarea: new_textarea(""),
            history: Vec::new(),
            history_index: None,
            draft: String::new(),
        }
    }
}

impl CommandLineState {
    /// Activate the command line for new input while preserving history.
    pub fn activate(&mut self) {
        self.visible = true;
        self.history_index = None;
        self.draft.clear();
        self.set_text("");
    }
}

/// Build a pre-configured textarea with placeholder and border.
fn new_textarea(text: &str) -> TextArea<'static> {
    let mut area = TextArea::from([text.to_string()]);
    area.set_placeholder_text("Type a command (Tab to complete, Esc to cancel)");
    area.set_block(Block::default().borders(Borders::ALL).title("Command"));
    area.move_cursor(CursorMove::End);
    area
}

impl CommandLineState {
    /// Reset the visible textarea content and move cursor to the end.
    fn set_text(&mut self, text: &str) {
        self.textarea = new_textarea(text);
    }

    /// Return the current buffer contents as a single string.
    fn current_text(&self) -> String {
        self.textarea.lines().join("\n")
    }

    /// Insert a key event into the textarea using default mappings.
    fn feed_textarea(&mut self, code: KeyCode) {
        if let Some(input) = keycode_to_input(code) {
            self.textarea.input(input);
            // Leaving history navigation resets the draft marker.
            self.history_index = None;
            self.draft.clear();
        }
    }

    /// Push a command into history, deduplicating and capping length.
    fn push_history(&mut self, entry: &str) {
        let trimmed = entry.trim();
        if trimmed.is_empty() {
            self.history_index = None;
            return;
        }
        if let Some(pos) = self.history.iter().position(|h| h == trimmed) {
            self.history.remove(pos);
        }
        self.history.push(trimmed.to_string());
        if self.history.len() > MAX_HISTORY {
            let drain = self.history.len().saturating_sub(MAX_HISTORY);
            self.history.drain(0..drain);
        }
        self.history_index = None;
        self.draft.clear();
    }

    /// Navigate to an older history entry (Up arrow).
    fn history_prev(&mut self) {
        if self.history.is_empty() {
            return;
        }
        if self.history_index.is_none() {
            self.draft = self.current_text();
            self.history_index = Some(self.history.len().saturating_sub(1));
        } else if let Some(idx) = self.history_index {
            if idx > 0 {
                self.history_index = Some(idx - 1);
            }
        }
        if let Some(idx) = self.history_index {
            if let Some(cmd) = self.history.get(idx).cloned() {
                self.set_text(&cmd);
            }
        }
    }

    /// Navigate to a newer history entry (Down arrow).
    fn history_next(&mut self) {
        let Some(idx) = self.history_index else { return; };
        if idx + 1 < self.history.len() {
            let next = idx + 1;
            if let Some(cmd) = self.history.get(next).cloned() {
                self.history_index = Some(next);
                self.set_text(&cmd);
            }
        } else {
            // Exiting history navigation restores the draft text.
            self.history_index = None;
            let draft = self.draft.clone();
            self.set_text(&draft);
        }
    }

    /// Completion with access to app for path-aware hints.
    pub fn apply_completion_with(&mut self, cwd: Option<&Path>) {
        let input = self.current_text();
        let trimmed = input.trim_end();
        if trimmed.is_empty() {
            return;
        }

        // Determine the last token to complete (split by whitespace).
        let last_ws = trimmed.rfind(char::is_whitespace);
        let (head, tail) = match last_ws {
            Some(idx) => (&trimmed[..idx + 1], &trimmed[idx + 1..]),
            None => ("", trimmed),
        };

        // Path-style token? Apply path completion when token looks like a path.
        if let Some(cwd) = cwd {
            if is_pathy_token(tail) {
                if let Some(completed) = complete_path(tail, cwd) {
                    let new_text = format!("{}{}", head, completed);
                    self.set_text(&new_text);
                    return;
                }
            }
        }

        // Fallback to command completion.
        let matches: Vec<&str> = crate::runner::commands::known_commands()
            .filter(|cmd| cmd.starts_with(tail))
            .collect();
        if matches.is_empty() {
            return;
        }
        let replacement = if matches.len() == 1 {
            matches[0].to_string()
        } else {
            longest_common_prefix(&matches).unwrap_or_else(|| tail.to_string())
        };
        if replacement.len() > tail.len() || matches.len() == 1 {
            let new_text = format!("{}{}", head, replacement);
            self.set_text(&new_text);
        }
    }

    /// Hide the command line and clear transient state.
    fn close(&mut self) {
        self.visible = false;
        self.history_index = None;
        self.draft.clear();
        self.set_text("");
    }
}

/// Render the command-line textarea into the given area.
pub fn render(f: &mut Frame, area: Rect, state: &CommandLineState) {
    f.render_widget(&state.textarea, area);
}

/// Handle input while the command line is active.
///
/// Returns `Ok(true)` only when the application should exit; otherwise `Ok(false)`
/// so the main loop continues.
pub fn handle_input(app: &mut App, code: KeyCode) -> anyhow::Result<bool> {
    let mut to_execute: Option<String> = None;

    let cwd_for_completion = if matches!(code, KeyCode::Tab) {
        Some(app.active_panel().cwd.clone())
    } else {
        None
    };

    if let Some(cmd) = &mut app.command_line {
        match code {
            KeyCode::Esc => cmd.close(),
            KeyCode::Enter => {
                let current = cmd.current_text();
                cmd.push_history(&current);
                to_execute = Some(current);
                cmd.close();
            }
            KeyCode::Up => cmd.history_prev(),
            KeyCode::Down => cmd.history_next(),
            KeyCode::Tab => {
                let cwd = cwd_for_completion.as_deref();
                cmd.apply_completion_with(cwd);
            }
            _ => cmd.feed_textarea(code),
        }
    }

    if let Some(cmd) = to_execute {
        let _ = crate::runner::commands::execute_command(app, &cmd);
    }

    Ok(false)
}

/// Map the crate-local KeyCode into a tui-textarea Input.
fn keycode_to_input(code: KeyCode) -> Option<TextInput> {
    let key = match code {
        KeyCode::Char(c) => TextKey::Char(c),
        KeyCode::Backspace => TextKey::Backspace,
        KeyCode::Left => TextKey::Left,
        KeyCode::Right => TextKey::Right,
        KeyCode::Up => TextKey::Up,
        KeyCode::Down => TextKey::Down,
        KeyCode::Home => TextKey::Home,
        KeyCode::End => TextKey::End,
        KeyCode::PageUp => TextKey::PageUp,
        KeyCode::PageDown => TextKey::PageDown,
        KeyCode::Delete => TextKey::Delete,
        KeyCode::Tab => TextKey::Tab,
        KeyCode::Esc => TextKey::Esc,
        _ => return None,
    };
    Some(TextInput { key, ctrl: false, alt: false, shift: false })
}

/// Compute the longest common prefix from a set of strings.
fn longest_common_prefix(words: &[&str]) -> Option<String> {
    let first = words.first()?.to_string();
    let mut prefix = first;
    for w in words.iter().skip(1) {
        let mut len = 0;
        for (a, b) in prefix.chars().zip(w.chars()) {
            if a == b {
                len += a.len_utf8();
            } else {
                break;
            }
        }
        prefix.truncate(len);
        if prefix.is_empty() {
            break;
        }
    }
    Some(prefix)
}

/// Return true if the token should be treated as a path (absolute, relative, or contains a separator).
fn is_pathy_token(token: &str) -> bool {
    token.starts_with('/') || token.starts_with('.') || token.starts_with('~') || token.contains(std::path::MAIN_SEPARATOR)
}

/// Attempt to complete a filesystem path relative to `cwd` using the provided token.
fn complete_path(token: &str, cwd: &Path) -> Option<String> {
    // Expand simple "~" to home if present.
    let token = if token.starts_with("~") {
        if let Some(home) = directories_next::BaseDirs::new().map(|b| b.home_dir().to_path_buf()) {
            token.replacen("~", home.to_string_lossy().as_ref(), 1)
        } else {
            token.to_string()
        }
    } else {
        token.to_string()
    };

    let candidate_path = PathBuf::from(&token);
    let (dir, prefix) = if candidate_path.is_absolute() {
        if candidate_path.is_dir() {
            (candidate_path.clone(), String::new())
        } else {
            (candidate_path.parent().unwrap_or_else(|| Path::new("/")).to_path_buf(), candidate_path.file_name().map(|s| s.to_string_lossy().to_string()).unwrap_or_default())
        }
    } else {
        // Relative
        let joined = cwd.join(&candidate_path);
        if token.ends_with(std::path::MAIN_SEPARATOR) || joined.is_dir() {
            (joined, String::new())
        } else {
            (
                joined.parent().unwrap_or(cwd).to_path_buf(),
                joined.file_name().map(|s| s.to_string_lossy().to_string()).unwrap_or_default(),
            )
        }
    };

    let rd = fs::read_dir(&dir).ok()?;
    let mut matches = Vec::new();
    for entry in rd.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with(&prefix) {
            let built = if candidate_path.is_absolute() {
                dir.join(&name)
            } else {
                // Convert back to relative form with respect to cwd
                dir.strip_prefix(cwd).unwrap_or(&dir).join(&name)
            };
            let mut s = pathbuf_to_string(&built);
            if entry.path().is_dir() && !s.ends_with(std::path::MAIN_SEPARATOR) {
                s.push(std::path::MAIN_SEPARATOR);
            }
            matches.push(s);
        }
    }

    if matches.is_empty() {
        None
    } else if matches.len() == 1 {
        Some(matches.remove(0))
    } else {
        longest_common_prefix(matches.iter().map(|s| s.as_str()).collect::<Vec<_>>().as_slice())
    }
}

fn pathbuf_to_string(p: &Path) -> String {
    p.to_string_lossy().to_string()
}
