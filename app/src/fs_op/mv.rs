use std::fs;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;

/// Rename a path within the same parent directory (keeps parent).
pub fn rename_path<P: AsRef<Path>>(path: P, new_name: &str) -> Result<()> {
    let p = path.as_ref();
    let parent = p
        .parent()
        .ok_or_else(|| anyhow::anyhow!("path has no parent: {:?}", p))?;
    let dest = parent.join(new_name);
    fs::rename(p, &dest).with_context(|| format!("renaming {:?} -> {:?}", p, dest))?;
    Ok(())
}

/// Copy path to `dest`. If `src` is a directory, copy recursively into `dest`.
pub fn copy_path<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dest: Q) -> Result<()> {
    let s = src.as_ref();
    let d = dest.as_ref();
    if s.is_dir() {
        // ensure dest exists
        fs::create_dir_all(d).with_context(|| format!("creating dest dir {:?}", d))?;
        for entry in fs::read_dir(s).with_context(|| format!("reading dir {:?}", s))? {
            let ent = entry?;
            let file_name = ent.file_name();
            let src_child = ent.path();
            let dest_child = d.join(file_name);
            if src_child.is_dir() {
                copy_path(&src_child, &dest_child)?;
            } else {
                fs::copy(&src_child, &dest_child)
                    .with_context(|| format!("copying {:?} -> {:?}", src_child, dest_child))?;
            }
        }
    } else {
        // dest may be directory or file path. If dest is dir, copy into it.
        let final_dest = if d.exists() && d.is_dir() {
            d.join(
                s.file_name()
                    .ok_or_else(|| anyhow::anyhow!("source has no filename"))?,
            )
        } else {
            d.to_path_buf()
        };
        if let Some(parent) = final_dest.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("creating parent for {:?}", final_dest))?;
        }
        fs::copy(s, &final_dest).with_context(|| format!("copying {:?} -> {:?}", s, final_dest))?;
    }
    Ok(())
}

/// Move (rename) path to `dest`. If `rename` fails (cross-device), fallback to copy+remove.
pub fn move_path<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dest: Q) -> Result<()> {
    let s = src.as_ref();
    let d = dest.as_ref();
    // If destination is an existing directory, move into it
    let final_dest = if d.exists() && d.is_dir() {
        d.join(
            s.file_name()
                .ok_or_else(|| anyhow::anyhow!("src has no filename"))?,
        )
    } else {
        d.to_path_buf()
    };

    match fs::rename(s, &final_dest) {
        Ok(_) => return Ok(()),
        Err(e) => {
            // try fallback
            copy_path(s, &final_dest).with_context(|| {
                format!("fallback copying {:?} -> {:?}: {:?}", s, final_dest, e)
            })?;
            // remove original (file or dir)
            if s.is_dir() {
                fs::remove_dir_all(s).with_context(|| format!("removing dir {:?}", s))?;
            } else if s.exists() {
                fs::remove_file(s).with_context(|| format!("removing file {:?}", s))?;
            }
            Ok(())
        }
    }
}
