use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use ui::{SharedString, table_row::TableRow};

use crate::{
    table_data_engine::TableDataEngine,
    types::{AnyColumn, DataRow, TableCell},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FilterEntryState {
    Available { is_applied: bool },
    Unavailable { blocked_by: AnyColumn },
}

#[derive(Debug, Clone)]
pub struct FilterEntry {
    /// Content to display. None if cell is virtual
    pub content: Option<SharedString>,
    /// List of rows in which this value occurs
    pub rows: Vec<DataRow>,
}

impl FilterEntry {
    pub(crate) fn occurred_times(&self) -> usize {
        self.rows.len()
    }
}

#[derive(Debug, Default, Clone)]
pub(crate) struct FilterStack {
    /// Columns in the order their first filter was applied, used to compute cascade availability
    activation_order: Vec<AnyColumn>,
    /// Which cell values are currently allowed for each filtered column
    retention_config: HashMap<AnyColumn, HashSet<Option<SharedString>>>,
}

impl TableDataEngine {
    pub(crate) fn has_active_filters(&self, col: AnyColumn) -> bool {
        self.filter_stack.retention_config.contains_key(&col)
    }

    /// Get available filters for a specific column with cascade behavior.
    ///
    /// A filter entry is "unavailable" when all of its rows are hidden by a
    /// filter on an earlier-activated column, meaning selecting it would show
    /// zero rows. The cascade walk stops at `column` so that the column's own
    /// current filter does not affect its own entry availability.
    pub(crate) fn get_filters_for_column(
        &self,
        column: AnyColumn,
    ) -> anyhow::Result<Arc<Vec<(FilterEntry, FilterEntryState)>>> {
        let all_column_entries = self
            .all_filters
            .get(&column)
            .ok_or_else(|| anyhow::anyhow!("Expected {column:?} to have filter entries"))?;

        let mut unavailable_entries: HashMap<Option<SharedString>, AnyColumn> = HashMap::new();

        for &column_applied_previously in &self.filter_stack.activation_order {
            if column_applied_previously == column {
                break;
            }

            let retained_values = self
                .filter_stack
                .retention_config
                .get(&column_applied_previously)
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "Expected {column_applied_previously:?} to have retained entries \
                         as it is present in the filter stack"
                    )
                })?;

            // Rows that survive the filter on `column_applied_previously`
            let retained_rows: HashSet<DataRow> = self
                .contents
                .rows
                .iter()
                .enumerate()
                .filter(|(_, row)| {
                    let cell_value = row
                        .get(column_applied_previously)
                        .and_then(|cell| cell.display_value().cloned());
                    retained_values.contains(&cell_value)
                })
                .map(|(index, _)| DataRow(index))
                .collect();

            // An entry is unavailable when none of its rows survive the parent filter
            for entry in all_column_entries {
                if !entry.rows.iter().any(|row| retained_rows.contains(row)) {
                    unavailable_entries.insert(entry.content.clone(), column_applied_previously);
                }
            }
        }

        let empty = HashSet::new();
        let active_column_filters = self
            .filter_stack
            .retention_config
            .get(&column)
            .unwrap_or(&empty);

        Ok(Arc::new(
            all_column_entries
                .iter()
                .map(|entry| {
                    let state = if let Some(&blocked_by) = unavailable_entries.get(&entry.content) {
                        FilterEntryState::Unavailable { blocked_by }
                    } else {
                        FilterEntryState::Available {
                            is_applied: active_column_filters.contains(&entry.content),
                        }
                    };
                    (entry.clone(), state)
                })
                .collect(),
        ))
    }

    pub(crate) fn clear_filters_for_col(&mut self, col: AnyColumn) {
        self.filter_stack
            .activation_order
            .retain(|&entry| entry != col);
        self.filter_stack.retention_config.remove(&col);
    }

    /// Toggle a filter value for a column. Returns `true` if the filter was
    /// added, `false` if it was removed.
    pub(crate) fn toggle_filter(
        &mut self,
        column: AnyColumn,
        value: Option<SharedString>,
    ) -> anyhow::Result<bool> {
        let is_currently_applied = self
            .filter_stack
            .retention_config
            .get(&column)
            .is_some_and(|filters| filters.contains(&value));

        if is_currently_applied {
            self.remove_filter(column, value)?;
            Ok(false)
        } else {
            self.apply_filter(column, value);
            Ok(true)
        }
    }

    fn remove_filter(
        &mut self,
        column: AnyColumn,
        value: Option<SharedString>,
    ) -> anyhow::Result<()> {
        let entries = self
            .filter_stack
            .retention_config
            .get_mut(&column)
            .ok_or_else(|| {
                anyhow::anyhow!("Expected {column:?} to be present in active filters")
            })?;

        debug_assert!(
            entries.contains(&value),
            "Expected value to be present in {column:?} active filters"
        );

        if entries.len() == 1 {
            self.filter_stack.retention_config.remove(&column);
            self.filter_stack
                .activation_order
                .retain(|&entry| entry != column);
        } else {
            entries.remove(&value);
        }
        Ok(())
    }

    fn apply_filter(&mut self, column: AnyColumn, value: Option<SharedString>) {
        // Track the column only on its first activation to preserve cascade order
        if !self.filter_stack.activation_order.contains(&column) {
            self.filter_stack.activation_order.push(column);
        }
        self.filter_stack
            .retention_config
            .entry(column)
            .or_default()
            .insert(value);
    }
}

/// Calculate available filter entries for each column from the table data.
pub fn calculate_available_filters(
    content_rows: &[TableRow<TableCell>],
    number_of_cols: usize,
) -> HashMap<AnyColumn, Vec<FilterEntry>> {
    let mut available_filters = HashMap::new();

    for col_idx in 0..number_of_cols {
        let column = AnyColumn::new(col_idx);
        let mut value_to_rows: HashMap<Option<SharedString>, Vec<DataRow>> = HashMap::new();

        for (row_index, row) in content_rows.iter().enumerate() {
            let cell_value = row
                .get(column)
                .and_then(|cell| cell.display_value().cloned());
            value_to_rows
                .entry(cell_value)
                .or_default()
                .push(DataRow(row_index));
        }

        let filter_entries: Vec<FilterEntry> = value_to_rows
            .into_iter()
            .map(|(content, rows)| FilterEntry { content, rows })
            .collect();

        available_filters.insert(column, filter_entries);
    }

    available_filters
}

/// Returns the set of data rows that survive all active filters in the stack.
pub fn retain_rows(
    content_rows: &[TableRow<TableCell>],
    filter_stack: &FilterStack,
) -> HashSet<DataRow> {
    let config = &filter_stack.retention_config;
    if config.is_empty() {
        return (0..content_rows.len()).map(DataRow).collect();
    }

    content_rows
        .iter()
        .enumerate()
        .filter(|(_, row)| {
            config.iter().all(|(col, allowed_values)| {
                let cell_value = row.get(*col).and_then(|cell| cell.display_value().cloned());
                allowed_values.contains(&cell_value)
            })
        })
        .map(|(index, _)| DataRow(index))
        .collect()
}
