use crate::app::{Action, App, Mode};
use crate::errors;
use crate::input::KeyCode;
use crate::app::settings::keybinds;

/// Handle input when the application is in a confirmation dialog.
///
/// The function returns `Ok(false)` for historical compatibility with the
/// event loop (it currently never requests the app to quit). It will
/// transition `app.mode` back to `Mode::Normal` when the dialog is closed,
/// and will execute the provided `on_yes` `Action` when the user confirms.
pub fn handle_confirm(app: &mut App, code: KeyCode) -> anyhow::Result<bool> {
    if let Mode::Confirm { on_yes, selected, .. } = &mut app.mode {
        // Left/right both toggle when there are only two options.
        if keybinds::is_left(&code) || keybinds::is_right(&code) {
            toggle_selected(selected);
        } else if keybinds::is_enter(&code)
            || keybinds::is_char(&code, 'y')
            || keybinds::is_char(&code, 'Y')
        {
            // perform the affirmative action
            let action = on_yes.clone();
            app.mode = Mode::Normal;
            execute_action(app, action);
        } else if keybinds::is_char(&code, 'n') || keybinds::is_esc(&code) {
            // cancel
            app.mode = Mode::Normal;
        }
    }

    Ok(false)
}

/// Toggle a binary selection index (0 <-> 1).
fn toggle_selected(selected: &mut usize) {
    *selected = 1usize.saturating_sub(*selected);
}

/// Convert a filesystem operation error into a `Mode::Message` on the app.
fn set_error_message(app: &mut App, err: &crate::fs_op::error::FsOpError) {
    let msg = errors::render_fsop_error(err, None, None, None);
    app.mode = Mode::Message {
        title: "Error".to_string(),
        content: msg,
        buttons: vec!["OK".to_string()],
        selected: 0,
        actions: None,
    };
}

/// Execute an `Action` coming from a confirmation dialog and surface any
/// filesystem errors as a message mode.
fn execute_action(app: &mut App, action: Action) {
    match action {
        Action::DeleteSelected => {
            if let Err(err) = app.delete_selected() {
                set_error_message(app, &err);
            }
        }
        Action::CopyTo(p) => {
            if let Err(err) = app.copy_selected_to(p) {
                set_error_message(app, &err);
            }
        }
        Action::MoveTo(p) => {
            if let Err(err) = app.move_selected_to(p) {
                set_error_message(app, &err);
            }
        }
        Action::RenameTo(name) => {
            if let Err(err) = app.rename_selected_to(name) {
                set_error_message(app, &err);
            }
        }
        Action::NewFile(name) => {
            if let Err(err) = app.new_file(name) {
                set_error_message(app, &err);
            }
        }
        Action::NewDir(name) => {
            if let Err(err) = app.new_dir(name) {
                set_error_message(app, &err);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::toggle_selected;

    #[test]
    fn toggle_switches_between_zero_and_one() {
        let mut v = 0usize;
        toggle_selected(&mut v);
        assert_eq!(v, 1);
        toggle_selected(&mut v);
        assert_eq!(v, 0);
    }
}
