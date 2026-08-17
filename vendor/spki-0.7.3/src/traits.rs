//! Traits for encoding/decoding SPKI public keys.

use crate::{AlgorithmIdentifier, Error, Result, SubjectPublicKeyInfoRef};
use der::{EncodeValue, Tagged};

#[cfg(feature = "alloc")]
use {
    crate::AlgorithmIdentifierOwned,
    der::{asn1::BitString, Any, Document},
};

#[cfg(feature = "pem")]
use {
    alloc::string::String,
    der::pem::{LineEnding, PemLabel},
};

#[cfg(feature = "std")]
use std::path::Path;

#[cfg(doc)]
use crate::SubjectPublicKeyInfo;

/// Parse a public key object from an encoded SPKI document.
pub trait DecodePublicKey: Sized {
    /// Deserialize object from ASN.1 DER-encoded [`SubjectPublicKeyInfo`]
    /// (binary format).
    fn from_public_key_der(bytes: &[u8]) -> Result<Self>;

    /// Deserialize PEM-encoded [`SubjectPublicKeyInfo`].
    ///
    /// Keys in this format begin with the following delimiter:
    ///
    /// ```text
    /// -----BEGIN PUBLIC KEY-----
    /// ```
    #[cfg(feature = "pem")]
    fn from_public_key_pem(s: &str) -> Result<Self> {
        let (label, doc) = Document::from_pem(s)?;
        SubjectPublicKeyInfoRef::validate_pem_label(label)?;
        Self::from_public_key_der(doc.as_bytes())
    }

    /// Load public key object from an ASN.1 DER-encoded file on the local
    /// filesystem (binary format).
    #[cfg(feature = "std")]
    fn read_public_key_der_file(path: impl AsRef<Path>) -> Result<Self> {
        let doc = Document::read_der_file(path)?;
        Self::from_public_key_der(doc.as_bytes())
    }

    /// Load public key object from a PEM-encoded file on the local filesystem.
    #[cfg(all(feature = "pem", feature = "std"))]
    fn read_public_key_pem_file(path: impl AsRef<Path>) -> Result<Self> {
        let (label, doc) = Document::read_pem_file(path)?;
        SubjectPublicKeyInfoRef::validate_pem_label(&label)?;
        Self::from_public_key_der(doc.as_bytes())
    }
}

impl<T> DecodePublicKey for T
where
    T: for<'a> TryFrom<SubjectPublicKeyInfoRef<'a>, Error = Error>,
{
    fn from_public_key_der(bytes: &[u8]) -> Result<Self> {
        Self::try_from(SubjectPublicKeyInfoRef::try_from(bytes)?)
    }
}

/// Serialize a public key object to a SPKI-encoded document.
#[cfg(feature = "alloc")]
pub trait EncodePublicKey {
    /// Serialize a [`Document`] containing a SPKI-encoded public key.
    fn to_public_key_der(&self) -> Result<Document>;

    /// Serialize this public key as PEM-encoded SPKI with the given [`LineEnding`].
    #[cfg(feature = "pem")]
    fn to_public_key_pem(&self, line_ending: LineEnding) -> Result<String> {
        let doc = self.to_public_key_der()?;
        Ok(doc.to_pem(SubjectPublicKeyInfoRef::PEM_LABEL, line_ending)?)
    }

    /// Write ASN.1 DER-encoded public key to the given path
    #[cfg(feature = "std")]
    fn write_public_key_der_file(&self, path: impl AsRef<Path>) -> Result<()> {
        Ok(self.to_public_key_der()?.write_der_file(path)?)
    }

    /// Write ASN.1 DER-encoded public key to the given path
    #[cfg(all(feature = "pem", feature = "std"))]
    fn write_public_key_pem_file(
        &self,
        path: impl AsRef<Path>,
        line_ending: LineEnding,
    ) -> Result<()> {
        let doc = self.to_public_key_der()?;
        Ok(doc.write_pem_file(path, SubjectPublicKeyInfoRef::PEM_LABEL, line_ending)?)
    }
}

/// Returns `AlgorithmIdentifier` associated with the structure.
///
/// This is useful for e.g. keys for digital signature algorithms.
pub trait AssociatedAlgorithmIdentifier {
    /// Algorithm parameters.
    type Params: Tagged + EncodeValue;

    /// `AlgorithmIdentifier` for this structure.
    const ALGORITHM_IDENTIFIER: AlgorithmIdentifier<Self::Params>;
}

/// Returns `AlgorithmIdentifier` associated with the structure.
///
/// This is useful for e.g. keys for digital signature algorithms.
#[cfg(feature = "alloc")]
pub trait DynAssociatedAlgorithmIdentifier {
    /// `AlgorithmIdentifier` for this structure.
    fn algorithm_identifier(&self) -> Result<AlgorithmIdentifierOwned>;
}

#[cfg(feature = "alloc")]
impl<T> DynAssociatedAlgorithmIdentifier for T
where
    T: AssociatedAlgorithmIdentifier,
{
    fn algorithm_identifier(&self) -> Result<AlgorithmIdentifierOwned> {
        Ok(AlgorithmIdentifierOwned {
            oid: T::ALGORITHM_IDENTIFIER.oid,
            parameters: T::ALGORITHM_IDENTIFIER
                .parameters
                .as_ref()
                .map(Any::encode_from)
                .transpose()?,
        })
    }
}

/// Returns `AlgorithmIdentifier` associated with the signature system.
///
/// Unlike AssociatedAlgorithmIdentifier this is intended to be implemented for public and/or
/// private keys.
pub trait SignatureAlgorithmIdentifier {
    /// Algorithm parameters.
    type Params: Tagged + EncodeValue;

    /// `AlgorithmIdentifier` for the corresponding singature system.
    const SIGNATURE_ALGORITHM_IDENTIFIER: AlgorithmIdentifier<Self::Params>;
}

/// Returns `AlgorithmIdentifier` associated with the signature system.
///
/// Unlike AssociatedAlgorithmIdentifier this is intended to be implemented for public and/or
/// private keys.
#[cfg(feature = "alloc")]
pub trait DynSignatureAlgorithmIdentifier {
    /// `AlgorithmIdentifier` for the corresponding singature system.
    fn signature_algorithm_identifier(&self) -> Result<AlgorithmIdentifierOwned>;
}

#[cfg(feature = "alloc")]
impl<T> DynSignatureAlgorithmIdentifier for T
where
    T: SignatureAlgorithmIdentifier,
{
    fn signature_algorithm_identifier(&self) -> Result<AlgorithmIdentifierOwned> {
        Ok(AlgorithmIdentifierOwned {
            oid: T::SIGNATURE_ALGORITHM_IDENTIFIER.oid,
            parameters: T::SIGNATURE_ALGORITHM_IDENTIFIER
                .parameters
                .as_ref()
                .map(Any::encode_from)
                .transpose()?,
        })
    }
}

/// Returns the `BitString` encoding of the signature.
///
/// X.509 and CSR structures require signatures to be BitString encoded.
#[cfg(feature = "alloc")]
pub trait SignatureBitStringEncoding {
    /// `BitString` encoding for this signature.
    fn to_bitstring(&self) -> der::Result<BitString>;
}
