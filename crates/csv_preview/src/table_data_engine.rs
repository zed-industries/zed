//! This module defines core operations and config of tabular data view (CSV table)
//! It operates in 2 coordinate systems:
//! - `DataCellId` - indices of src data cells
//! - `DisplayCellId` - indices of data after applied transformations like sorting/filtering, which is used to render cell on the screen
//!
//! It's designed to contain core logic of operations without relying on `CsvPreviewView`, context or window handles.

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use crate::{
    table_data_engine::{
        filtering_by_column::{FilterEntry, FilterStack, calculate_available_filters, retain_rows},
        selection::{NavigationDirection, NavigationOperation, TableSelection},
        sorting_by_column::{AppliedSorting, sort_data_rows},
    },
    types::{
        AnyColumn, DataCellId, DataRow, DisplayCellId, DisplayRow, TableCell, TableLikeContent,
        TableRow,
    },
};

pub mod copy_selected;
pub mod filtering_by_column;
pub mod selection;
pub mod sorting_by_column;

#[derive(Default)]
pub(crate) struct TableDataEngine {
    pub filter_stack: FilterStack,
    /// All filters in unfiltered state
    all_filters: HashMap<AnyColumn, Vec<FilterEntry>>,
    pub applied_sorting: Option<AppliedSorting>,
    d2d_mapping: DisplayToDataMapping,
    pub contents: TableLikeContent,
    pub selection: TableSelection,
}

impl TableDataEngine {
    pub(crate) fn d2d_mapping(&self) -> &DisplayToDataMapping {
        &self.d2d_mapping
    }

    pub(crate) fn apply_sort(&mut self) {
        self.d2d_mapping
            .apply_sorting(self.applied_sorting, &self.contents.rows);
        self.d2d_mapping.merge_mappings();
    }

    /// Applies filtering to the data and produces display to data mapping from existing sorting
    pub(crate) fn apply_filtering(&mut self) {
        self.d2d_mapping
            .apply_filtering(&self.filter_stack, &self.contents.rows);
        // self.calculate_filters_with_availability();
        self.d2d_mapping.merge_mappings();
    }

    /// Applies sorting and filtering to the data and produces display to data mapping
    pub(crate) fn calculate_d2d_mapping(&mut self) {
        self.d2d_mapping
            .apply_sorting(self.applied_sorting, &self.contents.rows);
        self.d2d_mapping
            .apply_filtering(&self.filter_stack, &self.contents.rows);
        // self.calculate_filters_with_availability();
        self.d2d_mapping.merge_mappings();
    }

    pub fn calculate_available_filters(&mut self) {
        self.all_filters =
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

    pub(crate) fn start_mouse_selection(
        &mut self,
        display_cell_id: DisplayCellId,
        preserve_existing: bool,
    ) {
        self.selection
            .start_mouse_selection(display_cell_id, &self.d2d_mapping, preserve_existing);
    }

    pub(crate) fn extend_mouse_selection(
        &mut self,
        display_cell_id: &DisplayCellId,
        preserve_existing: bool,
    ) {
        self.selection.extend_mouse_selection(
            display_cell_id.row,
            display_cell_id.col,
            &self.d2d_mapping,
            preserve_existing,
        );
    }
}

/// Relation of Display (rendered) rows to Data (src) rows with applied transformations
/// Transformations applied:
/// - sorting by column
/// - todo: filtering
#[derive(Debug, Default)]
pub struct DisplayToDataMapping {
    /// All rows sorted, regardless of applied filtering. Applied every time sorting changes
    pub sorted_mapping: HashMap<DisplayRow, DataRow>,
    /// All rows filtered out, regardless of applied sorting. Applied every time filtering changes
    pub retained_rows: HashSet<DataRow>,
    /// Filtered and sorted rows. Computed cheaply from `sorted_mapping` and `filtered_out_rows`
    pub mapping: Arc<HashMap<DisplayRow, DataRow>>,
}

impl DisplayToDataMapping {
    pub(crate) fn display_to_data_cell(&self, display_cid: &DisplayCellId) -> DataCellId {
        let data_row = self.get_data_row(display_cid.row).unwrap_or_else(|| {
            panic!("Expected {display_cid:?} to correspond to real DataCell, but it's not")
        });
        DataCellId::new(data_row, display_cid.col)
    }
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
    pub fn visible_row_count(&self) -> usize {
        self.mapping.len()
    }

    /// Computes sorting
    fn apply_sorting(&mut self, sorting: Option<AppliedSorting>, rows: &[TableRow<TableCell>]) {
        let data_rows: Vec<DataRow> = (0..rows.len()).map(DataRow).collect();

        let sorted_rows = if let Some(sorting) = sorting {
            sort_data_rows(&rows, data_rows, sorting)
        } else {
            data_rows
        };

        self.sorted_mapping = sorted_rows
            .into_iter()
            .enumerate()
            .map(|(index, row)| (DisplayRow(index), row))
            .collect();
    }

    /// Computes filtering and applies pre-computed sorting results to the mapping
    pub(super) fn apply_filtering(
        &mut self,
        filter_stack: &FilterStack,
        rows: &[TableRow<TableCell>],
    ) {
        self.retained_rows = retain_rows(rows, filter_stack);
    }

    /// Take pre-computed sorting and filtering results, and apply them to the mapping
    fn merge_mappings(&mut self) {
        let sorted_rows = self.sorted_mapping.len();
        let retained_rows = self.retained_rows.len();
        log::debug!(
            "Going to merge mappings with {sorted_rows} sorted rows and {retained_rows} retained rows"
        );

        let retained_ordered_rows: Vec<DataRow> = self
            .sorted_mapping
            .values()
            .filter(|row| self.retained_rows.contains(row))
            .cloned()
            .collect();

        self.mapping = Arc::new(
            retained_ordered_rows
                .into_iter()
                .enumerate()
                .map(|(index, row)| (DisplayRow(index), row))
                .collect(),
        );
    }
}
