//! Traits for parsing objects from PKCS#1 encoded documents

use crate::Result;

#[cfg(feature = "alloc")]
use der::{Document, SecretDocument};

#[cfg(feature = "pem")]
use {
    crate::LineEnding,
    alloc::string::String,
    der::{pem::PemLabel, zeroize::Zeroizing},
};

#[cfg(feature = "pkcs8")]
use {
    crate::{ALGORITHM_ID, ALGORITHM_OID},
    der::asn1::BitStringRef,
};

#[cfg(feature = "std")]
use std::path::Path;

#[cfg(all(feature = "alloc", feature = "pkcs8"))]
use der::Decode;

#[cfg(all(feature = "alloc", any(feature = "pem", feature = "pkcs8")))]
use crate::{RsaPrivateKey, RsaPublicKey};

/// Parse an [`RsaPrivateKey`] from a PKCS#1-encoded document.
pub trait DecodeRsaPrivateKey: Sized {
    /// Deserialize PKCS#1 private key from ASN.1 DER-encoded data
    /// (binary format).
    fn from_pkcs1_der(bytes: &[u8]) -> Result<Self>;

    /// Deserialize PKCS#1-encoded private key from PEM.
    ///
    /// Keys in this format begin with the following:
    ///
    /// ```text
    /// -----BEGIN RSA PRIVATE KEY-----
    /// ```
    #[cfg(feature = "pem")]
    fn from_pkcs1_pem(s: &str) -> Result<Self> {
        let (label, doc) = SecretDocument::from_pem(s)?;
        RsaPrivateKey::validate_pem_label(label)?;
        Self::from_pkcs1_der(doc.as_bytes())
    }

    /// Load PKCS#1 private key from an ASN.1 DER-encoded file on the local
    /// filesystem (binary format).
    #[cfg(feature = "std")]
    fn read_pkcs1_der_file(path: impl AsRef<Path>) -> Result<Self> {
        Self::from_pkcs1_der(SecretDocument::read_der_file(path)?.as_bytes())
    }

    /// Load PKCS#1 private key from a PEM-encoded file on the local filesystem.
    #[cfg(all(feature = "pem", feature = "std"))]
    fn read_pkcs1_pem_file(path: impl AsRef<Path>) -> Result<Self> {
        let (label, doc) = SecretDocument::read_pem_file(path)?;
        RsaPrivateKey::validate_pem_label(&label)?;
        Self::from_pkcs1_der(doc.as_bytes())
    }
}

/// Parse a [`RsaPublicKey`] from a PKCS#1-encoded document.
pub trait DecodeRsaPublicKey: Sized {
    /// Deserialize object from ASN.1 DER-encoded [`RsaPublicKey`]
    /// (binary format).
    fn from_pkcs1_der(bytes: &[u8]) -> Result<Self>;

    /// Deserialize PEM-encoded [`RsaPublicKey`].
    ///
    /// Keys in this format begin with the following:
    ///
    /// ```text
    /// -----BEGIN RSA PUBLIC KEY-----
    /// ```
    #[cfg(feature = "pem")]
    fn from_pkcs1_pem(s: &str) -> Result<Self> {
        let (label, doc) = Document::from_pem(s)?;
        RsaPublicKey::validate_pem_label(label)?;
        Self::from_pkcs1_der(doc.as_bytes())
    }

    /// Load [`RsaPublicKey`] from an ASN.1 DER-encoded file on the local
    /// filesystem (binary format).
    #[cfg(feature = "std")]
    fn read_pkcs1_der_file(path: impl AsRef<Path>) -> Result<Self> {
        let doc = Document::read_der_file(path)?;
        Self::from_pkcs1_der(doc.as_bytes())
    }

    /// Load [`RsaPublicKey`] from a PEM-encoded file on the local filesystem.
    #[cfg(all(feature = "pem", feature = "std"))]
    fn read_pkcs1_pem_file(path: impl AsRef<Path>) -> Result<Self> {
        let (label, doc) = Document::read_pem_file(path)?;
        RsaPublicKey::validate_pem_label(&label)?;
        Self::from_pkcs1_der(doc.as_bytes())
    }
}

/// Serialize a [`RsaPrivateKey`] to a PKCS#1 encoded document.
#[cfg(feature = "alloc")]
pub trait EncodeRsaPrivateKey {
    /// Serialize a [`SecretDocument`] containing a PKCS#1-encoded private key.
    fn to_pkcs1_der(&self) -> Result<SecretDocument>;

    /// Serialize this private key as PEM-encoded PKCS#1 with the given [`LineEnding`].
    #[cfg(feature = "pem")]
    fn to_pkcs1_pem(&self, line_ending: LineEnding) -> Result<Zeroizing<String>> {
        let doc = self.to_pkcs1_der()?;
        Ok(doc.to_pem(RsaPrivateKey::PEM_LABEL, line_ending)?)
    }

    /// Write ASN.1 DER-encoded PKCS#1 private key to the given path.
    #[cfg(feature = "std")]
    fn write_pkcs1_der_file(&self, path: impl AsRef<Path>) -> Result<()> {
        Ok(self.to_pkcs1_der()?.write_der_file(path)?)
    }

    /// Write ASN.1 DER-encoded PKCS#1 private key to the given path.
    #[cfg(all(feature = "pem", feature = "std"))]
    fn write_pkcs1_pem_file(&self, path: impl AsRef<Path>, line_ending: LineEnding) -> Result<()> {
        let doc = self.to_pkcs1_der()?;
        Ok(doc.write_pem_file(path, RsaPrivateKey::PEM_LABEL, line_ending)?)
    }
}

/// Serialize a [`RsaPublicKey`] to a PKCS#1-encoded document.
#[cfg(feature = "alloc")]
pub trait EncodeRsaPublicKey {
    /// Serialize a [`Document`] containing a PKCS#1-encoded public key.
    fn to_pkcs1_der(&self) -> Result<Document>;

    /// Serialize this public key as PEM-encoded PKCS#1 with the given line ending.
    #[cfg(feature = "pem")]
    fn to_pkcs1_pem(&self, line_ending: LineEnding) -> Result<String> {
        let doc = self.to_pkcs1_der()?;
        Ok(doc.to_pem(RsaPublicKey::PEM_LABEL, line_ending)?)
    }

    /// Write ASN.1 DER-encoded public key to the given path.
    #[cfg(feature = "std")]
    fn write_pkcs1_der_file(&self, path: impl AsRef<Path>) -> Result<()> {
        Ok(self.to_pkcs1_der()?.write_der_file(path)?)
    }

    /// Write ASN.1 DER-encoded public key to the given path.
    #[cfg(all(feature = "pem", feature = "std"))]
    fn write_pkcs1_pem_file(&self, path: impl AsRef<Path>, line_ending: LineEnding) -> Result<()> {
        let doc = self.to_pkcs1_der()?;
        Ok(doc.write_pem_file(path, RsaPublicKey::PEM_LABEL, line_ending)?)
    }
}

#[cfg(feature = "pkcs8")]
impl<T> DecodeRsaPrivateKey for T
where
    T: for<'a> TryFrom<pkcs8::PrivateKeyInfo<'a>, Error = pkcs8::Error>,
{
    fn from_pkcs1_der(private_key: &[u8]) -> Result<Self> {
        Ok(Self::try_from(pkcs8::PrivateKeyInfo {
            algorithm: ALGORITHM_ID,
            private_key,
            public_key: None,
        })?)
    }
}

#[cfg(feature = "pkcs8")]
impl<T> DecodeRsaPublicKey for T
where
    T: for<'a> TryFrom<pkcs8::SubjectPublicKeyInfoRef<'a>, Error = pkcs8::spki::Error>,
{
    fn from_pkcs1_der(public_key: &[u8]) -> Result<Self> {
        Ok(Self::try_from(pkcs8::SubjectPublicKeyInfoRef {
            algorithm: ALGORITHM_ID,
            subject_public_key: BitStringRef::from_bytes(public_key)?,
        })?)
    }
}

#[cfg(all(feature = "alloc", feature = "pkcs8"))]
impl<T: pkcs8::EncodePrivateKey> EncodeRsaPrivateKey for T {
    fn to_pkcs1_der(&self) -> Result<SecretDocument> {
        let pkcs8_doc = self.to_pkcs8_der()?;
        let pkcs8_key = pkcs8::PrivateKeyInfo::from_der(pkcs8_doc.as_bytes())?;
        pkcs8_key.algorithm.assert_algorithm_oid(ALGORITHM_OID)?;
        RsaPrivateKey::from_der(pkcs8_key.private_key)?.try_into()
    }
}

#[cfg(all(feature = "alloc", feature = "pkcs8"))]
impl<T: pkcs8::EncodePublicKey> EncodeRsaPublicKey for T {
    fn to_pkcs1_der(&self) -> Result<Document> {
        let doc = self.to_public_key_der()?;
        let spki = pkcs8::SubjectPublicKeyInfoRef::from_der(doc.as_bytes())?;
        spki.algorithm.assert_algorithm_oid(ALGORITHM_OID)?;
        RsaPublicKey::from_der(spki.subject_public_key.raw_bytes())?.try_into()
    }
}
