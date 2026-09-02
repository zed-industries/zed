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
    Available {
        is_applied: bool,
    },
    /// Selecting this value would leave zero rows given every other active column's
    /// filter. Still carries `is_applied`: a value can be both currently checked
    /// *and* blocked (another column's filter was applied afterward), and the UI
    /// needs to keep showing it checked so the user can uncheck it.
    Unavailable {
        blocked_by: AnyColumn,
        is_applied: bool,
    },
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
    /// Which cell values are currently allowed for each filtered column
    retention_config: HashMap<AnyColumn, HashSet<Option<SharedString>>>,
}

impl TableDataEngine {
    pub(crate) fn has_active_filters(&self, col: AnyColumn) -> bool {
        self.filter_stack.retention_config.contains_key(&col)
    }

    /// Marks an entry unavailable if choosing it would leave zero rows, so users
    /// aren't offered filter values that lead to an empty table. Every other
    /// active column's filter counts toward this, regardless of activation
    /// order: `column`'s own filter is excluded (so its own current selection
    /// doesn't block its other values), but a column filtered after `column`
    /// blocks exactly as much as one filtered before it.
    pub(crate) fn get_filters_for_column(
        &self,
        column: AnyColumn,
    ) -> anyhow::Result<Arc<Vec<(FilterEntry, FilterEntryState)>>> {
        let all_column_entries = self
            .all_filters
            .get(&column)
            .ok_or_else(|| anyhow::anyhow!("Expected {column:?} to have filter entries"))?;

        let empty = HashSet::new();
        let active_column_filters = self
            .filter_stack
            .retention_config
            .get(&column)
            .unwrap_or(&empty);

        // Rows that survive every *other* active column's filter. `column`'s own
        // filter is excluded so its entries reflect what selecting them would
        // add rather than being constrained by the current selection.
        let rows_passing_other_filters =
            retain_rows(&self.contents.rows, &self.filter_stack, Some(column));

        // Only used to populate `Unavailable::blocked_by`, which the UI doesn't
        // display by name; any other currently active column is a valid choice.
        let blocking_column = self
            .filter_stack
            .retention_config
            .keys()
            .find(|&&col| col != column)
            .copied();

        all_column_entries
            .iter()
            .map(|entry| {
                let adjusted_rows: Vec<DataRow> = entry
                    .rows
                    .iter()
                    .filter(|row| rows_passing_other_filters.contains(row))
                    .copied()
                    .collect();

                let is_applied = active_column_filters.contains(&entry.content);
                let state = if adjusted_rows.is_empty() {
                    let blocked_by = blocking_column.ok_or_else(|| {
                        anyhow::anyhow!(
                            "Expected an active column other than {column:?} to block \
                             {:?} when it has no rows passing other filters",
                            entry.content
                        )
                    })?;
                    FilterEntryState::Unavailable {
                        blocked_by,
                        is_applied,
                    }
                } else {
                    FilterEntryState::Available { is_applied }
                };

                Ok((
                    FilterEntry {
                        content: entry.content.clone(),
                        rows: adjusted_rows,
                    },
                    state,
                ))
            })
            .collect::<anyhow::Result<Vec<_>>>()
            .map(Arc::new)
    }

    pub(crate) fn clear_filters_for_col(&mut self, col: AnyColumn) {
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
        } else {
            entries.remove(&value);
        }
        Ok(())
    }

    fn apply_filter(&mut self, column: AnyColumn, value: Option<SharedString>) {
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

/// Returns the set of data rows that survive all active filters in the stack,
/// optionally ignoring the filter on `exclude` without cloning the stack.
pub fn retain_rows(
    content_rows: &[TableRow<TableCell>],
    filter_stack: &FilterStack,
    exclude: Option<AnyColumn>,
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
                if Some(*col) == exclude {
                    return true;
                }
                let cell_value = row.get(*col).and_then(|cell| cell.display_value().cloned());
                allowed_values.contains(&cell_value)
            })
        })
        .map(|(index, _)| DataRow(index))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::TableLikeContent;
    use proptest::prelude::*;

    fn small_value() -> impl Strategy<Value = &'static str> {
        prop::sample::select(&["a", "b", "c"])
    }

    /// Generates a small rectangular table (header + data rows) built
    /// from a tiny value alphabet, so duplicate values across rows/columns
    /// are frequent enough to exercise cross-column blocking. Returns the
    /// column count alongside the table text since callers need it to bound
    /// generated column indices.
    fn table_text_strategy() -> impl Strategy<Value = (usize, String)> {
        (2usize..=6, 2usize..=3).prop_flat_map(|(rows, cols)| {
            prop::collection::vec(prop::collection::vec(small_value(), cols), rows).prop_map(
                move |grid| {
                    let header = (0..cols)
                        .map(|c| format!("c{c}"))
                        .collect::<Vec<_>>()
                        .join(",");
                    let mut lines = vec![header];
                    lines.extend(grid.into_iter().map(|row| row.join(",")));
                    (cols, lines.join("\n"))
                },
            )
        })
    }

    fn build_engine(table_text: &str) -> TableDataEngine {
        let mut engine = TableDataEngine::default();
        engine.contents = Arc::new(TableLikeContent::from_str(table_text.to_string()));
        engine.calculate_available_filters();
        engine
    }

    /// Resolves raw `(column_index, value_index)` pairs (generated modulo
    /// nothing in particular, so out of range in general) into concrete,
    /// deduplicated `(AnyColumn, value)` toggles by reading each column's
    /// entries from an untouched engine and wrapping indices to valid ranges.
    fn resolve_toggles(
        engine: &TableDataEngine,
        cols: usize,
        raw: &[(usize, usize)],
    ) -> Vec<(AnyColumn, Option<SharedString>)> {
        let mut toggles = Vec::new();
        for &(col_idx, value_idx) in raw {
            let column = AnyColumn::new(col_idx % cols);
            let entries = engine.get_filters_for_column(column).unwrap();
            if entries.is_empty() {
                continue;
            }
            let value = entries[value_idx % entries.len()].0.content.clone();
            let pair = (column, value);
            if !toggles.contains(&pair) {
                toggles.push(pair);
            }
        }
        toggles
    }

    /// Per-column `(value, is_available, is_applied)` triples, flattening
    /// `FilterEntryState` for comparison while dropping `blocked_by`: any
    /// other currently active column is an equally valid blocker, so its
    /// *identity* legitimately differs across activation orders, but
    /// availability and applied-ness must not.
    type StateSnapshot = Vec<(AnyColumn, Vec<(Option<SharedString>, bool, bool)>)>;

    fn apply_and_snapshot(
        engine: &mut TableDataEngine,
        cols: usize,
        toggles: &[(AnyColumn, Option<SharedString>)],
    ) -> StateSnapshot {
        for (column, value) in toggles {
            engine.toggle_filter(*column, value.clone()).unwrap();
        }

        (0..cols)
            .map(|col_idx| {
                let column = AnyColumn::new(col_idx);
                let entries = engine.get_filters_for_column(column).unwrap();
                let mut column_snapshot: Vec<(Option<SharedString>, bool, bool)> = entries
                    .iter()
                    .map(|(entry, state)| match *state {
                        FilterEntryState::Available { is_applied } => {
                            (entry.content.clone(), true, is_applied)
                        }
                        FilterEntryState::Unavailable { is_applied, .. } => {
                            (entry.content.clone(), false, is_applied)
                        }
                    })
                    .collect();
                // `all_column_entries` comes from a `HashMap`, so its iteration
                // order isn't stable across engine instances built from the
                // same data; sort so that's not mistaken for a real divergence.
                column_snapshot.sort();
                (column, column_snapshot)
            })
            .collect()
    }

    proptest! {
        #[test]
        fn filter_states_are_order_independent(
            (cols, table_text) in table_text_strategy(),
            raw_toggles in prop::collection::vec((0usize..3, 0usize..3), 0..8),
        ) {
            let toggles = resolve_toggles(&build_engine(&table_text), cols, &raw_toggles);

            let mut forward_engine = build_engine(&table_text);
            let forward = apply_and_snapshot(&mut forward_engine, cols, &toggles);

            let mut reversed = toggles.clone();
            reversed.reverse();
            let mut backward_engine = build_engine(&table_text);
            let backward = apply_and_snapshot(&mut backward_engine, cols, &reversed);

            prop_assert_eq!(forward, backward);
        }
    }
}
