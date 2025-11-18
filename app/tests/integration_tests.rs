use assert_fs::prelude::*;

// `App` is re-exported from the library `lib.rs` so tests can access it directly.
use app::App;

#[test]
fn test_basic_file_ops() -> Result<(), Box<dyn std::error::Error>> {
    // remember current dir and restore at end
    let orig = std::env::current_dir()?;

    let temp = assert_fs::TempDir::new()?;
    temp.child("file1.txt").write_str("hello")?;
    temp.child("dirA/file2.txt").write_str("data")?;

    std::env::set_current_dir(temp.path())?;

    let mut app = App::new()?;

    // entries should include our files/dirs in both panels
    assert!(app.left.entries.iter().any(|e| e.name == "file1.txt"));
    assert!(app.left.entries.iter().any(|e| e.name == "dirA"));
    assert!(app.right.entries.iter().any(|e| e.name == "file1.txt"));
    assert!(app.right.entries.iter().any(|e| e.name == "dirA"));

    // select file1 and copy it to a new dest dir
    let idx = app.left.entries.iter().position(|e| e.name == "file1.txt").unwrap();
    app.left.selected = idx;

    let dest_dir = temp.path().join("copy_dest");
    std::fs::create_dir_all(&dest_dir)?;
    app.copy_selected_to(dest_dir.clone())?;
    assert!(dest_dir.join("file1.txt").exists());

    // rename file1
    app.rename_selected_to("file1_renamed.txt".to_string())?;
    assert!(temp.child("file1_renamed.txt").exists());

    // create new file and dir
    app.new_file("new_file.txt".to_string())?;
    assert!(temp.child("new_file.txt").exists());
    app.new_dir("new_dir".to_string())?;
    assert!(temp.child("new_dir").exists());

    // delete the new file by selecting it and deleting
    if let Some(pos) = app.left.entries.iter().position(|e| e.name == "new_file.txt") {
        app.left.selected = pos;
        app.delete_selected()?;
        assert!(!temp.child("new_file.txt").exists());
    }

    // move dirA to moved_dir
    if let Some(pos) = app.left.entries.iter().position(|e| e.name == "dirA") {
        app.left.selected = pos;
        let moved_to = temp.path().join("moved_dir/");
        app.move_selected_to(moved_to.clone())?;
        assert!(moved_to.join("file2.txt").exists());
    }

    std::env::set_current_dir(orig)?;
    temp.close()?;
    Ok(())
}
