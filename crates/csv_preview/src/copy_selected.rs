use gpui::ClipboardItem;
use ui::{Context, SharedString, Window};
use workspace::{Toast, Workspace, notifications::NotificationId};

use std::collections::BTreeMap;

use crate::{CopySelected, CsvPreviewView, settings::CopyFormat};
use std::collections::HashSet;

impl CsvPreviewView {
    pub(crate) fn copy_selected(
        &mut self,
        _: &CopySelected,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let selected_cells = self.selection.get_selected_cells();

        if selected_cells.is_empty() {
            return;
        }
        let copy_format = self.settings.copy_format;
        let full_content = &self.contents;

        // Group selected cells by row, then by column for proper TSV formatting
        let mut rows_data: BTreeMap<usize, BTreeMap<usize, String>> = BTreeMap::new();

        for cell_id in selected_cells {
            let row_idx = cell_id.row.get();
            let col_idx = cell_id.col.get();

            if let Some(row) = (&full_content.rows).get(row_idx) {
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

        let toast_info = calculate_toast_info(selected_cells, copy_format, &rows_data);

        let content = if copy_format == CopyFormat::Markdown {
            format_as_markdown_table(&full_content.headers, &rows_data)
        } else {
            // Build CSV/TSV format: determine global column range for entire selection
            let mut lines = Vec::new();

            // Find the global min and max columns across all rows
            let mut global_min_col = usize::MAX;
            let mut global_max_col = 0;
            for columns in rows_data.values() {
                if !columns.is_empty() {
                    let row_min = *columns.keys().next().unwrap();
                    let row_max = *columns.keys().last().unwrap();
                    global_min_col = global_min_col.min(row_min);
                    global_max_col = global_max_col.max(row_max);
                }
            }

            for (_row_idx, columns) in rows_data {
                if columns.is_empty() {
                    continue;
                }

                // Build the row using global column range, filling empty cells
                let mut row_cells = Vec::new();
                for col in global_min_col..=global_max_col {
                    let cell_value = columns.get(&col).cloned().unwrap_or_default();
                    row_cells.push(cell_value);
                }

                let separator = match copy_format {
                    CopyFormat::Tsv => "\t",
                    CopyFormat::Csv => ",",
                    CopyFormat::Semicolon => ";",
                    CopyFormat::Markdown => unreachable!(),
                };

                // Escape cells if they contain separators, quotes, or newlines
                let formatted_cells: Vec<String> = match copy_format {
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

        // Show toast notification
        if let Some(Some(workspace)) = window.root() {
            show_toast_with_copy_results(cx, copy_format, toast_info, workspace);
        }
    }
}

fn format_as_markdown_table(
    all_table_headers: &[SharedString],
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
            all_table_headers
                .get(col_idx)
                .map(|h| h.as_ref().replace('\n', "<br>").replace('|', "\\|"))
                .unwrap_or_else(|| format!("Col {}", col_idx + 1))
        })
        .collect();

    // Add header row
    markdown_lines.push(format!("| {} |", header_cells.join(" | ")));

    // Add separator row
    let separator_cells: Vec<String> = sorted_columns.iter().map(|_| "---".to_string()).collect();
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

///// Notifications /////

#[derive(Debug)]
struct ToastInfo {
    selected_cell_count: usize,
    rectangle_dimensions: (usize, usize),
    empty_cells_count: usize,
}

fn show_toast_with_copy_results(
    cx: &mut Context<'_, CsvPreviewView>,
    copy_format: CopyFormat,
    toast_info: ToastInfo,
    workspace: gpui::Entity<Workspace>,
) {
    let format_name = match copy_format {
        CopyFormat::Tsv => "TSV",
        CopyFormat::Csv => "CSV",
        CopyFormat::Semicolon => "Semicolon",
        CopyFormat::Markdown => "Markdown",
    };

    let (rows, cols) = toast_info.rectangle_dimensions;
    let message = if toast_info.selected_cell_count == 1 {
        format!("1 cell copied as {}", format_name)
    } else if toast_info.empty_cells_count == 0 {
        format!(
            "{} cells copied as {} ({}×{})",
            toast_info.selected_cell_count, format_name, rows, cols
        )
    } else {
        format!(
            "{} cells copied as {} ({}×{}, {} empty)",
            toast_info.selected_cell_count, format_name, rows, cols, toast_info.empty_cells_count
        )
    };

    workspace.update(cx, |workspace: &mut Workspace, cx| {
        struct CsvCopyToast;
        workspace.show_toast(
            Toast::new(NotificationId::unique::<CsvCopyToast>(), message).autohide(),
            cx,
        );
    });
}

fn calculate_toast_info(
    selected_cells: &HashSet<crate::types::DataCellId>,
    copy_format: CopyFormat,
    rows_data: &BTreeMap<usize, BTreeMap<usize, String>>,
) -> ToastInfo {
    let selected_cell_count = selected_cells.len();
    let (rectangle_dimensions, empty_cells_count) = if copy_format == CopyFormat::Markdown {
        // For markdown, use the selected columns approach
        let mut selected_columns: HashSet<usize> = HashSet::new();
        for columns in rows_data.values() {
            selected_columns.extend(columns.keys());
        }
        let cols = selected_columns.len();
        let rows = rows_data.len();
        let total_cells = rows * cols;
        let empty_cells = total_cells - selected_cell_count;
        ((rows, cols), empty_cells)
    } else {
        // For CSV/TSV, calculate global column range
        let mut global_min_col = usize::MAX;
        let mut global_max_col = 0;
        for columns in rows_data.values() {
            if !columns.is_empty() {
                let row_min = *columns.keys().next().unwrap();
                let row_max = *columns.keys().last().unwrap();
                global_min_col = global_min_col.min(row_min);
                global_max_col = global_max_col.max(row_max);
            }
        }
        let cols = if global_min_col <= global_max_col {
            global_max_col - global_min_col + 1
        } else {
            0
        };
        let rows = rows_data.len();
        let total_cells = rows * cols;
        let empty_cells = total_cells - selected_cell_count;
        ((rows, cols), empty_cells)
    };
    ToastInfo {
        selected_cell_count,
        rectangle_dimensions,
        empty_cells_count,
    }
}
