use crate::database::Database;
use crate::error::Error;

use std::fmt::Debug;

pub trait Column: 'static + Send + Sync + Debug {
    type Database: Database<Column = Self>;

    /// Gets the column ordinal.
    ///
    /// This can be used to unambiguously refer to this column within a row in case more than
    /// one column have the same name
    fn ordinal(&self) -> usize;

    /// Gets the column name or alias.
    ///
    /// The column name is unreliable (and can change between database minor versions) if this
    /// column is an expression that has not been aliased.
    fn name(&self) -> &str;

    /// Gets the type information for the column.
    fn type_info(&self) -> &<Self::Database as Database>::TypeInfo;
}

/// A type that can be used to index into a [`Row`] or [`Statement`].
///
/// The [`get`] and [`try_get`] methods of [`Row`] accept any type that implements `ColumnIndex`.
/// This trait is implemented for strings which are used to look up a column by name, and for
/// `usize` which is used as a positional index into the row.
///
/// [`Row`]: crate::row::Row
/// [`Statement`]: crate::statement::Statement
/// [`get`]: crate::row::Row::get
/// [`try_get`]: crate::row::Row::try_get
///
pub trait ColumnIndex<T: ?Sized>: Debug {
    /// Returns a valid positional index into the row or statement, [`ColumnIndexOutOfBounds`], or,
    /// [`ColumnNotFound`].
    ///
    /// [`ColumnNotFound`]: Error::ColumnNotFound
    /// [`ColumnIndexOutOfBounds`]: Error::ColumnIndexOutOfBounds
    fn index(&self, container: &T) -> Result<usize, Error>;
}

impl<T: ?Sized, I: ColumnIndex<T> + ?Sized> ColumnIndex<T> for &'_ I {
    #[inline]
    fn index(&self, row: &T) -> Result<usize, Error> {
        (**self).index(row)
    }
}

#[macro_export]
macro_rules! impl_column_index_for_row {
    ($R:ident) => {
        impl $crate::column::ColumnIndex<$R> for usize {
            fn index(&self, row: &$R) -> Result<usize, $crate::error::Error> {
                let len = $crate::row::Row::len(row);

                if *self >= len {
                    return Err($crate::error::Error::ColumnIndexOutOfBounds { len, index: *self });
                }

                Ok(*self)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_column_index_for_statement {
    ($S:ident) => {
        impl $crate::column::ColumnIndex<$S<'_>> for usize {
            fn index(&self, statement: &$S<'_>) -> Result<usize, $crate::error::Error> {
                let len = $crate::statement::Statement::columns(statement).len();

                if *self >= len {
                    return Err($crate::error::Error::ColumnIndexOutOfBounds { len, index: *self });
                }

                Ok(*self)
            }
        }
    };
}
