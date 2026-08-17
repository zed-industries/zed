use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
#[cfg(feature = "std")]
use core::iter;
#[cfg(feature = "std")]
use std::io::{self, ErrorKind};

use pki_types::{
    pem, CertificateDer, CertificateRevocationListDer, CertificateSigningRequestDer,
    PrivatePkcs1KeyDer, PrivatePkcs8KeyDer, PrivateSec1KeyDer, SubjectPublicKeyInfoDer,
};

/// The contents of a single recognised block in a PEM file.
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum Item {
    /// A DER-encoded x509 certificate.
    ///
    /// Appears as "CERTIFICATE" in PEM files.
    X509Certificate(CertificateDer<'static>),

    /// A DER-encoded Subject Public Key Info; as specified in RFC 7468.
    ///
    /// Appears as "PUBLIC KEY" in PEM files.
    SubjectPublicKeyInfo(SubjectPublicKeyInfoDer<'static>),

    /// A DER-encoded plaintext RSA private key; as specified in PKCS #1/RFC 3447
    ///
    /// Appears as "RSA PRIVATE KEY" in PEM files.
    Pkcs1Key(PrivatePkcs1KeyDer<'static>),

    /// A DER-encoded plaintext private key; as specified in PKCS #8/RFC 5958
    ///
    /// Appears as "PRIVATE KEY" in PEM files.
    Pkcs8Key(PrivatePkcs8KeyDer<'static>),

    /// A Sec1-encoded plaintext private key; as specified in RFC 5915
    ///
    /// Appears as "EC PRIVATE KEY" in PEM files.
    Sec1Key(PrivateSec1KeyDer<'static>),

    /// A Certificate Revocation List; as specified in RFC 5280
    ///
    /// Appears as "X509 CRL" in PEM files.
    Crl(CertificateRevocationListDer<'static>),

    /// A Certificate Signing Request; as specified in RFC 2986
    ///
    /// Appears as "CERTIFICATE REQUEST" in PEM files.
    Csr(CertificateSigningRequestDer<'static>),
}

impl Item {
    #[cfg(feature = "std")]
    fn from_buf(rd: &mut dyn io::BufRead) -> Result<Option<Self>, pem::Error> {
        loop {
            match pem::from_buf(rd)? {
                Some((kind, data)) => match Self::from_kind(kind, data) {
                    Some(item) => return Ok(Some(item)),
                    None => continue,
                },

                None => return Ok(None),
            }
        }
    }

    fn from_slice(pem: &[u8]) -> Result<Option<(Self, &[u8])>, pem::Error> {
        let mut iter = <(pem::SectionKind, Vec<u8>) as pem::PemObject>::pem_slice_iter(pem);

        for found in iter.by_ref() {
            match found {
                Ok((kind, data)) => match Self::from_kind(kind, data) {
                    Some(item) => return Ok(Some((item, iter.remainder()))),
                    None => continue,
                },
                Err(err) => return Err(err.into()),
            }
        }

        Ok(None)
    }

    fn from_kind(kind: pem::SectionKind, data: Vec<u8>) -> Option<Self> {
        use pem::SectionKind::*;
        match kind {
            Certificate => Some(Self::X509Certificate(data.into())),
            PublicKey => Some(Self::SubjectPublicKeyInfo(data.into())),
            RsaPrivateKey => Some(Self::Pkcs1Key(data.into())),
            PrivateKey => Some(Self::Pkcs8Key(data.into())),
            EcPrivateKey => Some(Self::Sec1Key(data.into())),
            Crl => Some(Self::Crl(data.into())),
            Csr => Some(Self::Csr(data.into())),
            _ => None,
        }
    }
}

/// Errors that may arise when parsing the contents of a PEM file
///
/// This differs from [`rustls_pki_types::pem::Error`] because it is `PartialEq`;
/// it is retained for compatibility.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// a section is missing its "END marker" line
    MissingSectionEnd {
        /// the expected "END marker" line that was not found
        end_marker: Vec<u8>,
    },

    /// syntax error found in the line that starts a new section
    IllegalSectionStart {
        /// line that contains the syntax error
        line: Vec<u8>,
    },

    /// base64 decode error
    Base64Decode(String),
}

#[cfg(feature = "std")]
impl From<Error> for io::Error {
    fn from(error: Error) -> Self {
        match error {
            Error::MissingSectionEnd { end_marker } => io::Error::new(
                ErrorKind::InvalidData,
                format!(
                    "section end {:?} missing",
                    String::from_utf8_lossy(&end_marker)
                ),
            ),

            Error::IllegalSectionStart { line } => io::Error::new(
                ErrorKind::InvalidData,
                format!(
                    "illegal section start: {:?}",
                    String::from_utf8_lossy(&line)
                ),
            ),

            Error::Base64Decode(err) => io::Error::new(ErrorKind::InvalidData, err),
        }
    }
}

impl From<pem::Error> for Error {
    fn from(error: pem::Error) -> Self {
        match error {
            pem::Error::MissingSectionEnd { end_marker } => Error::MissingSectionEnd { end_marker },
            pem::Error::IllegalSectionStart { line } => Error::IllegalSectionStart { line },
            pem::Error::Base64Decode(str) => Error::Base64Decode(str),

            // this is a necessary bodge to funnel any new errors into our existing type
            // (to which we can add no new variants)
            other => Error::Base64Decode(format!("{other:?}")),
        }
    }
}

/// Extract and decode the next PEM section from `input`
///
/// - `Ok(None)` is returned if there is no PEM section to read from `input`
/// - Syntax errors and decoding errors produce a `Err(...)`
/// - Otherwise each decoded section is returned with a `Ok(Some((Item::..., remainder)))` where
///   `remainder` is the part of the `input` that follows the returned section
pub fn read_one_from_slice(input: &[u8]) -> Result<Option<(Item, &[u8])>, Error> {
    Item::from_slice(input).map_err(Into::into)
}

/// Extract and decode the next PEM section from `rd`.
///
/// - Ok(None) is returned if there is no PEM section read from `rd`.
/// - Underlying IO errors produce a `Err(...)`
/// - Otherwise each decoded section is returned with a `Ok(Some(Item::...))`
///
/// You can use this function to build an iterator, for example:
/// `for item in iter::from_fn(|| read_one(rd).transpose()) { ... }`
#[cfg(feature = "std")]
pub fn read_one(rd: &mut dyn io::BufRead) -> Result<Option<Item>, io::Error> {
    Item::from_buf(rd).map_err(|err| match err {
        pem::Error::Io(io) => io,
        other => Error::from(other).into(),
    })
}

/// Extract and return all PEM sections by reading `rd`.
#[cfg(feature = "std")]
pub fn read_all(rd: &mut dyn io::BufRead) -> impl Iterator<Item = Result<Item, io::Error>> + '_ {
    iter::from_fn(move || read_one(rd).transpose())
}
