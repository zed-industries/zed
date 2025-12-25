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
use ui::SharedString;

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

    /// Takes applied filters/sorting, source content and produces display to data mapping
    pub(crate) fn calculate_d2d_mapping(&mut self) {
        // Recalculate available filters from current content
        self.available_filters =
            calculate_available_filters(&self.contents.rows, self.contents.number_of_cols);

        let data_rows: Vec<DataRow> = (0..self.contents.rows.len()).map(DataRow).collect();

        let filtered_rows =
            filter_data_rows(&self.contents.rows, data_rows, &self.applied_filtering);

        let sorted_rows = if let Some(sorting) = self.applied_sorting {
            sort_data_rows(&self.contents.rows, filtered_rows, sorting)
        } else {
            filtered_rows
        };

        // Create mapping from display position to data row
        let mapping: HashMap<DisplayRow, DataRow> = sorted_rows
            .iter()
            .enumerate()
            .map(|(display_idx, &data_idx)| (DisplayRow::from(display_idx), data_idx))
            .collect();

        let data = { DisplayToDataMapping { mapping } };
        self.d2d_mapping = Arc::new(data);
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
    pub(crate) fn toggle_filter(&mut self, column: AnyColumn, content: SharedString) -> bool {
        use std::hash::{DefaultHasher, Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        let hash = hasher.finish();

        let is_currently_applied = self.applied_filtering.is_filter_applied(column, hash);

        if is_currently_applied {
            self.applied_filtering.remove_filter(column, hash);
            false // Filter was removed
        } else {
            self.applied_filtering.add_filter(column, content);
            true // Filter was added
        }
    }

    /// Clear all filters for a specific column
    pub(crate) fn clear_column_filters(&mut self, column: AnyColumn) {
        self.applied_filtering.clear_column_filters(column);
    }

    /// Clear all applied filters
    pub(crate) fn clear_all_filters(&mut self) {
        self.applied_filtering.clear_all_filters();
    }

    /// Check if any filters are applied
    pub(crate) fn has_filters(&self) -> bool {
        !self.applied_filtering.is_empty()
    }

    /// Get available filters for a specific column
    pub(crate) fn get_available_filters_for_column(
        &self,
        column: AnyColumn,
    ) -> Option<&Vec<FilterEntry>> {
        self.available_filters.get(&column)
    }

    /// Get all available filters
    pub(crate) fn get_all_available_filters(&self) -> &AvailableFilters {
        &self.available_filters
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
