// `min` and `io` are used in `methods` submodule; import there instead.

// chrono imported in Panel (file metadata reading)

use self::panel::Panel;
use super::types::{Mode, Side, SortKey};

/// Alias for the receiver sending progress updates from background workers.
type OpProgressReceiver = std::sync::mpsc::Receiver<crate::runner::progress::ProgressUpdate>;

/// Alias for a shared atomic cancel flag used by background operations.
type OpCancelFlag = std::sync::Arc<std::sync::atomic::AtomicBool>;

/// Alias for sending decisions back to the background worker when asking the
/// user how to resolve a file operation conflict.
type OpDecisionSender = std::sync::mpsc::Sender<crate::runner::progress::OperationDecision>;

/// Central application state.
///
/// This struct holds the two panels, UI state, settings and optional
/// communication channels used by background file operations.
pub struct App {
    /// Left-hand panel.
    pub left: Panel,
    /// Right-hand panel.
    pub right: Panel,
    /// Currently active side (left or right).
    pub active: Side,
    /// Current editor mode.
    pub mode: Mode,
    /// Current sort key.
    pub sort: SortKey,
    /// Order direction for the current sort key.
    pub sort_order: crate::app::types::SortOrder,
    /// Index of the currently selected menu item.
    pub menu_index: usize,
    /// Whether the top-level menu has keyboard focus.
    pub menu_focused: bool,
    /// UI state for dropdowns and submenu selection.
    pub menu_state: crate::ui::menu_model::MenuState,
    /// Whether the preview pane is visible in the UI.
    pub preview_visible: bool,
    /// Whether the dedicated file-stats column is visible in the UI.
    pub file_stats_visible: bool,
    /// Optional command-line state when user opens the command input.
    pub command_line: Option<crate::ui::command_line::CommandLineState>,
    /// User settings loaded from disk.
    pub settings: crate::app::settings::write_settings::Settings,
    /// Receiver for progress updates from background file operations.
    pub op_progress_rx: Option<OpProgressReceiver>,
    /// Cancel flag shared with background operation thread (if any).
    pub op_cancel_flag: Option<OpCancelFlag>,
    /// Sender for communicating user's decision back to the background worker
    /// when a file-exists conflict is presented.
    pub op_decision_tx: Option<OpDecisionSender>,
    /// Last mouse click timestamp (used for double-click detection).
    pub last_mouse_click_time: Option<std::time::Instant>,
    /// Last mouse click position (column, row).
    pub last_mouse_click_pos: Option<(u16, u16)>,
    /// Whether a drag operation is currently active.
    pub drag_active: bool,
    /// Drag start position (column, row).
    pub drag_start: Option<(u16, u16)>,
    /// Current drag position (column, row).
    pub drag_current: Option<(u16, u16)>,
    /// Which mouse button started the drag.
    pub drag_button: Option<crate::input::mouse::MouseButton>,
}

// submodules live in `app/src/app/core/`
pub mod panel;
// Re-export the canonical path helpers into the `app::core` namespace so
// code referencing `crate::app::core::path` continues to work without using
// the deprecated `app::path` shim.
pub use crate::fs_op::path;
mod navigation;
mod preview;
pub mod preview_helpers;

mod init;
mod utils;
mod methods;

/// Maximum bytes to read for a file preview (100 KiB). Made public so
/// integration tests can verify preview truncation.
pub const MAX_PREVIEW_BYTES: usize = 100 * 1024;

impl App {
    /// Associated constant mirroring the module-level `MAX_PREVIEW_BYTES` so
    /// older code/tests that reference `App::MAX_PREVIEW_BYTES` continue to
    /// compile.
    pub const MAX_PREVIEW_BYTES: usize = crate::app::core::MAX_PREVIEW_BYTES;
    /// Return mutable reference to the currently active panel.
    ///
    /// This helper is crate-visible to allow internal modules to operate on
    /// the active panel without exposing it in the public API.
    pub fn active_panel_mut(&mut self) -> &mut Panel {
        match self.active {
            Side::Left => &mut self.left,
            Side::Right => &mut self.right,
        }
    }

    /// Return a reference to the currently active panel (non-mutable).
    pub fn active_panel(&self) -> &Panel {
        match self.active {
            Side::Left => &self.left,
            Side::Right => &self.right,
        }
    }

    /// Return a mutable reference to the panel identified by `side`.
    pub fn panel_mut(&mut self, side: Side) -> &mut Panel {
        match side {
            Side::Left => &mut self.left,
            Side::Right => &mut self.right,
        }
    }

    /// Return the currently selected index for the active panel's file
    /// listing, or `None` if the selection points to a header/parent entry.
    pub fn selected_index(&self) -> Option<usize> {
        let panel = self.active_panel();

        // One header entry is always present.
        let header_count = 1usize;

        // If the current working directory has a parent, there is an extra
        // parent directory entry at the start of the listing.
        let parent_count = panel.cwd.parent().is_some() as usize;

        panel.selected.checked_sub(header_count + parent_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_preview_bytes_is_100kib() {
        assert_eq!(MAX_PREVIEW_BYTES, 100 * 1024);
    }
}
