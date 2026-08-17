// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#![cfg(debug_assertions)]

use crate::fips::{assert_fips_status_indicator, FipsServiceStatus};
use crate::tls_prf::{Secret, P_SHA256, P_SHA384, P_SHA512};

macro_rules! prf_test {
    ($name:ident, $alg:expr, $size:expr, $label:expr, $expect:path) => {
        #[test]
        fn $name() {
            let secret = Secret::new($alg, &[42u8; $size]).expect("secret created");

            assert_fips_status_indicator!(secret.derive($label, b"seed", $size), $expect)
                .expect("derive successful");
        }
    };
}

prf_test!(
    sha256_extended_master_secret,
    &P_SHA256,
    32,
    b"extended master secret",
    FipsServiceStatus::Approved
);
prf_test!(
    sha384_extended_master_secret,
    &P_SHA384,
    48,
    b"extended master secret",
    FipsServiceStatus::Approved
);
prf_test!(
    sha512_extended_master_secret,
    &P_SHA512,
    64,
    b"extended master secret",
    FipsServiceStatus::Approved
);
prf_test!(
    sha256_master_secret,
    &P_SHA256,
    32,
    b"master secret",
    FipsServiceStatus::NonApproved
);
prf_test!(
    sha384_master_secret,
    &P_SHA384,
    48,
    b"master secret",
    FipsServiceStatus::NonApproved
);
prf_test!(
    sha512_master_secret,
    &P_SHA512,
    64,
    b"master secret",
    FipsServiceStatus::NonApproved
);
