// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#![cfg(debug_assertions)]

use crate::cipher::{
    DecryptingKey, EncryptingKey, PaddedBlockDecryptingKey, PaddedBlockEncryptingKey,
    StreamingDecryptingKey, StreamingEncryptingKey, UnboundCipherKey, AES_128, AES_192, AES_256,
};
#[cfg(feature = "legacy-des")]
#[allow(deprecated)]
use crate::cipher::{
    DES_EDE3_FOR_LEGACY_USE_ONLY, DES_EDE_FOR_LEGACY_USE_ONLY, DES_FOR_LEGACY_USE_ONLY,
};
use crate::fips::{assert_fips_status_indicator, FipsServiceStatus};

const TEST_KEY_128_BIT: [u8; 16] = [
    0x9f, 0xd9, 0x41, 0xc3, 0xa6, 0xfe, 0xb9, 0x26, 0x2a, 0x35, 0xa7, 0x44, 0xbb, 0xc0, 0x3a, 0x6a,
];

const TEST_KEY_192_BIT: [u8; 24] = [
    0x50, 0x2a, 0x6a, 0xb3, 0x69, 0x84, 0xaf, 0x26, 0x8b, 0xf4, 0x23, 0xc7, 0xf5, 0x09, 0x20, 0x52,
    0x07, 0xfc, 0x15, 0x52, 0xaf, 0x4a, 0x91, 0xe5,
];

const TEST_KEY_256_BIT: [u8; 32] = [
    0xd8, 0x32, 0x58, 0xa9, 0x5a, 0x62, 0x6c, 0x99, 0xc4, 0xe6, 0xb5, 0x3f, 0x97, 0x90, 0x62, 0xbe,
    0x71, 0x0f, 0xd5, 0xe1, 0xd4, 0xfe, 0x95, 0xb3, 0x03, 0x46, 0xa5, 0x8e, 0x36, 0xad, 0x18, 0xe3,
];

#[cfg(feature = "legacy-des")]
const TEST_KEY_DES: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];

#[cfg(feature = "legacy-des")]
const TEST_KEY_DES_EDE: [u8; 16] = [
    0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01,
];

#[cfg(feature = "legacy-des")]
const TEST_KEY_DES_EDE3: [u8; 24] = [
    0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01,
    0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23,
];

const TEST_MESSAGE: &str = "test message";

macro_rules! block_api {
    ($name:ident, $alg:expr, $encrypt_mode:path, $decrypt_mode:path, $key:expr, $expect:path) => {
        #[test]
        #[allow(deprecated)]
        fn $name() {
            let key = $encrypt_mode(UnboundCipherKey::new($alg, $key).unwrap()).unwrap();

            let mut in_out = Vec::from(TEST_MESSAGE);

            let context = assert_fips_status_indicator!(key.encrypt(&mut in_out), $expect).unwrap();

            let key = $decrypt_mode(UnboundCipherKey::new($alg, $key).unwrap()).unwrap();

            let in_out =
                assert_fips_status_indicator!(key.decrypt(&mut in_out, context), $expect).unwrap();

            assert_eq!(TEST_MESSAGE.as_bytes(), in_out);
        }
    };
}

macro_rules! streaming_api {
    ($name:ident, $alg:expr, $encrypt_mode:path, $decrypt_mode:path, $key:expr, $expect:path) => {
        #[test]
        #[allow(deprecated)]
        fn $name() {
            let mut key = $encrypt_mode(UnboundCipherKey::new($alg, $key).unwrap()).unwrap();

            let input = TEST_MESSAGE.as_bytes();
            let mut encrypt_output = vec![0u8; TEST_MESSAGE.len() + $alg.block_len()];

            let mut buffer_update = key.update(&input, &mut encrypt_output).unwrap();

            let outlen = buffer_update.written().len();
            let (context, buffer_update) =
                assert_fips_status_indicator!(key.finish(buffer_update.remainder_mut()), $expect)
                    .unwrap();

            let outlen = outlen + buffer_update.written().len();

            let ciphertext = &encrypt_output[0..outlen];
            let mut decrypt_output = vec![0u8; outlen + $alg.block_len()];
            let mut key =
                $decrypt_mode(UnboundCipherKey::new($alg, $key).unwrap(), context).unwrap();

            let mut buffer_update = key.update(ciphertext, &mut decrypt_output).unwrap();

            let outlen = buffer_update.written().len();
            let buffer_update =
                assert_fips_status_indicator!(key.finish(buffer_update.remainder_mut()), $expect)
                    .unwrap();

            let outlen = outlen + buffer_update.written().len();
            let plaintext = &decrypt_output[0..outlen];

            assert_eq!(TEST_MESSAGE.as_bytes(), plaintext);
        }
    };
}

streaming_api!(
    streaming_aes_128_cbc_pkcs7,
    &AES_128,
    StreamingEncryptingKey::cbc_pkcs7,
    StreamingDecryptingKey::cbc_pkcs7,
    &TEST_KEY_128_BIT,
    FipsServiceStatus::Approved
);

streaming_api!(
    streaming_aes_128_ctr,
    &AES_128,
    StreamingEncryptingKey::ctr,
    StreamingDecryptingKey::ctr,
    &TEST_KEY_128_BIT,
    FipsServiceStatus::Approved
);

streaming_api!(
    streaming_aes_192_cbc_pkcs7,
    &AES_192,
    StreamingEncryptingKey::cbc_pkcs7,
    StreamingDecryptingKey::cbc_pkcs7,
    &TEST_KEY_192_BIT,
    FipsServiceStatus::Approved
);

streaming_api!(
    streaming_aes_192_ctr,
    &AES_192,
    StreamingEncryptingKey::ctr,
    StreamingDecryptingKey::ctr,
    &TEST_KEY_192_BIT,
    FipsServiceStatus::Approved
);

streaming_api!(
    streaming_aes_256_cbc_pkcs7,
    &AES_256,
    StreamingEncryptingKey::cbc_pkcs7,
    StreamingDecryptingKey::cbc_pkcs7,
    &TEST_KEY_256_BIT,
    FipsServiceStatus::Approved
);

streaming_api!(
    streaming_aes_256_ctr,
    &AES_256,
    StreamingEncryptingKey::ctr,
    StreamingDecryptingKey::ctr,
    &TEST_KEY_256_BIT,
    FipsServiceStatus::Approved
);

block_api!(
    block_aes_128_cbc_pkcs7,
    &AES_128,
    PaddedBlockEncryptingKey::cbc_pkcs7,
    PaddedBlockDecryptingKey::cbc_pkcs7,
    &TEST_KEY_128_BIT,
    FipsServiceStatus::Approved
);

block_api!(
    block_aes_128_ctr,
    &AES_128,
    EncryptingKey::ctr,
    DecryptingKey::ctr,
    &TEST_KEY_128_BIT,
    FipsServiceStatus::Approved
);

block_api!(
    block_aes_192_cbc_pkcs7,
    &AES_192,
    PaddedBlockEncryptingKey::cbc_pkcs7,
    PaddedBlockDecryptingKey::cbc_pkcs7,
    &TEST_KEY_192_BIT,
    FipsServiceStatus::Approved
);

block_api!(
    block_aes_192_ctr,
    &AES_192,
    EncryptingKey::ctr,
    DecryptingKey::ctr,
    &TEST_KEY_192_BIT,
    FipsServiceStatus::Approved
);

block_api!(
    block_aes_256_cbc_pkcs7,
    &AES_256,
    PaddedBlockEncryptingKey::cbc_pkcs7,
    PaddedBlockDecryptingKey::cbc_pkcs7,
    &TEST_KEY_256_BIT,
    FipsServiceStatus::Approved
);

block_api!(
    block_aes_256_ctr,
    &AES_256,
    EncryptingKey::ctr,
    DecryptingKey::ctr,
    &TEST_KEY_256_BIT,
    FipsServiceStatus::Approved
);

#[cfg(feature = "legacy-des")]
block_api!(
    block_des_cbc_pkcs7,
    &DES_FOR_LEGACY_USE_ONLY,
    PaddedBlockEncryptingKey::cbc_pkcs7,
    PaddedBlockDecryptingKey::cbc_pkcs7,
    &TEST_KEY_DES,
    FipsServiceStatus::NonApproved
);

#[cfg(feature = "legacy-des")]
block_api!(
    block_des_ecb_pkcs7,
    &DES_FOR_LEGACY_USE_ONLY,
    PaddedBlockEncryptingKey::ecb_pkcs7,
    PaddedBlockDecryptingKey::ecb_pkcs7,
    &TEST_KEY_DES,
    FipsServiceStatus::NonApproved
);

#[cfg(feature = "legacy-des")]
streaming_api!(
    streaming_des_cbc_pkcs7,
    &DES_FOR_LEGACY_USE_ONLY,
    StreamingEncryptingKey::cbc_pkcs7,
    StreamingDecryptingKey::cbc_pkcs7,
    &TEST_KEY_DES,
    FipsServiceStatus::NonApproved
);

#[cfg(feature = "legacy-des")]
streaming_api!(
    streaming_des_ecb_pkcs7,
    &DES_FOR_LEGACY_USE_ONLY,
    StreamingEncryptingKey::ecb_pkcs7,
    StreamingDecryptingKey::ecb_pkcs7,
    &TEST_KEY_DES,
    FipsServiceStatus::NonApproved
);

#[cfg(feature = "legacy-des")]
block_api!(
    block_des_ede_cbc_pkcs7,
    &DES_EDE_FOR_LEGACY_USE_ONLY,
    PaddedBlockEncryptingKey::cbc_pkcs7,
    PaddedBlockDecryptingKey::cbc_pkcs7,
    &TEST_KEY_DES_EDE,
    FipsServiceStatus::NonApproved
);

#[cfg(feature = "legacy-des")]
block_api!(
    block_des_ede_ecb_pkcs7,
    &DES_EDE_FOR_LEGACY_USE_ONLY,
    PaddedBlockEncryptingKey::ecb_pkcs7,
    PaddedBlockDecryptingKey::ecb_pkcs7,
    &TEST_KEY_DES_EDE,
    FipsServiceStatus::NonApproved
);

#[cfg(feature = "legacy-des")]
block_api!(
    block_des_ede3_cbc_pkcs7,
    &DES_EDE3_FOR_LEGACY_USE_ONLY,
    PaddedBlockEncryptingKey::cbc_pkcs7,
    PaddedBlockDecryptingKey::cbc_pkcs7,
    &TEST_KEY_DES_EDE3,
    FipsServiceStatus::NonApproved
);

#[cfg(feature = "legacy-des")]
block_api!(
    block_des_ede3_ecb_pkcs7,
    &DES_EDE3_FOR_LEGACY_USE_ONLY,
    PaddedBlockEncryptingKey::ecb_pkcs7,
    PaddedBlockDecryptingKey::ecb_pkcs7,
    &TEST_KEY_DES_EDE3,
    FipsServiceStatus::NonApproved
);

#[cfg(feature = "legacy-des")]
streaming_api!(
    streaming_des_ede_cbc_pkcs7,
    &DES_EDE_FOR_LEGACY_USE_ONLY,
    StreamingEncryptingKey::cbc_pkcs7,
    StreamingDecryptingKey::cbc_pkcs7,
    &TEST_KEY_DES_EDE,
    FipsServiceStatus::NonApproved
);

#[cfg(feature = "legacy-des")]
streaming_api!(
    streaming_des_ede_ecb_pkcs7,
    &DES_EDE_FOR_LEGACY_USE_ONLY,
    StreamingEncryptingKey::ecb_pkcs7,
    StreamingDecryptingKey::ecb_pkcs7,
    &TEST_KEY_DES_EDE,
    FipsServiceStatus::NonApproved
);

#[cfg(feature = "legacy-des")]
streaming_api!(
    streaming_des_ede3_cbc_pkcs7,
    &DES_EDE3_FOR_LEGACY_USE_ONLY,
    StreamingEncryptingKey::cbc_pkcs7,
    StreamingDecryptingKey::cbc_pkcs7,
    &TEST_KEY_DES_EDE3,
    FipsServiceStatus::NonApproved
);

#[cfg(feature = "legacy-des")]
streaming_api!(
    streaming_des_ede3_ecb_pkcs7,
    &DES_EDE3_FOR_LEGACY_USE_ONLY,
    StreamingEncryptingKey::ecb_pkcs7,
    StreamingDecryptingKey::ecb_pkcs7,
    &TEST_KEY_DES_EDE3,
    FipsServiceStatus::NonApproved
);
