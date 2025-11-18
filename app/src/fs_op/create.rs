use std::fs::OpenOptions;
use std::path::Path;

use anyhow::{Context, Result};

/// Create an empty file at `path`. Fails if the file already exists.
pub fn create_file<P: AsRef<Path>>(path: P) -> Result<()> {
    let p = path.as_ref();
    if let Some(parent) = p.parent() {
        std::fs::create_dir_all(parent).with_context(|| format!("creating parent dirs for {:?}", p))?;
    }
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(p)
        .with_context(|| format!("creating file {:?}", p))?;
    Ok(())
}

/// Create directory and parents.
pub fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<()> {
    std::fs::create_dir_all(path.as_ref()).with_context(|| format!("creating dir {:?}", path.as_ref()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn create_file_and_dir() {
        let td = tempdir().unwrap();
        let dir = td.path().join("a/b");
        let file = dir.join("f.txt");
        create_dir_all(&dir).unwrap();
        create_file(&file).unwrap();
        assert!(file.exists());
    }
}
