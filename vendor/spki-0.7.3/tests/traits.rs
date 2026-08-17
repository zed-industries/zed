//! Tests for SPKI encoding/decoding traits.

#![cfg(any(feature = "pem", feature = "std"))]

use der::{Decode, Encode};
use spki::{DecodePublicKey, Document, EncodePublicKey, Error, Result, SubjectPublicKeyInfoRef};

#[cfg(feature = "pem")]
use spki::der::pem::LineEnding;

#[cfg(feature = "std")]
use tempfile::tempdir;

#[cfg(all(feature = "pem", feature = "std"))]
use std::fs;

/// Ed25519 `SubjectPublicKeyInfo` encoded as ASN.1 DER
const ED25519_DER_EXAMPLE: &[u8] = include_bytes!("examples/ed25519-pub.der");

/// Ed25519 public key encoded as PEM
#[cfg(feature = "pem")]
const ED25519_PEM_EXAMPLE: &str = include_str!("examples/ed25519-pub.pem");

/// Mock key type for testing trait impls against.
pub struct MockKey(Vec<u8>);

impl AsRef<[u8]> for MockKey {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl EncodePublicKey for MockKey {
    fn to_public_key_der(&self) -> Result<Document> {
        Ok(Document::from_der(self.as_ref())?)
    }
}

impl TryFrom<SubjectPublicKeyInfoRef<'_>> for MockKey {
    type Error = Error;

    fn try_from(spki: SubjectPublicKeyInfoRef<'_>) -> Result<MockKey> {
        Ok(MockKey(spki.to_der()?))
    }
}

#[cfg(feature = "pem")]
#[test]
fn from_public_key_pem() {
    let key = MockKey::from_public_key_pem(ED25519_PEM_EXAMPLE).unwrap();
    assert_eq!(key.as_ref(), ED25519_DER_EXAMPLE);
}

#[cfg(feature = "std")]
#[test]
fn read_public_key_der_file() {
    let key = MockKey::read_public_key_der_file("tests/examples/ed25519-pub.der").unwrap();
    assert_eq!(key.as_ref(), ED25519_DER_EXAMPLE);
}

#[cfg(all(feature = "pem", feature = "std"))]
#[test]
fn read_public_key_pem_file() {
    let key = MockKey::read_public_key_pem_file("tests/examples/ed25519-pub.pem").unwrap();
    assert_eq!(key.as_ref(), ED25519_DER_EXAMPLE);
}

#[cfg(feature = "pem")]
#[test]
fn to_public_key_pem() {
    let pem = MockKey(ED25519_DER_EXAMPLE.to_vec())
        .to_public_key_pem(LineEnding::LF)
        .unwrap();

    assert_eq!(pem, ED25519_PEM_EXAMPLE);
}

#[cfg(feature = "std")]
#[test]
fn write_public_key_der_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("example.der");
    MockKey(ED25519_DER_EXAMPLE.to_vec())
        .write_public_key_der_file(&path)
        .unwrap();

    let key = MockKey::read_public_key_der_file(&path).unwrap();
    assert_eq!(key.as_ref(), ED25519_DER_EXAMPLE);
}

#[cfg(all(feature = "pem", feature = "std"))]
#[test]
fn write_public_key_pem_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("example.pem");
    MockKey(ED25519_DER_EXAMPLE.to_vec())
        .write_public_key_pem_file(&path, LineEnding::LF)
        .unwrap();

    let pem = fs::read_to_string(path).unwrap();
    assert_eq!(&pem, ED25519_PEM_EXAMPLE);
}
