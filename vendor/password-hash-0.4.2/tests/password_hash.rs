//! Tests for `PasswordHash` encoding/decoding.
//!
//! Each test implements a different permutation of the possible combinations
//! of the string encoding, and ensures password hashes round trip under each
//! of the conditions.

use password_hash::{Ident, ParamsString, PasswordHash, Salt};

const EXAMPLE_ALGORITHM: Ident = Ident::new_unwrap("argon2d");
const EXAMPLE_SALT: &str = "saltsaltsaltsaltsalt";
const EXAMPLE_HASH: &[u8] = &[
    0x85, 0xab, 0x21, 0x85, 0xab, 0x21, 0x85, 0xab, 0x21, 0x85, 0xab, 0x21, 0x85, 0xab, 0x21, 0x85,
    0xab, 0x21, 0x85, 0xab, 0x21, 0x85, 0xab, 0x21, 0x85, 0xab, 0x21, 0x85, 0xab, 0x21, 0x85, 0xab,
];

/// Example parameters
fn example_params() -> ParamsString {
    let mut params = ParamsString::new();
    params.add_decimal("a", 1).unwrap();
    params.add_decimal("b", 2).unwrap();
    params.add_decimal("c", 3).unwrap();
    params
}

#[test]
fn algorithm_alone() {
    let ph = PasswordHash::new("$argon2d").unwrap();
    assert_eq!(ph.algorithm, EXAMPLE_ALGORITHM);

    let s = ph.to_string();
    assert_eq!(s, "$argon2d");

    let ph2 = PasswordHash::try_from(s.as_str()).unwrap();
    assert_eq!(ph, ph2);
}

#[test]
fn params() {
    let ph = PasswordHash {
        algorithm: EXAMPLE_ALGORITHM,
        version: None,
        params: example_params(),
        salt: None,
        hash: None,
    };

    let s = ph.to_string();
    assert_eq!(s, "$argon2d$a=1,b=2,c=3");

    let ph2 = PasswordHash::try_from(s.as_str()).unwrap();
    assert_eq!(ph, ph2);
}

#[test]
fn salt() {
    let ph = PasswordHash {
        algorithm: EXAMPLE_ALGORITHM,
        version: None,
        params: ParamsString::new(),
        salt: Some(Salt::new(EXAMPLE_SALT).unwrap()),
        hash: None,
    };

    let s = ph.to_string();
    assert_eq!(s, "$argon2d$saltsaltsaltsaltsalt");

    let ph2 = PasswordHash::try_from(s.as_str()).unwrap();
    assert_eq!(ph, ph2);
}

#[test]
fn one_param_and_salt() {
    let mut params = ParamsString::new();
    params.add_decimal("a", 1).unwrap();

    let ph = PasswordHash {
        algorithm: EXAMPLE_ALGORITHM,
        version: None,
        params,
        salt: Some(Salt::new(EXAMPLE_SALT).unwrap()),
        hash: None,
    };

    let s = ph.to_string();
    assert_eq!(s, "$argon2d$a=1$saltsaltsaltsaltsalt");

    let ph2 = PasswordHash::try_from(s.as_str()).unwrap();
    assert_eq!(ph, ph2);
}

#[test]
fn params_and_salt() {
    let ph = PasswordHash {
        algorithm: EXAMPLE_ALGORITHM,
        version: None,
        params: example_params(),
        salt: Some(Salt::new(EXAMPLE_SALT).unwrap()),
        hash: None,
    };

    let s = ph.to_string();
    assert_eq!(s, "$argon2d$a=1,b=2,c=3$saltsaltsaltsaltsalt");

    let ph2 = PasswordHash::try_from(s.as_str()).unwrap();
    assert_eq!(ph, ph2);
}

#[test]
fn salt_and_hash() {
    let ph = PasswordHash {
        algorithm: EXAMPLE_ALGORITHM,
        version: None,
        params: ParamsString::default(),
        salt: Some(Salt::new(EXAMPLE_SALT).unwrap()),
        hash: Some(EXAMPLE_HASH.try_into().unwrap()),
    };

    let s = ph.to_string();
    assert_eq!(
        s,
        "$argon2d$saltsaltsaltsaltsalt$hashhashhashhashhashhashhashhashhashhashhas"
    );

    let ph2 = PasswordHash::try_from(s.as_str()).unwrap();
    assert_eq!(ph, ph2);
}

#[test]
fn all_fields() {
    let ph = PasswordHash {
        algorithm: EXAMPLE_ALGORITHM,
        version: None,
        params: example_params(),
        salt: Some(Salt::new(EXAMPLE_SALT).unwrap()),
        hash: Some(EXAMPLE_HASH.try_into().unwrap()),
    };

    let s = ph.to_string();
    assert_eq!(
        s,
        "$argon2d$a=1,b=2,c=3$saltsaltsaltsaltsalt$hashhashhashhashhashhashhashhashhashhashhas"
    );

    let ph2 = PasswordHash::try_from(s.as_str()).unwrap();
    assert_eq!(ph, ph2);
}
