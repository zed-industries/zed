//! Type definitions for CSV table coordinates and cell identifiers.
//!
//! Provides newtypes for self-documenting coordinate systems:
//! - Display coordinates: Visual positions in rendered table
//! - Data coordinates: Original CSV data positions

use std::ops::Deref;

/// Visual row position in rendered table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DisplayRow(pub usize);

impl DisplayRow {
    /// Create a new display row
    pub fn new(row: usize) -> Self {
        Self(row)
    }

    /// Get the inner row value
    pub fn get(self) -> usize {
        self.0
    }
}

/// Original CSV row position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DataRow(pub usize);

impl DataRow {
    /// Create a new data row
    pub fn new(row: usize) -> Self {
        Self(row)
    }

    /// Get the inner row value
    pub fn get(self) -> usize {
        self.0
    }
}

impl Deref for DataRow {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<usize> for DisplayRow {
    fn from(row: usize) -> Self {
        DisplayRow::new(row)
    }
}

impl From<usize> for DataRow {
    fn from(row: usize) -> Self {
        DataRow::new(row)
    }
}

/// Column position in CSV table.
///
/// Currently represents both display and data coordinate systems since
/// column reordering is not yet implemented. When column reordering is added,
/// this will need to be split into `DisplayColumn` and `DataColumn` types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AnyColumn(pub usize);

impl AnyColumn {
    /// Create a new column ID
    pub fn new(col: usize) -> Self {
        Self(col)
    }

    /// Get the inner column value
    pub fn get(self) -> usize {
        self.0
    }
}

impl Deref for AnyColumn {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<usize> for AnyColumn {
    fn from(col: usize) -> Self {
        AnyColumn::new(col)
    }
}

/// Visual cell position in rendered table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DisplayCellId {
    pub row: DisplayRow,
    pub col: AnyColumn,
}

impl DisplayCellId {
    /// Create a new display cell ID
    pub fn new(row: impl Into<DisplayRow>, col: impl Into<AnyColumn>) -> Self {
        Self {
            row: row.into(),
            col: col.into(),
        }
    }

    /// Returns (row, column)
    pub fn to_raw(&self) -> (usize, usize) {
        (self.row.0, self.col.0)
    }
}

/// Original CSV cell position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DataCellId {
    pub row: DataRow,
    pub col: AnyColumn,
}

impl DataCellId {
    /// Create a new data cell ID
    pub fn new(row: impl Into<DataRow>, col: impl Into<AnyColumn>) -> Self {
        Self {
            row: row.into(),
            col: col.into(),
        }
    }

    pub fn to_raw(&self) -> (usize, usize) {
        (self.row.0, self.col.0)
    }
}
