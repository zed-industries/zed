use std::{error, fmt};

/// An enumeration of buffer creation errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// No choices were provided to the Unstructured::choose call
    EmptyChoose,
    /// There was not enough underlying data to fulfill some request for raw
    /// bytes.
    ///
    /// Note that outside of [`Unstructured::bytes`][crate::Unstructured::bytes],
    /// most APIs do *not* return this error when running out of underlying arbitrary bytes
    /// but silently return some default value instead.
    NotEnoughData,
    /// The input bytes were not of the right format
    IncorrectFormat,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::EmptyChoose => write!(
                f,
                "`arbitrary::Unstructured::choose` must be given a non-empty set of choices"
            ),
            Error::NotEnoughData => write!(
                f,
                "There is not enough underlying raw data to construct an `Arbitrary` instance"
            ),
            Error::IncorrectFormat => write!(
                f,
                "The raw data is not of the correct format to construct this type"
            ),
        }
    }
}

impl error::Error for Error {}

/// A `Result` with the error type fixed as `arbitrary::Error`.
///
/// Either an `Ok(T)` or `Err(arbitrary::Error)`.
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod tests {
    // Often people will import our custom `Result` type because 99.9% of
    // results in a file will be `arbitrary::Result` but then have that one last
    // 0.1% that want to have a custom error type. Don't make them prefix that
    // 0.1% as `std::result::Result`; instead, let `arbitrary::Result` have an
    // overridable error type.
    #[test]
    fn can_use_custom_error_types_with_result() -> super::Result<(), String> {
        Ok(())
    }
}
