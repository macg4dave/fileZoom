pub mod app;
pub mod errors;
pub mod fs_op;
pub mod input;
#[path = "runner/mod.rs"]
pub mod runner;
#[path = "ui/mod.rs"]
pub mod ui;

pub use crate::app::path;
pub use crate::app::{Action, App, Entry, InputKind, Mode, Side, SortKey};

pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_returns_expected() {
        assert_eq!(greet("Alice"), "Hello, Alice!");
    }
}
