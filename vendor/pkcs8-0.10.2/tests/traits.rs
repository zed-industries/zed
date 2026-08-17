//! Tests for PKCS#8 encoding/decoding traits.

#![cfg(any(feature = "pem", feature = "std"))]

use der::Encode;
use pkcs8::{DecodePrivateKey, EncodePrivateKey, Error, PrivateKeyInfo, Result, SecretDocument};

#[cfg(feature = "pem")]
use pkcs8::der::pem::LineEnding;

#[cfg(feature = "std")]
use tempfile::tempdir;

#[cfg(all(feature = "pem", feature = "std"))]
use std::fs;

/// Ed25519 `PrivateKeyInfo` encoded as ASN.1 DER
const ED25519_DER_EXAMPLE: &[u8] = include_bytes!("examples/ed25519-priv-pkcs8v1.der");

/// Ed25519 private key encoded as PEM
#[cfg(feature = "pem")]
const ED25519_PEM_EXAMPLE: &str = include_str!("examples/ed25519-priv-pkcs8v1.pem");

/// Mock key type for testing trait impls against.
pub struct MockKey(Vec<u8>);

impl AsRef<[u8]> for MockKey {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl EncodePrivateKey for MockKey {
    fn to_pkcs8_der(&self) -> Result<SecretDocument> {
        Ok(SecretDocument::try_from(self.as_ref())?)
    }
}

impl TryFrom<PrivateKeyInfo<'_>> for MockKey {
    type Error = Error;

    fn try_from(pkcs8: PrivateKeyInfo<'_>) -> Result<MockKey> {
        Ok(MockKey(pkcs8.to_der()?))
    }
}

#[cfg(feature = "pem")]
#[test]
fn from_pkcs8_pem() {
    let key = MockKey::from_pkcs8_pem(ED25519_PEM_EXAMPLE).unwrap();
    assert_eq!(key.as_ref(), ED25519_DER_EXAMPLE);
}

#[cfg(feature = "std")]
#[test]
fn read_pkcs8_der_file() {
    let key = MockKey::read_pkcs8_der_file("tests/examples/ed25519-priv-pkcs8v1.der").unwrap();
    assert_eq!(key.as_ref(), ED25519_DER_EXAMPLE);
}

#[cfg(all(feature = "pem", feature = "std"))]
#[test]
fn read_pkcs8_pem_file() {
    let key = MockKey::read_pkcs8_pem_file("tests/examples/ed25519-priv-pkcs8v1.pem").unwrap();
    assert_eq!(key.as_ref(), ED25519_DER_EXAMPLE);
}

#[cfg(feature = "pem")]
#[test]
fn to_pkcs8_pem() {
    let pem = MockKey(ED25519_DER_EXAMPLE.to_vec())
        .to_pkcs8_pem(LineEnding::LF)
        .unwrap();

    assert_eq!(&*pem, ED25519_PEM_EXAMPLE);
}

#[cfg(feature = "std")]
#[test]
fn write_pkcs8_der_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("example.der");
    MockKey(ED25519_DER_EXAMPLE.to_vec())
        .write_pkcs8_der_file(&path)
        .unwrap();

    let key = MockKey::read_pkcs8_der_file(&path).unwrap();
    assert_eq!(key.as_ref(), ED25519_DER_EXAMPLE);
}

#[cfg(all(feature = "pem", feature = "std"))]
#[test]
fn write_pkcs8_pem_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("example.pem");
    MockKey(ED25519_DER_EXAMPLE.to_vec())
        .write_pkcs8_pem_file(&path, LineEnding::LF)
        .unwrap();

    let pem = fs::read_to_string(path).unwrap();
    assert_eq!(&pem, ED25519_PEM_EXAMPLE);
}
