//! Tests for PKCS#1 encoding/decoding traits.

#![cfg(any(feature = "pem", feature = "std"))]

use der::SecretDocument;
use pkcs1::{DecodeRsaPrivateKey, EncodeRsaPrivateKey, Result};

#[cfg(feature = "pem")]
use pkcs1::der::pem::LineEnding;

#[cfg(feature = "std")]
use tempfile::tempdir;

#[cfg(all(feature = "pem", feature = "std"))]
use std::fs;

/// PKCS#1 `RsaPrivateKey` encoded as ASN.1 DER
const RSA_2048_PRIV_DER_EXAMPLE: &[u8] = include_bytes!("examples/rsa2048-priv.der");

/// PKCS#1 `RsaPrivateKey` encoded as PEM
#[cfg(feature = "pem")]
const RSA_2048_PRIV_PEM_EXAMPLE: &str = include_str!("examples/rsa2048-priv.pem");

/// Mock RSA private key type for testing trait impls against.
pub struct MockPrivateKey(Vec<u8>);

impl AsRef<[u8]> for MockPrivateKey {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl DecodeRsaPrivateKey for MockPrivateKey {
    fn from_pkcs1_der(bytes: &[u8]) -> Result<MockPrivateKey> {
        Ok(MockPrivateKey(bytes.to_vec()))
    }
}

impl EncodeRsaPrivateKey for MockPrivateKey {
    fn to_pkcs1_der(&self) -> Result<SecretDocument> {
        Ok(SecretDocument::try_from(self.as_ref())?)
    }
}

#[cfg(feature = "pem")]
#[test]
fn from_pkcs1_pem() {
    let key = MockPrivateKey::from_pkcs1_pem(RSA_2048_PRIV_PEM_EXAMPLE).unwrap();
    assert_eq!(key.as_ref(), RSA_2048_PRIV_DER_EXAMPLE);
}

#[cfg(feature = "std")]
#[test]
fn read_pkcs1_der_file() {
    let key = MockPrivateKey::read_pkcs1_der_file("tests/examples/rsa2048-priv.der").unwrap();
    assert_eq!(key.as_ref(), RSA_2048_PRIV_DER_EXAMPLE);
}

#[cfg(all(feature = "pem", feature = "std"))]
#[test]
fn read_pkcs1_pem_file() {
    let key = MockPrivateKey::read_pkcs1_pem_file("tests/examples/rsa2048-priv.pem").unwrap();
    assert_eq!(key.as_ref(), RSA_2048_PRIV_DER_EXAMPLE);
}

#[cfg(feature = "pem")]
#[test]
fn to_pkcs1_pem() {
    let pem = MockPrivateKey(RSA_2048_PRIV_DER_EXAMPLE.to_vec())
        .to_pkcs1_pem(LineEnding::LF)
        .unwrap();

    assert_eq!(&*pem, RSA_2048_PRIV_PEM_EXAMPLE);
}

#[cfg(feature = "std")]
#[test]
fn write_pkcs1_der_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("example.der");
    MockPrivateKey(RSA_2048_PRIV_DER_EXAMPLE.to_vec())
        .write_pkcs1_der_file(&path)
        .unwrap();

    let key = MockPrivateKey::read_pkcs1_der_file(&path).unwrap();
    assert_eq!(key.as_ref(), RSA_2048_PRIV_DER_EXAMPLE);
}

#[cfg(all(feature = "pem", feature = "std"))]
#[test]
fn write_pkcs1_pem_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("example.pem");
    MockPrivateKey(RSA_2048_PRIV_DER_EXAMPLE.to_vec())
        .write_pkcs1_pem_file(&path, LineEnding::LF)
        .unwrap();

    let pem = fs::read_to_string(path).unwrap();
    assert_eq!(&pem, RSA_2048_PRIV_PEM_EXAMPLE);
}
