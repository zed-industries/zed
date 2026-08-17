// Copyright 2016 Brian Smith.
// SPDX-License-Identifier: ISC
// Modifications copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

//! The [chacha20-poly1305@openssh.com] AEAD-ish construct.
//!
//! This should only be used by SSH implementations. It has a similar, but
//! different API from `aws_lc_rs::aead` because the construct cannot use the same
//! API as `aws_lc_rs::aead` due to the way the construct handles the encrypted
//! packet length.
//!
//! The concatenation of a and b is denoted `a||b`. `K_1` and `K_2` are defined
//! in the [chacha20-poly1305@openssh.com] specification. `packet_length`,
//! `padding_length`, `payload`, and `random padding` are defined in
//! [RFC 4253]. The term `plaintext` is used as a shorthand for
//! `padding_length||payload||random padding`.
//!
//! [chacha20-poly1305@openssh.com]:
//!    http://cvsweb.openbsd.org/cgi-bin/cvsweb/src/usr.bin/ssh/PROTOCOL.chacha20poly1305?annotate=HEAD
//! [RFC 4253]: https://tools.ietf.org/html/rfc4253
//!
//! # FIPS
//! The APIs offered in this module must not be used.

use super::{poly1305, Nonce, Tag};
use crate::cipher::block::BLOCK_LEN;
use crate::cipher::chacha::{self, ChaCha20Key};
use crate::endian::BigEndian;
use crate::iv::FixedLength;
use crate::{constant_time, error};

/// A key for sealing packets.
pub struct SealingKey {
    key: Key,
}

impl SealingKey {
    /// Constructs a new `SealingKey`.
    #[must_use]
    pub fn new(key_material: &[u8; KEY_LEN]) -> SealingKey {
        SealingKey {
            key: Key::new(key_material),
        }
    }

    /// Seals (encrypts and signs) a packet.
    ///
    /// On input, `plaintext_in_ciphertext_out` must contain the unencrypted
    /// `packet_length||plaintext` where `plaintext` is the
    /// `padding_length||payload||random padding`. It will be overwritten by
    /// `encrypted_packet_length||ciphertext`, where `encrypted_packet_length`
    /// is encrypted with `K_1` and `ciphertext` is encrypted by `K_2`.
    //
    // # FIPS
    // This method must not be used.
    #[inline]
    pub fn seal_in_place(
        &self,
        sequence_number: u32,
        plaintext_in_ciphertext_out: &mut [u8],
        tag_out: &mut [u8; TAG_LEN],
    ) {
        let nonce = make_nonce(sequence_number);
        let poly_key = derive_poly1305_key(&self.key.k_2, Nonce(FixedLength::from(nonce.as_ref())));

        {
            let (len_in_out, data_and_padding_in_out) =
                plaintext_in_ciphertext_out.split_at_mut(PACKET_LENGTH_LEN);

            self.key.k_1.encrypt_in_place(nonce.as_ref(), len_in_out, 0);
            self.key
                .k_2
                .encrypt_in_place(nonce.as_ref(), data_and_padding_in_out, 1);
        }

        let Tag(tag, tag_len) = poly1305::sign(poly_key, plaintext_in_ciphertext_out);
        debug_assert_eq!(TAG_LEN, tag_len);
        tag_out.copy_from_slice(tag.as_ref());
    }
}

/// A key for opening packets.
pub struct OpeningKey {
    key: Key,
}

impl OpeningKey {
    /// Constructs a new `OpeningKey`.
    #[must_use]
    pub fn new(key_material: &[u8; KEY_LEN]) -> OpeningKey {
        OpeningKey {
            key: Key::new(key_material),
        }
    }

    /// Returns the decrypted, but unauthenticated, packet length.
    ///
    /// Importantly, the result won't be authenticated until `open_in_place` is
    /// called.
    //
    // # FIPS
    // This method must not be used.
    #[inline]
    #[must_use]
    pub fn decrypt_packet_length(
        &self,
        sequence_number: u32,
        encrypted_packet_length: [u8; PACKET_LENGTH_LEN],
    ) -> [u8; PACKET_LENGTH_LEN] {
        let mut packet_length = encrypted_packet_length;
        let nonce = make_nonce(sequence_number);
        self.key
            .k_1
            .encrypt_in_place(nonce.as_ref(), &mut packet_length, 0);
        packet_length
    }

    /// Opens (authenticates and decrypts) a packet.
    ///
    /// `ciphertext_in_plaintext_out` must be of the form
    /// `encrypted_packet_length||ciphertext` where `ciphertext` is the
    /// encrypted `plaintext`. When the function succeeds the ciphertext is
    /// replaced by the plaintext and the result is `Ok(plaintext)`, where
    /// `plaintext` is `&ciphertext_in_plaintext_out[PACKET_LENGTH_LEN..]`;
    /// otherwise the contents of `ciphertext_in_plaintext_out` are unspecified
    /// and must not be used.
    ///
    /// # Errors
    /// `error::Unspecified` when ciphertext is invalid
    //
    // # FIPS
    // This method must not be used.
    #[inline]
    pub fn open_in_place<'a>(
        &self,
        sequence_number: u32,
        ciphertext_in_plaintext_out: &'a mut [u8],
        tag: &[u8; TAG_LEN],
    ) -> Result<&'a [u8], error::Unspecified> {
        let nonce = make_nonce(sequence_number);

        // We must verify the tag before decrypting so that
        // `ciphertext_in_plaintext_out` is unmodified if verification fails.
        // This is beyond what we guarantee.
        let poly_key = derive_poly1305_key(&self.key.k_2, Nonce(FixedLength::from(nonce.as_ref())));
        verify(poly_key, ciphertext_in_plaintext_out, tag)?;

        let plaintext_in_ciphertext_out = &mut ciphertext_in_plaintext_out[PACKET_LENGTH_LEN..];
        self.key
            .k_2
            .encrypt_in_place(nonce.as_ref(), plaintext_in_ciphertext_out, 1);

        Ok(plaintext_in_ciphertext_out)
    }
}

struct Key {
    k_1: ChaCha20Key,
    k_2: ChaCha20Key,
}

impl Key {
    fn new(key_material: &[u8; KEY_LEN]) -> Key {
        // The first half becomes K_2 and the second half becomes K_1.
        let (k_2, k_1) = key_material.split_at(chacha::KEY_LEN);
        let k_1: [u8; chacha::KEY_LEN] = k_1.try_into().unwrap();
        let k_2: [u8; chacha::KEY_LEN] = k_2.try_into().unwrap();
        Key {
            k_1: ChaCha20Key::from(k_1),
            k_2: ChaCha20Key::from(k_2),
        }
    }
}

#[inline]
fn make_nonce(sequence_number: u32) -> Nonce {
    Nonce::from(BigEndian::from(sequence_number))
}

/// The length of key.
pub const KEY_LEN: usize = chacha::KEY_LEN * 2;

/// The length in bytes of the `packet_length` field in a SSH packet.
pub const PACKET_LENGTH_LEN: usize = 4; // 32 bits

/// The length in bytes of an authentication tag.
pub const TAG_LEN: usize = BLOCK_LEN;

#[inline]
fn verify(key: poly1305::Key, msg: &[u8], tag: &[u8; TAG_LEN]) -> Result<(), error::Unspecified> {
    let Tag(calculated_tag, _) = poly1305::sign(key, msg);
    constant_time::verify_slices_are_equal(calculated_tag.as_ref(), tag)
}

#[inline]
#[allow(clippy::needless_pass_by_value)]
pub(super) fn derive_poly1305_key(chacha_key: &ChaCha20Key, nonce: Nonce) -> poly1305::Key {
    let mut key_bytes = [0u8; 2 * BLOCK_LEN];
    chacha_key.encrypt_in_place(nonce.as_ref(), &mut key_bytes, 0);
    poly1305::Key::new(key_bytes)
}

#[cfg(test)]
mod tests {
    use crate::aead::chacha20_poly1305_openssh::{
        derive_poly1305_key, OpeningKey, SealingKey, KEY_LEN, TAG_LEN,
    };
    use crate::aead::Nonce;
    use crate::cipher::chacha::ChaCha20Key;
    use crate::endian::{BigEndian, FromArray, LittleEndian};
    use crate::test;

    #[test]
    fn derive_poly1305_test() {
        let chacha_key =
            test::from_hex("98bef1469be7269837a45bfbc92a5a6ac762507cf96443bf33b96b1bd4c6f8f6")
                .unwrap();
        let expected_poly1305_key =
            test::from_hex("759de17d6d6258a436e36ecf75e3f00e4d9133ec05c4c855a9ec1a4e4e873b9d")
                .unwrap();
        let chacha_key = chacha_key.as_slice();
        let chacha_key_bytes: [u8; 32] = <[u8; 32]>::try_from(chacha_key).unwrap();
        let chacha_key = ChaCha20Key::from(chacha_key_bytes);
        {
            let iv = Nonce::from(&[45u32, 897, 4567]);
            let poly1305_key = derive_poly1305_key(&chacha_key, iv);
            assert_eq!(&expected_poly1305_key, &poly1305_key.key_and_nonce);
        }

        {
            let iv = Nonce::from(&LittleEndian::<u32>::from_array(&[45u32, 897, 4567]));
            let poly1305_key = derive_poly1305_key(&chacha_key, iv);
            assert_eq!(&expected_poly1305_key, &poly1305_key.key_and_nonce);
        }

        {
            let iv = Nonce::from(&BigEndian::<u32>::from_array(&[45u32, 897, 4567]));
            let poly1305_key = derive_poly1305_key(&chacha_key, iv);
            assert_ne!(&expected_poly1305_key, &poly1305_key.key_and_nonce);
        }
    }

    #[test]
    #[allow(clippy::cast_possible_truncation)]
    fn test_decrypt_packet_length() {
        let key_bytes: [u8; KEY_LEN] = test::from_dirty_hex("98bef1469be7269837a45bfbc92a5a6ac762\
        507cf96443bf33b96b1bd4c6f8f6759de17d6d6258a436e36ecf75e3f00e4d9133ec05c4c855a9ec1a4e4e873b9d")
            .try_into().unwrap();

        let sealing_key = SealingKey::new(&key_bytes);
        let opening_key = OpeningKey::new(&key_bytes);

        let plaintext = b"Hello World!";
        let packet_length = plaintext.len() as u32;
        let packet_length = packet_length.to_be_bytes();
        let mut in_out = Vec::new();

        in_out.extend_from_slice(&packet_length);
        in_out.extend_from_slice(plaintext);

        let mut tag = [0u8; TAG_LEN];
        sealing_key.seal_in_place(0, &mut in_out, &mut tag);

        let encrypted_length: [u8; 4] = in_out[0..4].to_owned().try_into().unwrap();
        let decrypted_length = opening_key.decrypt_packet_length(0, encrypted_length);
        let decrypted_length = u32::from_be_bytes(decrypted_length);
        assert_eq!(plaintext.len() as u32, decrypted_length);
    }

    #[test]
    fn test_types() {
        test::compile_time_assert_send::<OpeningKey>();
        test::compile_time_assert_sync::<OpeningKey>();

        test::compile_time_assert_send::<SealingKey>();
        test::compile_time_assert_sync::<SealingKey>();
    }
}
