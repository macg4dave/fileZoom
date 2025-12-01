use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::env;
use std::sync::Mutex;

use fileZoom::ui::UIState;
use ratatui::layout::{Constraint, Direction, Layout, Rect};

static ENV_LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

#[test]
fn settings_defaults_snapshot() {
    let settings = fileZoom::app::settings::write_settings::Settings::default();
    insta::assert_json_snapshot!(
        settings,
        @r###"
        {
          "theme": "default",
          "show_hidden": false,
          "left_panel_width": 40,
          "right_panel_width": 40,
          "file_stats_visible": false,
          "file_stats_width": 10,
          "context_actions": [
            "View",
            "Edit",
            "Permissions",
            "Cancel"
          ],
          "mouse_enabled": true,
          "mouse_double_click_ms": 500,
          "show_cli_listing": true,
          "prefer_integrated_vim": false
        }
        "###
    );
}

#[test]
fn keymap_defaults_snapshot() {
    let bindings = fileZoom::app::settings::runtime_keybinds::default_bindings_for_tests();
    insta::assert_json_snapshot!(
        bindings,
        @r###"
        {
          "backspace": [
            "Backspace"
          ],
          "copy": [
            "Char(c)"
          ],
          "delete": [
            "Char(d)"
          ],
          "down": [
            "Down"
          ],
          "enter": [
            "Enter"
          ],
          "esc": [
            "Esc"
          ],
          "f5": [
            "F5"
          ],
          "f6": [
            "F6"
          ],
          "left": [
            "Left"
          ],
          "mv": [
            "Char(m)"
          ],
          "new_dir": [
            "Char(N)"
          ],
          "new_file": [
            "Char(n)"
          ],
          "page_down": [
            "PageDown"
          ],
          "page_up": [
            "PageUp"
          ],
          "quit": [
            "Char(q)"
          ],
          "refresh": [
            "Char(r)"
          ],
          "rename": [
            "Char(R)"
          ],
          "right": [
            "Right"
          ],
          "sort": [
            "Char(s)"
          ],
          "tab": [
            "Tab"
          ],
          "toggle_selection": [
            "Space"
          ],
          "toggle_sort_direction": [
            "Char(S)"
          ],
          "up": [
            "Up"
          ]
        }
        "###
    );
}

#[test]
fn config_paths_snapshot() {
    let _g = ENV_LOCK.lock().unwrap();
    let prev_home = env::var("HOME").ok();
    let prev_xdg = env::var("XDG_CONFIG_HOME").ok();

    env::set_var("HOME", "/home/snapshot");
    env::set_var("XDG_CONFIG_HOME", "/home/snapshot/.config");

    let cfg_file = fileZoom::app::settings::write_settings::config_file_path().unwrap();
    let project_cfg = fileZoom::app::settings::config_dirs::project_config_dir();
    let cache_dir = fileZoom::app::settings::config_dirs::user_cache_dir();

    let data = BTreeMap::from([
        ("config_file", cfg_file.display().to_string()),
        ("project_config_dir", project_cfg.display().to_string()),
        ("cache_dir", cache_dir.display().to_string()),
    ]);

    if let Some(v) = prev_home { env::set_var("HOME", v); } else { env::remove_var("HOME"); }
    if let Some(v) = prev_xdg { env::set_var("XDG_CONFIG_HOME", v); } else { env::remove_var("XDG_CONFIG_HOME"); }

    insta::assert_json_snapshot!(
        data,
        @r###"
        {
          "cache_dir": "/home/snapshot/Library/Caches/com.macg4dave.fileZoom",
          "config_file": "/home/snapshot/.config/fileZoom/settings.toml",
          "project_config_dir": "/home/snapshot/Library/Application Support/com.macg4dave.fileZoom"
        }
        "###
    );
}

#[test]
fn layout_defaults_snapshot() {
    let area = Rect::new(0, 0, 120, 30);
    let constraints = vec![
        Constraint::Min(1),
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(2),
    ];
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);
    let main = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(chunks[2]);

    let snapshot = BTreeMap::from([
        ("menu_height", chunks[0].height),
        ("header_height", chunks[1].height),
        ("main_height", chunks[2].height),
        ("footer_height", chunks[3].height),
        ("left_width", main[0].width),
        ("right_width", main[1].width),
        ("preview_visible_default", UIState::default().preview_text.is_some() as u16),
    ]);

    insta::assert_json_snapshot!(
        snapshot,
        @r###"
        {
          "footer_height": 2,
          "header_height": 3,
          "left_width": 66,
          "main_height": 12,
          "menu_height": 13,
          "preview_visible_default": 0,
          "right_width": 54
        }
        "###
    );
}
