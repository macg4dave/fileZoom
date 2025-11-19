use fileZoom::ui::panels::format_entry_line;
use fileZoom::app::Entry;
use chrono::Local;

#[test]
fn format_entry_line_for_file_and_dir() {
    let now = Local::now();
    let file = Entry {
        name: "file.txt".to_string(),
        path: std::path::PathBuf::from("/tmp/file.txt"),
        is_dir: false,
        size: 1234,
        modified: Some(now),
    };
    let dir = Entry {
        name: "somedir".to_string(),
        path: std::path::PathBuf::from("/tmp/somedir"),
        is_dir: true,
        size: 0,
        modified: None,
    };
    let fline = format_entry_line(&file);
    assert!(fline.contains("file.txt"));
    assert!(fline.contains("1234"));
    assert!(fline.contains(&now.format("%Y-%m-%d %H:%M").to_string()));

    let dline = format_entry_line(&dir);
    assert!(dline.contains("somedir"));
    assert!(dline.contains("<dir>"));
    assert!(dline.contains("-"));
}
