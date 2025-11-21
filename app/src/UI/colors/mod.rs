use ratatui::style::{Style, Color};
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Clone, Debug)]
pub struct Colors { pub preview_block_style: Style }

static CURRENT: Lazy<Mutex<Colors>> = Lazy::new(|| Mutex::new(Colors { preview_block_style: Style::default() }));

pub fn set_theme(name: &str) {
	let mut g = CURRENT.lock().unwrap();
	match name {
		"dark" => *g = Colors { preview_block_style: Style::default().fg(Color::White).bg(Color::Black) },
		"light" => *g = Colors { preview_block_style: Style::default().fg(Color::Black).bg(Color::White) },
		_ => *g = Colors { preview_block_style: Style::default() },
	}
}

pub fn current() -> Colors { CURRENT.lock().unwrap().clone() }

pub fn toggle() {
	let mut g = CURRENT.lock().unwrap();
	if g.preview_block_style.bg == Some(Color::Black) {
		*g = Colors { preview_block_style: Style::default().fg(Color::Black).bg(Color::White) }
	} else {
		*g = Colors { preview_block_style: Style::default().fg(Color::White).bg(Color::Black) }
	}
}
