//! Types and traits for passing arguments to SQL queries.

use crate::database::Database;
use crate::encode::Encode;
use crate::error::BoxDynError;
use crate::types::Type;
use std::fmt::{self, Write};

/// A tuple of arguments to be sent to the database.
// This lint is designed for general collections, but `Arguments` is not meant to be as such.
#[allow(clippy::len_without_is_empty)]
pub trait Arguments<'q>: Send + Sized + Default {
    type Database: Database;

    /// Reserves the capacity for at least `additional` more values (of `size` total bytes) to
    /// be added to the arguments without a reallocation.
    fn reserve(&mut self, additional: usize, size: usize);

    /// Add the value to the end of the arguments.
    fn add<T>(&mut self, value: T) -> Result<(), BoxDynError>
    where
        T: 'q + Encode<'q, Self::Database> + Type<Self::Database>;

    /// The number of arguments that were already added.
    fn len(&self) -> usize;

    fn format_placeholder<W: Write>(&self, writer: &mut W) -> fmt::Result {
        writer.write_str("?")
    }
}

pub trait IntoArguments<'q, DB: Database>: Sized + Send {
    fn into_arguments(self) -> <DB as Database>::Arguments<'q>;
}

// NOTE: required due to lack of lazy normalization
#[macro_export]
macro_rules! impl_into_arguments_for_arguments {
    ($Arguments:path) => {
        impl<'q>
            $crate::arguments::IntoArguments<
                'q,
                <$Arguments as $crate::arguments::Arguments<'q>>::Database,
            > for $Arguments
        {
            fn into_arguments(self) -> $Arguments {
                self
            }
        }
    };
}

/// used by the query macros to prevent supernumerary `.bind()` calls
pub struct ImmutableArguments<'q, DB: Database>(pub <DB as Database>::Arguments<'q>);

impl<'q, DB: Database> IntoArguments<'q, DB> for ImmutableArguments<'q, DB> {
    fn into_arguments(self) -> <DB as Database>::Arguments<'q> {
        self.0
    }
}

// TODO: Impl `IntoArguments` for &[&dyn Encode]
// TODO: Impl `IntoArguments` for (impl Encode, ...) x16
