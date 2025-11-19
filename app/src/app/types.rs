use chrono::{DateTime, Local};
use std::path::PathBuf;

/// A directory entry displayed in a panel.
///
/// This is a lightweight representation used by the UI layer; it intentionally
/// stores a `PathBuf` and a precomputed `name` to avoid repeated allocations
/// while rendering.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: u64,
    pub modified: Option<DateTime<Local>>,
}

/// Keys by which listings may be sorted.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SortKey {
    Name,
    Size,
    Modified,
}

/// Mode represents the global UI mode/state the application may be in.
///
/// - `Normal` is the default browsing mode.
/// - `Confirm` is used for yes/no prompts (for example, delete).
/// - `Message` displays an information dialog with buttons.
/// - `Input` requests textual input from the user.
#[derive(Clone, Debug)]
pub enum Mode {
    Normal,
    Confirm {
        msg: String,
        on_yes: Action,
        selected: usize,
    },
    Message {
        title: String,
        content: String,
        buttons: Vec<String>,
        selected: usize,
    },
    Input {
        prompt: String,
        buffer: String,
        kind: InputKind,
    },
}

/// The kind of input requested from the user. This guides how the input buffer
/// is interpreted (e.g. a destination path vs a filename).
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum InputKind {
    Copy,
    Move,
    Rename,
    NewFile,
    NewDir,
    ChangePath,
}

/// Actions represent high-level user requests executed by the runner.
///
/// These are intentionally simple enums so they can be passed around the UI
/// and executed by `runner::commands::perform_action`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    DeleteSelected,
    CopyTo(PathBuf),
    MoveTo(PathBuf),
    RenameTo(String),
    NewFile(String),
    NewDir(String),
}

/// Which panel is active.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Side {
    Left,
    Right,
}


