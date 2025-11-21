//! Small, testable model for the top menu and helpers used by the UI/input
//! handlers.
use std::cmp;

/// Possible actions that a menu item can trigger.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuAction {
    Settings,
    NewFile,
    NewDir,
    Quit,
    Help,
    About,
    Copy,
    Move,
    Sort,
    Noop,
}

/// A single menu item; label plus optional submenu and optional action.
#[derive(Clone, Debug)]
pub struct MenuItem {
    pub label: &'static str,
    pub submenu: Option<Vec<MenuItem>>,
    pub action: Option<MenuAction>,
}

impl MenuItem {
    pub fn new(label: &'static str) -> Self {
        Self { label, submenu: None, action: None }
    }

    pub fn with_action(label: &'static str, action: MenuAction) -> Self {
        Self { label, submenu: None, action: Some(action) }
    }

    pub fn with_submenu(label: &'static str, submenu: Vec<MenuItem>) -> Self {
        Self { label, submenu: Some(submenu), action: None }
    }
}

/// A small container holding the canonical menu structure.
#[derive(Clone, Debug)]
pub struct MenuModel(pub Vec<MenuItem>);

impl MenuModel {
    /// Default menu shown on the top bar. Keep labels in the canonical
    /// order so existing tests and code that depend on `menu_labels()` work.
    pub fn default_model() -> Self {
        Self(vec![
            MenuItem::with_submenu("File", vec![
                MenuItem::with_action("Quit", MenuAction::Quit),
            ]),
            MenuItem::with_action("Copy", MenuAction::Copy),
            MenuItem::with_action("Move", MenuAction::Move),
            MenuItem::with_submenu("New", vec![
                MenuItem::with_action("File", MenuAction::NewFile),
                MenuItem::with_action("Dir", MenuAction::NewDir),
            ]),
            MenuItem::with_action("Sort", MenuAction::Sort),
            MenuItem::with_action("Settings", MenuAction::Settings),
            MenuItem::with_action("Help", MenuAction::Help),
        ])
    }

    pub fn labels(&self) -> Vec<&'static str> {
        self.0.iter().map(|mi| mi.label).collect()
    }
}

/// Small UI state for the menu: which top tab is highlighted, if a submenu
/// is open and which submenu index is selected.
#[derive(Clone, Debug, Default)]
pub struct MenuState {
    pub top_index: usize,
    pub submenu_index: Option<usize>,
    pub open: bool,
}

impl MenuState {
    pub fn index_for_x(x: u16, width: u16, labels: &[&str]) -> usize {
        let n = labels.len();
        if n == 0 || width == 0 { return 0usize; }
        // Distribute width evenly: map x -> index by simple proportion.
        let idx = (x as usize).saturating_mul(n).saturating_div((width as usize).max(1));
        cmp::min(idx, n.saturating_sub(1))
    }

    pub fn open_top(&mut self, idx: usize) {
        self.top_index = idx;
        self.open = true;
        self.submenu_index = Some(0);
    }

    pub fn toggle_top(&mut self, idx: usize) {
        if self.open && self.top_index == idx {
            self.close();
        } else {
            self.open_top(idx);
        }
    }

    pub fn close(&mut self) {
        self.open = false;
        self.submenu_index = None;
    }

    pub fn select_next(&mut self, total: usize) {
        if total == 0 { self.submenu_index = None; return; }
        let mut cur = self.submenu_index.unwrap_or(0);
        cur = (cur + 1) % total;
        self.submenu_index = Some(cur);
    }

    pub fn select_prev(&mut self, total: usize) {
        if total == 0 { self.submenu_index = None; return; }
        let mut cur = self.submenu_index.unwrap_or(0);
        cur = (cur + total - 1) % total;
        self.submenu_index = Some(cur);
    }

    pub fn selected_action(&self, model: &MenuModel) -> Option<MenuAction> {
        let top = model.0.get(self.top_index)?;
        if let Some(sub) = &top.submenu {
            let sidx = self.submenu_index?;
            return sub.get(sidx).and_then(|mi| mi.action);
        }
        // if top has no submenu, return its direct action
        top.action
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_for_x_basic() {
        let labels = ["One", "Two", "Three"];
        // width 3: each column maps to an index
        assert_eq!(MenuState::index_for_x(0, 3, &labels), 0);
        assert_eq!(MenuState::index_for_x(1, 3, &labels), 1);
        assert_eq!(MenuState::index_for_x(2, 3, &labels), 2);
    }

    #[test]
    fn index_for_x_bounds() {
        let labels = ["A", "B"];
        assert_eq!(MenuState::index_for_x(0, 1, &labels), 0);
        assert_eq!(MenuState::index_for_x(0, 0, &labels), 0);
    }

    #[test]
    fn open_toggle_close() {
        let mut s = MenuState::default();
        s.open_top(2);
        assert!(s.open);
        assert_eq!(s.top_index, 2);
        assert_eq!(s.submenu_index, Some(0));
        s.toggle_top(2);
        assert!(!s.open);
    }

    #[test]
    fn select_wrap() {
        let mut s = MenuState::default();
        s.open_top(0);
        s.select_next(2);
        assert_eq!(s.submenu_index, Some(1));
        s.select_next(2);
        assert_eq!(s.submenu_index, Some(0));
        s.select_prev(2);
        assert_eq!(s.submenu_index, Some(1));
    }
}
