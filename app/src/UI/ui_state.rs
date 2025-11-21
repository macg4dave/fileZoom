use serde::Serialize;

/// Thin view model passed to renderers — keeps widget code testable and small.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UIState {
    pub left_list: Vec<String>,
    pub selected_index: usize,
    pub preview_text: Option<String>,
    pub progress: u16,
}

impl UIState {
    pub fn sample() -> Self {
        Self { left_list: vec!["one".into(), "two".into(), "three".into()], selected_index: 0, preview_text: Some("preview".into()), progress: 25 }
    }
}
