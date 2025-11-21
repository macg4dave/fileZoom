use ratatui::style::Style;

#[derive(Clone)]
pub struct Colors { pub preview_block_style: Style }

pub fn set_theme(_name: &str) { /* no-op scaffold */ }

pub fn current() -> Colors { Colors { preview_block_style: Style::default() } }

pub fn toggle() { /* toggle theme placeholder */ }
