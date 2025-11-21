use assert_fs::prelude::*;
use fileZoom::app::{App, Mode, Panel};
use fileZoom::input::KeyCode;

// Test that an unknown/other context-menu label is handled by showing a
// message dialog indicating the action is not implemented.
#[test]
fn unknown_context_menu_label_shows_not_implemented_message() {
    let temp = assert_fs::TempDir::new().unwrap();
    let f = temp.child("unknown.txt");
    f.write_str("data").unwrap();

    let cwd = temp.path().to_path_buf();
    let mut app = App::new().unwrap();
    app.left = Panel::new(cwd.clone());
    app.refresh().unwrap();

    // select our file in the left panel
    let mut idx = None;
    for (i, e) in app.left.entries.iter().enumerate() {
        if e.name == "unknown.txt" {
            idx = Some(i);
            break;
        }
    }
    assert!(idx.is_some());
    let header_count = 1usize;
    let parent_count = if app.left.cwd.parent().is_some() { 1usize } else { 0usize };
    app.left.selected = header_count + parent_count + idx.unwrap();

    // Replace the mode with a context menu that contains an unknown label.
    let file_path = app.left.selected_entry().unwrap().path.clone();
    app.mode = Mode::ContextMenu {
        title: "Test".to_string(),
        options: vec!["NotARealAction".to_string()],
        selected: 0,
        path: file_path,
    };

    // Press Enter to activate the unknown option.
    fileZoom::runner::handlers::handle_key(&mut app, KeyCode::Enter, 10).unwrap();

    match &app.mode {
        Mode::Message { title, content, .. } => {
            assert!(title.contains("Action") || title.contains("Action"));
            assert!(content.contains("not implemented") || content.contains("not implemented"));
        }
        _ => panic!("expected Mode::Message after activating unknown context action"),
    }

    temp.close().unwrap();
}

// Test boundary navigation: ensure selection does not underflow/overflow.
#[test]
fn context_menu_navigation_bounds() {
    let temp = assert_fs::TempDir::new().unwrap();
    let f = temp.child("nav.txt");
    f.write_str("x").unwrap();

    let cwd = temp.path().to_path_buf();
    let mut app = App::new().unwrap();
    app.left = Panel::new(cwd.clone());
    app.refresh().unwrap();

    // select entry
    let mut idx = None;
    for (i, e) in app.left.entries.iter().enumerate() {
        if e.name == "nav.txt" {
            idx = Some(i);
            break;
        }
    }
    assert!(idx.is_some());
    let header_count = 1usize;
    let parent_count = if app.left.cwd.parent().is_some() { 1usize } else { 0usize };
    app.left.selected = header_count + parent_count + idx.unwrap();

    // Context menu with a single option
    let path = app.left.selected_entry().unwrap().path.clone();
    app.mode = Mode::ContextMenu {
        title: "NavTest".to_string(),
        options: vec!["OnlyOne".to_string()],
        selected: 0,
        path,
    };

    // Press Left/Up should keep selected at 0
    fileZoom::runner::handlers::handle_key(&mut app, KeyCode::Left, 10).unwrap();
    match &app.mode {
        Mode::ContextMenu { selected, .. } => assert_eq!(*selected, 0),
        _ => panic!("expected ContextMenu mode after Left"),
    }

    fileZoom::runner::handlers::handle_key(&mut app, KeyCode::Up, 10).unwrap();
    match &app.mode {
        Mode::ContextMenu { selected, .. } => assert_eq!(*selected, 0),
        _ => panic!("expected ContextMenu mode after Up"),
    }

    // With two options, selected should clamp at last index when at last
    app.mode = Mode::ContextMenu {
        title: "NavTest2".to_string(),
        options: vec!["One".to_string(), "Two".to_string()],
        selected: 1,
        path: app.left.selected_entry().unwrap().path.clone(),
    };

    // Press Right/Down should keep selected at last index (1)
    fileZoom::runner::handlers::handle_key(&mut app, KeyCode::Right, 10).unwrap();
    match &app.mode {
        Mode::ContextMenu { selected, .. } => assert_eq!(*selected, 1),
        _ => panic!("expected ContextMenu mode after Right"),
    }

    fileZoom::runner::handlers::handle_key(&mut app, KeyCode::Down, 10).unwrap();
    match &app.mode {
        Mode::ContextMenu { selected, .. } => assert_eq!(*selected, 1),
        _ => panic!("expected ContextMenu mode after Down"),
    }

    temp.close().unwrap();
}
