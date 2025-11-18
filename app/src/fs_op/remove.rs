use std::fs;
use std::path::Path;
use std::fmt;

/// Errors returned from `remove_path`.
#[derive(Debug)]
pub enum RemoveError {
    Io(std::io::Error),
}

impl fmt::Display for RemoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RemoveError::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for RemoveError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RemoveError::Io(e) => Some(e),
        }
    }
}

impl From<std::io::Error> for RemoveError {
    fn from(e: std::io::Error) -> Self {
        RemoveError::Io(e)
    }
}

/// Remove a file or directory (recursively for directories).
pub fn remove_path<P: AsRef<Path>>(path: P) -> Result<(), RemoveError> {
    let p = path.as_ref();
    if p.is_dir() {
        fs::remove_dir_all(p).map_err(RemoveError::Io)?;
    } else if p.exists() {
        fs::remove_file(p).map_err(RemoveError::Io)?;
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
