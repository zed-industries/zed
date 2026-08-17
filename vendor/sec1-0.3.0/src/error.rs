//! Error types

use core::fmt;

#[cfg(feature = "pem")]
use der::pem;

/// Result type with `sec1` crate's [`Error`] type.
pub type Result<T> = core::result::Result<T, Error>;

/// Error type
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// ASN.1 DER-related errors.
    #[cfg(feature = "der")]
    #[cfg_attr(docsrs, doc(cfg(feature = "der")))]
    Asn1(der::Error),

    /// Cryptographic errors.
    ///
    /// These can be used by EC implementations to signal that a key is
    /// invalid for cryptographic reasons. This means the document parsed
    /// correctly, but one of the values contained within was invalid, e.g.
    /// a number expected to be a prime was not a prime.
    Crypto,

    /// PKCS#8 errors.
    #[cfg(feature = "pkcs8")]
    Pkcs8(pkcs8::Error),

    /// Errors relating to the `Elliptic-Curve-Point-to-Octet-String` or
    /// `Octet-String-to-Elliptic-Curve-Point` encodings.
    PointEncoding,

    /// Version errors
    Version,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "der")]
            Error::Asn1(err) => write!(f, "SEC1 ASN.1 error: {}", err),
            Error::Crypto => f.write_str("SEC1 cryptographic error"),
            #[cfg(feature = "pkcs8")]
            Error::Pkcs8(err) => write!(f, "{}", err),
            Error::PointEncoding => f.write_str("elliptic curve point encoding error"),
            Error::Version => f.write_str("SEC1 version error"),
        }
    }
}

#[cfg(feature = "der")]
#[cfg_attr(docsrs, doc(cfg(feature = "der")))]
impl From<der::Error> for Error {
    fn from(err: der::Error) -> Error {
        Error::Asn1(err)
    }
}

#[cfg(feature = "pem")]
impl From<pem::Error> for Error {
    fn from(err: pem::Error) -> Error {
        der::Error::from(err).into()
    }
}

#[cfg(feature = "pkcs8")]
impl From<pkcs8::Error> for Error {
    fn from(err: pkcs8::Error) -> Error {
        Error::Pkcs8(err)
    }
}

#[cfg(feature = "pkcs8")]
impl From<pkcs8::spki::Error> for Error {
    fn from(err: pkcs8::spki::Error) -> Error {
        Error::Pkcs8(pkcs8::Error::PublicKey(err))
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
