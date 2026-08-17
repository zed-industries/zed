use crate::column::ColumnIndex;
use crate::database::Database;
use crate::decode::Decode;
use crate::error::{mismatched_types, Error};

use crate::type_info::TypeInfo;
use crate::types::Type;
use crate::value::ValueRef;

/// Represents a single row from the database.
///
/// [`FromRow`]: crate::row::FromRow
/// [`Query::fetch`]: crate::query::Query::fetch
pub trait Row: Unpin + Send + Sync + 'static {
    type Database: Database<Row = Self>;

    /// Returns `true` if this row has no columns.
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of columns in this row.
    #[inline]
    fn len(&self) -> usize {
        self.columns().len()
    }

    /// Gets the column information at `index`.
    ///
    /// A string index can be used to access a column by name and a `usize` index
    /// can be used to access a column by position.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    /// See [`try_column`](Self::try_column) for a non-panicking version.
    fn column<I>(&self, index: I) -> &<Self::Database as Database>::Column
    where
        I: ColumnIndex<Self>,
    {
        self.try_column(index).unwrap()
    }

    /// Gets the column information at `index` or a `ColumnIndexOutOfBounds` error if out of bounds.
    fn try_column<I>(&self, index: I) -> Result<&<Self::Database as Database>::Column, Error>
    where
        I: ColumnIndex<Self>,
    {
        Ok(&self.columns()[index.index(self)?])
    }

    /// Gets all columns in this statement.
    fn columns(&self) -> &[<Self::Database as Database>::Column];

    /// Index into the database row and decode a single value.
    ///
    /// A string index can be used to access a column by name and a `usize` index
    /// can be used to access a column by position.
    ///
    /// # Panics
    ///
    /// Panics if the column does not exist or its value cannot be decoded into the requested type.
    /// See [`try_get`](Self::try_get) for a non-panicking version.
    ///
    #[inline]
    #[track_caller]
    fn get<'r, T, I>(&'r self, index: I) -> T
    where
        I: ColumnIndex<Self>,
        T: Decode<'r, Self::Database> + Type<Self::Database>,
    {
        self.try_get::<T, I>(index).unwrap()
    }

    /// Index into the database row and decode a single value.
    ///
    /// Unlike [`get`](Self::get), this method does not check that the type
    /// being returned from the database is compatible with the Rust type and blindly tries
    /// to decode the value.
    ///
    /// # Panics
    ///
    /// Panics if the column does not exist or its value cannot be decoded into the requested type.
    /// See [`try_get_unchecked`](Self::try_get_unchecked) for a non-panicking version.
    ///
    #[inline]
    fn get_unchecked<'r, T, I>(&'r self, index: I) -> T
    where
        I: ColumnIndex<Self>,
        T: Decode<'r, Self::Database>,
    {
        self.try_get_unchecked::<T, I>(index).unwrap()
    }

    /// Index into the database row and decode a single value.
    ///
    /// A string index can be used to access a column by name and a `usize` index
    /// can be used to access a column by position.
    ///
    /// # Errors
    ///
    ///  * [`ColumnNotFound`] if the column by the given name was not found.
    ///  * [`ColumnIndexOutOfBounds`] if the `usize` index was greater than the number of columns in the row.
    ///  * [`ColumnDecode`] if the value could not be decoded into the requested type.
    ///
    /// [`ColumnDecode`]: Error::ColumnDecode
    /// [`ColumnNotFound`]: Error::ColumnNotFound
    /// [`ColumnIndexOutOfBounds`]: Error::ColumnIndexOutOfBounds
    ///
    fn try_get<'r, T, I>(&'r self, index: I) -> Result<T, Error>
    where
        I: ColumnIndex<Self>,
        T: Decode<'r, Self::Database> + Type<Self::Database>,
    {
        let value = self.try_get_raw(&index)?;

        if !value.is_null() {
            let ty = value.type_info();

            if !ty.is_null() && !T::compatible(&ty) {
                return Err(Error::ColumnDecode {
                    index: format!("{index:?}"),
                    source: mismatched_types::<Self::Database, T>(&ty),
                });
            }
        }

        T::decode(value).map_err(|source| Error::ColumnDecode {
            index: format!("{index:?}"),
            source,
        })
    }

    /// Index into the database row and decode a single value.
    ///
    /// Unlike [`try_get`](Self::try_get), this method does not check that the type
    /// being returned from the database is compatible with the Rust type and blindly tries
    /// to decode the value.
    ///
    /// # Errors
    ///
    ///  * [`ColumnNotFound`] if the column by the given name was not found.
    ///  * [`ColumnIndexOutOfBounds`] if the `usize` index was greater than the number of columns in the row.
    ///  * [`ColumnDecode`] if the value could not be decoded into the requested type.
    ///
    /// [`ColumnDecode`]: Error::ColumnDecode
    /// [`ColumnNotFound`]: Error::ColumnNotFound
    /// [`ColumnIndexOutOfBounds`]: Error::ColumnIndexOutOfBounds
    ///
    #[inline]
    fn try_get_unchecked<'r, T, I>(&'r self, index: I) -> Result<T, Error>
    where
        I: ColumnIndex<Self>,
        T: Decode<'r, Self::Database>,
    {
        let value = self.try_get_raw(&index)?;

        T::decode(value).map_err(|source| Error::ColumnDecode {
            index: format!("{index:?}"),
            source,
        })
    }

    /// Index into the database row and decode a single value.
    ///
    /// # Errors
    ///
    ///  * [`ColumnNotFound`] if the column by the given name was not found.
    ///  * [`ColumnIndexOutOfBounds`] if the `usize` index was greater than the number of columns in the row.
    ///
    /// [`ColumnNotFound`]: Error::ColumnNotFound
    /// [`ColumnIndexOutOfBounds`]: Error::ColumnIndexOutOfBounds
    ///
    fn try_get_raw<I>(&self, index: I) -> Result<<Self::Database as Database>::ValueRef<'_>, Error>
    where
        I: ColumnIndex<Self>;
}
