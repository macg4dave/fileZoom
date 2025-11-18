use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui::Frame;

use crate::app::App;
use crate::app::Mode;

pub fn draw_list<B: Backend>(f: &mut Frame<B>, area: Rect, app: &App) {
    let list_height = (area.height as usize).saturating_sub(2); // account for borders/title
    let visible = if app.entries.len() > app.offset { &app.entries[app.offset..std::cmp::min(app.offset + list_height, app.entries.len())] } else { &[] };

    let items: Vec<ListItem> = visible.iter().map(|e| {
        let name = &e.name;
        let size = if e.is_dir { "<dir>".to_string() } else { format!("{}", e.size) };
        let mtime = e.modified.map(|d| d.format("%Y-%m-%d %H:%M").to_string()).unwrap_or_else(|| "-".to_string());
        let text = format!("{:<40.40} {:>10} {:>16}", name, size, mtime);
        let style = if e.is_dir { Style::default().fg(Color::Blue) } else { Style::default() };
        ListItem::new(Spans::from(vec![Span::styled(text, style)]))
    }).collect();

    let title = format!("{}  (sorted)", app.cwd.display());
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).bg(Color::Gray));

    let mut state = tui::widgets::ListState::default();
    if app.selected >= app.offset && app.selected < app.offset + list_height {
        state.select(Some(app.selected - app.offset));
    } else {
        state.select(None);
    }
    f.render_stateful_widget(list, area, &mut state);
}

pub fn draw_preview<B: Backend>(f: &mut Frame<B>, area: Rect, app: &App) {
    let max_lines = (area.height as usize).saturating_sub(2);
    let lines: Vec<&str> = app.preview.lines().collect();
    let visible = if app.preview_offset < lines.len() { &lines[app.preview_offset..std::cmp::min(app.preview_offset + max_lines, lines.len())] } else { &[] };
    let text = visible.iter().fold(String::new(), |mut acc, l| { acc.push_str(l); acc.push('\n'); acc });
    let preview = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Preview"));
    f.render_widget(preview, area);
}

pub fn draw_modal<B: Backend>(f: &mut Frame<B>, area: Rect, prompt: &str, content: &str) {
    let w = area.width.min(80);
    let h = area.height.min(10);
    let x = (area.width - w) / 2 + area.x;
    let y = (area.height - h) / 2 + area.y;
    let rect = Rect::new(x, y, w, h);
    let p = Paragraph::new(content.to_string()).block(Block::default().borders(Borders::ALL).title(prompt));
    f.render_widget(p, rect);
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
        .split(f.size());

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    draw_list(f, main_chunks[0], app);
    draw_preview(f, main_chunks[1], app);

    let help = Paragraph::new("↑/↓:navigate  PgUp/PgDn:page  Enter:open  Backspace:up  d:delete  c:copy  m:move  R:rename  n:new file  N:new dir  s:sort  q:quit")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[1]);

    // Modal
    match &app.mode {
        Mode::Confirm { msg, .. } => draw_modal(f, f.size(), "Confirm", msg),
        Mode::Input { prompt, buffer, .. } => draw_modal(f, f.size(), prompt, buffer),
        Mode::Normal => {}
    }
}
