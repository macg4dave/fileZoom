use std::path::Path;

/// Return true if path exists.
pub fn exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}

/// Return true if path is a directory.
pub fn is_dir<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_dir()
}

/// Return true if path is a regular file.
pub fn is_file<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_file()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn stat_checks() {
        let td = tempdir().unwrap();
        let f = td.path().join("file.txt");
        std::fs::write(&f, b"ok").unwrap();
        assert!(exists(&f));
        assert!(is_file(&f));
        assert!(!is_dir(&f));
    }
}
