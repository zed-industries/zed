// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#![allow(clippy::module_name_repetitions)]

use super::{PrivateDecryptingKey, PublicEncryptingKey};
use crate::aws_lc::{
    EVP_PKEY_CTX_set_rsa_padding, EVP_PKEY_decrypt, EVP_PKEY_decrypt_init, EVP_PKEY_encrypt,
    EVP_PKEY_encrypt_init, EVP_PKEY_CTX, RSA_PKCS1_PADDING,
};
use crate::error::Unspecified;
use crate::fips::indicator_check;
use crate::ptr::LcPtr;
use core::fmt::Debug;

/// RSA PKCS1-v1.5 public key for encryption.
pub struct Pkcs1PublicEncryptingKey {
    public_key: PublicEncryptingKey,
}

impl Pkcs1PublicEncryptingKey {
    /// Constructs an `Pkcs1PublicEncryptingKey` from a `PublicEncryptingKey`.
    /// # Errors
    /// * `Unspecified`: Any error that occurs while attempting to construct an RSA-OAEP public key.
    pub fn new(public_key: PublicEncryptingKey) -> Result<Self, Unspecified> {
        Ok(Self { public_key })
    }

    /// Encrypts the contents in `plaintext` and writes the corresponding ciphertext to `ciphertext`.
    /// Returns the subslice of `ciphertext` containing the ciphertext output.
    ///
    /// # Max Plaintext Length
    /// The provided length of `plaintext` must be at most [`Self::max_plaintext_size`].
    ///
    /// # Sizing `output`
    /// The length of `output` must be greater than or equal to [`Self::ciphertext_size`].
    ///
    /// # Errors
    /// * `Unspecified` for any error that occurs while encrypting `plaintext`.
    pub fn encrypt<'ciphertext>(
        &self,
        plaintext: &[u8],
        ciphertext: &'ciphertext mut [u8],
    ) -> Result<&'ciphertext mut [u8], Unspecified> {
        let mut pkey_ctx = self.public_key.0.create_EVP_PKEY_CTX()?;

        if 1 != unsafe { EVP_PKEY_encrypt_init(pkey_ctx.as_mut_ptr()) } {
            return Err(Unspecified);
        }

        configure_pkcs1_crypto_operation(&mut pkey_ctx)?;

        let mut out_len = ciphertext.len();

        if 1 != indicator_check!(unsafe {
            EVP_PKEY_encrypt(
                pkey_ctx.as_mut_ptr(),
                ciphertext.as_mut_ptr(),
                &mut out_len,
                plaintext.as_ptr(),
                plaintext.len(),
            )
        }) {
            return Err(Unspecified);
        }

        Ok(&mut ciphertext[..out_len])
    }

    /// Returns the RSA key size in bytes.
    #[must_use]
    pub fn key_size_bytes(&self) -> usize {
        self.public_key.key_size_bytes()
    }

    /// Returns the RSA key size in bits.
    #[must_use]
    pub fn key_size_bits(&self) -> usize {
        self.public_key.key_size_bits()
    }

    /// Returns the max plaintext that could be encrypted using this key.
    #[must_use]
    pub fn max_plaintext_size(&self) -> usize {
        const RSA_PKCS1_PADDING_SIZE: usize = 11; // crypto/fipsmodule/rsa/internal.h
        self.key_size_bytes() - RSA_PKCS1_PADDING_SIZE
    }

    /// Returns the max ciphertext size that will be output by `Self::encrypt`.
    #[must_use]
    pub fn ciphertext_size(&self) -> usize {
        self.key_size_bytes()
    }
}

impl Debug for Pkcs1PublicEncryptingKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Pkcs1PublicEncryptingKey")
            .finish_non_exhaustive()
    }
}

/// RSA PKCS1-v1.5 private key for decryption.
pub struct Pkcs1PrivateDecryptingKey {
    private_key: PrivateDecryptingKey,
}

impl Pkcs1PrivateDecryptingKey {
    /// Constructs an `Pkcs1PrivateDecryptingKey` from a `PrivateDecryptingKey`.
    /// # Errors
    /// * `Unspecified`: Any error that occurs while attempting to construct an RSA-OAEP public key.
    pub fn new(private_key: PrivateDecryptingKey) -> Result<Self, Unspecified> {
        Ok(Self { private_key })
    }

    /// Decrypts the contents in `ciphertext` and writes the corresponding plaintext to `plaintext`.
    /// Returns the subslice of `plaintext` containing the plaintext output.
    ///
    /// # Max Ciphertext Length
    /// The provided length of `ciphertext` must be [`Self::key_size_bytes`].
    ///
    /// # Sizing `output`
    /// The length of `output` must be greater than or equal to [`Self::min_output_size`].
    ///
    /// # Errors
    /// * `Unspecified` for any error that occurs while decrypting `ciphertext`.
    pub fn decrypt<'plaintext>(
        &self,
        ciphertext: &[u8],
        plaintext: &'plaintext mut [u8],
    ) -> Result<&'plaintext mut [u8], Unspecified> {
        let mut pkey_ctx = self.private_key.0.create_EVP_PKEY_CTX()?;

        if 1 != unsafe { EVP_PKEY_decrypt_init(pkey_ctx.as_mut_ptr()) } {
            return Err(Unspecified);
        }

        configure_pkcs1_crypto_operation(&mut pkey_ctx)?;

        let mut out_len = plaintext.len();

        if 1 != indicator_check!(unsafe {
            EVP_PKEY_decrypt(
                pkey_ctx.as_mut_ptr(),
                plaintext.as_mut_ptr(),
                &mut out_len,
                ciphertext.as_ptr(),
                ciphertext.len(),
            )
        }) {
            return Err(Unspecified);
        }

        Ok(&mut plaintext[..out_len])
    }

    /// Returns the RSA key size in bytes.
    #[must_use]
    pub fn key_size_bytes(&self) -> usize {
        self.private_key.key_size_bytes()
    }

    /// Returns the RSA key size in bits.
    #[must_use]
    pub fn key_size_bits(&self) -> usize {
        self.private_key.key_size_bits()
    }

    /// Returns the minimum plaintext buffer size required for `Self::decrypt`.
    #[must_use]
    pub fn min_output_size(&self) -> usize {
        self.key_size_bytes()
    }
}

impl Debug for Pkcs1PrivateDecryptingKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Pkcs1PrivateDecryptingKey")
            .finish_non_exhaustive()
    }
}

fn configure_pkcs1_crypto_operation(
    evp_pkey_ctx: &mut LcPtr<EVP_PKEY_CTX>,
) -> Result<(), Unspecified> {
    if 1 != unsafe { EVP_PKEY_CTX_set_rsa_padding(evp_pkey_ctx.as_mut_ptr(), RSA_PKCS1_PADDING) } {
        return Err(Unspecified);
    }

    Ok(())
}
