//! Base64 encoding tests.
//!
//! # B64 Notes
//!
//! "B64" is a ubset of the standard Base64 encoding (RFC 4648, section 4) which
//! omits padding (`=`) as well as extra whitespace, as described in the PHC
//! string format specification:
//!
//! <https://github.com/P-H-C/phc-string-format/blob/master/phc-sf-spec.md#b64>

use password_hash::{Output, Salt};

// Example salt encoded as a B64 string.
const EXAMPLE_SALT_B64: &str = "REVBREJFRUZERUFEQkVFRg";
const EXAMPLE_SALT_RAW: &[u8] = b"DEADBEEFDEADBEEF";

// Example PHF output encoded as a B64 string.
const EXAMPLE_OUTPUT_B64: &str =
    "REVBREJFRUZERUFEQkVFRkRFQURCRUVGREVBREJFRUZERUFEQkVFRkRFQURCRUVGREVBREJFRUZERUFEQkVFRg";
const EXAMPLE_OUTPUT_RAW: &[u8] =
    b"DEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEF";

#[test]
fn salt_roundtrip() {
    let mut buffer = [0u8; 64];
    let salt = Salt::new(EXAMPLE_SALT_B64).unwrap();
    assert_eq!(salt.as_ref(), EXAMPLE_SALT_B64);

    let salt_decoded = salt.b64_decode(&mut buffer).unwrap();
    assert_eq!(salt_decoded, EXAMPLE_SALT_RAW);
}

#[test]
fn output_roundtrip() {
    let out = EXAMPLE_OUTPUT_B64.parse::<Output>().unwrap();
    assert_eq!(out.as_ref(), EXAMPLE_OUTPUT_RAW);
    assert_eq!(out.to_string(), EXAMPLE_OUTPUT_B64);
}
