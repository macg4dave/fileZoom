use fileZoom::app::Entry;
use fileZoom::app::Action;
use std::path::PathBuf;

#[test]
fn action_debug_and_clone() {
    let p = PathBuf::from("/tmp/x");
    let a = Action::CopyTo(p.clone());
    let b = a.clone();
    assert_eq!(format!("{:?}", a), format!("{:?}", b));
}

#[test]
fn entry_equality() {
    let e1 = Entry {
        name: "a".to_string(),
        path: PathBuf::from("a"),
        is_dir: false,
        size: 0,
        modified: None,
    };
    let e2 = e1.clone();
    assert_eq!(e1, e2);
}
