//! Type definitions for CSV table coordinates and cell identifiers.
//!
//! Provides newtypes for self-documenting coordinate systems:
//! - Display coordinates: Visual positions in rendered table
//! - Data coordinates: Original CSV data positions

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

/// Visual cell position in rendered table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayCellId {
    pub row: DisplayRow,
    pub col: usize,
}

impl DisplayCellId {
    /// Create a new display cell ID
    pub fn new(row: DisplayRow, col: usize) -> Self {
        Self { row, col }
    }

    /// Create a new display cell ID from raw values
    pub fn from_raw(row: usize, col: usize) -> Self {
        Self {
            row: DisplayRow::new(row),
            col,
        }
    }
}

/// Original CSV cell position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DataCellId {
    pub row: DataRow,
    pub col: usize,
}

impl DataCellId {
    /// Create a new data cell ID
    pub fn new(row: DataRow, col: usize) -> Self {
        Self { row, col }
    }
}
