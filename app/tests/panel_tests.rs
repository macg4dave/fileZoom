use assert_fs::prelude::*;
use fileZoom::app::{core::App as CoreApp, Panel, StartOptions};
use fileZoom::Entry;
use std::path::PathBuf;

#[test]
fn panel_defaults_and_selected_entry() {
    let cwd = PathBuf::from("/tmp");
    let p = Panel::new(cwd.clone());
    assert_eq!(p.cwd, cwd);
    assert_eq!(p.entries.len(), 0);
    assert_eq!(p.selected, 0);
    assert_eq!(p.offset, 0);
    assert_eq!(p.preview, "");
    assert_eq!(p.preview_offset, 0);
}

#[test]
fn select_next_prev_and_clamp() {
    let mut p = Panel::new(PathBuf::from("/"));
    // populate entries with mock entries
    p.entries = (0..5)
        .map(|i| {
            Entry::file(
                format!("f{}", i),
                PathBuf::from(format!("/f{}", i)),
                0,
                None,
            )
        })
        .collect();
    assert_eq!(p.selected, 0);
    p.select_next();
    assert_eq!(p.selected, 1);
    p.select_prev();
    assert_eq!(p.selected, 0);
    // move to last
    for _ in 0..10 {
        p.select_next();
    }
    // With UI indices (header + entries), the last selectable UI row is
    // header + entries.len() - 1 => 1 + 5 - 1 = 5
    assert_eq!(p.selected, 5);
    // clamp down when entries shrink
    p.entries.truncate(2);
    p.clamp_selected();
    // After truncating to 2 entries, max UI row is 1 + 2 - 1 = 2
    assert_eq!(p.selected, 2);
}

#[test]
fn ensure_selected_visible_basic() {
    let mut p = Panel::new(PathBuf::from("/"));
    p.entries = (0..10)
        .map(|i| {
            Entry::file(
                format!("f{}", i),
                PathBuf::from(format!("/f{}", i)),
                0,
                None,
            )
        })
        .collect();
    // viewport of 3 rows
    let h = 3;
    p.selected = 0;
    p.offset = 0;
    p.ensure_selected_visible(h);
    assert_eq!(p.offset, 0);

    p.selected = 2;
    p.ensure_selected_visible(h);
    assert_eq!(p.offset, 0);

    p.selected = 3;
    p.ensure_selected_visible(h);
    assert_eq!(p.offset, 1);

    p.selected = 9;
    p.ensure_selected_visible(h);
    // offset should be such that selected is visible within viewport
    assert!(p.offset + h > p.selected);
}

#[test]
fn ensure_selected_visible_zero_height_and_single_item() {
    // zero height: should reset offset to 0 regardless of entries
    let mut p = Panel::new(PathBuf::from("/"));
    p.entries = (0..3)
        .map(|i| {
            Entry::file(
                format!("f{}", i),
                PathBuf::from(format!("/f{}", i)),
                0,
                None,
            )
        })
        .collect();
    p.offset = 2;
    p.selected = 2;
    p.ensure_selected_visible(0);
    assert_eq!(p.offset, 0);

    // single item viewport: ensure offset keeps selected visible
    let mut q = Panel::new(PathBuf::from("/"));
    q.entries = (0..1)
        .map(|i| {
            Entry::file(
                format!("g{}", i),
                PathBuf::from(format!("/g{}", i)),
                0,
                None,
            )
        })
        .collect();
    q.selected = 0;
    q.offset = 5; // intentionally out of range
    q.ensure_selected_visible(1);
    assert_eq!(q.offset, 0);
}

#[test]
fn quick_filter_preserves_selection_when_match() {
    let temp = assert_fs::TempDir::new().unwrap();
    temp.child("keep.log").write_str("x").unwrap();
    temp.child("other.txt").write_str("y").unwrap();

    let opts = StartOptions { start_dir: Some(temp.path().to_path_buf()), ..Default::default() };
    let mut app = CoreApp::with_options(&opts).expect("init app");

    // Select keep.log explicitly.
    if let Some(idx) = app.left.entries.iter().position(|e| e.name == "keep.log") {
        let parent_rows = app.left.cwd.parent().is_some() as usize;
        app.left.selected = 1 + parent_rows + idx;
    }

    app.left.set_filter("*.log").unwrap();
    app.refresh_active().unwrap();
    assert_eq!(app.left.selected_entry().unwrap().name, "keep.log");

    // Clearing filter should keep selection on the same entry if it still exists.
    app.left.set_filter("").unwrap();
    app.refresh_active().unwrap();
    assert_eq!(app.left.selected_entry().unwrap().name, "keep.log");
}
