use crate::table_data::TableData;

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

/// Generate ordered row indices based on current ordering settings.
/// Note: ordering.col_idx refers to CSV data columns (0-based), not display columns
/// (display columns include the line number column at index 0)
pub fn generate_ordered_indecies(ordering: Option<Ordering>, contents: &TableData) -> Vec<usize> {
    let indices: Vec<usize> = (0..contents.rows.len()).collect();

    let Some(ordering) = ordering else {
        return indices;
    };

    order_indices(contents, indices, ordering)
}

fn order_indices(contents: &TableData, mut indices: Vec<usize>, ordering: Ordering) -> Vec<usize> {
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
