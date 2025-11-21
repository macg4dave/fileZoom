use crate::app::Mode;
use crate::input::KeyCode;
use crate::app::settings::keybinds;
use crate::app::App;

/// Adjust the double-click timeout (milliseconds) by `step` and clamp to
/// the supported range [100, 5000]. The `step` may be negative.
fn adjust_double_click_ms(value: &mut u64, step: i64) {
    let new = (*value as i128).saturating_add(step as i128);
    *value = new.clamp(100, 5000) as u64;
}

/// Handle keys while the Settings modal is active.
///
/// Returns `Ok(false)` to match the handler convention used elsewhere in
/// the application (non-consuming by default). The function mutates
/// `app.mode` and `app.settings` in-place based on key input.
pub fn handle_settings(app: &mut App, code: KeyCode) -> anyhow::Result<bool> {
    // Selected indices: 0 = mouse_enabled, 1 = double_click_ms, 2 = Save, 3 = Cancel
    if let Mode::Settings { selected } = &mut app.mode {
        // Escape always exits settings.
        if keybinds::is_esc(&code) {
            app.mode = Mode::Normal;
            return Ok(false);
        }

        // Navigation: up/down wrap within 0..=3
        if keybinds::is_up(&code) {
            *selected = (*selected + 4 - 1) % 4; // safe wrap subtract
            return Ok(false);
        }

        if keybinds::is_down(&code) {
            *selected = (*selected + 1) % 4;
            return Ok(false);
        }

        // Left/Right/+/ - only affect the double-click ms when selected == 1
        if *selected == 1 {
            if keybinds::is_left(&code) || keybinds::is_char(&code, '-') {
                adjust_double_click_ms(&mut app.settings.mouse_double_click_ms, -50);
                return Ok(false);
            }
            if keybinds::is_right(&code) || keybinds::is_char(&code, '+') {
                adjust_double_click_ms(&mut app.settings.mouse_double_click_ms, 50);
                return Ok(false);
            }
        }

        // Activate / toggle / enter
        if keybinds::is_enter(&code) || keybinds::is_toggle_selection(&code) {
            match *selected {
                0 => {
                    app.settings.mouse_enabled = !app.settings.mouse_enabled;
                }
                1 => {
                    // Numeric field: enter does nothing
                }
                2 => {
                    // Save settings and show a message modal on success/failure
                    match crate::app::settings::save_settings(&app.settings) {
                        Ok(_) => {
                            app.mode = Mode::Message {
                                title: "Settings Saved".to_string(),
                                content: "Settings persisted".to_string(),
                                buttons: vec!["OK".to_string()],
                                selected: 0,
                                actions: None,
                            };
                        }
                        Err(e) => {
                            app.mode = Mode::Message {
                                title: "Error".to_string(),
                                content: format!("Failed to save settings: {}", e),
                                buttons: vec!["OK".to_string()],
                                selected: 0,
                                actions: None,
                            };
                        }
                    }
                }
                3 => {
                    app.mode = Mode::Normal;
                }
                _ => {}
            }
            return Ok(false);
        }
    }

    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::adjust_double_click_ms;

    #[test]
    fn adjust_double_click_ms_in_bounds() {
        let mut v = 200u64;
        adjust_double_click_ms(&mut v, 50);
        assert_eq!(v, 250);
        adjust_double_click_ms(&mut v, -100);
        // should not go below 100
        assert_eq!(v, 150);
        adjust_double_click_ms(&mut v, -1000);
        assert_eq!(v, 100);
        adjust_double_click_ms(&mut v, 10000);
        assert_eq!(v, 5000);
    }
}
