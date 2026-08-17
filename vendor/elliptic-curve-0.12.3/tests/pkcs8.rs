//! PKCS#8 tests

#![cfg(all(feature = "dev", feature = "pkcs8"))]

use elliptic_curve::{
    dev::{PublicKey, SecretKey},
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey},
    sec1::ToEncodedPoint,
};
use hex_literal::hex;

/// DER-encoded PKCS#8 public key
const PKCS8_PUBLIC_KEY_DER: &[u8; 91] = include_bytes!("examples/pkcs8-public-key.der");

/// PEM-encoded PKCS#8 public key
#[cfg(feature = "pem")]
const PKCS8_PUBLIC_KEY_PEM: &str = include_str!("examples/pkcs8-public-key.pem");

/// Example encoded scalar value
const EXAMPLE_SCALAR: [u8; 32] =
    hex!("AABBCCDDEEFF0000000000000000000000000000000000000000000000000001");

/// Example PKCS#8 private key
fn example_private_key() -> der::SecretDocument {
    SecretKey::from_be_bytes(&EXAMPLE_SCALAR)
        .unwrap()
        .to_pkcs8_der()
        .unwrap()
}

#[test]
fn decode_pkcs8_private_key_from_der() {
    let secret_key = SecretKey::from_pkcs8_der(example_private_key().as_bytes()).unwrap();
    assert_eq!(secret_key.to_be_bytes().as_slice(), &EXAMPLE_SCALAR);
}

#[test]
fn decode_pkcs8_public_key_from_der() {
    let public_key = PublicKey::from_public_key_der(&PKCS8_PUBLIC_KEY_DER[..]).unwrap();
    let expected_sec1_point = hex!("041CACFFB55F2F2CEFD89D89EB374B2681152452802DEEA09916068137D839CF7FC481A44492304D7EF66AC117BEFE83A8D08F155F2B52F9F618DD447029048E0F");
    assert_eq!(
        public_key.to_encoded_point(false).as_bytes(),
        &expected_sec1_point[..]
    );
}

#[test]
#[cfg(feature = "pem")]
fn decode_pkcs8_public_key_from_pem() {
    let public_key = PKCS8_PUBLIC_KEY_PEM.parse::<PublicKey>().unwrap();

    // Ensure key parses equivalently to DER
    let der_key = PublicKey::from_public_key_der(&PKCS8_PUBLIC_KEY_DER[..]).unwrap();
    assert_eq!(public_key, der_key);
}
