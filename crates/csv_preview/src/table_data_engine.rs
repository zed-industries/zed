//! This module defines core operations and config of tabular data view (CSV table)
//! It operates in 2 coordinate systems:
//! - `DataCellId` - indices of src data cells
//! - `DisplayCellId` - indices of data after applied transformations like sorting/filtering, which is used to render cell on the screen
//!
//! It's designed to contain core logic of operations without relying on `CsvPreviewView`, context or window handles.

use std::{collections::HashMap, sync::Arc};

use ui::table_row::TableRow;

use crate::{
    table_data_engine::sorting_by_column::{AppliedSorting, sort_data_rows},
    types::{DataRow, DisplayRow, TableCell, TableLikeContent},
};

pub mod sorting_by_column;

#[derive(Default)]
pub(crate) struct TableDataEngine {
    pub applied_sorting: Option<AppliedSorting>,
    d2d_mapping: DisplayToDataMapping,
    pub contents: TableLikeContent,
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

    /// Applies sorting and filtering to the data and produces display to data mapping
    pub(crate) fn calculate_d2d_mapping(&mut self) {
        self.d2d_mapping
            .apply_sorting(self.applied_sorting, &self.contents.rows);
        // self.calculate_filters_with_availability();
        self.d2d_mapping.merge_mappings();
    }
}

/// Relation of Display (rendered) rows to Data (src) rows with applied transformations
/// Transformations applied:
/// - sorting by column
#[derive(Debug, Default)]
pub struct DisplayToDataMapping {
    /// All rows sorted, regardless of applied filtering. Applied every time sorting changes
    pub sorted_rows: Vec<DataRow>,
    /// Filtered and sorted rows. Computed cheaply from `sorted_mapping` and `filtered_out_rows`
    pub mapping: Arc<HashMap<DisplayRow, DataRow>>,
}

impl DisplayToDataMapping {
    /// Get the data row for a given display row
    pub fn get_data_row(&self, display_row: DisplayRow) -> Option<DataRow> {
        self.mapping.get(&display_row).copied()
    }

    /// Get the number of filtered rows
    pub fn visible_row_count(&self) -> usize {
        log::debug!("Visible row count: {}", self.mapping.len());
        self.mapping.len()
    }

    /// Computes sorting
    fn apply_sorting(&mut self, sorting: Option<AppliedSorting>, rows: &[TableRow<TableCell>]) {
        let data_rows: Vec<DataRow> = (0..rows.len()).map(DataRow).collect();

        let sorted_rows = if let Some(sorting) = sorting {
            log::debug!("Sorting data rows by {sorting:?}");
            sort_data_rows(&rows, data_rows, sorting)
        } else {
            log::debug!("Disabling sorting");
            data_rows
        };

        self.sorted_rows = sorted_rows;
        log::trace!("Sorted mapping: {:?}", self.sorted_rows);
    }

    /// Take pre-computed sorting and filtering results, and apply them to the mapping
    fn merge_mappings(&mut self) {
        self.mapping = Arc::new(
            self.sorted_rows
                .iter()
                .enumerate()
                .map(|(display, data)| (DisplayRow(display), *data))
                .collect(),
        );
    }
}
