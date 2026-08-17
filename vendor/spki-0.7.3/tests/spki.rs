//! `SubjectPublicKeyInfo` tests.

use der::asn1::ObjectIdentifier;
use hex_literal::hex;
use spki::SubjectPublicKeyInfoRef;

#[cfg(feature = "alloc")]
use der::Encode;

#[cfg(feature = "pem")]
use der::{pem::LineEnding, EncodePem};

/// Elliptic Curve (P-256) `SubjectPublicKeyInfo` encoded as ASN.1 DER
const EC_P256_DER_EXAMPLE: &[u8] = include_bytes!("examples/p256-pub.der");

/// Ed25519 `SubjectPublicKeyInfo` encoded as ASN.1 DER
#[cfg(any(feature = "alloc", feature = "fingerprint"))]
const ED25519_DER_EXAMPLE: &[u8] = include_bytes!("examples/ed25519-pub.der");

/// RSA-2048 `SubjectPublicKeyInfo` encoded as ASN.1 DER
const RSA_2048_DER_EXAMPLE: &[u8] = include_bytes!("examples/rsa2048-pub.der");

/// Elliptic Curve (P-256) public key encoded as PEM
#[cfg(feature = "pem")]
const EC_P256_PEM_EXAMPLE: &str = include_str!("examples/p256-pub.pem");

/// Ed25519 public key encoded as PEM
#[cfg(feature = "pem")]
const ED25519_PEM_EXAMPLE: &str = include_str!("examples/ed25519-pub.pem");

/// RSA-2048 PKCS#8 public key encoded as PEM
#[cfg(feature = "pem")]
const RSA_2048_PEM_EXAMPLE: &str = include_str!("examples/rsa2048-pub.pem");

/// The SPKI fingerprint for `ED25519_SPKI_FINGERPRINT` as a Base64 string
///
/// Generated using `cat ed25519-pub.der | openssl dgst -binary -sha256 | base64`
#[cfg(all(feature = "alloc", feature = "base64", feature = "fingerprint"))]
const ED25519_SPKI_FINGERPRINT_BASE64: &str = "Vd1MdLDkhTTi9OFzzs61DfjyenrCqomRzHrpFOAwvO0=";

/// The SPKI fingerprint for `ED25519_SPKI_FINGERPRINT` as straight hash bytes
///
/// Generated using `cat ed25519-pub.der | openssl dgst -sha256`
#[cfg(feature = "fingerprint")]
const ED25519_SPKI_FINGERPRINT: &[u8] =
    &hex!("55dd4c74b0e48534e2f4e173ceceb50df8f27a7ac2aa8991cc7ae914e030bced");

#[test]
fn decode_ec_p256_der() {
    let spki = SubjectPublicKeyInfoRef::try_from(EC_P256_DER_EXAMPLE).unwrap();

    assert_eq!(spki.algorithm.oid, "1.2.840.10045.2.1".parse().unwrap());

    assert_eq!(
        spki.algorithm
            .parameters
            .unwrap()
            .decode_as::<ObjectIdentifier>()
            .unwrap(),
        "1.2.840.10045.3.1.7".parse().unwrap()
    );

    assert_eq!(spki.subject_public_key.raw_bytes(), &hex!("041CACFFB55F2F2CEFD89D89EB374B2681152452802DEEA09916068137D839CF7FC481A44492304D7EF66AC117BEFE83A8D08F155F2B52F9F618DD447029048E0F")[..]);
}

#[test]
#[cfg(feature = "fingerprint")]
fn decode_ed25519_and_fingerprint_spki() {
    // Repeat the decode test from the pkcs8 crate
    let spki = SubjectPublicKeyInfoRef::try_from(ED25519_DER_EXAMPLE).unwrap();

    assert_eq!(spki.algorithm.oid, "1.3.101.112".parse().unwrap());
    assert_eq!(spki.algorithm.parameters, None);
    assert_eq!(
        spki.subject_public_key.raw_bytes(),
        &hex!("4D29167F3F1912A6F7ADFA293A051A15C05EC67B8F17267B1C5550DCE853BD0D")[..]
    );

    // Check the fingerprint
    assert_eq!(
        spki.fingerprint_bytes().unwrap().as_slice(),
        ED25519_SPKI_FINGERPRINT
    );
}

#[test]
#[cfg(all(feature = "alloc", feature = "base64", feature = "fingerprint"))]
fn decode_ed25519_and_fingerprint_base64() {
    // Repeat the decode test from the pkcs8 crate
    let spki = SubjectPublicKeyInfoRef::try_from(ED25519_DER_EXAMPLE).unwrap();

    assert_eq!(spki.algorithm.oid, "1.3.101.112".parse().unwrap());
    assert_eq!(spki.algorithm.parameters, None);
    assert_eq!(
        spki.subject_public_key.raw_bytes(),
        &hex!("4D29167F3F1912A6F7ADFA293A051A15C05EC67B8F17267B1C5550DCE853BD0D")[..]
    );

    // Check the fingerprint
    assert_eq!(
        spki.fingerprint_base64().unwrap(),
        ED25519_SPKI_FINGERPRINT_BASE64
    );
}

#[test]
fn decode_rsa_2048_der() {
    let spki = SubjectPublicKeyInfoRef::try_from(RSA_2048_DER_EXAMPLE).unwrap();

    assert_eq!(spki.algorithm.oid, "1.2.840.113549.1.1.1".parse().unwrap());
    assert!(spki.algorithm.parameters.unwrap().is_null());
    assert_eq!(spki.subject_public_key.raw_bytes(), &hex!("3082010A0282010100B6C42C515F10A6AAF282C63EDBE24243A170F3FA2633BD4833637F47CA4F6F36E03A5D29EFC3191AC80F390D874B39E30F414FCEC1FCA0ED81E547EDC2CD382C76F61C9018973DB9FA537972A7C701F6B77E0982DFC15FC01927EE5E7CD94B4F599FF07013A7C8281BDF22DCBC9AD7CABB7C4311C982F58EDB7213AD4558B332266D743AED8192D1884CADB8B14739A8DADA66DC970806D9C7AC450CB13D0D7C575FB198534FC61BC41BC0F0574E0E0130C7BBBFBDFDC9F6A6E2E3E2AFF1CBEAC89BA57884528D55CFB08327A1E8C89F4E003CF2888E933241D9D695BCBBACDC90B44E3E095FA37058EA25B13F5E295CBEAC6DE838AB8C50AF61E298975B872F0203010001")[..]);
}

#[test]
#[cfg(feature = "alloc")]
fn encode_ec_p256_der() {
    let pk = SubjectPublicKeyInfoRef::try_from(EC_P256_DER_EXAMPLE).unwrap();
    let pk_encoded = pk.to_der().unwrap();
    assert_eq!(EC_P256_DER_EXAMPLE, pk_encoded.as_slice());
}

#[test]
#[cfg(feature = "alloc")]
fn encode_ed25519_der() {
    let pk = SubjectPublicKeyInfoRef::try_from(ED25519_DER_EXAMPLE).unwrap();
    let pk_encoded = pk.to_der().unwrap();
    assert_eq!(ED25519_DER_EXAMPLE, pk_encoded.as_slice());
}

#[test]
#[cfg(feature = "alloc")]
fn encode_rsa_2048_der() {
    let pk = SubjectPublicKeyInfoRef::try_from(RSA_2048_DER_EXAMPLE).unwrap();
    let pk_encoded = pk.to_der().unwrap();
    assert_eq!(RSA_2048_DER_EXAMPLE, pk_encoded.as_slice());
}

#[test]
#[cfg(feature = "pem")]
fn encode_ec_p256_pem() {
    let pk = SubjectPublicKeyInfoRef::try_from(EC_P256_DER_EXAMPLE).unwrap();
    let pk_encoded = pk.to_pem(LineEnding::LF).unwrap();
    assert_eq!(EC_P256_PEM_EXAMPLE, pk_encoded);
}

#[test]
#[cfg(feature = "pem")]
fn encode_ed25519_pem() {
    let pk = SubjectPublicKeyInfoRef::try_from(ED25519_DER_EXAMPLE).unwrap();
    let pk_encoded = pk.to_pem(LineEnding::LF).unwrap();
    assert_eq!(ED25519_PEM_EXAMPLE, pk_encoded);
}

#[test]
#[cfg(feature = "pem")]
fn encode_rsa_2048_pem() {
    let pk = SubjectPublicKeyInfoRef::try_from(RSA_2048_DER_EXAMPLE).unwrap();
    let pk_encoded = pk.to_pem(LineEnding::LF).unwrap();
    assert_eq!(RSA_2048_PEM_EXAMPLE, pk_encoded);
}
