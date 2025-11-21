use ratatui::{layout::Rect, widgets::{List, ListItem, Block, Borders}};
use ratatui::Frame;
use crate::ui::{UIState, Theme};

pub fn render(f: &mut Frame, area: Rect, state: &UIState, _theme: &Theme) {
    let items: Vec<ListItem> = state.left_list.iter().map(|s| ListItem::new(s.clone())).collect();
    let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Files"));
    f.render_widget(list, area);
}
