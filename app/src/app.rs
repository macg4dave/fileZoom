use std::cmp::min;
use std::fs;
use std::io;
use std::path::PathBuf;

use chrono::{DateTime, Local};

#[derive(Clone)]
pub struct Entry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: u64,
    pub modified: Option<DateTime<Local>>,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SortKey { Name, Size, Modified }

pub enum Mode {
    Normal,
    Confirm { msg: String, on_yes: Action },
    Input { prompt: String, buffer: String, kind: InputKind },
}

pub enum InputKind { Copy, Move, Rename, NewFile, NewDir }

#[allow(dead_code)]
pub enum Action { DeleteSelected, CopyTo(PathBuf), MoveTo(PathBuf), RenameTo(String), NewFile(String), NewDir(String) }

pub struct App {
    pub cwd: PathBuf,
    pub entries: Vec<Entry>,
    pub selected: usize,
    pub offset: usize,
    pub preview: String,
    pub preview_offset: usize,
    pub mode: Mode,
    pub sort: SortKey,
    pub sort_desc: bool,
}

impl App {
    pub fn new() -> io::Result<Self> {
        let cwd = std::env::current_dir()?;
        let mut app = App { cwd, entries: Vec::new(), selected: 0, offset: 0, preview: String::new(), preview_offset: 0, mode: Mode::Normal, sort: SortKey::Name, sort_desc: false };
        app.refresh()?;
        Ok(app)
    }

    pub fn refresh(&mut self) -> io::Result<()> {
        let mut ents = Vec::new();
        for entry in fs::read_dir(&self.cwd)? {
            let e = entry?;
            let meta = e.metadata()?;
            let modified = meta.modified().ok().map(|t| DateTime::<Local>::from(t));
            ents.push(Entry { name: e.file_name().to_string_lossy().into_owned(), path: e.path(), is_dir: meta.is_dir(), size: meta.len(), modified });
        }
        match self.sort {
            SortKey::Name => ents.sort_by(|a,b| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
            SortKey::Size => ents.sort_by(|a,b| a.size.cmp(&b.size)),
            SortKey::Modified => ents.sort_by(|a,b| a.modified.cmp(&b.modified)),
        }
        if self.sort_desc { ents.reverse(); }
        // keep directories first when sorting by name
        if self.sort == SortKey::Name {
            ents.sort_by(|a,b| match (a.is_dir, b.is_dir) { (true,false) => std::cmp::Ordering::Less, (false,true) => std::cmp::Ordering::Greater, _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()) });
            if self.sort_desc { ents.reverse(); }
        }

        self.entries = ents;
        self.selected = min(self.selected, self.entries.len().saturating_sub(1));
        self.update_preview();
        Ok(())
    }

    pub fn update_preview(&mut self) {
        self.preview_offset = 0;
        if let Some(e) = self.entries.get(self.selected) {
            if e.is_dir {
                let mut s = format!("Directory: {}\n", e.path.display());
                if let Ok(list) = fs::read_dir(&e.path) {
                    for ent in list.flatten().take(50) {
                        s.push_str(&format!("{}\n", ent.file_name().to_string_lossy()));
                    }
                }
                self.preview = s;
            } else {
                match fs::read_to_string(&e.path) {
                    Ok(txt) => self.preview = txt,
                    Err(_) => self.preview = format!("Binary or unreadable file: {}", e.name),
                }
            }
        } else {
            self.preview.clear();
        }
    }

    pub fn ensure_selection_visible(&mut self, list_height: usize) {
        if self.selected < self.offset {
            self.offset = self.selected;
        } else if self.selected >= self.offset + list_height {
            self.offset = self.selected + 1 - list_height;
        }
    }

    pub fn next(&mut self, list_height: usize) {
        if !self.entries.is_empty() {
            self.selected = min(self.selected + 1, self.entries.len() - 1);
            self.ensure_selection_visible(list_height);
            self.update_preview();
        }
    }

    pub fn previous(&mut self, list_height: usize) {
        if !self.entries.is_empty() {
            self.selected = self.selected.saturating_sub(1);
            self.ensure_selection_visible(list_height);
            self.update_preview();
        }
    }

    pub fn page_down(&mut self, list_height: usize) {
        if !self.entries.is_empty() {
            self.selected = min(self.selected + list_height, self.entries.len() - 1);
            self.ensure_selection_visible(list_height);
            self.update_preview();
        }
    }

    pub fn page_up(&mut self, list_height: usize) {
        if !self.entries.is_empty() {
            self.selected = self.selected.saturating_sub(list_height);
            self.ensure_selection_visible(list_height);
            self.update_preview();
        }
    }

    pub fn enter(&mut self) -> io::Result<()> {
        if let Some(e) = self.entries.get(self.selected) {
            if e.is_dir {
                self.cwd = e.path.clone();
                self.refresh()?;
            }
        }
        Ok(())
    }

    pub fn go_up(&mut self) -> io::Result<()> {
        if let Some(parent) = self.cwd.parent() {
            self.cwd = parent.to_path_buf();
            self.refresh()?;
        }
        Ok(())
    }

    pub fn delete_selected(&mut self) -> io::Result<()> {
        if let Some(e) = self.entries.get(self.selected) {
            if e.is_dir {
                fs::remove_dir_all(&e.path)?;
            } else {
                fs::remove_file(&e.path)?;
            }
            self.refresh()?;
        }
        Ok(())
    }

    pub fn copy_selected_to(&mut self, dst: PathBuf) -> io::Result<()> {
        if let Some(src) = self.entries.get(self.selected) {
            let target = if dst.is_dir() || dst.to_string_lossy().ends_with('/') {
                dst.join(&src.name)
            } else {
                dst
            };
            if src.is_dir {
                // simple recursive copy
                self.copy_recursive(&src.path, &target)?;
            } else {
                if let Some(p) = target.parent() { fs::create_dir_all(p)?; }
                fs::copy(&src.path, &target)?;
            }
            self.refresh()?;
        }
        Ok(())
    }

    pub fn move_selected_to(&mut self, dst: PathBuf) -> io::Result<()> {
        if let Some(src) = self.entries.get(self.selected) {
            let target = if dst.is_dir() || dst.to_string_lossy().ends_with('/') {
                dst.join(&src.name)
            } else {
                dst
            };
            if let Some(p) = target.parent() { fs::create_dir_all(p)?; }
            fs::rename(&src.path, &target)?;
            self.refresh()?;
        }
        Ok(())
    }

    pub fn rename_selected_to(&mut self, name: String) -> io::Result<()> {
        if let Some(src) = self.entries.get(self.selected) {
            let target = self.cwd.join(name);
            fs::rename(&src.path, &target)?;
            self.refresh()?;
        }
        Ok(())
    }

    pub fn new_file(&mut self, name: String) -> io::Result<()> {
        let p = self.cwd.join(name);
        if let Some(parent) = p.parent() { fs::create_dir_all(parent)?; }
        fs::File::create(p)?;
        self.refresh()?;
        Ok(())
    }

    pub fn new_dir(&mut self, name: String) -> io::Result<()> {
        let p = self.cwd.join(name);
        fs::create_dir_all(p)?;
        self.refresh()?;
        Ok(())
    }

    fn copy_recursive(&self, src: &PathBuf, dst: &PathBuf) -> io::Result<()> {
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let file_name = entry.file_name();
            let child_src = entry.path();
            let child_dst = dst.join(file_name);
            if child_src.is_dir() {
                self.copy_recursive(&child_src, &child_dst)?;
            } else {
                if let Some(p) = child_dst.parent() { fs::create_dir_all(p)?; }
                fs::copy(&child_src, &child_dst)?;
            }
        }
        Ok(())
    }
}
