use gpui::ListAlignment;

use crate::{CsvPreviewView, types::AnyColumn};

mod copy_selected;
mod selection_handlers;

impl CsvPreviewView {
    pub fn clear_filters(&mut self, col_idx: AnyColumn) {
        self.engine.clear_filters(col_idx);
        self.engine.calculate_d2d_mapping();
        self.list_state = gpui::ListState::new(
            self.engine.d2d_mapping.filtered_row_count(),
            ListAlignment::Top,
            ui::px(1.),
        );
    }
}
