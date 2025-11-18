use std::collections::HashMap;

/// Load the embedded templates from `errors_output` and parse them into a map.
fn load_templates() -> HashMap<String, String> {
    // Embed the templates at compile time so runtime doesn't depend on source files.
    // Use the TOML templates file. We prefer `errors_output.toml` for clarity.
    const RAW: &str = include_str!("errors_output.toml");
    let mut map = HashMap::new();

    for line in RAW.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
            continue;
        }

        if let Some(idx) = line.find('=') {
            let key = line[..idx].trim();
            let mut value = line[idx + 1..].trim();
            // Strip leading/trailing quotes if present
            if value.starts_with('"') && value.ends_with('"') && value.len() >= 2 {
                value = &value[1..value.len() - 1];
            }

            map.insert(key.to_string(), value.to_string());
        }
    }

    map
}

fn template_or_default(map: &HashMap<String, String>, key: &str, default: &str) -> String {
    map.get(key).cloned().unwrap_or_else(|| default.to_string())
}

/// Render an error into a user-friendly string. The function will first try to use
/// templates embedded from `errors_output`. If a template isn't present, it falls
/// back to a sensible default.
///
/// Parameters (all optional except `err`):
/// - `path`: primary path involved in the error
/// - `src` / `dst`: source/destination paths for move operations
pub fn render_io_error(
    err: &std::io::Error,
    path: Option<&str>,
    src: Option<&str>,
    dst: Option<&str>,
) -> String {
    let templates = load_templates();

    use std::io::ErrorKind;
    match err.kind() {
        ErrorKind::NotFound => {
            let tmpl = template_or_default(&templates, "path_not_found", "Path not found: {path}");
            return tmpl.replace("{path}", path.unwrap_or("<unknown>"));
        }
        ErrorKind::PermissionDenied => {
            let tmpl =
                template_or_default(&templates, "permission_denied", "Permission denied: {path}");
            return tmpl.replace("{path}", path.unwrap_or("<unknown>"));
        }
        ErrorKind::AlreadyExists => {
            let tmpl = template_or_default(
                &templates,
                "already_exists",
                "Target already exists: {path}",
            );
            return tmpl.replace("{path}", path.or(src).or(dst).unwrap_or("<unknown>"));
        }
        ErrorKind::InvalidInput => {
            let tmpl = template_or_default(&templates, "invalid_input", "Invalid input: {details}");
            return tmpl.replace("{details}", &format!("{}", err));
        }
        ErrorKind::BrokenPipe
        | ErrorKind::UnexpectedEof
        | ErrorKind::WouldBlock
        | ErrorKind::TimedOut => {
            let tmpl = template_or_default(&templates, "io_error", "I/O error: {err}");
            return tmpl.replace("{err}", &format!("{}", err));
        }
        _ => {
            // For other errors, attempt to map a move-specific template, then generic.
            if let (Some(s), Some(d)) = (src, dst) {
                let tmpl = template_or_default(
                    &templates,
                    "unable_to_move",
                    "Unable to move {src} to {dst} ({err})",
                );
                return tmpl
                    .replace("{src}", s)
                    .replace("{dst}", d)
                    .replace("{err}", &format!("{}", err));
            }

            // Fallback to a generic I/O error template
            let tmpl = template_or_default(&templates, "io_error", "I/O error: {err}");
            tmpl.replace("{err}", &format!("{}", err))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_not_found() {
        let e = io::Error::from(io::ErrorKind::NotFound);
        let out = render_io_error(&e, Some("/no/such/path"), None, None);
        assert!(out.contains("Path not found") || out.contains("not found"));
    }

    #[test]
    fn test_permission_denied() {
        let e = io::Error::from(io::ErrorKind::PermissionDenied);
        let out = render_io_error(&e, Some("/root/secret"), None, None);
        assert!(out.contains("Permission denied") || out.contains("Insufficient"));
    }

    #[test]
    fn test_already_exists() {
        let e = io::Error::from(io::ErrorKind::AlreadyExists);
        let out = render_io_error(&e, Some("/tmp/existing"), None, None);
        assert!(out.contains("already") || out.contains("exists"));
    }

    #[test]
    fn test_move_error_uses_src_dst() {
        let e = io::Error::new(io::ErrorKind::Other, "rename failed");
        let out = render_io_error(&e, None, Some("a.txt"), Some("b.txt"));
        assert!(out.contains("a.txt") && out.contains("b.txt"));
    }
}
