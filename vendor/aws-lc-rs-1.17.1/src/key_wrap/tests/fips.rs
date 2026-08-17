// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#![cfg(debug_assertions)]

use crate::fips::{assert_fips_status_indicator, FipsServiceStatus};
use crate::key_wrap::{AesKek, KeyWrap, KeyWrapPadded, AES_128, AES_256};

const K_128: &[u8] = &[
    0x60, 0x43, 0xb2, 0x73, 0xe9, 0x71, 0x26, 0x5e, 0x53, 0x8a, 0x6c, 0xcd, 0x5d, 0x5a, 0x11, 0xe4,
];

const K_256: &[u8] = &[
    0x15, 0x52, 0x45, 0x0c, 0x60, 0xf3, 0x10, 0xfb, 0xc8, 0x41, 0x98, 0xe5, 0xfd, 0x70, 0x7d, 0x04,
    0x8f, 0x81, 0xbf, 0x9a, 0xdc, 0x63, 0x90, 0xed, 0xe5, 0xb0, 0x4b, 0x3c, 0xe4, 0x06, 0x54, 0xba,
];

const P: &[u8] = &[
    0xf2, 0x64, 0x5b, 0xa4, 0xba, 0xed, 0xa7, 0xec, 0xbc, 0x12, 0xa6, 0xad, 0x46, 0x76, 0x95, 0xa0,
];

macro_rules! nist_aes_key_wrap_test {
    ($name:ident, $alg:expr, $key:expr, $plaintext:expr) => {
        #[test]
        fn $name() {
            let k = $key;
            let p = $plaintext;

            let kek = AesKek::new($alg, k).expect("key creation successful");

            let mut output = vec![0u8; p.len() + 15];

            let wrapped = Vec::from(assert_fips_status_indicator!(
                kek.wrap(P, &mut output).expect("wrap successful"),
                FipsServiceStatus::Approved
            ));

            let kek = AesKek::new($alg, k).expect("key creation successful");

            let mut output = vec![
                0u8;
                if p.len() % 8 != 0 {
                    p.len() + (8 - (p.len() % 8))
                } else {
                    p.len()
                }
            ];

            let _unwrapped = assert_fips_status_indicator!(
                kek.unwrap(&wrapped, &mut output).expect("wrap successful"),
                FipsServiceStatus::Approved
            );
        }
    };
}

macro_rules! nist_aes_key_wrap_with_padding_test {
    ($name:ident, $alg:expr, $key:expr, $plaintext:expr) => {
        #[test]
        fn $name() {
            let k = $key;
            let p = $plaintext;

            let kek = AesKek::new($alg, k).expect("key creation successful");

            let mut output = vec![0u8; p.len() + 15];

            let wrapped = Vec::from(assert_fips_status_indicator!(
                kek.wrap_with_padding(P, &mut output)
                    .expect("wrap successful"),
                FipsServiceStatus::Approved
            ));

            let kek = AesKek::new($alg, k).expect("key creation successful");

            let mut output = vec![
                0u8;
                if p.len() % 8 != 0 {
                    p.len() + (8 - (p.len() % 8))
                } else {
                    p.len()
                }
            ];

            let _unwrapped = assert_fips_status_indicator!(
                kek.unwrap_with_padding(&wrapped, &mut output)
                    .expect("wrap successful"),
                FipsServiceStatus::Approved
            );
        }
    };
}

nist_aes_key_wrap_with_padding_test!(kwp_aes128, &AES_128, K_128, P);
nist_aes_key_wrap_test!(kw_aes128, &AES_128, K_128, P);
nist_aes_key_wrap_with_padding_test!(kwp_aes256, &AES_256, K_256, P);
nist_aes_key_wrap_test!(kw_aes256, &AES_256, K_256, P);
