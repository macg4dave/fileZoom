use std::path::{Path, PathBuf};

/// Resolve and validate a user-supplied path for changing panel cwd.
///
/// Expands a leading `~` to the user's home directory (if available), treats
/// relative paths as relative to `base`, and returns an error string when the
/// resolved path does not exist or is not a directory.
pub fn resolve_path(input: &str, base: &Path) -> Result<PathBuf, String> {
    if input.trim().is_empty() {
        return Err("empty path".to_string());
    }

    // Expand tilde (~) to home directory when present.
    let candidate: PathBuf = if input.starts_with('~') {
        if let Some(home) = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")) {
            let rest = input.trim_start_matches('~');
            let mut p = PathBuf::from(home);
            if !rest.is_empty() {
                p.push(rest.trim_start_matches(|c| c == '/' || c == '\\'));
            }
            p
        } else {
            return Err("could not determine home directory".to_string());
        }
    } else {
        let p = PathBuf::from(input);
        if p.is_absolute() {
            p
        } else {
            base.join(p)
        }
    };

    if candidate.exists() {
        if candidate.is_dir() {
            Ok(candidate)
        } else {
            Err(format!("not a directory: {}", candidate.display()))
        }
    } else {
        Err(format!("path does not exist: {}", candidate.display()))
    }
}
