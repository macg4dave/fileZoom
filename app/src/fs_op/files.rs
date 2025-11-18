// Re-export moved modules to preserve the previous public API which used
// `fs_op::files::inspect_permissions`, `fs_op::files::move_path`, etc.
pub use crate::fs_op::permissions::{format_unix_mode, change_permissions, inspect_permissions, PermissionInfo};
pub use crate::fs_op::mv::{copy_path, move_path, rename_path};
pub use crate::fs_op::create::{create_file, create_dir_all};
pub use crate::fs_op::remove::remove_path;
pub use crate::fs_op::stat::{exists, is_dir, is_file};

