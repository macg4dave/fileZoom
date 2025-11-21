use crate::app::App;
use crate::app::Mode;
use crate::input::KeyCode;
use std::sync::atomic::Ordering;

/// Handle input while the UI is in `Progress` mode.
///
/// Currently this only handles the Escape key which signals cancellation
/// of the in-flight background operation. When `Esc` is received the
/// optional `op_cancel_flag` is consumed (taken) and set to `true` so
/// background workers may observe the request to stop. The UI `Mode` is
/// updated in-place to reflect a cancelling state.
///
/// Returns `Ok(false)` to indicate no immediate screen redraw request is
/// required by the caller.
pub fn handle_progress(app: &mut App, code: KeyCode) -> anyhow::Result<bool> {
    if let KeyCode::Esc = code {
        if let Some(flag) = app.op_cancel_flag.take() {
            flag.store(true, Ordering::SeqCst);
        }

        if let Mode::Progress { message, cancelled, .. } = &mut app.mode {
            // Update the progress message and mark cancelled in-place
            *message = "Cancelling...".to_string();
            *cancelled = true;
        }
    }

    Ok(false)
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::{atomic::AtomicBool, Arc};
    use std::sync::atomic::Ordering as AtomicOrdering;

    #[test]
    fn esc_sets_cancel_flag_and_updates_mode() {
        // Construct a minimal App mirroring the crate's initializer.
        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let mut app = App {
            left: crate::app::Panel::new(cwd.clone()),
            right: crate::app::Panel::new(cwd),
            active: crate::app::Side::Left,
            mode: Mode::Normal,
            sort: crate::app::types::SortKey::Name,
            sort_order: crate::app::types::SortOrder::Ascending,
            menu_index: 0,
            menu_focused: false,
            preview_visible: false,
            command_line: None,
            settings: crate::app::settings::write_settings::Settings::default(),
            op_progress_rx: None,
            op_cancel_flag: None,
            op_decision_tx: None,
            last_mouse_click_time: None,
            last_mouse_click_pos: None,
            drag_active: false,
            drag_start: None,
            drag_current: None,
            drag_button: None,
        };

        // Prepare a cancel flag shared with the handler.
        let flag = Arc::new(AtomicBool::new(false));
        app.op_cancel_flag = Some(flag.clone());

        // Put the app into Progress mode.
        app.mode = Mode::Progress {
            title: "Test".into(),
            processed: 1,
            total: 10,
            message: "Working".into(),
            cancelled: false,
        };

        // Invoke handler with Escape.
        let res = handle_progress(&mut app, KeyCode::Esc).expect("handler failed");
        assert!(!res, "handler returns Ok(false)");

        // The shared flag must have been set and taken from the app.
        assert!(flag.load(AtomicOrdering::SeqCst));
        assert!(app.op_cancel_flag.is_none());

        // Mode should reflect the cancelling state.
        if let Mode::Progress { message, .. } = &app.mode {
            assert_eq!(message, "Cancelling...");
        } else {
            panic!("unexpected mode after escape: {:?}", app.mode);
        }

        assert!(matches!(app.mode, Mode::Progress { cancelled: true, .. }));
    }

    #[test]
    fn non_esc_key_is_noop_preserves_flag_and_mode() {
        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let mut app = App {
            left: crate::app::Panel::new(cwd.clone()),
            right: crate::app::Panel::new(cwd),
            active: crate::app::Side::Left,
            mode: Mode::Normal,
            sort: crate::app::types::SortKey::Name,
            sort_order: crate::app::types::SortOrder::Ascending,
            menu_index: 0,
            menu_focused: false,
            preview_visible: false,
            command_line: None,
            settings: crate::app::settings::write_settings::Settings::default(),
            op_progress_rx: None,
            op_cancel_flag: None,
            op_decision_tx: None,
            last_mouse_click_time: None,
            last_mouse_click_pos: None,
            drag_active: false,
            drag_start: None,
            drag_current: None,
            drag_button: None,
        };

        // Prepare a cancel flag and set it, but keep it attached to app.
        let flag = Arc::new(AtomicBool::new(false));
        app.op_cancel_flag = Some(flag.clone());

        // Put the app into Progress mode with initial values.
        app.mode = Mode::Progress {
            title: "Test2".into(),
            processed: 2,
            total: 20,
            message: "Working".into(),
            cancelled: false,
        };

        // Invoke handler with a non-Esc key (Char)
        let res = handle_progress(&mut app, KeyCode::Char('x')).expect("handler failed");
        assert!(!res, "handler returns Ok(false)");

        // The shared flag should remain untouched and still present.
        assert!(!flag.load(AtomicOrdering::SeqCst));
        assert!(app.op_cancel_flag.is_some());

        // Mode should remain unchanged (message and cancelled unchanged)
        if let Mode::Progress { message, cancelled, processed, total, .. } = &app.mode {
            assert_eq!(message, "Working");
            assert!(!*cancelled);
            assert_eq!(*processed, 2);
            assert_eq!(*total, 20);
        } else {
            panic!("unexpected mode after non-esc: {:?}", app.mode);
        }
    }

    #[test]
    fn non_esc_without_flag_leaves_mode_unmodified() {
        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let mut app = App {
            left: crate::app::Panel::new(cwd.clone()),
            right: crate::app::Panel::new(cwd),
            active: crate::app::Side::Left,
            mode: Mode::Normal,
            sort: crate::app::types::SortKey::Name,
            sort_order: crate::app::types::SortOrder::Ascending,
            menu_index: 0,
            menu_focused: false,
            preview_visible: false,
            command_line: None,
            settings: crate::app::settings::write_settings::Settings::default(),
            op_progress_rx: None,
            op_cancel_flag: None,
            op_decision_tx: None,
            last_mouse_click_time: None,
            last_mouse_click_pos: None,
            drag_active: false,
            drag_start: None,
            drag_current: None,
            drag_button: None,
        };

        // Put the app into Progress mode with initial values and no flag.
        app.mode = Mode::Progress {
            title: "Test3".into(),
            processed: 3,
            total: 30,
            message: "Working".into(),
            cancelled: false,
        };

        // Invoke handler with a non-Esc key (Enter)
        let res = handle_progress(&mut app, KeyCode::Enter).expect("handler failed");
        assert!(!res, "handler returns Ok(false)");

        // Since there was no flag, op_cancel_flag should remain None.
        assert!(app.op_cancel_flag.is_none());

        // Mode should remain unchanged.
        if let Mode::Progress { message, cancelled, processed, total, .. } = &app.mode {
            assert_eq!(message, "Working");
            assert!(!*cancelled);
            assert_eq!(*processed, 3);
            assert_eq!(*total, 30);
        } else {
            panic!("unexpected mode after non-esc: {:?}", app.mode);
        }
    }
}
