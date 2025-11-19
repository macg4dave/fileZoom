use fileZoom::fs_op::permissions::inspect_permissions;
use tempfile::tempdir;

#[test]
fn probe_writes_do_not_leave_tmp_files() {
    let td = tempdir().unwrap();
    let p = td.path().to_path_buf();
    // perform a write probe for the directory
    let info = inspect_permissions(&p, true).unwrap();
    assert!(info.can_write || !info.can_write); // just ensure call completed

    // ensure no leftover atomic temp files
    let mut tmp_leftovers = 0;
    for e in std::fs::read_dir(&p).unwrap() {
        if let Ok(e) = e {
            if let Some(name) = e.file_name().to_str() {
                if name.starts_with(".tmp_atomic_write.") {
                    tmp_leftovers += 1;
                }
            }
        }
    }
    assert_eq!(tmp_leftovers, 0, "found leftover atomic temp files after probe");
}
