use super::*;

impl App {
    pub fn ensure_selection_visible(&mut self, list_height: usize) {
        let panel = match self.active {
            Side::Left => &mut self.left,
            Side::Right => &mut self.right,
        };
        if panel.selected < panel.offset {
            panel.offset = panel.selected;
        } else if panel.selected >= panel.offset + list_height {
            panel.offset = panel.selected + 1 - list_height;
        }
    }

    pub fn next(&mut self, list_height: usize) {
        match self.active {
            Side::Left => {
                if !self.left.entries.is_empty() {
                    self.left.selected = min(self.left.selected + 1, self.left.entries.len() - 1);
                }
            }
            Side::Right => {
                if !self.right.entries.is_empty() {
                    self.right.selected =
                        min(self.right.selected + 1, self.right.entries.len() - 1);
                }
            }
        }
        self.ensure_selection_visible(list_height);
        self.update_preview_for(self.active);
    }

    pub fn previous(&mut self, list_height: usize) {
        match self.active {
            Side::Left => {
                if !self.left.entries.is_empty() {
                    self.left.selected = self.left.selected.saturating_sub(1);
                }
            }
            Side::Right => {
                if !self.right.entries.is_empty() {
                    self.right.selected = self.right.selected.saturating_sub(1);
                }
            }
        }
        self.ensure_selection_visible(list_height);
        self.update_preview_for(self.active);
    }

    pub fn page_down(&mut self, list_height: usize) {
        match self.active {
            Side::Left => {
                if !self.left.entries.is_empty() {
                    self.left.selected = min(
                        self.left.selected + list_height,
                        self.left.entries.len() - 1,
                    );
                }
            }
            Side::Right => {
                if !self.right.entries.is_empty() {
                    self.right.selected = min(
                        self.right.selected + list_height,
                        self.right.entries.len() - 1,
                    );
                }
            }
        }
        self.ensure_selection_visible(list_height);
        self.update_preview_for(self.active);
    }

    pub fn page_up(&mut self, list_height: usize) {
        match self.active {
            Side::Left => {
                if !self.left.entries.is_empty() {
                    self.left.selected = self.left.selected.saturating_sub(list_height);
                }
            }
            Side::Right => {
                if !self.right.entries.is_empty() {
                    self.right.selected = self.right.selected.saturating_sub(list_height);
                }
            }
        }
        self.ensure_selection_visible(list_height);
        self.update_preview_for(self.active);
    }
}
