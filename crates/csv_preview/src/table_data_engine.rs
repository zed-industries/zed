//! This module defines core operations and config of tabular data view (CSV table)
//! It operates in 2 coordinate systems:
//! - `DataCellId` - indices of src data cells
//! - `DisplayCellId` - indices of data after applied transformations like sorting/filtering, which is used to render cell on the screen
//!
//! It's designed to contain core logic of operations without relying on `CsvPreviewView`, context or window handles.

use std::{collections::HashMap, sync::Arc};

use crate::{
    table_data_engine::{
        filtering_by_column::AppliedFiltering,
        selection::{NavigationDirection, NavigationOperation, TableSelection},
        sorting_by_column::{AppliedSorting, sort_indices},
    },
    types::{DataRow, DisplayRow, TableLikeContent},
};

pub mod copy_selected;
pub mod selection;
pub mod sorting_by_column;
pub mod filtering_by_column {
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

    use crate::types::AnyColumn;

    pub struct AllowedCell(SharedString);
    #[derive(Default)]
    pub struct AppliedFiltering(HashMap<AnyColumn, Vec<AllowedCell>>);
}

pub(crate) struct TableDataEngine {
    pub applied_filtering: AppliedFiltering,
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
        let sorting = self.applied_sorting;
        let contents: &TableLikeContent = &self.contents;
        let indices: Vec<usize> = (0..contents.rows.len()).collect();

        let sorted_indices = if let Some(sorting) = sorting {
            sort_indices(contents, indices, sorting)
        } else {
            indices
        };

        // Create mapping from display position to data row
        let mapping: HashMap<DisplayRow, DataRow> = sorted_indices
            .iter()
            .enumerate()
            .map(|(display_idx, &data_idx)| {
                (DisplayRow::from(display_idx), DataRow::from(data_idx))
            })
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
}

/// Relation of Display (rendered) rows to Data (src) rows with applied transformations
/// Transformations applied:
/// - sorting by column
/// - todo: filtering
#[derive(Debug, Default)]
pub struct DisplayToDataMapping {
    mapping: HashMap<DisplayRow, DataRow>,
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
}
