use ratatui::layout::Rect;

/// Create a centered rectangle occupying `pct_x`% x `pct_y`% of `r`.
pub fn centered_rect(r: Rect, pct_x: u16, pct_y: u16) -> Rect {
    let pw = (r.width as u32 * pct_x as u32 / 100) as u16;
    let ph = (r.height as u32 * pct_y as u32 / 100) as u16;
    let x = r.x + (r.width.saturating_sub(pw) / 2);
    let y = r.y + (r.height.saturating_sub(ph) / 2);
    Rect::new(x, y, pw.max(1), ph.max(1))
}
