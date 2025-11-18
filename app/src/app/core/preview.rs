use super::*;
use std::io::Read;

impl App {
    pub fn update_preview_for(&mut self, side: Side) {
        let panel = match side {
            Side::Left => &mut self.left,
            Side::Right => &mut self.right,
        };
        panel.preview_offset = 0;
        if let Some(e) = panel.entries.get(panel.selected) {
            if e.is_dir {
                let mut s = format!("Directory: {}\n", e.path.display());
                if let Ok(list) = fs::read_dir(&e.path) {
                    for ent in list.flatten().take(50) {
                        s.push_str(&format!("{}\n", ent.file_name().to_string_lossy()));
                    }
                }
                panel.preview = s;
            } else {
                // Read up to MAX_PREVIEW_BYTES from the file for preview.
                match fs::File::open(&e.path) {
                    Ok(mut f) => {
                        let mut buf = Vec::new();
                        let read_bytes = match (&mut f)
                            .take(App::MAX_PREVIEW_BYTES as u64)
                            .read_to_end(&mut buf)
                        {
                            Ok(n) => n,
                            Err(_) => {
                                panel.preview = format!("Binary or unreadable file: {}", e.name);
                                return;
                            }
                        };
                        let preview = String::from_utf8_lossy(&buf).into_owned();
                        // If file is larger than what we read, append truncation note.
                        let truncated = match f.metadata() {
                            Ok(md) => (md.len() as usize) > read_bytes,
                            Err(_) => false,
                        };
                        if truncated {
                            panel.preview = format!("{}\n... (truncated)", preview);
                        } else {
                            panel.preview = preview;
                        }
                    }
                    Err(_) => panel.preview = format!("Binary or unreadable file: {}", e.name),
                }
            }
        } else {
            panel.preview.clear();
        }
    }
}
