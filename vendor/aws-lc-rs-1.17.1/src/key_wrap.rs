// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

//! Key Wrap Algorithms.
//!
//! # Examples
//! ```rust
//! # use std::error::Error;
//! # fn main() -> Result<(), Box<dyn Error>> {
//! use aws_lc_rs::key_wrap::{AesKek, KeyWrapPadded, AES_128};
//!
//! const KEY: &[u8] = &[
//!     0xa8, 0xe0, 0x6d, 0xa6, 0x25, 0xa6, 0x5b, 0x25, 0xcf, 0x50, 0x30, 0x82, 0x68, 0x30, 0xb6,
//!     0x61,
//! ];
//! const PLAINTEXT: &[u8] = &[0x43, 0xac, 0xff, 0x29, 0x31, 0x20, 0xdd, 0x5d];
//!
//! let kek = AesKek::new(&AES_128, KEY)?;
//!
//! let mut output = vec![0u8; PLAINTEXT.len() + 15];
//!
//! let ciphertext = kek.wrap_with_padding(PLAINTEXT, &mut output)?;
//!
//! let kek = AesKek::new(&AES_128, KEY)?;
//!
//! let mut output = vec![0u8; ciphertext.len()];
//!
//! let plaintext = kek.unwrap_with_padding(&*ciphertext, &mut output)?;
//!
//! assert_eq!(PLAINTEXT, plaintext);
//! # Ok(())
//! # }
//! ```

use crate::aws_lc::{
    AES_set_decrypt_key, AES_set_encrypt_key, AES_unwrap_key, AES_unwrap_key_padded, AES_wrap_key,
    AES_wrap_key_padded, AES_KEY,
};
use crate::error::Unspecified;
use crate::fips::indicator_check;
use crate::sealed::Sealed;
use core::fmt::Debug;
use core::mem::MaybeUninit;
use core::ptr::null;

mod tests;

/// The Key Wrapping Algorithm Identifier
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
pub enum BlockCipherId {
    /// AES Block Cipher with 128-bit key.
    Aes128,

    /// AES Block Cipher with 256-bit key.
    Aes256,
}

/// A key wrap block cipher.
pub trait BlockCipher: 'static + Debug + Sealed {
    /// The block cipher identifier.
    fn id(&self) -> BlockCipherId;

    /// The key size in bytes to be used with the block cipher.
    fn key_len(&self) -> usize;
}

/// An AES Block Cipher
pub struct AesBlockCipher {
    id: BlockCipherId,
    key_len: usize,
}

impl BlockCipher for AesBlockCipher {
    /// Returns the algorithm identifier.
    #[inline]
    fn id(&self) -> BlockCipherId {
        self.id
    }

    /// Returns the algorithm key length.
    #[inline]
    fn key_len(&self) -> usize {
        self.key_len
    }
}

impl Sealed for AesBlockCipher {}

impl Debug for AesBlockCipher {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.id, f)
    }
}

/// AES Block Cipher with 128-bit key.
pub const AES_128: AesBlockCipher = AesBlockCipher {
    id: BlockCipherId::Aes128,
    key_len: 16,
};

/// AES Block Cipher with 256-bit key.
pub const AES_256: AesBlockCipher = AesBlockCipher {
    id: BlockCipherId::Aes256,
    key_len: 32,
};

/// A Key Wrap (KW) algorithm implementation.
#[allow(clippy::module_name_repetitions)]
pub trait KeyWrap: Sealed {
    /// Peforms the key wrap encryption algorithm using a block cipher.
    /// It wraps `plaintext` and writes the corresponding ciphertext to `output`.
    ///
    /// # Errors
    /// * [`Unspecified`]: Any error that has occurred performing the operation.
    fn wrap<'output>(
        self,
        plaintext: &[u8],
        output: &'output mut [u8],
    ) -> Result<&'output mut [u8], Unspecified>;

    /// Peforms the key wrap decryption algorithm using a block cipher.
    /// It unwraps `ciphertext` and writes the corresponding plaintext to `output`.
    ///
    /// # Errors
    /// * [`Unspecified`]: Any error that has occurred performing the operation.
    fn unwrap<'output>(
        self,
        ciphertext: &[u8],
        output: &'output mut [u8],
    ) -> Result<&'output mut [u8], Unspecified>;
}

/// A Key Wrap with Padding (KWP) algorithm implementation.
#[allow(clippy::module_name_repetitions)]
pub trait KeyWrapPadded: Sealed {
    /// Peforms the key wrap padding encryption algorithm using a block cipher.
    /// It wraps and pads `plaintext` writes the corresponding ciphertext to `output`.
    ///
    /// # Errors
    /// * [`Unspecified`]: Any error that has occurred performing the operation.
    fn wrap_with_padding<'output>(
        self,
        plaintext: &[u8],
        output: &'output mut [u8],
    ) -> Result<&'output mut [u8], Unspecified>;

    /// Peforms the key wrap padding decryption algorithm using a block cipher.
    /// It unwraps the padded `ciphertext` and writes the corresponding plaintext to `output`.
    ///
    /// # Errors
    /// * [`Unspecified`]: Any error that has occurred performing the operation.
    fn unwrap_with_padding<'output>(
        self,
        ciphertext: &[u8],
        output: &'output mut [u8],
    ) -> Result<&'output mut [u8], Unspecified>;
}

/// AES Key Encryption Key.
pub type AesKek = KeyEncryptionKey<AesBlockCipher>;

/// The key-encryption key used with the selected cipher algorithn to wrap or unwrap a key.
///
/// Implements the NIST SP 800-38F key wrapping algoirthm.
///
/// The NIST specification is similar to that of RFC 3394 but with the following caveats:
/// * Specifies a maxiumum plaintext length that can be accepted.
/// * Allows implementations to specify a subset of valid lengths accepted.
/// * Allows for the usage of other 128-bit block ciphers other than AES.
pub struct KeyEncryptionKey<Cipher: BlockCipher> {
    cipher: &'static Cipher,
    key: Box<[u8]>,
}

impl<Cipher: BlockCipher> KeyEncryptionKey<Cipher> {
    /// Construct a new Key Encryption Key.
    ///
    /// # Errors
    /// * [`Unspecified`]: Any error that occurs constructing the key encryption key.
    pub fn new(cipher: &'static Cipher, key: &[u8]) -> Result<Self, Unspecified> {
        if key.len() != cipher.key_len() {
            return Err(Unspecified);
        }

        let key = Vec::from(key).into_boxed_slice();

        Ok(Self { cipher, key })
    }

    /// Returns the block cipher algorithm identifier configured for the key.
    #[must_use]
    pub fn block_cipher_id(&self) -> BlockCipherId {
        self.cipher.id()
    }
}

impl<Cipher: BlockCipher> Sealed for KeyEncryptionKey<Cipher> {}

impl KeyWrap for KeyEncryptionKey<AesBlockCipher> {
    /// Peforms the key wrap encryption algorithm using `KeyEncryptionKey`'s configured block cipher.
    /// It wraps `plaintext` and writes the corresponding ciphertext to `output`.
    ///
    /// # Validation
    /// * `plaintext.len()` must be a multiple of eight
    /// * `output.len() >= (input.len() + 8)`
    ///
    /// # Errors
    /// * [`Unspecified`]: An error occurred either due to `output` being insufficiently sized, `input` exceeding
    ///   the allowed input size, or for other unspecified reasons.
    fn wrap<'output>(
        self,
        plaintext: &[u8],
        output: &'output mut [u8],
    ) -> Result<&'output mut [u8], Unspecified> {
        if output.len() < plaintext.len() + 8 {
            return Err(Unspecified);
        }

        let mut aes_key = MaybeUninit::<AES_KEY>::uninit();

        let key_bits: u32 = (self.key.len() * 8).try_into().map_err(|_| Unspecified)?;

        if 0 != unsafe { AES_set_encrypt_key(self.key.as_ptr(), key_bits, aes_key.as_mut_ptr()) } {
            return Err(Unspecified);
        }

        let aes_key = unsafe { aes_key.assume_init() };

        // AWS-LC validates the following:
        // * in_len <= INT_MAX - 8
        // * in_len >= 16
        // * in_len % 8 == 0
        let out_len = indicator_check!(unsafe {
            AES_wrap_key(
                &aes_key,
                null(),
                output.as_mut_ptr(),
                plaintext.as_ptr(),
                plaintext.len(),
            )
        });

        if out_len == -1 {
            return Err(Unspecified);
        }

        let out_len: usize = out_len.try_into().map_err(|_| Unspecified)?;

        debug_assert_eq!(out_len, plaintext.len() + 8);

        Ok(&mut output[..out_len])
    }

    /// Peforms the key wrap decryption algorithm using `KeyEncryptionKey`'s configured block cipher.
    /// It unwraps `ciphertext` and writes the corresponding plaintext to `output`.
    ///
    /// # Validation
    /// * `ciphertext.len()` must be a multiple of 8
    /// * `output.len() >= (input.len() - 8)`
    ///
    /// # Errors
    /// * [`Unspecified`]: An error occurred either due to `output` being insufficiently sized, `input` exceeding
    ///   the allowed input size, or for other unspecified reasons.
    fn unwrap<'output>(
        self,
        ciphertext: &[u8],
        output: &'output mut [u8],
    ) -> Result<&'output mut [u8], Unspecified> {
        if output.len() < ciphertext.len() - 8 {
            return Err(Unspecified);
        }

        let mut aes_key = MaybeUninit::<AES_KEY>::uninit();

        if 0 != unsafe {
            AES_set_decrypt_key(
                self.key.as_ptr(),
                (self.key.len() * 8).try_into().map_err(|_| Unspecified)?,
                aes_key.as_mut_ptr(),
            )
        } {
            return Err(Unspecified);
        }

        let aes_key = unsafe { aes_key.assume_init() };

        // AWS-LC validates the following:
        // * in_len < INT_MAX
        // * in_len > 24
        // * in_len % 8 == 0
        let out_len = indicator_check!(unsafe {
            AES_unwrap_key(
                &aes_key,
                null(),
                output.as_mut_ptr(),
                ciphertext.as_ptr(),
                ciphertext.len(),
            )
        });

        if out_len == -1 {
            return Err(Unspecified);
        }

        let out_len: usize = out_len.try_into().map_err(|_| Unspecified)?;

        debug_assert_eq!(out_len, ciphertext.len() - 8);

        Ok(&mut output[..out_len])
    }
}

impl KeyWrapPadded for KeyEncryptionKey<AesBlockCipher> {
    /// Peforms the key wrap padding encryption algorithm using `KeyEncryptionKey`'s configured block cipher.
    /// It wraps and pads `plaintext` writes the corresponding ciphertext to `output`.
    ///
    /// # Validation
    /// * `output.len() >= (input.len() + 15)`
    ///
    /// # Errors
    /// * [`Unspecified`]: An error occurred either due to `output` being insufficiently sized, `input` exceeding
    ///   the allowed input size, or for other unspecified reasons.
    fn wrap_with_padding<'output>(
        self,
        plaintext: &[u8],
        output: &'output mut [u8],
    ) -> Result<&'output mut [u8], Unspecified> {
        let mut aes_key = MaybeUninit::<AES_KEY>::uninit();

        let key_bits: u32 = (self.key.len() * 8).try_into().map_err(|_| Unspecified)?;

        if 0 != unsafe { AES_set_encrypt_key(self.key.as_ptr(), key_bits, aes_key.as_mut_ptr()) } {
            return Err(Unspecified);
        }

        let aes_key = unsafe { aes_key.assume_init() };

        let mut out_len: usize = 0;

        // AWS-LC validates the following:
        // * in_len != 0
        // * in_len <= INT_MAX
        // * max_out >= required_padding + 8
        if 1 != indicator_check!(unsafe {
            AES_wrap_key_padded(
                &aes_key,
                output.as_mut_ptr(),
                &mut out_len,
                output.len(),
                plaintext.as_ptr(),
                plaintext.len(),
            )
        }) {
            return Err(Unspecified);
        }

        Ok(&mut output[..out_len])
    }

    /// Peforms the key wrap padding decryption algorithm using `KeyEncryptionKey`'s configured block cipher.
    /// It unwraps the padded `ciphertext` and writes the corresponding plaintext to `output`.
    ///
    /// # Sizing `output`
    /// `output.len() >= input.len()`.
    ///
    /// # Errors
    /// * [`Unspecified`]: An error occurred either due to `output` being insufficiently sized, `input` exceeding
    ///   the allowed input size, or for other unspecified reasons.
    fn unwrap_with_padding<'output>(
        self,
        ciphertext: &[u8],
        output: &'output mut [u8],
    ) -> Result<&'output mut [u8], Unspecified> {
        let mut aes_key = MaybeUninit::<AES_KEY>::uninit();

        if 0 != unsafe {
            AES_set_decrypt_key(
                self.key.as_ptr(),
                (self.key.len() * 8).try_into().map_err(|_| Unspecified)?,
                aes_key.as_mut_ptr(),
            )
        } {
            return Err(Unspecified);
        }

        let aes_key = unsafe { aes_key.assume_init() };

        let mut out_len: usize = 0;

        // AWS-LC validates the following:
        // * in_len >= AES_BLOCK_SIZE
        // * max_out >= in_len - 8
        if 1 != indicator_check!(unsafe {
            AES_unwrap_key_padded(
                &aes_key,
                output.as_mut_ptr(),
                &mut out_len,
                output.len(),
                ciphertext.as_ptr(),
                ciphertext.len(),
            )
        }) {
            return Err(Unspecified);
        }

        Ok(&mut output[..out_len])
    }
}

impl<Cipher: BlockCipher> Debug for KeyEncryptionKey<Cipher> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("KeyEncryptionKey")
            .field("cipher", &self.cipher)
            .finish_non_exhaustive()
    }
}
