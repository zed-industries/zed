// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use crate::aead::chacha20_poly1305_openssh::{OpeningKey, SealingKey};
use crate::fips::{assert_fips_status_indicator, FipsServiceStatus};

use super::TEST_MESSAGE;

#[test]
fn test() {
    let key_bytes = &[42u8; 64];
    let key = SealingKey::new(key_bytes);

    let mut message: Vec<u8> = Vec::new();

    #[allow(clippy::cast_possible_truncation)]
    message.extend_from_slice({
        let len = TEST_MESSAGE.len() as u32;
        &[
            ((len & 0xFF00_0000) >> 24) as u8,
            ((len & 0xFF_0000) >> 16) as u8,
            ((len & 0xFF00) >> 8) as u8,
            (len & 0xFF) as u8,
        ]
    });
    message.extend_from_slice(TEST_MESSAGE);

    let mut tag = [0u8; 16];

    assert_fips_status_indicator!(
        key.seal_in_place(1024, &mut message, &mut tag),
        FipsServiceStatus::NonApproved
    );

    let mut encrypted_packet_length = [0u8; 4];
    encrypted_packet_length.copy_from_slice(&message[0..4]);

    let key = OpeningKey::new(key_bytes);

    let packet_length = assert_fips_status_indicator!(
        key.decrypt_packet_length(1024, encrypted_packet_length),
        FipsServiceStatus::NonApproved
    );

    #[allow(clippy::cast_possible_truncation)]
    let expected_packet_length = TEST_MESSAGE.len() as u32;
    assert_eq!(
        expected_packet_length,
        (u32::from(packet_length[0]) << 24)
            | (u32::from(packet_length[1]) << 16)
            | (u32::from(packet_length[2]) << 8)
            | u32::from(packet_length[3])
    );

    let message = assert_fips_status_indicator!(
        key.open_in_place(1024, &mut message, &tag).unwrap(),
        FipsServiceStatus::NonApproved
    );
    assert_eq!(TEST_MESSAGE, message);
}
