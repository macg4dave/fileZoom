use ratatui::style::{Color, Style};

#[derive(Clone, Debug)]
pub struct Theme {
    pub bg: Color,
    pub fg: Color,
    pub accent: Color,
}

impl Theme {
    pub fn dark() -> Self {
        Self { bg: Color::Rgb(10, 11, 12), fg: Color::Gray, accent: Color::Cyan }
    }

    pub fn light() -> Self {
        Self { bg: Color::White, fg: Color::Black, accent: Color::Blue }
    }

    pub fn style_fg(&self) -> Style { Style::default().fg(self.fg).bg(self.bg) }
}
