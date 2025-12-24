//! A newtype for a table row that enforces a fixed column count at runtime.
//!
//! This type ensures that all rows in a table have the same width, preventing accidental creation or mutation of rows with inconsistent lengths.
//! It is especially useful for CSV or tabular data where rectangular invariants must be maintained, but the number of columns is only known at runtime.
//! By using `TableRow`, we gain stronger guarantees and safer APIs compared to a bare `Vec<T>`, without requiring const generics.

use std::any::type_name;

use crate::types::AnyColumn;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableRow<T>(Vec<T>);

impl<T> TableRow<T> {
    /// Constructs a `TableRow` from a `Vec<T>`, panicking if the length does not match `expected_length`.
    ///
    /// Use this when you want to ensure at construction time that the row has the correct number of columns.
    /// This enforces the rectangular invariant for table data, preventing accidental creation of malformed rows.
    ///
    /// # Panics
    /// Panics if `data.len() != expected_length`.
    pub fn from_vec(data: Vec<T>, expected_length: usize) -> Self {
        Self::try_from_vec(data, expected_length).unwrap_or_else(|e| {
            let name = type_name::<Vec<T>>();
            panic!("Expected {name} to be created successfully: {e}");
        })
    }

    /// Attempts to construct a `TableRow` from a `Vec<T>`, returning an error if the length does not match `expected_len`.
    ///
    /// This is a fallible alternative to `from_vec`, allowing you to handle inconsistent row lengths gracefully.
    /// Returns `Ok(TableRow)` if the length matches, or an `Err` with a descriptive message otherwise.
    pub fn try_from_vec(data: Vec<T>, expected_len: usize) -> Result<Self, String> {
        if data.len() != expected_len {
            Err(format!(
                "Row length {} does not match expected {}",
                data.len(),
                expected_len
            ))
        } else {
            Ok(Self(data))
        }
    }

    /// Returns reference to element by column id.
    ///
    /// # Panics if `col` is greater than `TableRow` len
    pub fn expect_get(&self, col: AnyColumn) -> &T {
        self.0.get(*col).unwrap_or_else(|| {
            panic!(
                "Expected table row of `{}` to have {col:?}",
                type_name::<T>()
            )
        })
    }

    pub fn get(&self, col: AnyColumn) -> Option<&T> {
        self.0.get(*col)
    }

    pub fn as_slice(&self) -> &[T] {
        &self.0
    }

    pub fn into_vec(self) -> Vec<T> {
        self.0
    }

    /// Transforms all elements within the row in length-safe way. Similar to `array_map` from stdlib
    pub fn map<F, U>(self, f: F) -> TableRow<U>
    where
        F: FnMut(T) -> U,
    {
        TableRow(self.0.into_iter().map(f).collect())
    }

    pub(crate) fn empty() -> TableRow<T> {
        TableRow(Vec::new())
    }
}

///// Convenience traits /////
pub trait IntoTableRow<T> {
    fn into_table_row(self, expected_length: usize) -> TableRow<T>;
}

impl<T> IntoTableRow<T> for Vec<T> {
    fn into_table_row(self, expected_length: usize) -> TableRow<T> {
        TableRow::from_vec(self, expected_length)
    }
}
