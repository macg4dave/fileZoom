use assert_fs::prelude::*;
use assert_fs::TempDir;
use fileZoom::app::Action;
use fileZoom::app::App;
use fileZoom::runner::commands::perform_action;
use std::env;
use std::sync::Mutex;

static TEST_CWD_LOCK: Mutex<()> = Mutex::new(());

fn find_index(app: &App, name: &str) -> Option<usize> {
    app.left.entries.iter().position(|e| {
        if e.name == name {
            return true;
        }
        if let Some(fname) = e.path.file_name().and_then(|s| s.to_str()) {
            return fname == name;
        }
        false
    })
}

#[test]
fn new_file_and_dir_actions_create_files() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempDir::new()?;
    // Acquire global cwd lock to avoid races when tests set the process cwd
    let _guard = TEST_CWD_LOCK.lock().unwrap();
    let orig = env::current_dir()?;
    env::set_current_dir(temp.path())?;

    let mut app = App::new()?;

    // New file
    perform_action(&mut app, Action::NewFile("a.txt".to_string()))?;
    assert!(temp.child("a.txt").exists());

    // New dir
    perform_action(&mut app, Action::NewDir("d1".to_string()))?;
    assert!(temp.child("d1").exists());

    // restore original cwd to avoid interfering with other tests
    env::set_current_dir(orig)?;
    drop(_guard);
    Ok(())
}

#[test]
fn copy_move_rename_delete_actions_work() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempDir::new()?;
    // Acquire global cwd lock to avoid races when tests set the process cwd
    let _guard = TEST_CWD_LOCK.lock().unwrap();
    let orig = env::current_dir()?;
    env::set_current_dir(temp.path())?;

    // create a source file
    temp.child("src.txt").write_str("hello")?;

    let mut app = App::new()?;

    // find src index and select it
    let idx = match find_index(&app, "src.txt") {
        Some(i) => i,
        None => {
            let names: Vec<String> = app.left.entries.iter().map(|e| e.name.clone()).collect();
            panic!("src.txt entry not found, entries={:?}", names);
        }
    };
    let header_count = 1usize;
    let parent_count = if app.left.cwd.parent().is_some() {
        1usize
    } else {
        0usize
    };
    app.left.selected = header_count + parent_count + idx;

    // copy to subdir 'out'
    let out = temp.child("out");
    out.create_dir_all()?;
    perform_action(&mut app, Action::CopyTo(out.path().to_path_buf()))?;
    assert!(out.child("src.txt").exists());

    // move a new file
    temp.child("mv.txt").write_str("mv")?;
    let mut app2 = App::new()?;
    let idx2 = find_index(&app2, "mv.txt").expect("mv.txt entry not found");
    let header_count = 1usize;
    let parent_count = if app2.left.cwd.parent().is_some() {
        1usize
    } else {
        0usize
    };
    app2.left.selected = header_count + parent_count + idx2;
    let dest = temp.child("moved");
    dest.create_dir_all()?;
    perform_action(&mut app2, Action::MoveTo(dest.path().to_path_buf()))?;
    assert!(dest.child("mv.txt").exists());

    // rename
    temp.child("rnm.txt").write_str("r")?;
    let mut app3 = App::new()?;
    let idx3 = find_index(&app3, "rnm.txt").expect("rnm.txt not found");
    let header_count = 1usize;
    let parent_count = if app3.left.cwd.parent().is_some() {
        1usize
    } else {
        0usize
    };
    app3.left.selected = header_count + parent_count + idx3;
    perform_action(&mut app3, Action::RenameTo("renamed.txt".to_string()))?;
    assert!(temp.child("renamed.txt").exists());

    // delete
    temp.child("del.txt").write_str("d")?;
    let mut app4 = App::new()?;
    let idx4 = find_index(&app4, "del.txt").expect("del.txt not found");
    let header_count = 1usize;
    let parent_count = if app4.left.cwd.parent().is_some() {
        1usize
    } else {
        0usize
    };
    app4.left.selected = header_count + parent_count + idx4;
    perform_action(&mut app4, Action::DeleteSelected)?;
    assert!(!temp.child("del.txt").exists());

    // restore original cwd to avoid interfering with other tests
    env::set_current_dir(orig)?;
    drop(_guard);
    Ok(())
}
