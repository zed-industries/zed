//! Test Cases
#![allow(
    // This allows the tests to be written more uniform and not have to special case the last clone().
    clippy::redundant_clone,
)]

mod utils;

use crate::utils::{check_deserialization, check_error_deserialization, is_equal};
use expect_test::expect;
use serde::{Deserialize, Serialize};
use serde_with::{
    base58::{Base58, Bitcoin, Flickr, Monero, Ripple, Standard},
    serde_as,
};

#[test]
fn base58_vec() {
    let check_equal = vec![vec![0, 1, 2, 13], vec![14, 5, 6, 7]];
    let check_deser = vec![vec![0xaa, 0xbc, 0xff], vec![0xe0, 0x7d]];
    let check_deser_from = r#"["zMFU","J5r"]"#;

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct BDefault(#[serde_as(as = "Vec<Base58>")] Vec<Vec<u8>>);

    is_equal(
        BDefault(check_equal.clone()),
        expect![[r#"
            [
              "1Ldz",
              "MnWp6"
            ]"#]],
    );

    // Check mixed padding deserialization
    check_deserialization(BDefault(check_deser.clone()), check_deser_from);

    check_error_deserialization::<BDefault>(
        r#"["0"]"#,
        expect!["provided string contained invalid character '0' at byte 0 at line 1 column 4"],
    );
    check_error_deserialization::<BDefault>(
        r#"["zz/"]"#,
        expect!["provided string contained invalid character '/' at byte 2 at line 1 column 6"],
    );
}

#[test]
fn base58_different_charsets() {
    let bytes = [
        0x69_u8, 0xb7, 0x1d, 0x79, 0xf8, 0x21, 0x8a, 0x39, 0x25, 0x9a, 0x7a, 0x29, 0xaa, 0xbb,
        0x2d, 0xba, 0xfc, 0x31, 0xcb, 0x30, 0x01, 0x08, 0x31, 0x05, 0x18, 0x72, 0x09, 0x28, 0xb3,
        0x0d, 0x38, 0xf4, 0x11, 0x49, 0x35, 0x15, 0x59, 0x76, 0x19, 0xd3, 0x5d, 0xb7, 0xe3, 0x9e,
        0xbb, 0xf3, 0xdf, 0xbf, 0x00,
    ];

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct B58Standard(#[serde_as(as = "Base58<Standard>")] Vec<u8>);

    is_equal(
        B58Standard(bytes.to_vec()),
        expect![[r#""J7k4TCzq3PosUrUEF8e6YEeDpsCLXAibdXjiLDeSvLUFV2KDwjzWsCeWG8EQtGuBJYf""#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct B58Bitcoin(#[serde_as(as = "Base58<Bitcoin>")] Vec<u8>);

    is_equal(
        B58Bitcoin(bytes.to_vec()),
        expect![[r#""J7k4TCzq3PosUrUEF8e6YEeDpsCLXAibdXjiLDeSvLUFV2KDwjzWsCeWG8EQtGuBJYf""#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct B58Flickr(#[serde_as(as = "Base58<Flickr>")] Vec<u8>);

    is_equal(
        B58Flickr(bytes.to_vec()),
        expect![[r#""i7K4scZQ3oNStRtef8D6xeDdPSckwaHACwJHkdDrVktfu2jdWJZvScDvg8epTgUbixE""#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct B58Monero(#[serde_as(as = "Base58<Monero>")] Vec<u8>);

    is_equal(
        B58Monero(bytes.to_vec()),
        expect![[r#""J7k4TCzq3PosUrUEF8e6YEeDpsCLXAibdXjiLDeSvLUFV2KDwjzWsCeWG8EQtGuBJYf""#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct B58Ripple(#[serde_as(as = "Base58<Ripple>")] Vec<u8>);

    is_equal(
        B58Ripple(bytes.to_vec()),
        expect![[r#""JfkhTUzqsPo17i7NE3eaYNeDF1ULXw5bdXj5LDeSvL7EVpKDAjzW1UeWG3NQtGuBJYC""#]],
    );
}
