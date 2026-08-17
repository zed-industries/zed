// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#![cfg(debug_assertions)]

use crate::cmac::{sign, verify, Key, AES_128, AES_192, AES_256, DES_EDE3_FOR_LEGACY_USE_ONLY};
use crate::fips::{assert_fips_status_indicator, FipsServiceStatus};
use crate::rand::{self, SystemRandom};

const TEST_MESSAGE: &str = "test message";

macro_rules! cmac_api {
    ($name:ident, $alg:expr, $key_len:expr, $expect:path) => {
        #[test]
        fn $name() -> Result<(), Box<dyn std::error::Error>> {
            let rng = SystemRandom::new();

            let key_value: [u8; $key_len] = rand::generate(&rng).unwrap().expose();

            let s_key = Key::new($alg, key_value.as_ref()).unwrap();

            let tag =
                assert_fips_status_indicator!(sign(&s_key, TEST_MESSAGE.as_bytes())?, $expect);

            let v_key = Key::new($alg, key_value.as_ref()).unwrap();

            assert_fips_status_indicator!(
                verify(&v_key, TEST_MESSAGE.as_bytes(), tag.as_ref())?,
                $expect
            );

            Ok(())
        }
    };
}

cmac_api!(aes_128, AES_128, 16, FipsServiceStatus::Approved);
cmac_api!(aes_192, AES_192, 24, FipsServiceStatus::NonApproved);
cmac_api!(aes_256, AES_256, 32, FipsServiceStatus::Approved);
cmac_api!(
    des_ede3,
    DES_EDE3_FOR_LEGACY_USE_ONLY,
    24,
    FipsServiceStatus::NonApproved
);
