//! Signature error types

use core::fmt::{self, Debug, Display};

#[cfg(feature = "std")]
use std::boxed::Box;

/// Result type.
///
/// A result with the `signature` crate's [`Error`] type.
pub type Result<T> = core::result::Result<T, Error>;

/// Signature errors.
///
/// This type is deliberately opaque as to avoid sidechannel leakage which
/// could potentially be used recover signing private keys or forge signatures
/// (e.g. [BB'06]).
///
/// When the `std` feature is enabled, it impls [`std::error::Error`] and
/// supports an optional [`std::error::Error::source`], which can be used by
/// things like remote signers (e.g. HSM, KMS) to report I/O or auth errors.
///
/// [BB'06]: https://en.wikipedia.org/wiki/Daniel_Bleichenbacher
#[derive(Default)]
pub struct Error {
    /// Prevent from being instantiated as `Error {}` when the `std` feature
    /// is disabled
    _private: (),

    /// Source of the error (if applicable).
    #[cfg(feature = "std")]
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl Error {
    /// Create a new error with no associated source
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new error with an associated source.
    ///
    /// **NOTE:** The "source" should **NOT** be used to propagate cryptographic
    /// errors e.g. signature parsing or verification errors. The intended use
    /// cases are for propagating errors related to external signers, e.g.
    /// communication/authentication errors with HSMs, KMS, etc.
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn from_source(
        source: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    ) -> Self {
        Self {
            _private: (),
            source: Some(source.into()),
        }
    }
}

impl Debug for Error {
    #[cfg(not(feature = "std"))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("signature::Error {}")
    }

    #[cfg(feature = "std")]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("signature::Error { source: ")?;

        if let Some(source) = &self.source {
            write!(f, "Some({})", source)?;
        } else {
            f.write_str("None")?;
        }

        f.write_str(" }")
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("signature error")?;

        #[cfg(feature = "std")]
        {
            if let Some(source) = &self.source {
                write!(f, ": {}", source)?;
            }
        }

        Ok(())
    }
}

#[cfg(feature = "std")]
impl From<Box<dyn std::error::Error + Send + Sync + 'static>> for Error {
    fn from(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Error {
        Self::from_source(source)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| source.as_ref() as &(dyn std::error::Error + 'static))
    }
}
