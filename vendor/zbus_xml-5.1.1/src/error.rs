use quick_xml::{de::DeError, se::SeError};
use std::{convert::Infallible, error, fmt};
use zvariant::Error as VariantError;

/// The error type for `zbus_names`.
///
/// The various errors that can be reported by this crate.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Error {
    Variant(VariantError),
    /// An XML error from quick_xml
    QuickXml(DeError),
    /// An XML serialization error from quick_xml
    QuickXmlSer(SeError),
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Variant(s), Self::Variant(o)) => s == o,
            (Self::QuickXml(_), Self::QuickXml(_)) => false,
            (Self::QuickXmlSer(_), Self::QuickXmlSer(_)) => false,
            (_, _) => false,
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Variant(e) => Some(e),
            Error::QuickXml(e) => Some(e),
            Error::QuickXmlSer(e) => Some(e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Variant(e) => write!(f, "{e}"),
            Error::QuickXml(e) => write!(f, "XML error: {e}"),
            Error::QuickXmlSer(e) => write!(f, "XML serialization error: {e}"),
        }
    }
}

impl From<VariantError> for Error {
    fn from(val: VariantError) -> Self {
        Error::Variant(val)
    }
}

impl From<DeError> for Error {
    fn from(val: DeError) -> Self {
        Error::QuickXml(val)
    }
}

impl From<SeError> for Error {
    fn from(val: SeError) -> Self {
        Error::QuickXmlSer(val)
    }
}

impl From<Infallible> for Error {
    fn from(i: Infallible) -> Self {
        match i {}
    }
}

/// Alias for a `Result` with the error type `zbus_xml::Error`.
pub type Result<T> = std::result::Result<T, Error>;
