use crate::types::{AnyColumn, DataRow, TableCell, TableRow};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SortDirection {
    Asc,
    Desc,
}

/// Config or currently active sorting
#[derive(Debug, Clone, Copy)]
pub struct AppliedSorting {
    /// 0-based column index
    pub col_idx: AnyColumn,
    /// Direction of sorting (asc/desc)
    pub direction: SortDirection,
}

pub fn sort_data_rows(
    content_rows: &[TableRow<TableCell>],
    mut data_row_ids: Vec<DataRow>,
    sorting: AppliedSorting,
) -> Vec<DataRow> {
    data_row_ids.sort_by(|&a, &b| {
        let row_a = &content_rows[*a];
        let row_b = &content_rows[*b];

        // TODO: Hanle nulls
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

    data_row_ids
}
