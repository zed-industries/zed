// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#![cfg(debug_assertions)]

use crate::{
    agreement::{
        agree_ephemeral, EphemeralPrivateKey, UnparsedPublicKey, ECDH_P256, ECDH_P384, ECDH_P521,
        X25519,
    },
    error::Unspecified,
    fips::{assert_fips_status_indicator, FipsServiceStatus},
    rand::SystemRandom,
};

macro_rules! agree_ephemeral_api {
    ($name:ident, $alg:expr, $expect:path) => {
        #[test]
        fn $name() {
            let rng = SystemRandom::new();

            let alice_private =
                assert_fips_status_indicator!(EphemeralPrivateKey::generate($alg, &rng), $expect)
                    .unwrap();
            let bob_private =
                assert_fips_status_indicator!(EphemeralPrivateKey::generate($alg, &rng), $expect)
                    .unwrap();

            let alice_public = alice_private.compute_public_key().unwrap();
            let alice_public = UnparsedPublicKey::new($alg, alice_public.as_ref());
            let bob_public = bob_private.compute_public_key().unwrap();
            let bob_public = UnparsedPublicKey::new($alg, bob_public.as_ref());

            let alice_secret = assert_fips_status_indicator!(
                agree_ephemeral(alice_private, &bob_public, Unspecified, |secret| {
                    Ok(Vec::from(secret))
                }),
                $expect
            )
            .unwrap();

            let bob_secret = assert_fips_status_indicator!(
                agree_ephemeral(bob_private, &alice_public, Unspecified, |secret| {
                    Ok(Vec::from(secret))
                }),
                $expect
            )
            .unwrap();

            assert_eq!(alice_secret, bob_secret);
        }
    };
}

agree_ephemeral_api!(ecdh_p256, &ECDH_P256, FipsServiceStatus::Approved);
agree_ephemeral_api!(ecdh_p384, &ECDH_P384, FipsServiceStatus::Approved);
agree_ephemeral_api!(ecdh_p521, &ECDH_P521, FipsServiceStatus::Approved);
agree_ephemeral_api!(x25519, &X25519, FipsServiceStatus::NonApproved);
