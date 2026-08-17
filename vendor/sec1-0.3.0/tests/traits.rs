//! Tests for SEC1 encoding/decoding traits.

#![cfg(any(feature = "pem", feature = "std"))]

use der::SecretDocument;
use sec1::{DecodeEcPrivateKey, EncodeEcPrivateKey, Result};

#[cfg(feature = "pem")]
use sec1::der::pem::LineEnding;

#[cfg(feature = "std")]
use tempfile::tempdir;

#[cfg(all(feature = "pem", feature = "std"))]
use std::fs;

/// SEC1 `EcPrivateKey` encoded as ASN.1 DER
const P256_DER_EXAMPLE: &[u8] = include_bytes!("examples/p256-priv.der");

/// SEC1 `EcPrivateKey` encoded as PEM
#[cfg(feature = "pem")]
const P256_PEM_EXAMPLE: &str = include_str!("examples/p256-priv.pem");

/// Mock private key type for testing trait impls against.
pub struct MockPrivateKey(Vec<u8>);

impl AsRef<[u8]> for MockPrivateKey {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl DecodeEcPrivateKey for MockPrivateKey {
    fn from_sec1_der(bytes: &[u8]) -> Result<MockPrivateKey> {
        Ok(MockPrivateKey(bytes.to_vec()))
    }
}

impl EncodeEcPrivateKey for MockPrivateKey {
    fn to_sec1_der(&self) -> Result<SecretDocument> {
        Ok(SecretDocument::try_from(self.as_ref())?)
    }
}

#[cfg(feature = "pem")]
#[test]
fn from_sec1_pem() {
    let key = MockPrivateKey::from_sec1_pem(P256_PEM_EXAMPLE).unwrap();
    assert_eq!(key.as_ref(), P256_DER_EXAMPLE);
}

#[cfg(feature = "std")]
#[test]
fn read_sec1_der_file() {
    let key = MockPrivateKey::read_sec1_der_file("tests/examples/p256-priv.der").unwrap();
    assert_eq!(key.as_ref(), P256_DER_EXAMPLE);
}

#[cfg(all(feature = "pem", feature = "std"))]
#[test]
fn read_sec1_pem_file() {
    let key = MockPrivateKey::read_sec1_pem_file("tests/examples/p256-priv.pem").unwrap();
    assert_eq!(key.as_ref(), P256_DER_EXAMPLE);
}

#[cfg(feature = "pem")]
#[test]
fn to_sec1_pem() {
    let pem = MockPrivateKey(P256_DER_EXAMPLE.to_vec())
        .to_sec1_pem(LineEnding::LF)
        .unwrap();

    assert_eq!(&*pem, P256_PEM_EXAMPLE);
}

#[cfg(feature = "std")]
#[test]
fn write_sec1_der_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("example.der");
    MockPrivateKey(P256_DER_EXAMPLE.to_vec())
        .write_sec1_der_file(&path)
        .unwrap();

    let key = MockPrivateKey::read_sec1_der_file(&path).unwrap();
    assert_eq!(key.as_ref(), P256_DER_EXAMPLE);
}

#[cfg(all(feature = "pem", feature = "std"))]
#[test]
fn write_sec1_pem_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("example.pem");
    MockPrivateKey(P256_DER_EXAMPLE.to_vec())
        .write_sec1_pem_file(&path, LineEnding::LF)
        .unwrap();

    let pem = fs::read_to_string(path).unwrap();
    assert_eq!(&pem, P256_PEM_EXAMPLE);
}
