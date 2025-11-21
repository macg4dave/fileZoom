use super::panel::Panel;

/// Small utilities shared across `app::core` submodules.
///
/// This module contains lightweight helpers related to panel/UI row
/// calculations and index translation used by multiple core files.
/// Keep helpers here small and purely functional so they are easy to
/// reason about and test.
pub(crate) fn ui_row_count(panel: &Panel) -> usize {
    // The UI always renders a synthetic header row (the path/header).
    // If the panel's `cwd` has a parent directory we render an
    // additional "parent" row that lets the user navigate up. The
    // remainder of the rows correspond to the domain `entries`.
    //
    // This returns the total number of UI rows that will be rendered
    // for the provided `panel`.
    1 + if panel.cwd.parent().is_some() { 1 } else { 0 } + panel.entries.len()
}

/// Translate a UI-selected row index into a domain `entries` index.
///
/// The UI index includes a synthetic header row at index `0`, and an
/// optional parent row at index `1` when the panel's current working
/// directory has a parent. If the provided `selected` index points to
/// one of those synthetic rows this function returns `None`.
///
/// # Parameters
///
/// - `selected`: the selected row index in the UI (0-based).
/// - `panel`: the `Panel` used to determine whether a parent row is
///   present and how many domain entries exist.
///
/// # Returns
///
/// - `Some(entry_index)` when the `selected` row maps to an entry in
///   `panel.entries`.
/// - `None` when the selection refers to the header or parent row.
pub(crate) fn ui_to_entry_index(selected: usize, panel: &Panel) -> Option<usize> {
    let header_count = 1usize;
    let parent_count = if panel.cwd.parent().is_some() { 1usize } else { 0usize };
    if selected >= header_count + parent_count {
        Some(selected - header_count - parent_count)
    } else {
        None
    }
}
