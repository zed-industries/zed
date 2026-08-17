// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#![cfg(debug_assertions)]

use crate::fips::{assert_fips_status_indicator, FipsServiceStatus};
use crate::rand::SystemRandom;
use crate::signature::{
    EcdsaKeyPair, Ed25519KeyPair, EdDSAParameters, KeyPair, RsaKeyPair, VerificationAlgorithm,
    ECDSA_P256_SHA256_ASN1, ECDSA_P256_SHA256_ASN1_SIGNING, ECDSA_P256_SHA256_FIXED,
    ECDSA_P256_SHA256_FIXED_SIGNING, ECDSA_P256_SHA384_ASN1, ECDSA_P384_SHA256_ASN1,
    ECDSA_P384_SHA384_ASN1, ECDSA_P384_SHA384_ASN1_SIGNING, ECDSA_P384_SHA384_FIXED,
    ECDSA_P384_SHA384_FIXED_SIGNING, ECDSA_P384_SHA3_384_ASN1, ECDSA_P384_SHA3_384_ASN1_SIGNING,
    ECDSA_P384_SHA3_384_FIXED, ECDSA_P384_SHA3_384_FIXED_SIGNING, ECDSA_P521_SHA3_512_ASN1,
    ECDSA_P521_SHA3_512_ASN1_SIGNING, ECDSA_P521_SHA3_512_FIXED, ECDSA_P521_SHA3_512_FIXED_SIGNING,
    ECDSA_P521_SHA512_ASN1, ECDSA_P521_SHA512_ASN1_SIGNING, ECDSA_P521_SHA512_FIXED,
    ECDSA_P521_SHA512_FIXED_SIGNING, RSA_PKCS1_1024_8192_SHA1_FOR_LEGACY_USE_ONLY,
    RSA_PKCS1_1024_8192_SHA256_FOR_LEGACY_USE_ONLY, RSA_PKCS1_1024_8192_SHA512_FOR_LEGACY_USE_ONLY,
    RSA_PKCS1_2048_8192_SHA256, RSA_PKCS1_2048_8192_SHA384, RSA_PKCS1_2048_8192_SHA512,
    RSA_PKCS1_SHA256, RSA_PKCS1_SHA384, RSA_PKCS1_SHA512, RSA_PSS_2048_8192_SHA256,
    RSA_PSS_2048_8192_SHA384, RSA_PSS_2048_8192_SHA512, RSA_PSS_SHA256, RSA_PSS_SHA384,
    RSA_PSS_SHA512,
};

mod keys;

use keys::*;

const TEST_MESSAGE: &str = "test message";

macro_rules! ecdsa_generate_sign_verify {
    ($name:ident, $sign_alg:expr, $verify_alg:expr, $generate_expect:path, $sign_verify_expect:path) => {
        #[test]
        fn $name() {
            let rng = SystemRandom::new();

            let key_document = assert_fips_status_indicator!(
                EcdsaKeyPair::generate_pkcs8($sign_alg, &rng),
                $generate_expect
            )
            .unwrap();

            let keypair = assert_fips_status_indicator!(
                EcdsaKeyPair::from_pkcs8($sign_alg, key_document.as_ref()),
                FipsServiceStatus::Approved
            )
            .unwrap();

            let signature = assert_fips_status_indicator!(
                keypair.sign(&rng, TEST_MESSAGE.as_bytes()),
                $sign_verify_expect
            )
            .unwrap();

            let public_key = keypair.public_key();

            assert_fips_status_indicator!(
                $verify_alg.verify_sig(
                    public_key.as_ref(),
                    TEST_MESSAGE.as_bytes(),
                    signature.as_ref()
                ),
                $sign_verify_expect
            )
            .unwrap();
        }
    };
}

ecdsa_generate_sign_verify!(
    ecdsa_p256_sha256_asn1,
    &ECDSA_P256_SHA256_ASN1_SIGNING,
    ECDSA_P256_SHA256_ASN1,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
ecdsa_generate_sign_verify!(
    ecdsa_p256_sha256_fixed,
    &ECDSA_P256_SHA256_FIXED_SIGNING,
    ECDSA_P256_SHA256_FIXED,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
ecdsa_generate_sign_verify!(
    ecdsa_p384_sha3_384_asn1,
    &ECDSA_P384_SHA3_384_ASN1_SIGNING,
    ECDSA_P384_SHA3_384_ASN1,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
ecdsa_generate_sign_verify!(
    ecdsa_p384_sha3_384_fixed,
    &ECDSA_P384_SHA3_384_FIXED_SIGNING,
    ECDSA_P384_SHA3_384_FIXED,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
ecdsa_generate_sign_verify!(
    ecdsa_p384_sha384_asn1,
    &ECDSA_P384_SHA384_ASN1_SIGNING,
    ECDSA_P384_SHA384_ASN1,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
ecdsa_generate_sign_verify!(
    ecdsa_p384_sha384_fixed,
    &ECDSA_P384_SHA384_FIXED_SIGNING,
    ECDSA_P384_SHA384_FIXED,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
ecdsa_generate_sign_verify!(
    ecdsa_p521_sha3_512_asn1,
    &ECDSA_P521_SHA3_512_ASN1_SIGNING,
    ECDSA_P521_SHA3_512_ASN1,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
ecdsa_generate_sign_verify!(
    ecdsa_p521_sha3_512_fixed,
    &ECDSA_P521_SHA3_512_FIXED_SIGNING,
    ECDSA_P521_SHA3_512_FIXED,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
ecdsa_generate_sign_verify!(
    ecdsa_p521_sha512_asn1,
    &ECDSA_P521_SHA512_ASN1_SIGNING,
    ECDSA_P521_SHA512_ASN1,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
ecdsa_generate_sign_verify!(
    ecdsa_p521_sha512_fixed,
    &ECDSA_P521_SHA512_FIXED_SIGNING,
    ECDSA_P521_SHA512_FIXED,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);

#[test]
fn ed25519() {
    let rng = SystemRandom::new();

    let key_document = assert_fips_status_indicator!(
        Ed25519KeyPair::generate_pkcs8(&rng),
        FipsServiceStatus::Approved
    )
    .unwrap();

    let keypair = assert_fips_status_indicator!(
        Ed25519KeyPair::from_pkcs8(key_document.as_ref()),
        FipsServiceStatus::Unset
    )
    .unwrap();

    let signature = assert_fips_status_indicator!(
        keypair.sign(TEST_MESSAGE.as_bytes()),
        FipsServiceStatus::Approved
    );

    let public_key = keypair.public_key();

    assert_fips_status_indicator!(
        EdDSAParameters.verify_sig(
            public_key.as_ref(),
            TEST_MESSAGE.as_bytes(),
            signature.as_ref()
        ),
        FipsServiceStatus::Approved
    )
    .unwrap();
}

macro_rules! ecdsa_verify {
    ($name:ident, $public_key:expr, $verify_alg:expr, $signature:expr, $expect:path) => {
        #[test]
        fn $name() {
            assert_fips_status_indicator!(
                $verify_alg.verify_sig($public_key, TEST_MESSAGE.as_bytes(), $signature),
                $expect
            )
            .unwrap();
        }
    };
}

ecdsa_verify!(
    ecdsa_p256_sha384_asn1,
    &TEST_P256_PUBLIC_BYTES[..],
    ECDSA_P256_SHA384_ASN1,
    &TEST_MESSGAE_P256_SHA384_ASN1[..],
    FipsServiceStatus::Approved
);
ecdsa_verify!(
    ecdsa_p384_sha256_asn1,
    &TEST_P384_PUBLIC_BYTES[..],
    ECDSA_P384_SHA256_ASN1,
    &TEST_MESSAGE_P384_SHA256_ASN1[..],
    FipsServiceStatus::Approved
);

macro_rules! rsa_sign_verify {
    ($name:ident, $key:expr, $sign_alg:expr, $verify_alg:expr, $sign_expect:path, $verify_expect:path) => {
        #[test]
        fn $name() {
            let rng = SystemRandom::new();

            let private_key = RsaKeyPair::from_pkcs8($key).unwrap();

            let mut signature = vec![0u8; private_key.public_modulus_len()];

            assert_fips_status_indicator!(
                private_key.sign($sign_alg, &rng, TEST_MESSAGE.as_bytes(), &mut signature,),
                $sign_expect
            )
            .unwrap();

            let public_key = private_key.public_key();

            assert_fips_status_indicator!(
                $verify_alg.verify_sig(public_key.as_ref(), TEST_MESSAGE.as_bytes(), &signature),
                $verify_expect
            )
            .unwrap();
        }
    };
}

rsa_sign_verify!(
    rsa_pkcs1_2048_sha256,
    &TEST_RSA_2048_PRIVATE_PKCS8_DER[..],
    &RSA_PKCS1_SHA256,
    RSA_PKCS1_2048_8192_SHA256,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
rsa_sign_verify!(
    rsa_pkcs1_2048_sha384,
    &TEST_RSA_2048_PRIVATE_PKCS8_DER[..],
    &RSA_PKCS1_SHA384,
    RSA_PKCS1_2048_8192_SHA384,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
rsa_sign_verify!(
    rsa_pkcs1_2048_sha512,
    &TEST_RSA_2048_PRIVATE_PKCS8_DER[..],
    &RSA_PKCS1_SHA512,
    RSA_PKCS1_2048_8192_SHA512,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
rsa_sign_verify!(
    rsa_pkcs1_3072_sha256,
    &TEST_RSA_3072_PRIVATE_PKCS8_DER[..],
    &RSA_PKCS1_SHA256,
    RSA_PKCS1_2048_8192_SHA256,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
rsa_sign_verify!(
    rsa_pkcs1_3072_sha384,
    &TEST_RSA_3072_PRIVATE_PKCS8_DER[..],
    &RSA_PKCS1_SHA384,
    RSA_PKCS1_2048_8192_SHA384,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
rsa_sign_verify!(
    rsa_pkcs1_3072_sha512,
    &TEST_RSA_3072_PRIVATE_PKCS8_DER[..],
    &RSA_PKCS1_SHA512,
    RSA_PKCS1_2048_8192_SHA512,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
rsa_sign_verify!(
    rsa_pkcs1_4096_sha256,
    &TEST_RSA_4096_PRIVATE_PKCS8_DER[..],
    &RSA_PKCS1_SHA256,
    RSA_PKCS1_2048_8192_SHA256,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
rsa_sign_verify!(
    rsa_pkcs1_4096_sha384,
    &TEST_RSA_4096_PRIVATE_PKCS8_DER[..],
    &RSA_PKCS1_SHA384,
    RSA_PKCS1_2048_8192_SHA384,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
rsa_sign_verify!(
    rsa_pkcs1_4096_sha512,
    &TEST_RSA_4096_PRIVATE_PKCS8_DER[..],
    &RSA_PKCS1_SHA512,
    RSA_PKCS1_2048_8192_SHA512,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
rsa_sign_verify!(
    rsa_pkcs1_8192_sha256,
    &TEST_RSA_8192_PRIVATE_PKCS8_DER[..],
    &RSA_PKCS1_SHA256,
    RSA_PKCS1_2048_8192_SHA256,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
rsa_sign_verify!(
    rsa_pkcs1_8192_sha384,
    &TEST_RSA_8192_PRIVATE_PKCS8_DER[..],
    &RSA_PKCS1_SHA384,
    RSA_PKCS1_2048_8192_SHA384,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
rsa_sign_verify!(
    rsa_pkcs1_8192_sha512,
    &TEST_RSA_8192_PRIVATE_PKCS8_DER[..],
    &RSA_PKCS1_SHA512,
    RSA_PKCS1_2048_8192_SHA512,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
rsa_sign_verify!(
    rsa_pss_2048_sha256,
    &TEST_RSA_2048_PRIVATE_PKCS8_DER[..],
    &RSA_PSS_SHA256,
    RSA_PSS_2048_8192_SHA256,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
rsa_sign_verify!(
    rsa_pss_2048_sha384,
    &TEST_RSA_2048_PRIVATE_PKCS8_DER[..],
    &RSA_PSS_SHA384,
    RSA_PSS_2048_8192_SHA384,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
rsa_sign_verify!(
    rsa_pss_2048_sha512,
    &TEST_RSA_2048_PRIVATE_PKCS8_DER[..],
    &RSA_PSS_SHA512,
    RSA_PSS_2048_8192_SHA512,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
rsa_sign_verify!(
    rsa_pss_4096_sha256,
    &TEST_RSA_4096_PRIVATE_PKCS8_DER[..],
    &RSA_PSS_SHA256,
    RSA_PSS_2048_8192_SHA256,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
rsa_sign_verify!(
    rsa_pss_4096_sha384,
    &TEST_RSA_4096_PRIVATE_PKCS8_DER[..],
    &RSA_PSS_SHA384,
    RSA_PSS_2048_8192_SHA384,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
rsa_sign_verify!(
    rsa_pss_4096_sha512,
    &TEST_RSA_4096_PRIVATE_PKCS8_DER[..],
    &RSA_PSS_SHA512,
    RSA_PSS_2048_8192_SHA512,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
rsa_sign_verify!(
    rsa_pss_8192_sha256,
    &TEST_RSA_8192_PRIVATE_PKCS8_DER[..],
    &RSA_PSS_SHA256,
    RSA_PSS_2048_8192_SHA256,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
rsa_sign_verify!(
    rsa_pss_8192_sha384,
    &TEST_RSA_8192_PRIVATE_PKCS8_DER[..],
    &RSA_PSS_SHA384,
    RSA_PSS_2048_8192_SHA384,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);
rsa_sign_verify!(
    rsa_pss_8192_sha512,
    &TEST_RSA_8192_PRIVATE_PKCS8_DER[..],
    &RSA_PSS_SHA512,
    RSA_PSS_2048_8192_SHA512,
    FipsServiceStatus::Approved,
    FipsServiceStatus::Approved
);

macro_rules! rsa_verify {
    ($name:ident, $key:expr, $verify_alg:expr, $signature:expr, $verify_expect:path) => {
        #[test]
        fn $name() {
            assert_fips_status_indicator!(
                $verify_alg.verify_sig($key, TEST_MESSAGE.as_bytes(), $signature),
                $verify_expect
            )
            .unwrap();
        }
    };
}

rsa_verify!(
    rsa_pkcs1_1024_sha1,
    &TEST_RSA_1024_PUBLIC_BYTES[..],
    RSA_PKCS1_1024_8192_SHA1_FOR_LEGACY_USE_ONLY,
    &TEST_MESSAGE_RSA_PKCS1_1024_SHA1,
    FipsServiceStatus::Approved
);
rsa_verify!(
    rsa_pkcs1_1024_sha256,
    &TEST_RSA_1024_PUBLIC_BYTES[..],
    RSA_PKCS1_1024_8192_SHA256_FOR_LEGACY_USE_ONLY,
    &TEST_MESSAGE_RSA_PKCS1_1024_SHA256,
    FipsServiceStatus::Approved
);
// TODO: Ring API never had SHA384 with RSA-1024?
// rsa_verify!(
//     rsa_pkcs1_1024_sha384,
//     &TEST_RSA_1024_PUBLIC_BYTES[..],
//     RSA_PKCS1_1024_8192_SHA38,
//     &TEST_MESSAGE_RSA_PKCS1_1024_SHA384,
//     FipsServiceStatus::Approved
// );
rsa_verify!(
    rsa_pkcs1_1024_sha512,
    &TEST_RSA_1024_PUBLIC_BYTES[..],
    RSA_PKCS1_1024_8192_SHA512_FOR_LEGACY_USE_ONLY,
    &TEST_MESSAGE_RSA_PKCS1_1024_SHA512,
    FipsServiceStatus::Approved
);
