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

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use ui::SharedString;

use crate::types::{AnyColumn, DataRow, TableCell, TableRow};

#[derive(Debug, Clone)]
pub struct FilterEntry {
    /// Pre-computed hash
    pub hash: u64,
    /// Content to display
    pub content: SharedString,
    /// Number of times this string occur in given column
    pub occured_times: usize,
}

pub type AllowedCellHash = u64;
#[derive(Debug, Default)]
pub struct AppliedFiltering(pub HashMap<AnyColumn, HashSet<u64>>);

pub type AvailableFilters = HashMap<AnyColumn, Arc<Vec<FilterEntry>>>;

impl AppliedFiltering {
    /// Remove a specific filter entry from a column
    pub fn remove_filter(&mut self, column: AnyColumn, hash: AllowedCellHash) {
        if let Some(column_filters) = self.0.get_mut(&column) {
            column_filters.remove(&hash);
            if column_filters.is_empty() {
                self.0.remove(&column);
            }
        }
    }

    /// Check if a specific filter is applied
    pub fn is_filter_applied(&self, column: AnyColumn, hash: AllowedCellHash) -> bool {
        self.0
            .get(&column)
            .map_or(false, |filters| filters.contains(&hash))
    }

    /// Get all applied filters for a column
    pub fn get_column_filters(&self, column: AnyColumn) -> Option<&HashSet<AllowedCellHash>> {
        self.0.get(&column)
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
        filter_entries.sort_by(|a, b| {
            b.occured_times
                .cmp(&a.occured_times)
                .then_with(|| a.content.cmp(&b.content))
        });

        available_filters.insert(column, filter_entries.into());
    }

    available_filters
}

pub fn retain_rows(
    content_rows: &[TableRow<TableCell>],
    config: &AppliedFiltering,
) -> HashSet<DataRow> {
    let config = &config.0;
    let content_len = content_rows.len();
    if config.is_empty() {
        log::debug!("No filters applied. Returning all {content_len} data rows.",);
        return (0..content_len).map(DataRow).collect();
    }

    log::debug!("Filtering data rows with config: {:#?}", config);

    content_rows
        .iter()
        .enumerate()
        .filter(|(dr, row)| {
            log::trace!("Filtering row {dr:?}: {:#?}", row);
            // For each column that has filters applied, check if the cell value is allowed
            config.iter().all(|(col, allowed_values)| {
                let cell = row.expect_get(*col);
                let cell_hash = cell.hash();
                log::trace!(
                    "Column: {col:?}, Cell: {:?}, hash: {}",
                    cell.display_value(),
                    cell_hash
                );
                log::trace!("Allowed values: {:#?}", allowed_values);
                allowed_values.contains(&cell_hash)
            })
        })
        .map(|(dr, _)| DataRow(dr))
        .collect()
}
