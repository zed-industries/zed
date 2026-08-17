// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#![cfg(debug_assertions)]

use crate::digest::{
    Context, SHA1_FOR_LEGACY_USE_ONLY, SHA224, SHA256, SHA384, SHA3_256, SHA3_384, SHA3_512,
    SHA512, SHA512_256,
};
use crate::fips::{assert_fips_status_indicator, FipsServiceStatus};

const TEST_MESSAGE: &str = "test message";

macro_rules! digest_api {
    ($name:ident, $alg:expr, $expect:path) => {
        #[test]
        fn $name() {
            // Regardless of the algorithm you can always construct the context, and the status
            // should will not change.
            let mut context =
                assert_fips_status_indicator!(Context::new($alg), FipsServiceStatus::Unset);

            // AWS-LC digest update API does not set the inidicator API.
            assert_fips_status_indicator!(
                context.update(TEST_MESSAGE.as_bytes()),
                FipsServiceStatus::Unset
            );

            // Finish API expected to set the service indicator.
            let _digest = assert_fips_status_indicator!(context.finish(), $expect);
        }
    };
}

digest_api!(sha1, &SHA1_FOR_LEGACY_USE_ONLY, FipsServiceStatus::Approved);
digest_api!(sha224, &SHA224, FipsServiceStatus::Approved);
digest_api!(sha256, &SHA256, FipsServiceStatus::Approved);
digest_api!(sha384, &SHA384, FipsServiceStatus::Approved);
digest_api!(sha512, &SHA512, FipsServiceStatus::Approved);
digest_api!(sha512_256, &SHA512_256, FipsServiceStatus::Approved);
digest_api!(sha3_256, &SHA3_256, FipsServiceStatus::Approved);
digest_api!(sha3_384, &SHA3_384, FipsServiceStatus::Approved);
digest_api!(sha3_512, &SHA3_512, FipsServiceStatus::Approved);
