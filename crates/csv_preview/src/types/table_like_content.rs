use crate::types::{DataCellId, DataRow, LineNumber, TableCell, TableRow};

/// Generic container struct of table-like data (CSV, TSV, etc)
#[derive(Clone)]
pub struct TableLikeContent {
    /// Number of data columns.
    /// Defines table width used to validate `TableRow` on creation
    pub number_of_cols: usize,
    pub headers: TableRow<TableCell>,
    pub rows: Vec<TableRow<TableCell>>,
    /// Follows the same indices as `rows`
    pub line_numbers: Vec<LineNumber>,
}

impl Default for TableLikeContent {
    fn default() -> Self {
        Self {
            number_of_cols: Default::default(),
            headers: TableRow::<TableCell>::empty(),
            rows: vec![],
            line_numbers: vec![],
        }
    }
}

impl TableLikeContent {
    pub fn get_cell(&self, id: &DataCellId) -> Option<&TableCell> {
        self.rows.get(*id.row)?.get(id.col)
    }

    pub(crate) fn get_row(&self, data_row: DataRow) -> Option<&TableRow<TableCell>> {
        self.rows.get(*data_row)
    }
}
