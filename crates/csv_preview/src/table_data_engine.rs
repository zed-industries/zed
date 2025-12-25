//! This module defines core operations and config of tabular data view (CSV table)
//! It operates in 2 coordinate systems:
//! - `DataCellId` - indices of src data cells
//! - `DisplayCellId` - indices of data after applied transformations like sorting/filtering, which is used to render cell on the screen

pub mod sorting_by_column;
pub(crate) struct TableDataEngine {
    // TODO: Move sorting here
}
