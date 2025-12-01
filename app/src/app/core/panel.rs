use crate::app::types::Entry;
use chrono::{DateTime, Local};
use globset::{GlobBuilder, GlobMatcher};
use std::collections::HashSet;
use std::io;
use std::path::PathBuf;
use walkdir::WalkDir;

/// Panel holds the minimal, UI-independent state for one side of the
/// dual-pane file manager. It intentionally keeps presentation details
/// (such as rendering rows) out of the model so the core can be tested
/// without a terminal.
/// Modes a panel can display entries in. UI may render these differently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PanelMode {
    /// Full listing (default) — all columns and metadata.
    #[default]
    Full,
    /// Brief listing: compact single-line entries.
    Brief,
    /// Tree listing: show recursive tree (UI decides how to present).
    Tree,
    /// Flat listing: flatten subdirectories into a single list.
    Flat,
    /// Quick view mode emphasising a selected entry with less metadata.
    QuickView,
}


pub struct Panel {
    /// Current working directory shown by this panel.
    pub cwd: PathBuf,
    /// Listing of filesystem entries in the directory. This vector
    /// is domain-only (contains no UI synthetic rows like the header or `..`).
    pub entries: Vec<Entry>,
    /// UI selection index including any synthetic header/parent rows.
    /// This keeps the visual selection intuitive without coupling the
    /// panel data model to presentation details.
    pub selected: usize,
    /// UI scroll offset (index of the top-most visible UI row).
    pub offset: usize,
    /// File preview text for the selected entry (if any).
    pub preview: String,
    /// Scroll offset for the preview text.
    pub preview_offset: usize,
    /// Selected entry indices for multi-selection (domain indexes into `entries`).
    pub selections: HashSet<usize>,
    /// The display mode for the panel (Full/Brief/Tree/Flat/QuickView).
    pub mode: PanelMode,
    /// Optional glob filter applied to entry names.
    pub filter_pattern: Option<String>,
    filter_matcher: Option<GlobMatcher>,
}

impl Panel {
    /// Create a new panel rooted at `cwd` with sensible defaults.
    pub fn new(cwd: PathBuf) -> Self {
        Panel {
            cwd,
            entries: Vec::new(),
            selected: 0,
            offset: 0,
            preview: String::new(),
            preview_offset: 0,
            selections: HashSet::new(),
            filter_pattern: None,
            filter_matcher: None,
            mode: PanelMode::default(),
        }
    }

    /// Set or clear the quick filter for this panel. Empty input clears it.
    pub fn set_filter(&mut self, pattern: &str) -> Result<(), globset::Error> {
        let trimmed = pattern.trim();
        if trimmed.is_empty() {
            self.filter_pattern = None;
            self.filter_matcher = None;
            return Ok(());
        }

        // If the pattern has no glob meta, treat it as a contains check by
        // surrounding it with '*' so simple strings still match.
        let has_meta = trimmed.chars().any(|c| matches!(c, '*' | '?' | '[' | '{'));
        let glob_str = if has_meta { trimmed.to_string() } else { format!("*{}*", trimmed) };
        let matcher = GlobBuilder::new(&glob_str)
            .case_insensitive(true)
            .literal_separator(true)
            .build()?
            .compile_matcher();

        self.filter_pattern = Some(trimmed.to_string());
        self.filter_matcher = Some(matcher);
        Ok(())
    }

    pub(crate) fn apply_filter(&self, entries: Vec<Entry>) -> Vec<Entry> {
        if let Some(m) = &self.filter_matcher {
            entries.into_iter().filter(|e| m.is_match(&e.name)).collect()
        } else {
            entries
        }
    }

    /// Toggle selection of the currently selected entry (if any).
    pub fn toggle_selection(&mut self) {
        if let Some(idx) = super::utils::ui_to_entry_index(self.selected, self) {
            // `HashSet::remove` returns whether the value was present.
            // If it wasn't present, insert it (toggle behaviour).
            if !self.selections.remove(&idx) {
                self.selections.insert(idx);
            }
        }
    }

    /// Select all entries in the current listing (domain indices).
    pub fn select_all(&mut self) {
        self.selections.clear();
        for i in 0..self.entries.len() {
            self.selections.insert(i);
        }
    }

    /// Invert current selection: items selected become unselected, and vice-versa.
    pub fn invert_selection(&mut self) {
        let mut new = HashSet::new();
        for i in 0..self.entries.len() {
            if !self.selections.contains(&i) {
                new.insert(i);
            }
        }
        self.selections = new;
    }

    /// Select entries whose name matches `pattern` (glob-style). This does not
    /// alter the panel's quick-filter — it performs an independent selection.
    pub fn select_by_pattern(&mut self, pattern: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let trimmed = pattern.trim();
        if trimmed.is_empty() {
            self.selections.clear();
            return Ok(());
        }

        // Advanced: support regex and attribute filters.
        // Regex: prefix pattern with "re:" to interpret the remainder as a regular expression.
        if let Some(re_pat) = trimmed.strip_prefix("re:") {
            let re = regex::RegexBuilder::new(re_pat.trim()).case_insensitive(true).build().map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
            self.selections.clear();
            for (i, e) in self.entries.iter().enumerate() {
                if re.is_match(&e.name) {
                    self.selections.insert(i);
                }
            }
            return Ok(());
        }

        // Attribute filters: size>num, size<num, size=num, modified>RFC3339, modified<RFC3339
        if trimmed.starts_with("size>") || trimmed.starts_with("size<") || trimmed.starts_with("size=") {
            let op = trimmed.chars().nth(4).unwrap_or('=');
            let num_str = trimmed[5..].trim();
            if let Ok(threshold) = num_str.parse::<u64>() {
                self.selections.clear();
                for (i, e) in self.entries.iter().enumerate() {
                    match op {
                        '>' => if e.size > threshold { self.selections.insert(i); },
                        '<' => if e.size < threshold { self.selections.insert(i); },
                        '=' => if e.size == threshold { self.selections.insert(i); },
                        _ => {}
                    }
                }
            }
            return Ok(());
        }

        if trimmed.starts_with("modified>") || trimmed.starts_with("modified<") {
            let op = trimmed.chars().nth(8).unwrap_or('>');
            let ts_str = trimmed[9..].trim();
            if let Ok(dt) = DateTime::parse_from_rfc3339(ts_str) {
                let dt_local: DateTime<Local> = dt.with_timezone(&Local);
                self.selections.clear();
                for (i, e) in self.entries.iter().enumerate() {
                    if let Some(mod_t) = e.modified {
                        match op {
                            '>' => if mod_t > dt_local { self.selections.insert(i); },
                            '<' => if mod_t < dt_local { self.selections.insert(i); },
                            _ => {}
                        }
                    }
                }
            }
            return Ok(());
        }

        let has_meta = trimmed.chars().any(|c| matches!(c, '*' | '?' | '[' | '{'));
        let glob_str = if has_meta { trimmed.to_string() } else { format!("*{}*", trimmed) };
        let matcher = GlobBuilder::new(&glob_str)
            .case_insensitive(true)
            .literal_separator(true)
            .build()?
            .compile_matcher();

        self.selections.clear();
        for (i, e) in self.entries.iter().enumerate() {
            if matcher.is_match(&e.name) {
                self.selections.insert(i);
            }
        }
        Ok(())
    }

    /// Cycle the panel's display mode through Full -> Brief -> Tree -> Flat -> QuickView -> Full.
    pub fn cycle_mode(&mut self) {
        self.mode = match self.mode {
            PanelMode::Full => PanelMode::Brief,
            PanelMode::Brief => PanelMode::Tree,
            PanelMode::Tree => PanelMode::Flat,
            PanelMode::Flat => PanelMode::QuickView,
            PanelMode::QuickView => PanelMode::Full,
        }
    }

    /// Clear all selections in this panel.
    pub fn clear_selections(&mut self) {
        self.selections.clear();
    }

    /// Return a reference to the currently selected entry, if present.
    /// Return a reference to the currently selected filesystem entry,
    /// if the UI selected index refers to an actual item (i.e. not the
    /// header or the parent row).
    pub fn selected_entry(&self) -> Option<&Entry> {
        super::utils::ui_to_entry_index(self.selected, self)
            .and_then(|idx| self.entries.get(idx))
    }

    /// Move selection down by one, clamping at the last UI row.
    pub fn select_next(&mut self) {
        let max_rows = super::utils::ui_row_count(self);
        if self.selected + 1 < max_rows {
            self.selected += 1;
        }
    }

    /// Move selection up by one, clamping at zero (makes header selectable).
    pub fn select_prev(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    /// Ensure `selected` is within bounds of the UI rows (header +
    /// maybe parent + entries).
    pub fn clamp_selected(&mut self) {
        let max_rows = super::utils::ui_row_count(self);
        if max_rows == 0 {
            self.selected = 0;
        } else {
            self.selected = std::cmp::min(self.selected, max_rows.saturating_sub(1));
        }
    }

    /// Adjust `offset` so the selected row is visible within a viewport of
    /// `height` rows. Note that UI rows include synthetic header and parent rows.
    pub fn ensure_selected_visible(&mut self, height: usize) {
        if height == 0 {
            self.offset = 0;
            return;
        }
        let total_rows = super::utils::ui_row_count(self);
        if total_rows == 0 {
            self.offset = 0;
            return;
        }
        // If `selected` is above the viewport, bring it to the top.
        if self.selected < self.offset {
            self.offset = self.selected;
            return;
        }

        // If `selected` is below the viewport, move the offset so it becomes
        // visible at the bottom of the viewport (or as low as possible).
        let max_offset = total_rows.saturating_sub(height);
        if self.selected >= self.offset + height {
            let desired = self.selected + 1 - height;
            self.offset = std::cmp::min(desired, max_offset);
        } else if self.offset > max_offset {
            // Clamp offset when viewport is larger than the remaining rows.
            self.offset = max_offset;
        }
    }

    /// Replace the preview text and reset the preview scroll offset.
    pub fn set_preview(&mut self, text: String) {
        self.preview = text;
        self.preview_offset = 0;
    }

    /// Read directory entries and return a Vec<Entry>.
    /// This centralises the filesystem access and metadata reading used by
    /// `App::refresh_panel` and keeps the Panel's path-related concerns in one place.
    /// Read the immediate children of the panel's `cwd` and return them as
    /// a `Vec<Entry>`. This is intentionally a thin wrapper around
    /// filesystem access so callers can handle errors appropriately.
    pub(crate) fn read_entries(&self) -> io::Result<Vec<Entry>> {
        let mut entries_vec = Vec::new();

        for dir_entry_result in WalkDir::new(&self.cwd)
            .min_depth(1)
            .max_depth(1)
            .follow_links(false)
        {
            let dir_entry = dir_entry_result
                .map_err(io::Error::other)?;

            let metadata = dir_entry.metadata()?;
            let modified_time = metadata.modified().ok().map(DateTime::<Local>::from);
            let name = dir_entry.file_name().to_string_lossy().into_owned();
            let path_buf = dir_entry.path().to_path_buf();

            let mut file_entry = if metadata.is_dir() {
                Entry::directory(name, path_buf.clone(), modified_time)
            } else {
                Entry::file(name, path_buf.clone(), metadata.len(), modified_time)
            };

            // Best-effort: populate permission/ownership flags using the
            // existing helpers. Failure to inspect is tolerated.
            if let Ok(perms) = crate::fs_op::permissions::inspect_permissions(&path_buf, false)
            {
                file_entry.unix_mode = perms.unix_mode;
                file_entry.can_read = Some(perms.can_read);
                file_entry.can_write = Some(perms.can_write);
                file_entry.can_execute = Some(perms.can_execute);
            }

            // Best-effort: uid/gid when available on unix platforms.
            #[cfg(unix)]
            {
                use std::os::unix::fs::MetadataExt;
                file_entry.uid = Some(metadata.uid());
                file_entry.gid = Some(metadata.gid());

                // Best-effort: resolve uid/gid to names for display
                // Use the `users` crate which works cross-platform.
                if let Some(u) = users::get_user_by_uid(metadata.uid()) {
                    file_entry.owner = Some(u.name().to_string_lossy().into_owned());
                }
                if let Some(g) = users::get_group_by_gid(metadata.gid()) {
                    file_entry.group = Some(g.name().to_string_lossy().into_owned());
                }
            }
            #[cfg(not(unix))]
            {
                // populate the uid/gid fields where possible via metadata but
                // avoid making platform assumptions about user/group resolution
                file_entry.uid = None;
                file_entry.gid = None;
            }

            entries_vec.push(file_entry);
        }

        Ok(entries_vec)
    }

    /// Get a recursive tree of entries starting at this panel's cwd.
    /// Returns entries paired with their depth (0 => immediate child).
    /// Depth is limited by `max_depth` to avoid pathological recursion in tests.
    pub fn tree_entries(&self, max_depth: usize) -> io::Result<Vec<(Entry, usize)>> {
        let mut entries_vec = Vec::new();

        for dir_entry_result in WalkDir::new(&self.cwd)
            .min_depth(1)
            .max_depth(max_depth)
            .follow_links(false)
        {
            let dir_entry = dir_entry_result.map_err(io::Error::other)?;

            let metadata = dir_entry.metadata()?;
            let depth = dir_entry.depth().saturating_sub(1) as usize;
            let modified_time = metadata.modified().ok().map(DateTime::<Local>::from);
            let name = dir_entry.file_name().to_string_lossy().into_owned();
            let path_buf = dir_entry.path().to_path_buf();

            let mut file_entry = if metadata.is_dir() {
                Entry::directory(name, path_buf.clone(), modified_time)
            } else {
                Entry::file(name, path_buf.clone(), metadata.len(), modified_time)
            };

            if let Ok(perms) = crate::fs_op::permissions::inspect_permissions(&path_buf, false) {
                file_entry.unix_mode = perms.unix_mode;
                file_entry.can_read = Some(perms.can_read);
                file_entry.can_write = Some(perms.can_write);
                file_entry.can_execute = Some(perms.can_execute);
            }

            entries_vec.push((file_entry, depth));
        }

        Ok(entries_vec)
    }

    /// Flatten the directory tree under this panel's cwd up to `max_depth`.
    /// The returned entries are in walk order.
    pub fn flat_entries(&self, max_depth: usize) -> io::Result<Vec<Entry>> {
        let mut out = Vec::new();
        for (e, _) in self.tree_entries(max_depth)? {
            out.push(e);
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;

    #[test]
    fn read_entries_returns_all_entries() {
        let temp = assert_fs::TempDir::new().unwrap();
        temp.child("a.txt").write_str("a").unwrap();
        temp.child("subdir").create_dir_all().unwrap();

        let p = Panel::new(temp.path().to_path_buf());
        let entries = p.read_entries().unwrap();
        // Expect at least the file and the directory
        let mut names: Vec<String> = entries.into_iter().map(|e| e.name).collect();
        names.sort();
        assert!(names.contains(&"a.txt".to_string()));
        assert!(names.contains(&"subdir".to_string()));
    }

    #[test]
    fn read_entries_empty_dir_returns_empty() {
        let temp = assert_fs::TempDir::new().unwrap();
        // no children created

        let p = Panel::new(temp.path().to_path_buf());
        let entries = p.read_entries().unwrap();
        assert!(entries.is_empty(), "expected no entries in empty temp dir");
    }

    #[test]
    fn read_entries_populates_permissions_and_owner() {
        let temp = assert_fs::TempDir::new().unwrap();
        let file = temp.child("foo.txt");
        file.write_str("hello").unwrap();

        let p = Panel::new(temp.path().to_path_buf());
        let entries = p.read_entries().unwrap();
        assert!(!entries.is_empty());
        let e = &entries[0];
        // Best-effort checks: permission flags should be set at least
        assert!(e.can_read.is_some(), "expected can_read to be present");
        assert!(e.can_write.is_some(), "expected can_write to be present");

        #[cfg(unix)]
        {
            assert!(e.unix_mode.is_some(), "expected unix_mode on unix");
            assert!(e.uid.is_some(), "expected uid on unix");
            assert!(e.gid.is_some(), "expected gid on unix");
        }
    }

    #[test]
    fn default_mode_is_full_and_can_change() {
        let p = Panel::new(std::path::PathBuf::from("/tmp"));
        assert_eq!(p.mode, PanelMode::Full);
        let mut p = p;
        p.mode = PanelMode::Tree;
        assert_eq!(p.mode, PanelMode::Tree);
    }

    #[test]
    fn select_all_and_invert_selection_behaviour() {
        let mut p = Panel::new(std::path::PathBuf::from("/tmp"));
        p.entries.push(crate::app::types::Entry::file("a.txt", std::path::PathBuf::from("/tmp/a.txt"), 10, None));
        p.entries.push(crate::app::types::Entry::file("b.txt", std::path::PathBuf::from("/tmp/b.txt"), 20, None));
        p.entries.push(crate::app::types::Entry::file("c.log", std::path::PathBuf::from("/tmp/c.log"), 30, None));

        p.select_all();
        assert_eq!(p.selections.len(), 3);

        p.invert_selection();
        // previously all selected -> inverted = none selected
        assert!(p.selections.is_empty());
    }

    #[test]
    fn select_by_pattern_selects_matching_entries() {
        let mut p = Panel::new(std::path::PathBuf::from("/tmp"));
        p.entries.push(crate::app::types::Entry::file("README.md", std::path::PathBuf::from("/tmp/README.md"), 10, None));
        p.entries.push(crate::app::types::Entry::file("main.rs", std::path::PathBuf::from("/tmp/main.rs"), 20, None));
        p.entries.push(crate::app::types::Entry::file("notes.txt", std::path::PathBuf::from("/tmp/notes.txt"), 30, None));

        // pattern without glob meta should match by substring
        p.select_by_pattern("main").unwrap();
        assert_eq!(p.selections.len(), 1);

        // select by extension pattern
        p.select_by_pattern("*.txt").unwrap();
        assert_eq!(p.selections.iter().map(|&i| p.entries[i].name.clone()).collect::<Vec<_>>(), vec!["notes.txt".to_string()]);

        // empty pattern clears selections
        p.select_by_pattern("").unwrap();
        assert!(p.selections.is_empty());
    }

    #[test]
    fn regex_and_attribute_selection() {
        let mut p = Panel::new(std::path::PathBuf::from("/tmp"));
        // use sizes and modified timestamps
        let now = chrono::Local::now();
        p.entries.push(crate::app::types::Entry::file("main.rs", std::path::PathBuf::from("/tmp/main.rs"), 10, Some(now)));
        p.entries.push(crate::app::types::Entry::file("readme.md", std::path::PathBuf::from("/tmp/readme.md"), 200, Some(now)));
        p.entries.push(crate::app::types::Entry::file("notes.txt", std::path::PathBuf::from("/tmp/notes.txt"), 500, Some(now)));

        // regex: match names ending with .rs
        p.select_by_pattern("re:.*\\.rs$").unwrap();
        assert_eq!(p.selections.len(), 1);

        // size > 100 matches two files
        p.select_by_pattern("size>100").unwrap();
        assert_eq!(p.selections.len(), 2);

        // modified > (a timestamp in the past) should select all
        let past = (now - chrono::Duration::days(1)).to_rfc3339();
        p.select_by_pattern(&format!("modified>{}", past)).unwrap();
        assert_eq!(p.selections.len(), 3);
    }

    #[test]
    fn tree_and_flat_traversal() {
        let temp = assert_fs::TempDir::new().unwrap();
        // Create nested structure: a/b/c/file.txt
        temp.child("a/b/c").create_dir_all().unwrap();
        temp.child("a/b/c/file.txt").write_str("hello").unwrap();

        let p = Panel::new(temp.path().to_path_buf());
        let tree = p.tree_entries(4).unwrap();
        assert!(tree.iter().any(|(e, _)| e.name == "a" || e.name == "file.txt"));

        let flat = p.flat_entries(4).unwrap();
        assert!(flat.iter().any(|e| e.name == "file.txt"));
    }
}
