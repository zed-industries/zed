//! This module defines core operations and config of tabular data view (CSV table)
//! It operates in 2 coordinate systems:
//! - `DataCellId` - indices of src data cells
//! - `DisplayCellId` - indices of data after applied transformations like sorting/filtering, which is used to render cell on the screen

use std::sync::Arc;

use crate::{
    table_data_engine::sorting_by_column::{
        AppliedSorting, DisplayToDataMapping, generate_sorted_indices,
    },
    table_like_content::TableLikeContent,
};

pub mod selection;
pub mod sorting_by_column;

pub(crate) struct TableDataEngine {
    pub applied_sorting: Option<AppliedSorting>,
    pub d2d_mapping: Arc<DisplayToDataMapping>,
    pub contents: TableLikeContent,
}

impl TableDataEngine {
    pub(crate) fn get_d2d_mapping(&self) -> &DisplayToDataMapping {
        self.d2d_mapping.as_ref()
    }

    // TODO: Rename to be generic processor and add docs
    pub(crate) fn re_run_sorting(&mut self) {
        self.d2d_mapping = Arc::new(generate_sorted_indices(
            self.applied_sorting,
            &self.contents,
        ));
    }
}
