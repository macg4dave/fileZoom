use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

/// Remove a file or directory (recursively for directories).
pub fn remove_path<P: AsRef<Path>>(path: P) -> Result<()> {
    let p = path.as_ref();
    if p.is_dir() {
        fs::remove_dir_all(p).with_context(|| format!("removing dir {:?}", p))?;
    } else if p.exists() {
        fs::remove_file(p).with_context(|| format!("removing file {:?}", p))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
