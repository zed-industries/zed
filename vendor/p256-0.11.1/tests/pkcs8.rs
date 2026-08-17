//! PKCS#8 tests

#![cfg(feature = "pkcs8")]

use hex_literal::hex;
use p256::{
    elliptic_curve::sec1::ToEncodedPoint,
    pkcs8::{DecodePrivateKey, DecodePublicKey},
};

#[cfg(feature = "pem")]
use p256::elliptic_curve::pkcs8::{EncodePrivateKey, EncodePublicKey};

/// DER-encoded PKCS#8 private key
const PKCS8_PRIVATE_KEY_DER: &[u8; 138] = include_bytes!("examples/pkcs8-private-key.der");

/// DER-encoded PKCS#8 public key
const PKCS8_PUBLIC_KEY_DER: &[u8; 91] = include_bytes!("examples/pkcs8-public-key.der");

/// PEM-encoded PKCS#8 private key
#[cfg(feature = "pem")]
const PKCS8_PRIVATE_KEY_PEM: &str = include_str!("examples/pkcs8-private-key.pem");

/// PEM-encoded PKCS#8 public key
#[cfg(feature = "pem")]
const PKCS8_PUBLIC_KEY_PEM: &str = include_str!("examples/pkcs8-public-key.pem");

#[test]
fn decode_pkcs8_private_key_from_der() {
    let secret_key = p256::SecretKey::from_pkcs8_der(&PKCS8_PRIVATE_KEY_DER[..]).unwrap();
    let expected_scalar = hex!("69624171561A63340DE0E7D869F2A05492558E1A04868B6A9F854A866788188D");
    assert_eq!(secret_key.to_be_bytes().as_slice(), &expected_scalar[..]);
}

#[test]
fn decode_pkcs8_public_key_from_der() {
    let public_key = p256::PublicKey::from_public_key_der(&PKCS8_PUBLIC_KEY_DER[..]).unwrap();
    let expected_sec1_point = hex!("041CACFFB55F2F2CEFD89D89EB374B2681152452802DEEA09916068137D839CF7FC481A44492304D7EF66AC117BEFE83A8D08F155F2B52F9F618DD447029048E0F");
    assert_eq!(
        public_key.to_encoded_point(false).as_bytes(),
        &expected_sec1_point[..]
    );
}

#[test]
#[cfg(feature = "pem")]
fn decode_pkcs8_private_key_from_pem() {
    let secret_key = PKCS8_PRIVATE_KEY_PEM.parse::<p256::SecretKey>().unwrap();

    // Ensure key parses equivalently to DER
    let der_key = p256::SecretKey::from_pkcs8_der(&PKCS8_PRIVATE_KEY_DER[..]).unwrap();
    assert_eq!(secret_key.to_be_bytes(), der_key.to_be_bytes());
}

#[test]
#[cfg(feature = "pem")]
fn decode_pkcs8_public_key_from_pem() {
    let public_key = PKCS8_PUBLIC_KEY_PEM.parse::<p256::PublicKey>().unwrap();

    // Ensure key parses equivalently to DER
    let der_key = p256::PublicKey::from_public_key_der(&PKCS8_PUBLIC_KEY_DER[..]).unwrap();
    assert_eq!(public_key, der_key);
}

#[test]
#[cfg(feature = "pem")]
fn encode_pkcs8_private_key_to_der() {
    let original_secret_key = p256::SecretKey::from_pkcs8_der(&PKCS8_PRIVATE_KEY_DER[..]).unwrap();
    let reencoded_secret_key = original_secret_key.to_pkcs8_der().unwrap();
    assert_eq!(reencoded_secret_key.as_bytes(), &PKCS8_PRIVATE_KEY_DER[..]);
}

#[test]
#[cfg(feature = "pem")]
fn encode_pkcs8_public_key_to_der() {
    let original_public_key =
        p256::PublicKey::from_public_key_der(&PKCS8_PUBLIC_KEY_DER[..]).unwrap();
    let reencoded_public_key = original_public_key.to_public_key_der().unwrap();
    assert_eq!(reencoded_public_key.as_ref(), &PKCS8_PUBLIC_KEY_DER[..]);
}

#[test]
#[cfg(feature = "pem")]
fn encode_pkcs8_private_key_to_pem() {
    let original_secret_key = p256::SecretKey::from_pkcs8_der(&PKCS8_PRIVATE_KEY_DER[..]).unwrap();
    let reencoded_secret_key = original_secret_key
        .to_pkcs8_pem(Default::default())
        .unwrap();
    assert_eq!(reencoded_secret_key.as_str(), PKCS8_PRIVATE_KEY_PEM);
}

#[test]
#[cfg(feature = "pem")]
fn encode_pkcs8_public_key_to_pem() {
    let original_public_key =
        p256::PublicKey::from_public_key_der(&PKCS8_PUBLIC_KEY_DER[..]).unwrap();
    let reencoded_public_key = original_public_key.to_string();
    assert_eq!(reencoded_public_key.as_str(), PKCS8_PUBLIC_KEY_PEM);
}
