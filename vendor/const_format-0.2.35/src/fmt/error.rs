// <_< clippy you silly
#![allow(clippy::enum_variant_names)]

use core::fmt::{self, Display};

/// An error while trying to write into a StrWriter.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    /// Attempted to write something into the buffer when there isn't enough space to write it.
    NotEnoughSpace,
    /// For compatibility with [`NotAsciiError`](../wrapper_types/struct.NotAsciiError.html)
    NotAscii,
    /// Attempted to index a string arguent by an range where one of the bounds
    /// was not on a char boundary.
    NotOnCharBoundary,
}

impl Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::NotEnoughSpace => {
                fmt.write_str("The was not enough space to write the formatted output")
            }
            Self::NotAscii => fmt.write_str("Attempted to write non-ascii text"),
            Self::NotOnCharBoundary => {
                fmt.write_str("Attempted to index a byte that's not on a char boundary.")
            }
        }
    }
}

macro_rules! index_vars{
    ($self:ident, $index:ident; $($variant:ident),* $(,)? ) => (
        enum Index{
            $($variant,)*
        }

        let $index = match &$self {
            $(Error::$variant{..} => 3300 + Index::$variant as usize,)*
        };
    )
}

impl Error {
    /// For panicking at compile-time, with a compile-time error that says what the error is.
    #[track_caller]
    pub const fn unwrap<T>(&self) -> T {
        index_vars! {
            self,i;
            NotEnoughSpace,
            NotAscii,
            NotOnCharBoundary,
        };

        match self {
            Error::NotEnoughSpace => ["The was not enough space to write the formatted output"][i],
            Error::NotAscii => ["Attempted to write non-ascii text"][i],
            Error::NotOnCharBoundary => {
                ["Attempted to index a byte that's not on a char boundary."][i]
            }
        };
        loop {}
    }
}

////////////////////////////////////////////////////////////////////////////////

/// The return type of most formatting functions
pub type Result<T = (), E = Error> = core::result::Result<T, E>;

////////////////////////////////////////////////////////////////////////////////

/// For converting types to [`const_format::Result`]
///
/// [`const_format::Result`]: ./type.Result.html
pub struct ToResult<T>(pub T);

impl ToResult<()> {
    ///
    #[inline(always)]
    pub const fn to_result(self) -> Result {
        Ok(())
    }
}

impl ToResult<Result> {
    ///
    #[inline(always)]
    pub const fn to_result(self) -> Result {
        self.0
    }
}
