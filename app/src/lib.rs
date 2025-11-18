pub mod app;
#[path = "ui/mod.rs"]
pub mod ui;
#[path = "runner/mod.rs"]
pub mod runner;
pub mod input;
pub mod fs_op;

pub use crate::app::{Action, App, Entry, InputKind, Mode, Side, SortKey};
pub use crate::app::path;

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
