use chrono::{DateTime, Local};
use std::path::PathBuf;

#[derive(Clone)]
pub struct Entry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: u64,
    pub modified: Option<DateTime<Local>>,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SortKey {
    Name,
    Size,
    Modified,
}

pub enum Mode {
    Normal,
    Confirm {
        msg: String,
        on_yes: Action,
    },
    Input {
        prompt: String,
        buffer: String,
        kind: InputKind,
    },
}

pub enum InputKind {
    Copy,
    Move,
    Rename,
    NewFile,
    NewDir,
    ChangePath,
}

#[allow(dead_code)]
pub enum Action {
    DeleteSelected,
    CopyTo(PathBuf),
    MoveTo(PathBuf),
    RenameTo(String),
    NewFile(String),
    NewDir(String),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Side {
    Left,
    Right,
}
