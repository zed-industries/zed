use gpui::ClipboardItem;
use ui::{Context, Window};

use std::collections::BTreeMap;

use crate::{CopySelected, CsvPreviewView, settings::CopyFormat};
use std::collections::HashSet;
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

        let content = if self.settings.copy_format == CopyFormat::Markdown {
            self.format_as_markdown_table(&rows_data)
        } else {
            // Build CSV/TSV format: determine column range for each row
            let mut lines = Vec::new();

            for (_row_idx, columns) in rows_data {
                if columns.is_empty() {
                    continue;
                }

                // Get the range of columns for this row
                let min_col = *columns.keys().next().unwrap();
                let max_col = *columns.keys().last().unwrap();

                // Build the row with separators between columns, filling empty cells
                let mut row_cells = Vec::new();
                for col in min_col..=max_col {
                    let cell_value = columns.get(&col).cloned().unwrap_or_default();
                    row_cells.push(cell_value);
                }

                let separator = match self.settings.copy_format {
                    CopyFormat::Tsv => "\t",
                    CopyFormat::Csv => ",",
                    CopyFormat::Semicolon => ";",
                    CopyFormat::Markdown => unreachable!(),
                };

                // Escape cells if they contain separators, quotes, or newlines
                let formatted_cells: Vec<String> = match self.settings.copy_format {
                    CopyFormat::Tsv => row_cells
                        .into_iter()
                        .map(|cell| {
                            if cell.contains('\t') || cell.contains('"') || cell.contains('\n') {
                                format!("\"{}\"", cell.replace("\"", "\"\""))
                            } else {
                                cell
                            }
                        })
                        .collect(),
                    CopyFormat::Csv => row_cells
                        .into_iter()
                        .map(|cell| {
                            if cell.contains(',') || cell.contains('"') || cell.contains('\n') {
                                format!("\"{}\"", cell.replace("\"", "\"\""))
                            } else {
                                cell
                            }
                        })
                        .collect(),
                    CopyFormat::Semicolon => row_cells
                        .into_iter()
                        .map(|cell| {
                            if cell.contains(';') || cell.contains('"') || cell.contains('\n') {
                                format!("\"{}\"", cell.replace("\"", "\"\""))
                            } else {
                                cell
                            }
                        })
                        .collect(),
                    CopyFormat::Markdown => unreachable!(),
                };

                lines.push(formatted_cells.join(separator));
            }

            lines.join("\n")
        };
        cx.write_to_clipboard(ClipboardItem::new_string(content));
    }

    fn format_as_markdown_table(
        &self,
        rows_data: &BTreeMap<usize, BTreeMap<usize, String>>,
    ) -> String {
        if rows_data.is_empty() {
            return String::new();
        }

        // Determine which columns are selected
        let mut selected_columns: HashSet<usize> = HashSet::new();
        for columns in rows_data.values() {
            selected_columns.extend(columns.keys());
        }
        let mut sorted_columns: Vec<usize> = selected_columns.into_iter().collect();
        sorted_columns.sort();

        // Build header row with column names
        let mut markdown_lines = Vec::new();
        let header_cells: Vec<String> = sorted_columns
            .iter()
            .map(|&col_idx| {
                self.contents
                    .headers
                    .get(col_idx)
                    .map(|h| h.as_ref().replace('\n', "<br>").replace('|', "\\|"))
                    .unwrap_or_else(|| format!("Col {}", col_idx + 1))
            })
            .collect();

        // Add header row
        markdown_lines.push(format!("| {} |", header_cells.join(" | ")));

        // Add separator row
        let separator_cells: Vec<String> =
            sorted_columns.iter().map(|_| "---".to_string()).collect();
        markdown_lines.push(format!("| {} |", separator_cells.join(" | ")));

        // Add data rows
        for (_row_idx, columns) in rows_data {
            let data_cells: Vec<String> = sorted_columns
                .iter()
                .map(|&col_idx| {
                    columns
                        .get(&col_idx)
                        .cloned()
                        .unwrap_or_default()
                        .replace('\n', "<br>")
                        .replace('|', "\\|")
                })
                .collect();

            markdown_lines.push(format!("| {} |", data_cells.join(" | ")));
        }

        markdown_lines.join("\n")
    }
}
