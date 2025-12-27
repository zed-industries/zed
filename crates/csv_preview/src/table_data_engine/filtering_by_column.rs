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

use itertools::Itertools as _;
use ui::SharedString;

use crate::{
    table_data_engine::TableDataEngine,
    types::{AnyColumn, DataRow, OptionExt, TableCell, TableRow},
};

/// Hash of a cell used in filtering
pub struct ValueHash(u64);

#[derive(Debug, Clone, Copy)]
pub enum FilterEntryState {
    Available { is_applied: bool },
    Unavailable { blocked_by: AnyColumn },
}
#[derive(Debug, Clone)]
pub struct FilterEntry {
    /// Pre-computed hash
    pub hash: u64,
    /// Content to display
    pub content: SharedString,
    /// List of rows, in which this value occurs
    pub rows: Vec<DataRow>,
}
impl FilterEntry {
    /// Number of times this string occur in given column
    pub(crate) fn occured_times(&self) -> usize {
        self.rows.len()
    }
}

pub type AllowedCellHash = u64;

#[derive(Debug, Default)]
pub(crate) struct FilterStack {
    /// The order in which filters were applied
    activation_order: Vec<AnyColumn>,
    /// Which FilterEntry values are currently active for each column
    retention_config: HashMap<AnyColumn, HashSet<AllowedCellHash>>,
}

impl TableDataEngine {
    pub(crate) fn has_active_filters(&self, col: AnyColumn) -> bool {
        self.filter_stack.retention_config.contains_key(&col)
    }

    /// Get available filters for a specific column with proper cascade behavior.
    /// Using Arc, as `cx` can be borrowed as mut at the same time.
    pub(crate) fn get_filters_for_column(
        &self,
        column: AnyColumn,
    ) -> Arc<Vec<(FilterEntry, FilterEntryState)>> {
        let all_column_entries = self
            .all_filters
            .get(&column)
            .expect_lazy(|| format!("Expected {column:?} to have filters entries"));

        log::debug!(
            "Retrieving filters for column: {column:?}. Filter stack: {:#?}",
            self.filter_stack
        );

        // Map of unavailable entries for `column` computed by checking if they are still visible after parent filters are applied
        let mut unavailable_column_entries = HashMap::new();
        // Compute unavailable filter entries for given column
        let mut iterator = self.filter_stack.activation_order.iter().enumerate();
        while let Some((ix, column_applied_previously)) = iterator.next() {
            if column_applied_previously == &column {
                log::trace!(
                    "In filter stack at index {ix} reached {column:?} from input. Stop calculating unavailable filters"
                );
                break;
            }

            let retained_entries = self.filter_stack.retention_config
                .get(&column_applied_previously)
                .expect_lazy(||format!("Expected {column_applied_previously:?} to have retained entries as it's present in the filter stack"));

            // Get rows which are skipped by given column filtering config
            let skipped_rows = self
                .contents
                .rows
                .iter()
                .enumerate()
                .filter(|(_, row)| {
                    retained_entries.contains(&row.expect_get(*column_applied_previously).hash())
                })
                .map(|(dr, _)| DataRow(dr))
                .collect_vec();

            for entry in all_column_entries {
                // If given column filter entry present only in rows which are skipped - unavailable
                if !entry.rows.iter().all(|r| skipped_rows.contains(r)) {
                    log::trace!(
                        "[{ix}] {column:?} entry {:?} is unavailable as all it's rows are skipped by parent columns",
                        entry.hash
                    );
                    unavailable_column_entries.insert(entry.hash, *column_applied_previously);
                }
            }
        }

        log::debug!(
            "For {column:?} found {} entries, {} of which are unavailable",
            all_column_entries.len(),
            unavailable_column_entries.len()
        );

        let empty = HashSet::new();
        let active_column_filters = self
            .filter_stack
            .retention_config
            .get(&column)
            .unwrap_or(&empty);

        Arc::new(
            all_column_entries
                .into_iter()
                .map(|e| {
                    (
                        e.clone(),
                        if let Some(blocked_by) = unavailable_column_entries.get(&e.hash).cloned() {
                            FilterEntryState::Unavailable { blocked_by }
                        } else {
                            FilterEntryState::Available {
                                is_applied: active_column_filters.contains(&e.hash),
                            }
                        },
                    )
                })
                .collect(),
        )
    }

    pub(crate) fn clear_filters_for_col(&mut self, col: AnyColumn) {
        self.filter_stack
            .activation_order
            .retain(|&entry| entry != col);
        self.filter_stack.retention_config.remove(&col);
    }

    /// Toggle a filter for a specific column and value
    /// If the filter is currently applied, it will be removed
    /// If the filter is not applied, it will be added
    pub(crate) fn toggle_filter(&mut self, column: AnyColumn, hash: u64) -> bool {
        let is_currently_applied = self
            .filter_stack
            .retention_config
            .get(&column)
            .map_or(false, |filters| filters.contains(&hash));
        log::debug!("Applied filters: {:#?}", self.filter_stack);

        if is_currently_applied {
            log::debug!("Removing filter for column {column:?} with hash {hash}");
            self.remove_filter(column, hash);
            false // Filter was removed
        } else {
            self.apply_filter(column, hash);
            true // Filter was added
        }
    }

    fn remove_filter(&mut self, column: AnyColumn, hash: u64) {
        let entries = self
            .filter_stack
            .retention_config
            .get_mut(&column)
            .expect_lazy(|| format!("Expected {column:?} to be present in active filters"));

        assert!(
            entries.contains(&hash),
            "Expected {hash} to be present in {column:?} active filters"
        );

        if entries.len() == 1 {
            log::debug!(
                "Hash {hash} was last in active filters for {column:?}. Removing column from active filters"
            );
            self.filter_stack.retention_config.remove(&column);
            self.filter_stack
                .activation_order
                .retain(|&entry| entry != column);
        } else {
            log::debug!(
                "Removing {hash} from active filters for {column:?}. Remaining filters: {:#?}",
                entries
            );
            entries.remove(&hash);
        }
    }

    fn apply_filter(&mut self, column: AnyColumn, hash: u64) {
        log::debug!("Applying filter for column {column:?} with hash {hash}");

        if self.filter_stack.activation_order.is_empty() {
            log::debug!("Adding {column:?} & {hash} as first filter");
        } else {
            log::debug!(
                "Adding {column:?} & {hash} as subsequent filter. Present filters: {:?}",
                self.filter_stack.activation_order
            );
        }
        self.filter_stack.activation_order.push(column);
        self.filter_stack
            .retention_config
            .entry(column)
            .or_default()
            .insert(hash);
    }
}

/// Calculate available filter entries for each column from the table data
pub fn calculate_available_filters(
    content_rows: &[TableRow<TableCell>],
    number_of_cols: usize,
) -> HashMap<AnyColumn, Vec<FilterEntry>> {
    let mut available_filters = HashMap::new();

    // For each column, collect all unique cell values and count occurrences
    for col_idx in 0..number_of_cols {
        let column = AnyColumn::new(col_idx);
        let mut cell_counts: HashMap<u64, (SharedString, Vec<DataRow>)> = HashMap::new();

        // Count occurrences of each cell value
        for (row_id, row) in content_rows.into_iter().enumerate() {
            let row_id = DataRow(row_id);
            if let Some(cell) = row.get(column) {
                if let Some(display_value) = cell.display_value() {
                    let hash = cell.hash();
                    match cell_counts.get_mut(&hash) {
                        Some((_, rows)) => rows.push(row_id),
                        None => {
                            cell_counts.insert(hash, (display_value.clone(), vec![row_id]));
                        }
                    }
                }
            }
        }

        // Convert to FilterEntry vec, sorted by content
        let filter_entries: Vec<FilterEntry> = cell_counts
            .into_iter()
            .map(|(hash, (content, rows))| FilterEntry {
                hash,
                content,
                rows,
            })
            .collect();

        // No sorting needed, as it must be done both by occured_times and availability (which is dynamic)

        available_filters.insert(column, filter_entries);
    }

    available_filters
}

pub fn retain_rows(
    content_rows: &[TableRow<TableCell>],
    filter_stack: &FilterStack,
) -> HashSet<DataRow> {
    let config = &filter_stack.retention_config;
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
