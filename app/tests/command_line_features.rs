use fileZoom::app::App;
use fileZoom::input::KeyCode;
use fileZoom::ui::command_line::CommandLineState;

#[test]
fn tab_completion_prefills_known_command() {
    let mut app = App::new().unwrap();
    app.command_line = Some(CommandLineState::default());

    for c in "menu-a".chars() {
        let _ = fileZoom::ui::command_line::handle_input(&mut app, KeyCode::Char(c)).unwrap();
    }
    let _ = fileZoom::ui::command_line::handle_input(&mut app, KeyCode::Tab).unwrap();

    let text = app.command_line.as_ref().unwrap().textarea.lines().join("\n");
    assert_eq!(text, "menu-activate");
}

#[test]
fn history_navigation_restores_previous_command() {
    let mut app = App::new().unwrap();
    app.command_line = Some(CommandLineState::default());

    for c in "toggle-preview".chars() {
        let _ = fileZoom::ui::command_line::handle_input(&mut app, KeyCode::Char(c)).unwrap();
    }
    let _ = fileZoom::ui::command_line::handle_input(&mut app, KeyCode::Enter).unwrap();
    assert!(app.command_line.as_ref().map(|c| !c.visible).unwrap_or(true));

    if let Some(ref mut cmd) = app.command_line {
        cmd.activate();
    }

    let _ = fileZoom::ui::command_line::handle_input(&mut app, KeyCode::Up).unwrap();
    let restored = app.command_line.as_ref().unwrap().textarea.lines().join("\n");
    assert_eq!(restored, "toggle-preview");

    let _ = fileZoom::ui::command_line::handle_input(&mut app, KeyCode::Down).unwrap();
    let cleared = app.command_line.as_ref().unwrap().textarea.lines().join("\n");
    assert!(cleared.is_empty());
}
