use std::path::PathBuf;

use super::types::Entry;

pub struct Panel {
    pub cwd: PathBuf,
    pub entries: Vec<Entry>,
    pub selected: usize,
    pub offset: usize,
    pub preview: String,
    pub preview_offset: usize,
}

impl Panel {
    pub fn new(cwd: PathBuf) -> Self {
        Panel {
            cwd,
            entries: Vec::new(),
            selected: 0,
            offset: 0,
            preview: String::new(),
            preview_offset: 0,
        }
    }
}
