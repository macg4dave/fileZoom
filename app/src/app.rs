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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Side { Left, Right }

pub struct Panel {
    pub cwd: PathBuf,
    pub entries: Vec<Entry>,
    pub selected: usize,
    pub offset: usize,
    pub preview: String,
    pub preview_offset: usize,
}

impl Panel {
    fn new(cwd: PathBuf) -> Self {
        Panel { cwd, entries: Vec::new(), selected: 0, offset: 0, preview: String::new(), preview_offset: 0 }
    }
}

pub struct App {
    pub left: Panel,
    pub right: Panel,
    pub active: Side,
    pub mode: Mode,
    pub sort: SortKey,
    pub sort_desc: bool,
}

impl App {
    pub fn new() -> io::Result<Self> {
        let cwd = std::env::current_dir()?;
        let mut app = App { left: Panel::new(cwd.clone()), right: Panel::new(cwd), active: Side::Left, mode: Mode::Normal, sort: SortKey::Name, sort_desc: false };
        app.refresh()?;
        Ok(app)
    }

    pub fn refresh(&mut self) -> io::Result<()> {
        self.refresh_panel(Side::Left)?;
        self.refresh_panel(Side::Right)?;
        Ok(())
    }

    fn refresh_panel(&mut self, side: Side) -> io::Result<()> {
        let panel = match side {
            Side::Left => &mut self.left,
            Side::Right => &mut self.right,
        };
        let mut ents = Vec::new();
        for entry in fs::read_dir(&panel.cwd)? {
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
        if self.sort == SortKey::Name {
            ents.sort_by(|a,b| match (a.is_dir, b.is_dir) { (true,false) => std::cmp::Ordering::Less, (false,true) => std::cmp::Ordering::Greater, _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()) });
            if self.sort_desc { ents.reverse(); }
        }

        panel.entries = ents;
        panel.selected = min(panel.selected, panel.entries.len().saturating_sub(1));
        self.update_preview_for(side);
        Ok(())
    }

    pub fn update_preview_for(&mut self, side: Side) {
        let panel = match side {
            Side::Left => &mut self.left,
            Side::Right => &mut self.right,
        };
        panel.preview_offset = 0;
        if let Some(e) = panel.entries.get(panel.selected) {
            if e.is_dir {
                let mut s = format!("Directory: {}\n", e.path.display());
                if let Ok(list) = fs::read_dir(&e.path) {
                    for ent in list.flatten().take(50) {
                        s.push_str(&format!("{}\n", ent.file_name().to_string_lossy()));
                    }
                }
                panel.preview = s;
            } else {
                match fs::read_to_string(&e.path) {
                    Ok(txt) => panel.preview = txt,
                    Err(_) => panel.preview = format!("Binary or unreadable file: {}", e.name),
                }
            }
        } else {
            panel.preview.clear();
        }
    }

    pub fn ensure_selection_visible(&mut self, list_height: usize) {
        let panel = match self.active {
            Side::Left => &mut self.left,
            Side::Right => &mut self.right,
        };
        if panel.selected < panel.offset {
            panel.offset = panel.selected;
        } else if panel.selected >= panel.offset + list_height {
            panel.offset = panel.selected + 1 - list_height;
        }
    }

    pub fn next(&mut self, list_height: usize) {
        match self.active {
            Side::Left => {
                if !self.left.entries.is_empty() {
                    self.left.selected = min(self.left.selected + 1, self.left.entries.len() - 1);
                }
            }
            Side::Right => {
                if !self.right.entries.is_empty() {
                    self.right.selected = min(self.right.selected + 1, self.right.entries.len() - 1);
                }
            }
        }
        self.ensure_selection_visible(list_height);
        self.update_preview_for(self.active);
    }

    pub fn previous(&mut self, list_height: usize) {
        match self.active {
            Side::Left => {
                if !self.left.entries.is_empty() {
                    self.left.selected = self.left.selected.saturating_sub(1);
                }
            }
            Side::Right => {
                if !self.right.entries.is_empty() {
                    self.right.selected = self.right.selected.saturating_sub(1);
                }
            }
        }
        self.ensure_selection_visible(list_height);
        self.update_preview_for(self.active);
    }

    pub fn page_down(&mut self, list_height: usize) {
        match self.active {
            Side::Left => {
                if !self.left.entries.is_empty() {
                    self.left.selected = min(self.left.selected + list_height, self.left.entries.len() - 1);
                }
            }
            Side::Right => {
                if !self.right.entries.is_empty() {
                    self.right.selected = min(self.right.selected + list_height, self.right.entries.len() - 1);
                }
            }
        }
        self.ensure_selection_visible(list_height);
        self.update_preview_for(self.active);
    }

    pub fn page_up(&mut self, list_height: usize) {
        match self.active {
            Side::Left => {
                if !self.left.entries.is_empty() {
                    self.left.selected = self.left.selected.saturating_sub(list_height);
                }
            }
            Side::Right => {
                if !self.right.entries.is_empty() {
                    self.right.selected = self.right.selected.saturating_sub(list_height);
                }
            }
        }
        self.ensure_selection_visible(list_height);
        self.update_preview_for(self.active);
    }

    pub fn enter(&mut self) -> io::Result<()> {
        match self.active {
            Side::Left => {
                let sel = self.left.selected;
                if let Some(e) = self.left.entries.get(sel) {
                    if e.is_dir {
                        self.left.cwd = e.path.clone();
                        self.refresh_panel(Side::Left)?;
                    }
                }
            }
            Side::Right => {
                let sel = self.right.selected;
                if let Some(e) = self.right.entries.get(sel) {
                    if e.is_dir {
                        self.right.cwd = e.path.clone();
                        self.refresh_panel(Side::Right)?;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn go_up(&mut self) -> io::Result<()> {
        match self.active {
            Side::Left => {
                if let Some(parent) = self.left.cwd.parent() {
                    self.left.cwd = parent.to_path_buf();
                    self.refresh_panel(Side::Left)?;
                }
            }
            Side::Right => {
                if let Some(parent) = self.right.cwd.parent() {
                    self.right.cwd = parent.to_path_buf();
                    self.refresh_panel(Side::Right)?;
                }
            }
        }
        Ok(())
    }

    pub fn delete_selected(&mut self) -> io::Result<()> {
        match self.active {
            Side::Left => {
                let sel = self.left.selected;
                if let Some(e) = self.left.entries.get(sel) {
                    if e.is_dir { fs::remove_dir_all(&e.path)?; } else { fs::remove_file(&e.path)?; }
                    self.refresh_panel(Side::Left)?;
                }
            }
            Side::Right => {
                let sel = self.right.selected;
                if let Some(e) = self.right.entries.get(sel) {
                    if e.is_dir { fs::remove_dir_all(&e.path)?; } else { fs::remove_file(&e.path)?; }
                    self.refresh_panel(Side::Right)?;
                }
            }
        }
        Ok(())
    }

    pub fn copy_selected_to(&mut self, dst: PathBuf) -> io::Result<()> {
        match self.active {
            Side::Left => {
                let sel = self.left.selected;
                if let Some(src) = self.left.entries.get(sel) {
                    let src_path = src.path.clone();
                    let src_name = src.name.clone();
                    let is_dir = src.is_dir;
                    let target = if dst.is_dir() || dst.to_string_lossy().ends_with('/') { dst.join(&src_name) } else { dst };
                    if is_dir { self.copy_recursive(&src_path, &target)?; } else { if let Some(p) = target.parent() { fs::create_dir_all(p)?; } fs::copy(&src_path, &target)?; }
                    self.refresh_panel(Side::Left)?;
                }
            }
            Side::Right => {
                let sel = self.right.selected;
                if let Some(src) = self.right.entries.get(sel) {
                    let src_path = src.path.clone();
                    let src_name = src.name.clone();
                    let is_dir = src.is_dir;
                    let target = if dst.is_dir() || dst.to_string_lossy().ends_with('/') { dst.join(&src_name) } else { dst };
                    if is_dir { self.copy_recursive(&src_path, &target)?; } else { if let Some(p) = target.parent() { fs::create_dir_all(p)?; } fs::copy(&src_path, &target)?; }
                    self.refresh_panel(Side::Right)?;
                }
            }
        }
        Ok(())
    }

    pub fn move_selected_to(&mut self, dst: PathBuf) -> io::Result<()> {
        match self.active {
            Side::Left => {
                let sel = self.left.selected;
                if let Some(src) = self.left.entries.get(sel) {
                    let src_path = src.path.clone();
                    let src_name = src.name.clone();
                    let target = if dst.is_dir() || dst.to_string_lossy().ends_with('/') { dst.join(&src_name) } else { dst };
                    if let Some(p) = target.parent() { fs::create_dir_all(p)?; }
                    fs::rename(&src_path, &target)?;
                    self.refresh_panel(Side::Left)?;
                }
            }
            Side::Right => {
                let sel = self.right.selected;
                if let Some(src) = self.right.entries.get(sel) {
                    let src_path = src.path.clone();
                    let src_name = src.name.clone();
                    let target = if dst.is_dir() || dst.to_string_lossy().ends_with('/') { dst.join(&src_name) } else { dst };
                    if let Some(p) = target.parent() { fs::create_dir_all(p)?; }
                    fs::rename(&src_path, &target)?;
                    self.refresh_panel(Side::Right)?;
                }
            }
        }
        Ok(())
    }

    pub fn rename_selected_to(&mut self, name: String) -> io::Result<()> {
        match self.active {
            Side::Left => {
                let sel = self.left.selected;
                if let Some(src) = self.left.entries.get(sel) {
                    let src_path = src.path.clone();
                    let target = self.left.cwd.join(name);
                    fs::rename(&src_path, &target)?;
                    self.refresh_panel(Side::Left)?;
                }
            }
            Side::Right => {
                let sel = self.right.selected;
                if let Some(src) = self.right.entries.get(sel) {
                    let src_path = src.path.clone();
                    let target = self.right.cwd.join(name);
                    fs::rename(&src_path, &target)?;
                    self.refresh_panel(Side::Right)?;
                }
            }
        }
        Ok(())
    }

    pub fn new_file(&mut self, name: String) -> io::Result<()> {
        match self.active {
            Side::Left => {
                let p = self.left.cwd.join(name);
                if let Some(parent) = p.parent() { fs::create_dir_all(parent)?; }
                fs::File::create(p)?;
                self.refresh_panel(Side::Left)?;
            }
            Side::Right => {
                let p = self.right.cwd.join(name);
                if let Some(parent) = p.parent() { fs::create_dir_all(parent)?; }
                fs::File::create(p)?;
                self.refresh_panel(Side::Right)?;
            }
        }
        Ok(())
    }

    pub fn new_dir(&mut self, name: String) -> io::Result<()> {
        match self.active {
            Side::Left => {
                let p = self.left.cwd.join(name);
                fs::create_dir_all(p)?;
                self.refresh_panel(Side::Left)?;
            }
            Side::Right => {
                let p = self.right.cwd.join(name);
                fs::create_dir_all(p)?;
                self.refresh_panel(Side::Right)?;
            }
        }
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
