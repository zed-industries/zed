use crate::database::Database;
use crate::decode::Decode;
use crate::error::{mismatched_types, Error};
use crate::type_info::TypeInfo;
use crate::types::Type;
use std::borrow::Cow;

/// An owned value from the database.
pub trait Value {
    type Database: Database<Value = Self>;

    /// Get this value as a reference.
    fn as_ref(&self) -> <Self::Database as Database>::ValueRef<'_>;

    /// Get the type information for this value.
    fn type_info(&self) -> Cow<'_, <Self::Database as Database>::TypeInfo>;

    /// Returns `true` if the SQL value is `NULL`.
    fn is_null(&self) -> bool;

    /// Decode this single value into the requested type.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be decoded into the requested type.
    /// See [`try_decode`](Self::try_decode) for a non-panicking version.
    ///
    #[inline]
    fn decode<'r, T>(&'r self) -> T
    where
        T: Decode<'r, Self::Database> + Type<Self::Database>,
    {
        self.try_decode::<T>().unwrap()
    }

    /// Decode this single value into the requested type.
    ///
    /// Unlike [`decode`](Self::decode), this method does not check that the type of this
    /// value is compatible with the Rust type and blindly tries to decode the value.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be decoded into the requested type.
    /// See [`try_decode_unchecked`](Self::try_decode_unchecked) for a non-panicking version.
    ///
    #[inline]
    fn decode_unchecked<'r, T>(&'r self) -> T
    where
        T: Decode<'r, Self::Database>,
    {
        self.try_decode_unchecked::<T>().unwrap()
    }

    /// Decode this single value into the requested type.
    ///
    /// # Errors
    ///
    ///  * [`Decode`] if the value could not be decoded into the requested type.
    ///
    /// [`Decode`]: Error::Decode
    ///
    #[inline]
    fn try_decode<'r, T>(&'r self) -> Result<T, Error>
    where
        T: Decode<'r, Self::Database> + Type<Self::Database>,
    {
        if !self.is_null() {
            let ty = self.type_info();

            if !ty.is_null() && !T::compatible(&ty) {
                return Err(Error::Decode(mismatched_types::<Self::Database, T>(&ty)));
            }
        }

        self.try_decode_unchecked()
    }

    /// Decode this single value into the requested type.
    ///
    /// Unlike [`try_decode`](Self::try_decode), this method does not check that the type of this
    /// value is compatible with the Rust type and blindly tries to decode the value.
    ///
    /// # Errors
    ///
    ///  * [`Decode`] if the value could not be decoded into the requested type.
    ///
    /// [`Decode`]: Error::Decode
    ///
    #[inline]
    fn try_decode_unchecked<'r, T>(&'r self) -> Result<T, Error>
    where
        T: Decode<'r, Self::Database>,
    {
        T::decode(self.as_ref()).map_err(Error::Decode)
    }
}

/// A reference to a single value from the database.
pub trait ValueRef<'r>: Sized {
    type Database: Database;

    /// Creates an owned value from this value reference.
    ///
    /// This is just a reference increment in PostgreSQL and MySQL and thus is `O(1)`. In SQLite,
    /// this is a copy.
    fn to_owned(&self) -> <Self::Database as Database>::Value;

    /// Get the type information for this value.
    fn type_info(&self) -> Cow<'_, <Self::Database as Database>::TypeInfo>;

    /// Returns `true` if the SQL value is `NULL`.
    fn is_null(&self) -> bool;
}
