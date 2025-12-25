use crate::{
    types::TableLikeContent,
    types::{AnyColumn, DataRow, DisplayRow},
};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SortDirection {
    Asc,
    Desc,
}

/// Config or currently active sorting
#[derive(Clone, Copy)]
pub struct AppliedSorting {
    /// 0-based column index
    pub col_idx: AnyColumn,
    /// Direction of sorting (asc/desc)
    pub direction: SortDirection,
}

/// Relation of Display (rendered) rows to Data (src) rows with applied transformations
/// Transformations applied:
/// - sorting by column
/// - todo: filtering
#[derive(Debug)]
pub struct DisplayToDataMapping {
    mapping: HashMap<DisplayRow, DataRow>,
}

impl DisplayToDataMapping {
    /// Get the data row for a given display row
    pub fn get_data_row(&self, display_row: DisplayRow) -> Option<DataRow> {
        self.mapping.get(&display_row).copied()
    }

    /// Get the display row for a given data row (reverse lookup)
    pub fn get_display_row(&self, data_row: DataRow) -> Option<DisplayRow> {
        self.mapping
            .iter()
            .find(|(_, mapped_data_row)| **mapped_data_row == data_row)
            .map(|(display_row, _)| *display_row)
    }
}

/// Generate sorted row indices based on current sorting settings.
/// Returns a mapping from DisplayRow to DataRow.
/// Note: sorting.col_idx refers to CSV data columns (0-based), not display columns
/// (display columns include the line number column at index 0)
pub fn generate_sorted_indices(
    sorting: Option<AppliedSorting>,
    contents: &TableLikeContent,
) -> DisplayToDataMapping {
    let indices: Vec<usize> = (0..contents.rows.len()).collect();

    let sorted_indices = if let Some(sorting) = sorting {
        sort_indices(contents, indices, sorting)
    } else {
        indices
    };

    // Create mapping from display position to data row
    let mapping: HashMap<DisplayRow, DataRow> = sorted_indices
        .iter()
        .enumerate()
        .map(|(display_idx, &data_idx)| (DisplayRow::from(display_idx), DataRow::from(data_idx)))
        .collect();

    DisplayToDataMapping { mapping }
}

fn sort_indices(
    contents: &TableLikeContent,
    mut indices: Vec<usize>,
    sorting: AppliedSorting,
) -> Vec<usize> {
    indices.sort_by(|&a, &b| {
        let row_a = &contents.rows[a];
        let row_b = &contents.rows[b];

        let val_a = row_a
            .get(sorting.col_idx)
            .and_then(|tc| tc.display_value())
            .map(|tc| tc.as_str())
            .unwrap_or("");
        let val_b = row_b
            .get(sorting.col_idx)
            .and_then(|tc| tc.display_value())
            .map(|tc| tc.as_str())
            .unwrap_or("");

        // Try numeric comparison first, fall back to string comparison
        let cmp = match (val_a.parse::<f64>(), val_b.parse::<f64>()) {
            (Ok(num_a), Ok(num_b)) => num_a
                .partial_cmp(&num_b)
                .unwrap_or(std::cmp::Ordering::Equal),
            _ => val_a.cmp(val_b),
        };

        match sorting.direction {
            SortDirection::Asc => cmp,
            SortDirection::Desc => cmp.reverse(),
        }
    });

    indices
}
