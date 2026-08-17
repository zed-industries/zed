//! Traits for parsing objects from PKCS#8 encoded documents

use crate::{Error, PrivateKeyInfo, Result};

#[cfg(feature = "alloc")]
use der::SecretDocument;

#[cfg(feature = "encryption")]
use {
    crate::EncryptedPrivateKeyInfo,
    rand_core::{CryptoRng, RngCore},
};

#[cfg(feature = "pem")]
use {crate::LineEnding, alloc::string::String, der::zeroize::Zeroizing};

#[cfg(feature = "pem")]
use der::pem::PemLabel;

#[cfg(feature = "std")]
use std::path::Path;

/// Parse a private key object from a PKCS#8 encoded document.
pub trait DecodePrivateKey: Sized {
    /// Deserialize PKCS#8 private key from ASN.1 DER-encoded data
    /// (binary format).
    fn from_pkcs8_der(bytes: &[u8]) -> Result<Self>;

    /// Deserialize encrypted PKCS#8 private key from ASN.1 DER-encoded data
    /// (binary format) and attempt to decrypt it using the provided password.
    #[cfg(feature = "encryption")]
    fn from_pkcs8_encrypted_der(bytes: &[u8], password: impl AsRef<[u8]>) -> Result<Self> {
        let doc = EncryptedPrivateKeyInfo::try_from(bytes)?.decrypt(password)?;
        Self::from_pkcs8_der(doc.as_bytes())
    }

    /// Deserialize PKCS#8-encoded private key from PEM.
    ///
    /// Keys in this format begin with the following delimiter:
    ///
    /// ```text
    /// -----BEGIN PRIVATE KEY-----
    /// ```
    #[cfg(feature = "pem")]
    fn from_pkcs8_pem(s: &str) -> Result<Self> {
        let (label, doc) = SecretDocument::from_pem(s)?;
        PrivateKeyInfo::validate_pem_label(label)?;
        Self::from_pkcs8_der(doc.as_bytes())
    }

    /// Deserialize encrypted PKCS#8-encoded private key from PEM and attempt
    /// to decrypt it using the provided password.
    ///
    /// Keys in this format begin with the following delimiter:
    ///
    /// ```text
    /// -----BEGIN ENCRYPTED PRIVATE KEY-----
    /// ```
    #[cfg(all(feature = "encryption", feature = "pem"))]
    fn from_pkcs8_encrypted_pem(s: &str, password: impl AsRef<[u8]>) -> Result<Self> {
        let (label, doc) = SecretDocument::from_pem(s)?;
        EncryptedPrivateKeyInfo::validate_pem_label(label)?;
        Self::from_pkcs8_encrypted_der(doc.as_bytes(), password)
    }

    /// Load PKCS#8 private key from an ASN.1 DER-encoded file on the local
    /// filesystem (binary format).
    #[cfg(feature = "std")]
    fn read_pkcs8_der_file(path: impl AsRef<Path>) -> Result<Self> {
        Self::from_pkcs8_der(SecretDocument::read_der_file(path)?.as_bytes())
    }

    /// Load PKCS#8 private key from a PEM-encoded file on the local filesystem.
    #[cfg(all(feature = "pem", feature = "std"))]
    fn read_pkcs8_pem_file(path: impl AsRef<Path>) -> Result<Self> {
        let (label, doc) = SecretDocument::read_pem_file(path)?;
        PrivateKeyInfo::validate_pem_label(&label)?;
        Self::from_pkcs8_der(doc.as_bytes())
    }
}

impl<T> DecodePrivateKey for T
where
    T: for<'a> TryFrom<PrivateKeyInfo<'a>, Error = Error>,
{
    fn from_pkcs8_der(bytes: &[u8]) -> Result<Self> {
        Self::try_from(PrivateKeyInfo::try_from(bytes)?)
    }
}

/// Serialize a private key object to a PKCS#8 encoded document.
#[cfg(feature = "alloc")]
pub trait EncodePrivateKey {
    /// Serialize a [`SecretDocument`] containing a PKCS#8-encoded private key.
    fn to_pkcs8_der(&self) -> Result<SecretDocument>;

    /// Create an [`SecretDocument`] containing the ciphertext of
    /// a PKCS#8 encoded private key encrypted under the given `password`.
    #[cfg(feature = "encryption")]
    fn to_pkcs8_encrypted_der(
        &self,
        rng: impl CryptoRng + RngCore,
        password: impl AsRef<[u8]>,
    ) -> Result<SecretDocument> {
        EncryptedPrivateKeyInfo::encrypt(rng, password, self.to_pkcs8_der()?.as_bytes())
    }

    /// Serialize this private key as PEM-encoded PKCS#8 with the given [`LineEnding`].
    #[cfg(feature = "pem")]
    fn to_pkcs8_pem(&self, line_ending: LineEnding) -> Result<Zeroizing<String>> {
        let doc = self.to_pkcs8_der()?;
        Ok(doc.to_pem(PrivateKeyInfo::PEM_LABEL, line_ending)?)
    }

    /// Serialize this private key as an encrypted PEM-encoded PKCS#8 private
    /// key using the `provided` to derive an encryption key.
    #[cfg(all(feature = "encryption", feature = "pem"))]
    fn to_pkcs8_encrypted_pem(
        &self,
        rng: impl CryptoRng + RngCore,
        password: impl AsRef<[u8]>,
        line_ending: LineEnding,
    ) -> Result<Zeroizing<String>> {
        let doc = self.to_pkcs8_encrypted_der(rng, password)?;
        Ok(doc.to_pem(EncryptedPrivateKeyInfo::PEM_LABEL, line_ending)?)
    }

    /// Write ASN.1 DER-encoded PKCS#8 private key to the given path
    #[cfg(feature = "std")]
    fn write_pkcs8_der_file(&self, path: impl AsRef<Path>) -> Result<()> {
        Ok(self.to_pkcs8_der()?.write_der_file(path)?)
    }

    /// Write ASN.1 DER-encoded PKCS#8 private key to the given path
    #[cfg(all(feature = "pem", feature = "std"))]
    fn write_pkcs8_pem_file(&self, path: impl AsRef<Path>, line_ending: LineEnding) -> Result<()> {
        let doc = self.to_pkcs8_der()?;
        Ok(doc.write_pem_file(path, PrivateKeyInfo::PEM_LABEL, line_ending)?)
    }
}
