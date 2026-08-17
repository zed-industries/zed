// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#![cfg(debug_assertions)]

use core::num::NonZeroU32;

use crate::fips::{assert_fips_status_indicator, FipsServiceStatus};
use crate::pbkdf2::{
    derive, verify, PBKDF2_HMAC_SHA1, PBKDF2_HMAC_SHA256, PBKDF2_HMAC_SHA384, PBKDF2_HMAC_SHA512,
};

macro_rules! pbkdf2_api {
    ($name:ident, $alg:expr, $secret_len:literal, $salt_len:literal, $iterations:literal, $expect:path) => {
        #[test]
        fn $name() {
            // secret len >= 14 for fips indicator
            let secret = vec![42u8; $secret_len];

            // salt len >= 16 for fips indicator
            let salt = vec![42u8; $salt_len];

            let mut out = vec![0u8; 1024];

            // iterations >= 1000
            let iterations = NonZeroU32::new($iterations).unwrap();

            assert_fips_status_indicator!(
                derive($alg, iterations, &salt, &secret, &mut out),
                $expect
            );

            assert_fips_status_indicator!(verify($alg, iterations, &salt, &secret, &out), $expect)
                .unwrap();
        }
    };
}

pbkdf2_api!(
    sha1_13secret_15_salt_999iter,
    PBKDF2_HMAC_SHA1,
    13,
    15,
    999,
    FipsServiceStatus::NonApproved
);
pbkdf2_api!(
    sha1_14secret_16_salt_1000iter,
    PBKDF2_HMAC_SHA1,
    14,
    16,
    1000,
    FipsServiceStatus::Approved
);
pbkdf2_api!(
    sha1_15secret_17_salt_1001iter,
    PBKDF2_HMAC_SHA1,
    15,
    16,
    1001,
    FipsServiceStatus::Approved
);
pbkdf2_api!(
    sha256_13secret_15_salt_999iter,
    PBKDF2_HMAC_SHA256,
    13,
    15,
    999,
    FipsServiceStatus::NonApproved
);
pbkdf2_api!(
    sha256_14secret_16_salt_1000iter,
    PBKDF2_HMAC_SHA256,
    14,
    16,
    1000,
    FipsServiceStatus::Approved
);
pbkdf2_api!(
    sha256_15secret_17_salt_1001iter,
    PBKDF2_HMAC_SHA256,
    15,
    16,
    1001,
    FipsServiceStatus::Approved
);
pbkdf2_api!(
    sha384_13secret_15_salt_999iter,
    PBKDF2_HMAC_SHA384,
    13,
    15,
    999,
    FipsServiceStatus::NonApproved
);
pbkdf2_api!(
    sha384_14secret_16_salt_1000iter,
    PBKDF2_HMAC_SHA384,
    14,
    16,
    1000,
    FipsServiceStatus::Approved
);
pbkdf2_api!(
    sha384_15secret_17_salt_1001iter,
    PBKDF2_HMAC_SHA384,
    15,
    16,
    1001,
    FipsServiceStatus::Approved
);
pbkdf2_api!(
    sha512_13secret_15_salt_999iter,
    PBKDF2_HMAC_SHA512,
    13,
    15,
    999,
    FipsServiceStatus::NonApproved
);
pbkdf2_api!(
    sha512_14secret_16_salt_1000iter,
    PBKDF2_HMAC_SHA512,
    14,
    16,
    1000,
    FipsServiceStatus::Approved
);
pbkdf2_api!(
    sha512_15secret_17_salt_1001iter,
    PBKDF2_HMAC_SHA512,
    15,
    16,
    1001,
    FipsServiceStatus::Approved
);
