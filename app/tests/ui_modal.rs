use fileZoom::ui::modal::centered_rect;
use ratatui::layout::Rect;

#[test]
fn centered_rect_within_bounds() {
    let area = Rect::new(0, 0, 100, 40);
    let r = centered_rect(area, 80, 10);
    assert_eq!(r.width, 80);
    assert_eq!(r.height, 10);
    assert_eq!(r.x, 10);
    assert_eq!(r.y, 15);
}

#[test]
fn centered_rect_shrinks_if_needed() {
    let area = Rect::new(5, 5, 20, 6);
    let r = centered_rect(area, 80, 10);
    assert_eq!(r.width, 20);
    assert_eq!(r.height, 6);
    assert_eq!(r.x, 5);
    assert_eq!(r.y, 5);
}
