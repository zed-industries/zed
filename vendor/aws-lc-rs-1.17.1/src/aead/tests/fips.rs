// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#![cfg(debug_assertions)]

mod chacha20_poly1305_openssh;
mod quic;

use crate::aead::nonce_sequence::Counter64Builder;
use crate::aead::{
    Aad, BoundKey, Nonce, OpeningKey, RandomizedNonceKey, SealingKey, TlsProtocolId,
    TlsRecordOpeningKey, TlsRecordSealingKey, UnboundKey, AES_128_GCM, AES_256_GCM,
    CHACHA20_POLY1305,
};
use crate::fips::{assert_fips_status_indicator, FipsServiceStatus};

const TEST_KEY_128_BIT: [u8; 16] = [
    0x9f, 0xd9, 0x41, 0xc3, 0xa6, 0xfe, 0xb9, 0x26, 0x2a, 0x35, 0xa7, 0x44, 0xbb, 0xc0, 0x3a, 0x6a,
];

const TEST_KEY_256_BIT: [u8; 32] = [
    0xd8, 0x32, 0x58, 0xa9, 0x5a, 0x62, 0x6c, 0x99, 0xc4, 0xe6, 0xb5, 0x3f, 0x97, 0x90, 0x62, 0xbe,
    0x71, 0x0f, 0xd5, 0xe1, 0xd4, 0xfe, 0x95, 0xb3, 0x03, 0x46, 0xa5, 0x8e, 0x36, 0xad, 0x18, 0xe3,
];

const TEST_NONCE_96_BIT: [u8; 12] = [
    0xe4, 0x39, 0x17, 0x95, 0x86, 0xcd, 0xcd, 0x5a, 0x1b, 0x46, 0x7b, 0x1d,
];

const TEST_MESSAGE: &[u8] = "test message".as_bytes();

macro_rules! nonce_sequence_api {
    ($name:ident, $alg:expr, $key:expr, $seal_expect:path, $open_expect:path) => {
        #[test]
        fn $name() {
            {
                let mut key = SealingKey::new(
                    UnboundKey::new($alg, $key).unwrap(),
                    Counter64Builder::new().build(),
                );

                let mut in_out = Vec::from(TEST_MESSAGE);

                assert_fips_status_indicator!(
                    key.seal_in_place_append_tag(Aad::empty(), &mut in_out),
                    $seal_expect
                )
                .unwrap();

                let mut key = OpeningKey::new(
                    UnboundKey::new($alg, $key).unwrap(),
                    Counter64Builder::new().build(),
                );

                let result = assert_fips_status_indicator!(
                    key.open_in_place(Aad::empty(), &mut in_out),
                    $open_expect
                )
                .unwrap();

                assert_eq!(TEST_MESSAGE, result);
            }

            {
                let mut key = SealingKey::new(
                    UnboundKey::new($alg, $key).unwrap(),
                    Counter64Builder::new().build(),
                );

                let mut in_out = Vec::from(TEST_MESSAGE);

                let tag = assert_fips_status_indicator!(
                    key.seal_in_place_separate_tag(Aad::empty(), &mut in_out),
                    $seal_expect
                )
                .unwrap();

                in_out.extend(tag.as_ref().iter());

                let mut key = OpeningKey::new(
                    UnboundKey::new($alg, $key).unwrap(),
                    Counter64Builder::new().build(),
                );

                let result = assert_fips_status_indicator!(
                    key.open_in_place(Aad::empty(), &mut in_out),
                    $open_expect
                )
                .unwrap();

                assert_eq!(TEST_MESSAGE, result);
            }
        }
    };
}

nonce_sequence_api!(
    aes_gcm_128_nonce_sequence_api,
    &AES_128_GCM,
    &TEST_KEY_128_BIT[..],
    FipsServiceStatus::NonApproved,
    FipsServiceStatus::Approved
);
nonce_sequence_api!(
    aes_gcm_256_nonce_sequence_api,
    &AES_256_GCM,
    &TEST_KEY_256_BIT[..],
    FipsServiceStatus::NonApproved,
    FipsServiceStatus::Approved
);
nonce_sequence_api!(
    chacha20_poly1305_nonce_sequence_api,
    &CHACHA20_POLY1305,
    &TEST_KEY_256_BIT[..],
    FipsServiceStatus::NonApproved,
    FipsServiceStatus::NonApproved
);

macro_rules! randnonce_api {
    ($name:ident, $alg:expr, $key:expr) => {
        #[test]
        fn $name() {
            let key = RandomizedNonceKey::new($alg, $key).unwrap();

            {
                let mut in_out = Vec::from(TEST_MESSAGE);
                let nonce = assert_fips_status_indicator!(
                    key.seal_in_place_append_tag(Aad::empty(), &mut in_out),
                    FipsServiceStatus::Approved
                )
                .unwrap();

                let in_out = assert_fips_status_indicator!(
                    key.open_in_place(nonce, Aad::empty(), &mut in_out),
                    FipsServiceStatus::Approved
                )
                .unwrap();

                assert_eq!(TEST_MESSAGE, in_out);
            }

            {
                let mut in_out = Vec::from(TEST_MESSAGE);

                let (nonce, tag) = assert_fips_status_indicator!(
                    key.seal_in_place_separate_tag(Aad::empty(), &mut in_out),
                    FipsServiceStatus::Approved
                )
                .unwrap();

                in_out.extend(tag.as_ref().iter());

                let in_out = assert_fips_status_indicator!(
                    key.open_in_place(nonce, Aad::empty(), &mut in_out),
                    FipsServiceStatus::Approved
                )
                .unwrap();

                assert_eq!(TEST_MESSAGE, in_out);
            }
        }
    };
    // Match for unsupported variants
    ($name:ident, $alg:expr, $key:expr, false) => {
        #[test]
        fn $name() {
            assert!(RandomizedNonceKey::new($alg, $key).is_err());
        }
    };
}

randnonce_api!(
    aes_gcm_128_randnonce_api,
    &AES_128_GCM,
    &TEST_KEY_128_BIT[..]
);
randnonce_api!(
    aes_gcm_256_randnonce_api,
    &AES_256_GCM,
    &TEST_KEY_256_BIT[..]
);
randnonce_api!(
    chacha20_poly1305_randnonce_api,
    &CHACHA20_POLY1305,
    &TEST_KEY_256_BIT[..],
    false
);

macro_rules! tls_nonce_api {
    ($name:ident, $alg:expr, $proto:expr, $key:expr) => {
        #[test]
        fn $name() {
            let mut key = TlsRecordSealingKey::new($alg, $proto, $key).unwrap();

            let mut in_out = Vec::from(TEST_MESSAGE);

            assert_fips_status_indicator!(
                key.seal_in_place_append_tag(
                    Nonce::from(&TEST_NONCE_96_BIT),
                    Aad::empty(),
                    &mut in_out,
                ),
                FipsServiceStatus::Approved
            )
            .unwrap();

            let key = TlsRecordOpeningKey::new($alg, $proto, $key).unwrap();

            let in_out = assert_fips_status_indicator!(
                key.open_in_place(Nonce::from(&TEST_NONCE_96_BIT), Aad::empty(), &mut in_out),
                FipsServiceStatus::Approved
            )
            .unwrap();

            assert_eq!(in_out, TEST_MESSAGE);
        }
    };
    // Match for unsupported variants
    ($name:ident, $alg:expr, $proto:expr, $key:expr, false) => {
        #[test]
        fn $name() {
            assert!(TlsRecordSealingKey::new($alg, $proto, $key).is_err());
            assert!(TlsRecordOpeningKey::new($alg, $proto, $key).is_err());
        }
    };
}

tls_nonce_api!(
    aes_128_tls12_nonce_api,
    &AES_128_GCM,
    TlsProtocolId::TLS12,
    &TEST_KEY_128_BIT
);
tls_nonce_api!(
    aes_256_tls12_nonce_api,
    &AES_256_GCM,
    TlsProtocolId::TLS12,
    &TEST_KEY_256_BIT
);
tls_nonce_api!(
    aes_128_tls13_nonce_api,
    &AES_128_GCM,
    TlsProtocolId::TLS13,
    &TEST_KEY_128_BIT
);
tls_nonce_api!(
    aes_256_tls13_nonce_api,
    &AES_256_GCM,
    TlsProtocolId::TLS13,
    &TEST_KEY_256_BIT
);
tls_nonce_api!(
    chaca20_poly1305_tls12_nonce_api,
    &CHACHA20_POLY1305,
    TlsProtocolId::TLS12,
    &TEST_KEY_256_BIT,
    false
);
tls_nonce_api!(
    chaca20_poly1305_tls13_nonce_api,
    &CHACHA20_POLY1305,
    TlsProtocolId::TLS13,
    &TEST_KEY_256_BIT,
    false
);
