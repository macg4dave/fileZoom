//! Navigation helpers for `App`.
//!
//! This module centralises small helpers that change the currently-selected
//! UI row in the active panel. The functions are intentionally lightweight
//! wrappers that delegate heavy-lifting (row counting, bounds checks and the
//! visible-region logic) to the `Panel` API and the `utils` helpers so the
//! behaviour stays easy to unit-test.

use super::utils;
use super::App;

impl App {
    /// Ensure the currently selected UI row is visible within the active
    /// panel's viewport of `viewport_height` rows.
    ///
    /// Delegates to `Panel::ensure_selected_visible` to keep scrolling
    /// behaviour local to the panel implementation.
    pub fn ensure_selection_visible(&mut self, viewport_height: usize) {
        self.active_panel_mut().ensure_selected_visible(viewport_height);
    }

    /// Small helper to perform a navigation operation on the active panel,
    /// then update visibility and preview state.
    fn apply_navigation<F>(&mut self, viewport_height: usize, mut op: F)
    where
        F: FnMut(&mut super::Panel),
    {
        let panel = self.active_panel_mut();
        // Perform the operation only when there are domain entries present.
        // The UI still needs to refresh visibility/preview even when the
        // domain is empty (header/parent rows still exist), so we don't
        // early-return here.
        if !panel.entries.is_empty() {
            op(panel);
        }

        self.ensure_selection_visible(viewport_height);
        self.update_preview_for(self.active);
    }

    /// Move active selection down by one UI row.
    pub fn select_next(&mut self, viewport_height: usize) {
        self.apply_navigation(viewport_height, |panel| panel.select_next());
    }

    /// Move active selection up by one UI row.
    pub fn select_prev(&mut self, viewport_height: usize) {
        self.apply_navigation(viewport_height, |panel| panel.select_prev());
    }

    /// Move active selection down by `viewport_height` rows (page down).
    ///
    /// Uses the panel's UI row count to compute a safe clamped destination
    /// index so we don't rely on internal structure of the `Panel` layout.
    pub fn select_page_down(&mut self, viewport_height: usize) {
        self.apply_navigation(viewport_height, |panel| {
            let max_rows = utils::ui_row_count(panel);
            if max_rows == 0 {
                panel.selected = 0;
                return;
            }
            let new = std::cmp::min(
                panel.selected.saturating_add(viewport_height),
                max_rows.saturating_sub(1),
            );
            panel.selected = new;
        });
    }

    /// Move active selection up by `viewport_height` rows (page up) using
    /// saturating subtraction so the value never underflows.
    pub fn select_page_up(&mut self, viewport_height: usize) {
        self.apply_navigation(viewport_height, |panel| {
            panel.selected = panel.selected.saturating_sub(viewport_height);
        });
    }
}
