use assert_fs::prelude::*;

use app::fs_op::files::{format_unix_mode, inspect_permissions};

#[test]
fn probe_write_dir() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new()?;

    let info = inspect_permissions(temp.path(), true)?;
    assert!(info.is_dir);
    assert!(info.can_read);
    assert!(info.can_write, "expected probe write to succeed in temp dir");

    // show the mode for debugging if available
    let _mode = format_unix_mode(info.unix_mode);

    temp.close()?;
    Ok(())
}

#[test]
fn metadata_readonly_file() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new()?;
    let file = temp.child("f.txt");
    file.write_str("hello")?;

    let p = file.path();
    let mut perm = std::fs::metadata(p)?.permissions();
    perm.set_readonly(true);
    std::fs::set_permissions(p, perm)?;

    let info = inspect_permissions(p, false)?;
    assert!(info.readonly, "expected metadata readonly to be true");

    temp.close()?;
    Ok(())
}
