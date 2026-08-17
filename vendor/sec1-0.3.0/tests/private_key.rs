//! SEC1 private key tests

#![cfg(feature = "der")]

use der::asn1::ObjectIdentifier;
use hex_literal::hex;
use sec1::{EcParameters, EcPrivateKey};

/// NIST P-256 SEC1 private key encoded as ASN.1 DER.
///
/// Note: this key is extracted from the corresponding `p256-priv.der`
/// example key in the `pkcs8` crate.
const P256_DER_EXAMPLE: &[u8] = include_bytes!("examples/p256-priv.der");

#[test]
fn decode_p256_der() {
    let key = EcPrivateKey::try_from(P256_DER_EXAMPLE).unwrap();

    // Extracted using:
    // $ openssl asn1parse -in tests/examples/p256-priv.pem
    assert_eq!(
        key.private_key,
        hex!("69624171561A63340DE0E7D869F2A05492558E1A04868B6A9F854A866788188D")
    );
    assert_eq!(
        key.parameters,
        Some(EcParameters::NamedCurve(
            ObjectIdentifier::new("1.2.840.10045.3.1.7").unwrap()
        ))
    );
    assert_eq!(key.public_key, Some(hex!("041CACFFB55F2F2CEFD89D89EB374B2681152452802DEEA09916068137D839CF7FC481A44492304D7EF66AC117BEFE83A8D08F155F2B52F9F618DD447029048E0F").as_ref()));
}
