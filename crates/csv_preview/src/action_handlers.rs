use gpui::ListAlignment;

use crate::{CsvPreviewView, types::AnyColumn};

mod copy_selected;
mod selection_handlers;

impl CsvPreviewView {
    pub fn clear_filters(&mut self, col_idx: AnyColumn) {
        self.engine.clear_filters_for_col(col_idx);
        self.engine.apply_filtering();
        self.list_state = gpui::ListState::new(
            self.engine.d2d_mapping().visible_row_count(),
            ListAlignment::Top,
            ui::px(1.),
        );
    }

    pub fn toggle_filter(&mut self, col_idx: AnyColumn, content_hash: u64) {
        self.engine.toggle_filter(col_idx, content_hash);
        self.engine.calculate_d2d_mapping();
        self.list_state = gpui::ListState::new(
            self.engine.d2d_mapping().visible_row_count(),
            gpui::ListAlignment::Top,
            ui::px(1.),
        );
    }
}
