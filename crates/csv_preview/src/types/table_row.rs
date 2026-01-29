//! A newtype for a table row that enforces a fixed column count at runtime.
//!
//! This type ensures that all rows in a table have the same width, preventing accidental creation or mutation of rows with inconsistent lengths.
//! It is especially useful for CSV or tabular data where rectangular invariants must be maintained, but the number of columns is only known at runtime.
//! By using `TableRow`, we gain stronger guarantees and safer APIs compared to a bare `Vec<T>`, without requiring const generics.

use std::{
    any::type_name,
    ops::{
        Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
    },
};

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

    /// Like [`map`], but borrows the row and clones each element before mapping.
    ///
    /// This is useful when you want to map over a borrowed row without consuming it,
    /// but your mapping function requires ownership of each element.
    ///
    /// # Difference
    /// - `map_cloned` takes `&self`, clones each element, and applies `f(T) -> U`.
    /// - [`map`] takes `self` by value and applies `f(T) -> U` directly, consuming the row.
    /// - [`map_ref`] takes `&self` and applies `f(&T) -> U` to references of each element.
    pub fn map_cloned<F, U>(&self, f: F) -> TableRow<U>
    where
        F: FnMut(T) -> U,
        T: Clone,
    {
        self.clone().map(f)
    }

    /// Consumes the row and transforms all elements within it in a length-safe way.
    ///
    /// # Difference
    /// - `map` takes ownership of the row (`self`) and applies `f(T) -> U` to each element.
    /// - Use this when you want to transform and consume the row in one step.
    /// - See also [`map_cloned`] (for mapping over a borrowed row with cloning) and [`map_ref`] (for mapping over references).
    pub fn map<F, U>(self, f: F) -> TableRow<U>
    where
        F: FnMut(T) -> U,
    {
        TableRow(self.0.into_iter().map(f).collect())
    }

    /// Borrows the row and transforms all elements by reference in a length-safe way.
    ///
    /// # Difference
    /// - `map_ref` takes `&self` and applies `f(&T) -> U` to each element by reference.
    /// - Use this when you want to map over a borrowed row without cloning or consuming it.
    /// - See also [`map`] (for consuming the row) and [`map_cloned`] (for mapping with cloning).
    pub fn map_ref<F, U>(&self, f: F) -> TableRow<U>
    where
        F: FnMut(&T) -> U,
    {
        TableRow(self.0.iter().map(f).collect())
    }

    pub(crate) fn empty() -> TableRow<T> {
        TableRow(Vec::new())
    }

    pub(crate) fn cols(&self) -> usize {
        self.0.len()
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

// pub trait IntoTableRowForArray<T> {
//     fn into_table_row(self) -> TableRow<T>;
// }

// impl<T, const COLS: usize> IntoTableRowForArray<T> for [T; COLS] {
//     fn into_table_row(self) -> TableRow<T> {
//         TableRow::from_vec(self.into(), COLS)
//     }
// }

// Index implementations for convenient access
impl<T> Index<usize> for TableRow<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for TableRow<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T> Index<AnyColumn> for TableRow<T> {
    type Output = T;

    fn index(&self, index: AnyColumn) -> &Self::Output {
        &self.0[*index]
    }
}

impl<T> IndexMut<AnyColumn> for TableRow<T> {
    fn index_mut(&mut self, index: AnyColumn) -> &mut Self::Output {
        &mut self.0[*index]
    }
}

// Range indexing implementations for slice operations
impl<T> Index<Range<usize>> for TableRow<T> {
    type Output = [T];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        <Vec<T> as Index<Range<usize>>>::index(&self.0, index)
    }
}

impl<T> Index<RangeFrom<usize>> for TableRow<T> {
    type Output = [T];

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        <Vec<T> as Index<RangeFrom<usize>>>::index(&self.0, index)
    }
}

impl<T> Index<RangeTo<usize>> for TableRow<T> {
    type Output = [T];

    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        <Vec<T> as Index<RangeTo<usize>>>::index(&self.0, index)
    }
}

impl<T> Index<RangeToInclusive<usize>> for TableRow<T> {
    type Output = [T];

    fn index(&self, index: RangeToInclusive<usize>) -> &Self::Output {
        <Vec<T> as Index<RangeToInclusive<usize>>>::index(&self.0, index)
    }
}

impl<T> Index<RangeFull> for TableRow<T> {
    type Output = [T];

    fn index(&self, index: RangeFull) -> &Self::Output {
        <Vec<T> as Index<RangeFull>>::index(&self.0, index)
    }
}

impl<T> Index<RangeInclusive<usize>> for TableRow<T> {
    type Output = [T];

    fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
        <Vec<T> as Index<RangeInclusive<usize>>>::index(&self.0, index)
    }
}

impl<T> IndexMut<RangeInclusive<usize>> for TableRow<T> {
    fn index_mut(&mut self, index: RangeInclusive<usize>) -> &mut Self::Output {
        <Vec<T> as IndexMut<RangeInclusive<usize>>>::index_mut(&mut self.0, index)
    }
}
