//! Traits for parsing objects from SEC1 encoded documents

use crate::Result;

#[cfg(feature = "alloc")]
use der::SecretDocument;

#[cfg(feature = "pem")]
use {crate::LineEnding, alloc::string::String, der::pem::PemLabel};

#[cfg(feature = "pkcs8")]
use {
    crate::{EcPrivateKey, ALGORITHM_OID},
    der::Decode,
};

#[cfg(feature = "std")]
use std::path::Path;

#[cfg(feature = "pem")]
use zeroize::Zeroizing;

/// Parse an [`EcPrivateKey`] from a SEC1-encoded document.
#[cfg_attr(docsrs, doc(cfg(feature = "der")))]
pub trait DecodeEcPrivateKey: Sized {
    /// Deserialize SEC1 private key from ASN.1 DER-encoded data
    /// (binary format).
    fn from_sec1_der(bytes: &[u8]) -> Result<Self>;

    /// Deserialize SEC1-encoded private key from PEM.
    ///
    /// Keys in this format begin with the following:
    ///
    /// ```text
    /// -----BEGIN EC PRIVATE KEY-----
    /// ```
    #[cfg(feature = "pem")]
    #[cfg_attr(docsrs, doc(cfg(feature = "pem")))]
    fn from_sec1_pem(s: &str) -> Result<Self> {
        let (label, doc) = SecretDocument::from_pem(s)?;
        EcPrivateKey::validate_pem_label(label)?;
        Self::from_sec1_der(doc.as_bytes())
    }

    /// Load SEC1 private key from an ASN.1 DER-encoded file on the local
    /// filesystem (binary format).
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    fn read_sec1_der_file(path: impl AsRef<Path>) -> Result<Self> {
        Self::from_sec1_der(SecretDocument::read_der_file(path)?.as_bytes())
    }

    /// Load SEC1 private key from a PEM-encoded file on the local filesystem.
    #[cfg(all(feature = "pem", feature = "std"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "pem")))]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    fn read_sec1_pem_file(path: impl AsRef<Path>) -> Result<Self> {
        let (label, doc) = SecretDocument::read_pem_file(path)?;
        EcPrivateKey::validate_pem_label(&label)?;
        Self::from_sec1_der(doc.as_bytes())
    }
}

/// Serialize a [`EcPrivateKey`] to a SEC1 encoded document.
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(all(feature = "alloc", feature = "der"))))]
pub trait EncodeEcPrivateKey {
    /// Serialize a [`SecretDocument`] containing a SEC1-encoded private key.
    fn to_sec1_der(&self) -> Result<SecretDocument>;

    /// Serialize this private key as PEM-encoded SEC1 with the given [`LineEnding`].
    ///
    /// To use the OS's native line endings, pass `Default::default()`.
    #[cfg(feature = "pem")]
    #[cfg_attr(docsrs, doc(cfg(feature = "pem")))]
    fn to_sec1_pem(&self, line_ending: LineEnding) -> Result<Zeroizing<String>> {
        let doc = self.to_sec1_der()?;
        Ok(doc.to_pem(EcPrivateKey::PEM_LABEL, line_ending)?)
    }

    /// Write ASN.1 DER-encoded SEC1 private key to the given path.
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    fn write_sec1_der_file(&self, path: impl AsRef<Path>) -> Result<()> {
        Ok(self.to_sec1_der()?.write_der_file(path)?)
    }

    /// Write ASN.1 DER-encoded SEC1 private key to the given path.
    #[cfg(all(feature = "pem", feature = "std"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "pem")))]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    fn write_sec1_pem_file(&self, path: impl AsRef<Path>, line_ending: LineEnding) -> Result<()> {
        let doc = self.to_sec1_der()?;
        Ok(doc.write_pem_file(path, EcPrivateKey::PEM_LABEL, line_ending)?)
    }
}

#[cfg(feature = "pkcs8")]
#[cfg_attr(docsrs, doc(cfg(feature = "pkcs8")))]
impl<T: pkcs8::DecodePrivateKey> DecodeEcPrivateKey for T {
    fn from_sec1_der(private_key: &[u8]) -> Result<Self> {
        let params_oid = EcPrivateKey::from_der(private_key)?
            .parameters
            .and_then(|params| params.named_curve());

        let algorithm = pkcs8::AlgorithmIdentifier {
            oid: ALGORITHM_OID,
            parameters: params_oid.as_ref().map(Into::into),
        };

        Ok(Self::try_from(pkcs8::PrivateKeyInfo {
            algorithm,
            private_key,
            public_key: None,
        })?)
    }
}

#[cfg(all(feature = "alloc", feature = "pkcs8"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "alloc", feature = "pkcs8"))))]
impl<T: pkcs8::EncodePrivateKey> EncodeEcPrivateKey for T {
    fn to_sec1_der(&self) -> Result<SecretDocument> {
        let doc = self.to_pkcs8_der()?;
        let pkcs8_key = pkcs8::PrivateKeyInfo::from_der(doc.as_bytes())?;
        pkcs8_key.algorithm.assert_algorithm_oid(ALGORITHM_OID)?;

        let mut pkcs1_key = EcPrivateKey::from_der(pkcs8_key.private_key)?;
        pkcs1_key.parameters = Some(pkcs8_key.algorithm.parameters_oid()?.into());
        pkcs1_key.try_into()
    }
}
