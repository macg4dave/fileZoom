// Lightweight compatibility shim which re-exports the canonical
// implementation of path helpers found under `fs_op::path`.
// Existing code that imports `crate::app::path` will continue to work,
// but use `fileZoom::fs_op::path` directly for new code.
pub mod core;
pub mod types;
pub mod settings;

pub use core::panel::Panel;
pub use types::{Action, Entry, InputKind, Mode, Side, SortKey};
