//! This module defines core operations and config of tabular data view (CSV table)
//! It operates in 2 coordinate systems:
//! - `DataCellId` - indices of src data cells
//! - `DisplayCellId` - indices of data after applied transformations like sorting/filtering, which is used to render cell on the screen
//!
//! It's designed to contain core logic of operations without relying on `CsvPreviewView`, context or window handles.

use std::{collections::HashMap, sync::Arc};

use crate::{
    table_data_engine::{
        filtering_by_column::{
            AppliedFiltering, AvailableFilters, FilterEntry, calculate_available_filters,
            filter_data_rows,
        },
        selection::{NavigationDirection, NavigationOperation, TableSelection},
        sorting_by_column::{AppliedSorting, sort_data_rows},
    },
    types::{AnyColumn, DataRow, DisplayRow, TableLikeContent},
};

pub mod copy_selected;
pub mod filtering_by_column;
pub mod selection;
pub mod sorting_by_column;

pub(crate) struct TableDataEngine {
    pub applied_filtering: AppliedFiltering,
    pub available_filters: AvailableFilters,
    pub applied_sorting: Option<AppliedSorting>,
    pub d2d_mapping: Arc<DisplayToDataMapping>,
    pub contents: TableLikeContent,
    pub selection: TableSelection,
}

impl TableDataEngine {
    pub(crate) fn get_d2d_mapping(&self) -> &DisplayToDataMapping {
        self.d2d_mapping.as_ref()
    }

    /// Cheaper than `calculate_d2d_mapping`, as it reorders in place existing data
    pub(crate) fn re_apply_sort(&mut self) {
        let mut existing = self
            .d2d_mapping
            .mapping
            .values()
            .cloned()
            .collect::<Vec<_>>();

        let sorted_rows = if let Some(sorting) = self.applied_sorting {
            sort_data_rows(&self.contents.rows, existing, sorting)
        } else {
            // Cancel special sorting by resetting the order
            existing.sort();
            existing
        };
        self.produce_and_store_mapping(sorted_rows);
    }

    /// Takes applied filters/sorting, source content and produces display to data mapping
    pub(crate) fn calculate_d2d_mapping(&mut self) {
        let data_rows: Vec<DataRow> = (0..self.contents.rows.len()).map(DataRow).collect();

        let filtered_rows =
            filter_data_rows(&self.contents.rows, data_rows, &self.applied_filtering);

        let sorted_rows = if let Some(sorting) = self.applied_sorting {
            sort_data_rows(&self.contents.rows, filtered_rows, sorting)
        } else {
            filtered_rows
        };

        // Create mapping from display position to data row
        self.produce_and_store_mapping(sorted_rows);
    }

    /// Takes sorted and filtered rows and produces display to data mapping
    fn produce_and_store_mapping(&mut self, sorted_rows: Vec<DataRow>) {
        let mapping: HashMap<DisplayRow, DataRow> = sorted_rows
            .iter()
            .enumerate()
            .map(|(display_idx, &data_idx)| (DisplayRow::from(display_idx), data_idx))
            .collect();

        let data = { DisplayToDataMapping { mapping } };
        self.d2d_mapping = Arc::new(data);
    }

    pub fn calculate_available_filters(&mut self) {
        self.available_filters =
            calculate_available_filters(&self.contents.rows, self.contents.number_of_cols);
    }

    pub(crate) fn change_selection(
        &mut self,
        direction: NavigationDirection,
        operation: NavigationOperation,
    ) {
        let max_rows = self.contents.rows.len();
        let max_cols = self.contents.number_of_cols;

        self.selection
            .navigate(direction, operation, &self.d2d_mapping, max_rows, max_cols);
    }

    /// Toggle a filter for a specific column and value
    /// If the filter is currently applied, it will be removed
    /// If the filter is not applied, it will be added
    pub(crate) fn toggle_filter(&mut self, column: AnyColumn, hash: u64) -> bool {
        let is_currently_applied = self.applied_filtering.is_filter_applied(column, hash);
        log::debug!("Applied filters: {:?}", self.applied_filtering);

        if is_currently_applied {
            log::debug!("Removing filter for column {column:?} with hash {hash}");
            self.applied_filtering.remove_filter(column, hash);
            false // Filter was removed
        } else {
            log::debug!("Applying filter for column {column:?} with hash {hash}");
            self.applied_filtering
                .0
                .entry(column)
                .or_default()
                .insert(hash);
            true // Filter was added
        }
    }

    /// Get available filters for a specific column
    pub(crate) fn get_available_filters_for_column(
        &self,
        column: AnyColumn,
    ) -> Arc<Vec<FilterEntry>> {
        self.available_filters
            .get(&column)
            .cloned()
            .unwrap_or_else(|| panic!("Expected filters to be present for column: {column:?}"))
    }
}

/// Relation of Display (rendered) rows to Data (src) rows with applied transformations
/// Transformations applied:
/// - sorting by column
/// - todo: filtering
#[derive(Debug, Default)]
pub struct DisplayToDataMapping {
    pub mapping: HashMap<DisplayRow, DataRow>,
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

    /// Get the number of filtered rows
    pub fn filtered_row_count(&self) -> usize {
        self.mapping.len()
    }
}
