use crate::{
    table_like_content::TableLikeContent,
    types::{DataRow, DisplayRow},
};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum OrderingDirection {
    Asc,
    Desc,
}

#[derive(Clone, Copy)]
pub struct Ordering {
    /// 0-based column index
    pub col_idx: usize,
    /// Direction of ordering
    pub direction: OrderingDirection,
}

/// Ordered indices mapping display positions to data positions
#[derive(Debug, Clone)]
pub struct OrderedIndices {
    mapping: HashMap<DisplayRow, DataRow>,
}

impl OrderedIndices {
    /// Get the data row for a given display row
    pub fn get_data_row(&self, display_row: DisplayRow) -> Option<DataRow> {
        self.mapping.get(&display_row).copied()
    }
}

/// Generate ordered row indices based on current ordering settings.
/// Returns a mapping from DisplayRow to DataRow.
/// Note: ordering.col_idx refers to CSV data columns (0-based), not display columns
/// (display columns include the line number column at index 0)
pub fn generate_ordered_indices(
    ordering: Option<Ordering>,
    contents: &TableLikeContent,
) -> OrderedIndices {
    let indices: Vec<usize> = (0..contents.rows.len()).collect();

    let ordered_indices = if let Some(ordering) = ordering {
        order_indices(contents, indices, ordering)
    } else {
        indices
    };

    // Create mapping from display position to data row
    let mapping = ordered_indices
        .into_iter()
        .enumerate()
        .map(|(display_idx, data_idx)| (DisplayRow::new(display_idx), DataRow::new(data_idx)))
        .collect();

    OrderedIndices { mapping }
}

fn order_indices(
    contents: &TableLikeContent,
    mut indices: Vec<usize>,
    ordering: Ordering,
) -> Vec<usize> {
    indices.sort_by(|&a, &b| {
        let row_a = &contents.rows[a];
        let row_b = &contents.rows[b];

        let val_a = row_a
            .get(ordering.col_idx)
            .map(|s| s.as_ref())
            .unwrap_or("");
        let val_b = row_b
            .get(ordering.col_idx)
            .map(|s| s.as_ref())
            .unwrap_or("");

        // Try numeric comparison first, fall back to string comparison
        let cmp = match (val_a.parse::<f64>(), val_b.parse::<f64>()) {
            (Ok(num_a), Ok(num_b)) => num_a
                .partial_cmp(&num_b)
                .unwrap_or(std::cmp::Ordering::Equal),
            _ => val_a.cmp(val_b),
        };

        match ordering.direction {
            OrderingDirection::Asc => cmp,
            OrderingDirection::Desc => cmp.reverse(),
        }
    });

    indices
}
