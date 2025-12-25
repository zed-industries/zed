use crate::{types::AnyColumn, types::TableLikeContent};

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

pub fn sort_indices(
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
