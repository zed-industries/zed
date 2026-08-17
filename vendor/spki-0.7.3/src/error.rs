//! Error types

use core::fmt;
use der::asn1::ObjectIdentifier;

/// Result type with `spki` crate's [`Error`] type.
pub type Result<T> = core::result::Result<T, Error>;

#[cfg(feature = "pem")]
use der::pem;

/// Error type
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// Algorithm parameters are missing.
    AlgorithmParametersMissing,

    /// ASN.1 DER-related errors.
    Asn1(der::Error),

    /// Malformed cryptographic key contained in a SPKI document.
    ///
    /// This is intended for relaying errors related to the raw data contained
    /// in [`SubjectPublicKeyInfo::subject_public_key`][`crate::SubjectPublicKeyInfo::subject_public_key`].
    KeyMalformed,

    /// Unknown algorithm OID.
    OidUnknown {
        /// Unrecognized OID value found in e.g. a SPKI `AlgorithmIdentifier`.
        oid: ObjectIdentifier,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::AlgorithmParametersMissing => {
                f.write_str("AlgorithmIdentifier parameters missing")
            }
            Error::Asn1(err) => write!(f, "ASN.1 error: {}", err),
            Error::KeyMalformed => f.write_str("SPKI cryptographic key data malformed"),
            Error::OidUnknown { oid } => {
                write!(f, "unknown/unsupported algorithm OID: {}", oid)
            }
        }
    }
}

impl From<der::Error> for Error {
    fn from(err: der::Error) -> Error {
        if let der::ErrorKind::OidUnknown { oid } = err.kind() {
            Error::OidUnknown { oid }
        } else {
            Error::Asn1(err)
        }
    }
}

#[cfg(feature = "pem")]
impl From<pem::Error> for Error {
    fn from(err: pem::Error) -> Error {
        der::Error::from(err).into()
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
