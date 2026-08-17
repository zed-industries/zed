//! PEM decoding tests

#![cfg(feature = "alloc")]

use pem_rfc7468::LineEnding;

#[test]
fn pkcs1_example() {
    let label = "RSA PRIVATE KEY";
    let bytes = include_bytes!("examples/pkcs1.der");
    let encoded = pem_rfc7468::encode_string(label, LineEnding::LF, bytes).unwrap();
    assert_eq!(&encoded, include_str!("examples/pkcs1.pem"));
}

#[test]
fn pkcs8_example() {
    let label = "PRIVATE KEY";
    let bytes = include_bytes!("examples/pkcs8.der");
    let encoded = pem_rfc7468::encode_string(label, LineEnding::LF, bytes).unwrap();
    assert_eq!(&encoded, include_str!("examples/pkcs8.pem"));
}
