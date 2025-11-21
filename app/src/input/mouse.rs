//! Crate-local mouse types and conversions.
//!
//! This module provides a small, crate-scoped representation of mouse
//! events so the rest of the codebase does not depend directly on
//! `crossterm` types. It exposes ergonomic `From` implementations to
//! convert `crossterm` events into the local types.

/// Logical mouse buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    /// Left mouse button.
    Left,
    /// Right mouse button.
    Right,
    /// Middle mouse button (wheel click).
    Middle,
    // (No other variants are currently defined by `crossterm`.)
}

/// Logical mouse event kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseEventKind {
    /// A button was pressed.
    Down(MouseButton),
    /// A button was released.
    Up(MouseButton),
    /// Mouse moved while a button is pressed.
    Drag(MouseButton),
    /// Wheel scrolled up.
    ScrollUp,
    /// Wheel scrolled down.
    ScrollDown,
    /// Mouse moved without a button pressed.
    Move,
    /// Any other / unrecognised event kind.
    Other,
}

/// Crate-level mouse event (position + kind).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseEvent {
    /// Column (x) position of the event.
    pub column: u16,
    /// Row (y) position of the event.
    pub row: u16,
    /// Kind of mouse event.
    pub kind: MouseEventKind,
}

impl From<crossterm::event::MouseButton> for MouseButton {
    fn from(btn: crossterm::event::MouseButton) -> Self {
        use crossterm::event::MouseButton as CtBtn;
        match btn {
            CtBtn::Left => MouseButton::Left,
            CtBtn::Right => MouseButton::Right,
            CtBtn::Middle => MouseButton::Middle,
        }
    }
}

impl From<crossterm::event::MouseEventKind> for MouseEventKind {
    fn from(kind: crossterm::event::MouseEventKind) -> Self {
        use crossterm::event::MouseEventKind as CtKind;
        match kind {
            CtKind::Down(btn) => MouseEventKind::Down(btn.into()),
            CtKind::Up(btn) => MouseEventKind::Up(btn.into()),
            CtKind::Drag(btn) => MouseEventKind::Drag(btn.into()),
            CtKind::ScrollUp => MouseEventKind::ScrollUp,
            CtKind::ScrollDown => MouseEventKind::ScrollDown,
            CtKind::Moved => MouseEventKind::Move,
            _ => MouseEventKind::Other,
        }
    }
}

impl From<crossterm::event::MouseEvent> for MouseEvent {
    fn from(me: crossterm::event::MouseEvent) -> Self {
        MouseEvent {
            column: me.column,
            row: me.row,
            kind: me.kind.into(),
        }
    }
}

/// Returns `true` if the supplied event is a left-button press.
pub fn is_left_down(ev: &MouseEvent) -> bool {
    matches!(ev.kind, MouseEventKind::Down(MouseButton::Left))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crossterm::event::{
        MouseButton as CtBtn,
        MouseEvent as CtEvent,
        MouseEventKind as CtKind,
        KeyModifiers as CtMods,
    };

    #[test]
    fn from_crossterm_down_left() {
        let ct = CtEvent { kind: CtKind::Down(CtBtn::Left), column: 10, row: 5, modifiers: CtMods::NONE };
        let me: MouseEvent = ct.into();
        assert_eq!(me.column, 10);
        assert_eq!(me.row, 5);
        assert_eq!(me.kind, MouseEventKind::Down(MouseButton::Left));
        assert!(is_left_down(&me));
    }

    #[test]
    fn from_crossterm_up_right() {
        let ct = CtEvent { kind: CtKind::Up(CtBtn::Right), column: 1, row: 2, modifiers: CtMods::NONE };
        let me: MouseEvent = ct.into();
        assert_eq!(me.kind, MouseEventKind::Up(MouseButton::Right));
        assert!(!is_left_down(&me));
    }

    #[test]
    fn from_crossterm_drag_middle() {
        let ct = CtEvent { kind: CtKind::Drag(CtBtn::Middle), column: 3, row: 4, modifiers: CtMods::NONE };
        let me: MouseEvent = ct.into();
        assert_eq!(me.kind, MouseEventKind::Drag(MouseButton::Middle));
    }

    #[test]
    fn scroll_and_move_kinds() {
        let up = CtEvent { kind: CtKind::ScrollUp, column: 0, row: 0, modifiers: CtMods::NONE };
        let down = CtEvent { kind: CtKind::ScrollDown, column: 0, row: 0, modifiers: CtMods::NONE };
        let mv = CtEvent { kind: CtKind::Moved, column: 0, row: 0, modifiers: CtMods::NONE };
        assert_eq!(MouseEvent::from(up).kind, MouseEventKind::ScrollUp);
        assert_eq!(MouseEvent::from(down).kind, MouseEventKind::ScrollDown);
        assert_eq!(MouseEvent::from(mv).kind, MouseEventKind::Move);
    }
}
