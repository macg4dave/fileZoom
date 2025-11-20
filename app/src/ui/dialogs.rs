use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap, Clear};
use ratatui::Frame;

use crate::ui::colors::current as theme_current;

/// Draw the settings dialog. Shows a simple list of settings and footer
/// buttons Save/Cancel. `selected` indexes the currently focused row:
/// 0 = mouse_enabled, 1 = mouse_double_click_ms, 2 = Save, 3 = Cancel.
pub fn draw_settings(
    f: &mut Frame,
    area: Rect,
    app: &crate::app::App,
    selected: usize,
) {
    let rect = crate::ui::modal::centered_rect(area, 60, 10);
    let theme = theme_current();

    // Build body lines showing current settings; use styling to highlight the
    // currently focused row rather than a simple marker character.
    use ratatui::text::{Span, Line};
    let mouse_on = if app.settings.mouse_enabled { "On" } else { "Off" };
    let ms = app.settings.mouse_double_click_ms;

    let mut lines: Vec<Line> = Vec::new();
    let normal_style = theme.help_block_style;
    let highlight_style = theme.highlight_style;

    let line0 = Line::from(vec![
        Span::raw("Mouse support: "),
        Span::styled(mouse_on, if selected == 0 { highlight_style } else { normal_style }),
    ]);
    let line1 = Line::from(vec![
        Span::raw("Double-click timeout (ms): "),
        Span::styled(format!("{}", ms), if selected == 1 { highlight_style } else { normal_style }),
    ]);
    lines.push(line0);
    lines.push(line1);

    let body = Paragraph::new(lines)
        .block(Block::default().borders(Borders::NONE))
        .wrap(Wrap { trim: true });

    let title_block = Block::default()
        .borders(Borders::ALL)
        .title("Settings")
        .style(theme.preview_block_style);

    f.render_widget(Clear, rect);
    f.render_widget(title_block, rect);

    let content_rect = Rect::new(
        rect.x + 1,
        rect.y + 1,
        rect.width.saturating_sub(2),
        rect.height.saturating_sub(3),
    );
    f.render_widget(body, content_rect);

    // Footer buttons Save / Cancel. Highlight according to selection index 2/3
    let mut btn_text = String::new();
    let buttons = ["Save", "Cancel"];
    for (i, b) in buttons.iter().enumerate() {
        if i > 0 { btn_text.push_str("    "); }
        let idx = i + 2;
        if selected == idx { btn_text.push_str(&format!("[{}]", b)); } else { btn_text.push_str(&format!(" {} ", b)); }
    }
    let buttons_para = Paragraph::new(btn_text)
        .block(Block::default())
        .style(theme.help_block_style);
    let buttons_rect = Rect::new(
        rect.x + 1,
        rect.y + rect.height.saturating_sub(2),
        rect.width.saturating_sub(2),
        1,
    );
    f.render_widget(buttons_para, buttons_rect);
}

/// Draw a centered dialog with a title, content and a small buttons/footer line.
pub fn draw_confirm(
    f: &mut Frame,
    area: Rect,
    prompt: &str,
    content: &str,
    buttons: &[&str],
    selected: usize,
) {
    let rect = crate::ui::modal::centered_rect(area, 60, 8);
    let theme = theme_current();

    let body = Paragraph::new(content.to_string())
        .block(Block::default().borders(Borders::NONE))
        .wrap(Wrap { trim: true });

    let title_block = Block::default()
        .borders(Borders::ALL)
        .title(prompt)
        .style(theme.preview_block_style);

    // clear background then render dialog block so popup stands out
    f.render_widget(Clear, rect);
    f.render_widget(title_block, rect);

    // content area inside the block (leave 1 cell margin)
    let content_rect = Rect::new(
        rect.x + 1,
        rect.y + 1,
        rect.width.saturating_sub(2),
        rect.height.saturating_sub(3),
    );
    f.render_widget(body, content_rect);

    // Render buttons with highlight for selected
    let mut btn_text = String::new();
    for (i, b) in buttons.iter().enumerate() {
        if i > 0 {
            btn_text.push_str("    ");
        }
        if i == selected {
            // will render full line then overlay highlight by rendering styled paragraph
            btn_text.push_str(&format!("[{}]", b));
        } else {
            btn_text.push_str(&format!(" {} ", b));
        }
    }

    let buttons_para = Paragraph::new(btn_text)
        .block(Block::default())
        .style(theme.help_block_style);
    let buttons_rect = Rect::new(
        rect.x + 1,
        rect.y + rect.height.saturating_sub(2),
        rect.width.saturating_sub(2),
        1,
    );
    f.render_widget(buttons_para, buttons_rect);
}

/// Draw a simple informational dialog with an OK hint.
pub fn draw_info(
    f: &mut Frame,
    area: Rect,
    title: &str,
    content: &str,
    buttons: &[&str],
    selected: usize,
) {
    let rect = crate::ui::modal::centered_rect(area, 60, 8);
    let theme = theme_current();

    let body = Paragraph::new(content.to_string())
        .block(Block::default().borders(Borders::NONE))
        .wrap(Wrap { trim: true });

    let title_block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .style(theme.preview_block_style);
    f.render_widget(Clear, rect);
    f.render_widget(title_block, rect);

    let content_rect = Rect::new(
        rect.x + 1,
        rect.y + 1,
        rect.width.saturating_sub(2),
        rect.height.saturating_sub(3),
    );
    f.render_widget(body, content_rect);

    // Render buttons similarly to confirm
    let mut btn_text = String::new();
    for (i, b) in buttons.iter().enumerate() {
        if i > 0 {
            btn_text.push_str("    ");
        }
        if i == selected {
            btn_text.push_str(&format!("[{}]", b));
        } else {
            btn_text.push_str(&format!(" {} ", b));
        }
    }

    let buttons_para = Paragraph::new(btn_text)
        .block(Block::default())
        .style(theme.help_block_style);
    let buttons_rect = Rect::new(
        rect.x + 1,
        rect.y + rect.height.saturating_sub(2),
        rect.width.saturating_sub(2),
        1,
    );
    f.render_widget(buttons_para, buttons_rect);
}

/// Draw an error dialog; styled like info but reserved for errors.
pub fn draw_error(
    f: &mut Frame,
    area: Rect,
    title: &str,
    content: &str,
    buttons: &[&str],
    selected: usize,
) {
    // Style title with red foreground for errors
    let rect = crate::ui::modal::centered_rect(area, 60, 8);
    let theme = theme_current();
    let title_style = theme.preview_block_style.fg(Color::Red);

    let body = Paragraph::new(content.to_string())
        .block(Block::default().borders(Borders::NONE))
        .wrap(Wrap { trim: true });

    let title_block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .style(title_style);
    f.render_widget(Clear, rect);
    f.render_widget(title_block, rect);

    let content_rect = Rect::new(
        rect.x + 1,
        rect.y + 1,
        rect.width.saturating_sub(2),
        rect.height.saturating_sub(3),
    );
    f.render_widget(body, content_rect);

    // Render buttons
    let mut btn_text = String::new();
    for (i, b) in buttons.iter().enumerate() {
        if i > 0 {
            btn_text.push_str("    ");
        }
        if i == selected {
            btn_text.push_str(&format!("[{}]", b));
        } else {
            btn_text.push_str(&format!(" {} ", b));
        }
    }
    let buttons_para = Paragraph::new(btn_text)
        .block(Block::default())
        .style(theme.help_block_style);
    let buttons_rect = Rect::new(
        rect.x + 1,
        rect.y + rect.height.saturating_sub(2),
        rect.width.saturating_sub(2),
        1,
    );
    f.render_widget(buttons_para, buttons_rect);
}
