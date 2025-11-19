use fileZoom::fs_op::remove::remove_path;
use tempfile::tempdir;

#[test]
fn remove_file_and_dir() {
    let td = tempdir().unwrap();
    let dir = td.path().join("sub");
    std::fs::create_dir_all(&dir).unwrap();
    let f = dir.join("f.txt");
    std::fs::write(&f, b"x").unwrap();
    remove_path(&f).unwrap();
    assert!(!f.exists());
    remove_path(&dir).unwrap();
    assert!(!dir.exists());
}
