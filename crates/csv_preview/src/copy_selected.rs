use gpui::ClipboardItem;
use ui::{Context, Window};

use std::collections::BTreeMap;

use crate::{CopySelected, CsvPreviewView};
impl CsvPreviewView {
    pub(crate) fn copy_selected(
        &mut self,
        _: &CopySelected,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let selected_cells = self.selection.get_selected_cells();

        if selected_cells.is_empty() {
            return;
        }

        // Group selected cells by row, then by column for proper TSV formatting
        let mut rows_data: BTreeMap<usize, BTreeMap<usize, String>> = BTreeMap::new();

        for cell_id in selected_cells {
            let row_idx = cell_id.row.get();
            let col_idx = cell_id.col;

            if let Some(row) = self.contents.rows.get(row_idx) {
                let cell_content = row
                    .get(col_idx)
                    .map(|s| s.as_ref().to_string())
                    .unwrap_or_default();

                rows_data
                    .entry(row_idx)
                    .or_default()
                    .insert(col_idx, cell_content);
            }
        }

        // Build TSV format: determine column range for each row
        let mut tsv_lines = Vec::new();

        for (_row_idx, columns) in rows_data {
            if columns.is_empty() {
                continue;
            }

            // Get the range of columns for this row
            let min_col = *columns.keys().next().unwrap();
            let max_col = *columns.keys().last().unwrap();

            // Build the row with tabs between columns, filling empty cells
            let mut row_cells = Vec::new();
            for col in min_col..=max_col {
                let cell_value = columns.get(&col).cloned().unwrap_or_default();
                row_cells.push(cell_value);
            }

            tsv_lines.push(row_cells.join("\t"));
        }

        let tsv_content = tsv_lines.join("\n");
        cx.write_to_clipboard(ClipboardItem::new_string(tsv_content));
    }
}
