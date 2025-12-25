/*
Parts:
filtering button
    filtering popover:
    - indexed filtering counts
applied filters:
- column to allowed-words

d2d_mapping:
- uses applied filters to filter out rows by each column

## Investigation:
figure out how other IDEs:
- handle shifts of columns
- if filtered in value stays in the filter if the data was changed
 */

use std::collections::HashMap;

use ui::SharedString;

use crate::types::{AnyColumn, DataRow, TableCell, TableRow};

#[derive(Clone)]
pub struct FilterEntry {
    /// Pre-computed hash
    pub hash: u64,
    /// Content to display
    pub content: SharedString,
    /// Number of times this string occur in given column
    pub occured_times: usize,
}

pub type AllowedCellHash = u64;
#[derive(Default)]
pub struct AppliedFiltering(HashMap<AnyColumn, HashMap<AllowedCellHash, FilterEntry>>);

pub type AvailableFilters = HashMap<AnyColumn, Vec<FilterEntry>>;

impl AppliedFiltering {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_filter(&mut self, column: AnyColumn, content: SharedString) -> u64 {
        use std::hash::{DefaultHasher, Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        let hash = hasher.finish();

        let entry = FilterEntry {
            hash,
            content: content.clone(),
            occured_times: 1, // This would normally be calculated from the data
        };

        self.0
            .entry(column)
            .or_insert_with(HashMap::new)
            .insert(hash, entry);

        hash
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Remove a specific filter entry from a column
    pub fn remove_filter(&mut self, column: AnyColumn, hash: AllowedCellHash) {
        if let Some(column_filters) = self.0.get_mut(&column) {
            column_filters.remove(&hash);
            if column_filters.is_empty() {
                self.0.remove(&column);
            }
        }
    }

    /// Clear all filters for a specific column
    pub fn clear_column_filters(&mut self, column: AnyColumn) {
        self.0.remove(&column);
    }

    /// Clear all filters
    pub fn clear_all_filters(&mut self) {
        self.0.clear();
    }

    /// Check if a specific filter is applied
    pub fn is_filter_applied(&self, column: AnyColumn, hash: AllowedCellHash) -> bool {
        self.0
            .get(&column)
            .map_or(false, |filters| filters.contains_key(&hash))
    }

    /// Get all applied filters for a column
    pub fn get_column_filters(
        &self,
        column: AnyColumn,
    ) -> Option<&HashMap<AllowedCellHash, FilterEntry>> {
        self.0.get(&column)
    }

    /// Get all columns that have filters applied
    pub fn get_filtered_columns(&self) -> Vec<AnyColumn> {
        self.0.keys().copied().collect()
    }
}

/// Calculate available filter entries for each column from the table data
pub fn calculate_available_filters(
    content_rows: &[TableRow<TableCell>],
    number_of_cols: usize,
) -> AvailableFilters {
    let mut available_filters = HashMap::new();

    // For each column, collect all unique cell values and count occurrences
    for col_idx in 0..number_of_cols {
        let column = AnyColumn::new(col_idx);
        let mut cell_counts: HashMap<u64, (SharedString, usize)> = HashMap::new();

        // Count occurrences of each cell value
        for row in content_rows {
            if let Some(cell) = row.get(column) {
                if let Some(display_value) = cell.display_value() {
                    let hash = cell.hash();
                    match cell_counts.get_mut(&hash) {
                        Some((_, count)) => *count += 1,
                        None => {
                            cell_counts.insert(hash, (display_value.clone(), 1));
                        }
                    }
                }
            }
        }

        // Convert to FilterEntry vec, sorted by content
        let mut filter_entries: Vec<FilterEntry> = cell_counts
            .into_iter()
            .map(|(hash, (content, occured_times))| FilterEntry {
                hash,
                content,
                occured_times,
            })
            .collect();

        // Sort by content for consistent ordering
        filter_entries.sort_by(|a, b| a.content.cmp(&b.content));

        available_filters.insert(column, filter_entries);
    }

    available_filters
}

pub fn filter_data_rows(
    content_rows: &[TableRow<TableCell>],
    data_row_ids: Vec<DataRow>,
    config: &AppliedFiltering,
) -> Vec<DataRow> {
    let config = &config.0;

    if config.is_empty() {
        return data_row_ids;
    }

    data_row_ids
        .into_iter()
        .filter(|dr| {
            let row = &content_rows[dr.get()];
            // For each column that has filters applied, check if the cell value is allowed
            config.iter().all(|(col, allowed_values)| {
                let cell_hash = row.expect_get(*col).hash();
                allowed_values.contains_key(&cell_hash)
            })
        })
        .collect()
}
