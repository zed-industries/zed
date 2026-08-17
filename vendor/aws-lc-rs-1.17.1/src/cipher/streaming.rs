// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use crate::aws_lc::{
    EVP_CIPHER_CTX_new, EVP_CIPHER_iv_length, EVP_CIPHER_key_length, EVP_DecryptFinal_ex,
    EVP_DecryptInit_ex, EVP_DecryptUpdate, EVP_EncryptFinal_ex, EVP_EncryptInit_ex,
    EVP_EncryptUpdate, EVP_CIPHER, EVP_CIPHER_CTX,
};
use crate::cipher::{
    Algorithm, DecryptionContext, EncryptionContext, OperatingMode, UnboundCipherKey,
};
use crate::error::Unspecified;
use crate::fips::indicator_check;
use crate::ptr::LcPtr;
use std::ptr::{null, null_mut};

use super::ConstPointer;

/// A key for streaming encryption operations.
pub struct StreamingEncryptingKey {
    algorithm: &'static Algorithm,
    mode: OperatingMode,
    cipher_ctx: LcPtr<EVP_CIPHER_CTX>,
    context: EncryptionContext,
    output_generated: usize,
}

unsafe impl Send for StreamingEncryptingKey {}

/// A struct indicating the portion of a buffer written to, and/or not written to, during an
/// encryption/decryption operation.
pub struct BufferUpdate<'a> {
    written: &'a [u8],
    remainder: &'a mut [u8],
}

impl<'a> BufferUpdate<'a> {
    fn new(out_buffer: &'a mut [u8], written_len: usize) -> Self {
        let (written, remainder) = out_buffer.split_at_mut(written_len);
        Self { written, remainder }
    }
}

impl BufferUpdate<'_> {
    /// Returns the slice from the buffer that was modified by the operation.
    #[must_use]
    pub fn written(&self) -> &[u8] {
        self.written
    }

    /// Returns the slice of the buffer that was not modified by the operation.
    #[must_use]
    pub fn remainder(&self) -> &[u8] {
        self.remainder
    }

    /// Returns a mutable slice of the buffer that was not modified by the operation.
    #[must_use]
    pub fn remainder_mut(&mut self) -> &mut [u8] {
        self.remainder
    }
}

fn evp_encrypt_init(
    cipher_ctx: &mut LcPtr<EVP_CIPHER_CTX>,
    cipher: &ConstPointer<EVP_CIPHER>,
    key: &[u8],
    iv: Option<&[u8]>,
) -> Result<(), Unspecified> {
    let iv_ptr: *const u8 = if let Some(iv) = iv {
        iv.as_ptr()
    } else {
        null()
    };

    // AWS-LC copies the key and iv values into the EVP_CIPHER_CTX, and thus can be dropped after this.
    if 1 != unsafe {
        EVP_EncryptInit_ex(
            cipher_ctx.as_mut_ptr(),
            cipher.as_const_ptr(),
            null_mut(),
            key.as_ptr(),
            iv_ptr,
        )
    } {
        return Err(Unspecified);
    }

    Ok(())
}

fn evp_decrypt_init(
    cipher_ctx: &mut LcPtr<EVP_CIPHER_CTX>,
    cipher: &ConstPointer<EVP_CIPHER>,
    key: &[u8],
    iv: Option<&[u8]>,
) -> Result<(), Unspecified> {
    let iv_ptr: *const u8 = if let Some(iv) = iv {
        iv.as_ptr()
    } else {
        null()
    };

    // AWS-LC copies the key and iv values into the EVP_CIPHER_CTX, and thus can be dropped after this.
    if 1 != unsafe {
        EVP_DecryptInit_ex(
            cipher_ctx.as_mut_ptr(),
            cipher.as_const_ptr(),
            null_mut(),
            key.as_ptr(),
            iv_ptr,
        )
    } {
        return Err(Unspecified);
    }

    Ok(())
}

impl StreamingEncryptingKey {
    #[allow(clippy::needless_pass_by_value)]
    fn new(
        key: UnboundCipherKey,
        mode: OperatingMode,
        context: EncryptionContext,
    ) -> Result<Self, Unspecified> {
        let algorithm = key.algorithm();
        if !algorithm.supports_mode(mode) {
            return Err(Unspecified);
        }
        // The streaming path passes raw key bytes to the EVP API rather than
        // going through `SymmetricCipherKey` construction.  Validate
        // algorithm-specific key constraints (e.g. DES weak-key / K1!=K2
        // checks) that would otherwise be missed.
        key.validate_key_material()?;
        let mut cipher_ctx = LcPtr::new(unsafe { EVP_CIPHER_CTX_new() })?;
        let cipher = mode.evp_cipher(key.algorithm);
        let key_bytes = key.key_bytes.as_ref();
        if key_bytes.len()
            != <usize>::try_from(unsafe { EVP_CIPHER_key_length(cipher.as_const_ptr()) }).unwrap()
        {
            return Err(Unspecified);
        }

        match &context {
            ctx @ EncryptionContext::Iv128(..) => {
                let iv = <&[u8]>::try_from(ctx)?;
                debug_assert_eq!(
                    iv.len(),
                    <usize>::try_from(unsafe { EVP_CIPHER_iv_length(cipher.as_const_ptr()) })
                        .unwrap()
                );
                evp_encrypt_init(&mut cipher_ctx, &cipher, key_bytes, Some(iv))?;
            }
            #[cfg(feature = "legacy-des")]
            ctx @ EncryptionContext::Iv64(..) => {
                let iv = <&[u8]>::try_from(ctx)?;
                debug_assert_eq!(
                    iv.len(),
                    <usize>::try_from(unsafe { EVP_CIPHER_iv_length(cipher.as_const_ptr()) })
                        .unwrap()
                );
                evp_encrypt_init(&mut cipher_ctx, &cipher, key_bytes, Some(iv))?;
            }
            EncryptionContext::None => {
                evp_encrypt_init(&mut cipher_ctx, &cipher, key_bytes, None)?;
            }
        }

        Ok(Self {
            algorithm,
            mode,
            cipher_ctx,
            context,
            output_generated: 0,
        })
    }

    fn update_internal<'a>(
        &mut self,
        input: &[u8],
        output: &'a mut [u8],
        min_outsize: usize,
    ) -> Result<BufferUpdate<'a>, Unspecified> {
        if output.len() < min_outsize {
            return Err(Unspecified);
        }
        let mut outlen: i32 = 0;
        let inlen: i32 = input.len().try_into()?;

        if 1 != unsafe {
            EVP_EncryptUpdate(
                self.cipher_ctx.as_mut_ptr(),
                output.as_mut_ptr(),
                &mut outlen,
                input.as_ptr(),
                inlen,
            )
        } {
            return Err(Unspecified);
        }
        let outlen: usize = outlen.try_into()?;
        debug_assert!(outlen <= min_outsize);
        self.output_generated += outlen;
        assert!(outlen <= output.len());

        Ok(BufferUpdate::new(output, outlen))
    }

    /// Updates the internal state of the key with the provided plaintext `input`,
    /// potentially writing bytes of ciphertext to `output`.
    ///
    /// The number of bytes written to `output` can be up to `input.len()`
    /// plus the block length of the algorithm (e.g., [`Algorithm::block_len`]) minus one.
    ///
    /// # Errors
    /// * Returns an error if the `output` buffer is smaller than the length of
    ///   the `input` plus the algorithm's block length (e.g. [`Algorithm::block_len`]) minus one.
    /// * May return an error if the length of `input` plus the algorithm's block length is larger than `i32::MAX`.
    pub fn update<'a>(
        &mut self,
        input: &[u8],
        output: &'a mut [u8],
    ) -> Result<BufferUpdate<'a>, Unspecified> {
        let min_outsize = input
            .len()
            .checked_add(self.algorithm().block_len())
            .ok_or(Unspecified)?
            .checked_sub(1)
            .ok_or(Unspecified)?;
        self.update_internal(input, output, min_outsize)
    }

    /// Updates the internal state of the key with the provided plaintext `input`,
    /// potentially writing bytes of ciphertext to `output`.
    ///
    /// This function has looser output buffer size requirements than [`Self::update`],
    /// calculating the minimum required size based on the total bytes of output generated
    /// and the cipher's block length. This is considered "less safe" because it's
    /// based on assumptions about the state of the underlying operations.
    ///
    /// The minimum output buffer size is calculated based on how many bytes are needed to
    /// reach the next block boundary after processing the input. If `next_total` is the sum
    /// of bytes already generated plus `input.len()`, then the minimum size is:
    /// `input.len() + ((block_len - (next_total % block_len)) % block_len)`
    ///
    /// # Errors
    /// Returns an error if the `output` buffer is smaller than the calculated minimum size,
    /// if the total output length overflows, or if the length of `input` is larger than
    /// `i32::MAX`.
    ///
    /// # Panics
    /// Panics if the number of bytes written by the cipher operation exceeds the output
    /// buffer length.
    pub fn less_safe_update<'a>(
        &mut self,
        input: &[u8],
        output: &'a mut [u8],
    ) -> Result<BufferUpdate<'a>, Unspecified> {
        let next_total = self
            .output_generated
            .checked_add(input.len())
            .ok_or(Unspecified)?;
        let extra_buffer_size = (self.algorithm().block_len
            - next_total.rem_euclid(self.algorithm().block_len))
        .rem_euclid(self.algorithm().block_len);
        let min_outsize = input
            .len()
            .checked_add(extra_buffer_size)
            .ok_or(Unspecified)?;
        self.update_internal(input, output, min_outsize)
    }

    /// Finishes the encryption operation, writing any remaining ciphertext to
    /// `output`.
    ///
    /// The number of bytes written to `output` can be up to the block length of
    /// [`Algorithm::block_len`].
    ///
    /// # Errors
    /// * Returns an error if the `output` buffer is smaller than the algorithm's
    ///   block length.
    pub fn finish(
        mut self,
        output: &mut [u8],
    ) -> Result<(DecryptionContext, BufferUpdate<'_>), Unspecified> {
        if output.len() < self.algorithm().block_len() {
            return Err(Unspecified);
        }
        let mut outlen: i32 = 0;

        if 1 != indicator_check!(unsafe {
            EVP_EncryptFinal_ex(
                self.cipher_ctx.as_mut_ptr(),
                output.as_mut_ptr(),
                &mut outlen,
            )
        }) {
            return Err(Unspecified);
        }
        let outlen: usize = outlen.try_into()?;
        debug_assert!(outlen <= self.algorithm().block_len());
        Ok((self.context.into(), BufferUpdate::new(output, outlen)))
    }

    /// Returns the cipher operating mode.
    #[must_use]
    pub fn mode(&self) -> OperatingMode {
        self.mode
    }

    /// Returns the cipher algorithm.
    #[must_use]
    pub fn algorithm(&self) -> &'static Algorithm {
        self.algorithm
    }

    /// Constructs a `StreamingEncryptingKey` for encrypting data using the CTR cipher mode.
    /// The resulting ciphertext will be the same length as the plaintext.
    ///
    /// # Errors
    /// Returns an error on an internal failure. With `legacy-des` enabled, also
    /// returned if `key`'s algorithm does not support CTR mode (e.g.
    /// `DES_FOR_LEGACY_USE_ONLY`, `DES_EDE_FOR_LEGACY_USE_ONLY`,
    /// `DES_EDE3_FOR_LEGACY_USE_ONLY`).
    pub fn ctr(key: UnboundCipherKey) -> Result<Self, Unspecified> {
        let context = key.algorithm().new_encryption_context(OperatingMode::CTR)?;
        Self::less_safe_ctr(key, context)
    }

    /// Constructs a `StreamingEncryptingKey` for encrypting data using the CTR cipher mode.
    /// The resulting ciphertext will be the same length as the plaintext.
    ///
    /// This is considered less safe because the caller could potentially construct
    /// an `EncryptionContext` from a previously used initialization vector (IV).
    ///
    /// # Errors
    /// Returns an error on an internal failure. With `legacy-des` enabled, also
    /// returned if `key`'s algorithm does not support CTR mode (e.g.
    /// `DES_FOR_LEGACY_USE_ONLY`, `DES_EDE_FOR_LEGACY_USE_ONLY`,
    /// `DES_EDE3_FOR_LEGACY_USE_ONLY`).
    pub fn less_safe_ctr(
        key: UnboundCipherKey,
        context: EncryptionContext,
    ) -> Result<Self, Unspecified> {
        Self::new(key, OperatingMode::CTR, context)
    }

    /// Constructs a `StreamingEncryptingKey` for encrypting data using the CBC cipher mode
    /// with pkcs7 padding.
    /// The resulting ciphertext will be longer than the plaintext; padding is added
    /// to fill the next block of ciphertext.
    ///
    /// # Errors
    /// Returns an error on an internal failure. With `legacy-des` enabled, also
    /// returned if `key` was constructed with `DES_FOR_LEGACY_USE_ONLY`,
    /// `DES_EDE_FOR_LEGACY_USE_ONLY` or `DES_EDE3_FOR_LEGACY_USE_ONLY` and the
    /// provided key material contains weak or semi-weak DES subkeys, or (for
    /// Triple DES) a degenerate subkey configuration (e.g. `K1 == K2` for 2TDEA,
    /// or any pairwise equality for 3TDEA).
    pub fn cbc_pkcs7(key: UnboundCipherKey) -> Result<Self, Unspecified> {
        let context = key.algorithm().new_encryption_context(OperatingMode::CBC)?;
        Self::less_safe_cbc_pkcs7(key, context)
    }

    /// Constructs a `StreamingEncryptingKey` for encrypting data using the CFB128 cipher mode.
    /// The resulting ciphertext will be the same length as the plaintext.
    ///
    /// # Errors
    /// Returns an error on an internal failure. With `legacy-des` enabled, also
    /// returned if `key`'s algorithm does not support CFB128 mode (e.g.
    /// `DES_FOR_LEGACY_USE_ONLY`, `DES_EDE_FOR_LEGACY_USE_ONLY`,
    /// `DES_EDE3_FOR_LEGACY_USE_ONLY`).
    pub fn cfb128(key: UnboundCipherKey) -> Result<Self, Unspecified> {
        let context = key
            .algorithm()
            .new_encryption_context(OperatingMode::CFB128)?;
        Self::less_safe_cfb128(key, context)
    }

    /// Constructs a `StreamingEncryptingKey` for encrypting using ECB cipher mode with PKCS7 padding.
    /// The resulting plaintext will be the same length as the ciphertext.
    ///
    /// # ☠️ ️️️DANGER ☠️
    /// Offered for computability purposes only. This is an extremely dangerous mode, and
    /// very likely not what you want to use.
    ///
    /// # Errors
    /// Returns an error on an internal failure. With `legacy-des` enabled, also
    /// returned if `key` was constructed with `DES_FOR_LEGACY_USE_ONLY`,
    /// `DES_EDE_FOR_LEGACY_USE_ONLY` or `DES_EDE3_FOR_LEGACY_USE_ONLY` and the
    /// provided key material contains weak or semi-weak DES subkeys, or (for
    /// Triple DES) a degenerate subkey configuration (e.g. `K1 == K2` for 2TDEA,
    /// or any pairwise equality for 3TDEA).
    pub fn ecb_pkcs7(key: UnboundCipherKey) -> Result<Self, Unspecified> {
        let context = key.algorithm().new_encryption_context(OperatingMode::ECB)?;
        Self::new(key, OperatingMode::ECB, context)
    }

    /// Constructs a `StreamingEncryptingKey` for encrypting data using the CFB128 cipher mode.
    /// The resulting ciphertext will be the same length as the plaintext.
    ///
    /// This is considered less safe because the caller could potentially construct
    /// an `EncryptionContext` from a previously used initialization vector (IV).
    ///
    /// # Errors
    /// Returns an error on an internal failure. With `legacy-des` enabled, also
    /// returned if `key`'s algorithm does not support CFB128 mode (e.g.
    /// `DES_FOR_LEGACY_USE_ONLY`, `DES_EDE_FOR_LEGACY_USE_ONLY`,
    /// `DES_EDE3_FOR_LEGACY_USE_ONLY`).
    pub fn less_safe_cfb128(
        key: UnboundCipherKey,
        context: EncryptionContext,
    ) -> Result<Self, Unspecified> {
        Self::new(key, OperatingMode::CFB128, context)
    }

    /// Constructs a `StreamingEncryptingKey` for encrypting data using the CBC cipher mode
    /// with pkcs7 padding.
    /// The resulting ciphertext will be longer than the plaintext; padding is added
    /// to fill the next block of ciphertext.
    ///
    /// This is considered less safe because the caller could potentially construct
    /// an `EncryptionContext` from a previously used initialization vector (IV).
    ///
    /// # Errors
    /// Returns an error on an internal failure. With `legacy-des` enabled, also
    /// returned if `key` was constructed with `DES_FOR_LEGACY_USE_ONLY`,
    /// `DES_EDE_FOR_LEGACY_USE_ONLY` or `DES_EDE3_FOR_LEGACY_USE_ONLY` and the
    /// provided key material contains weak or semi-weak DES subkeys, or (for
    /// Triple DES) a degenerate subkey configuration (e.g. `K1 == K2` for 2TDEA,
    /// or any pairwise equality for 3TDEA).
    pub fn less_safe_cbc_pkcs7(
        key: UnboundCipherKey,
        context: EncryptionContext,
    ) -> Result<Self, Unspecified> {
        Self::new(key, OperatingMode::CBC, context)
    }
}

/// A key for streaming decryption operations.
pub struct StreamingDecryptingKey {
    algorithm: &'static Algorithm,
    mode: OperatingMode,
    cipher_ctx: LcPtr<EVP_CIPHER_CTX>,
    output_generated: usize,
}

unsafe impl Send for StreamingDecryptingKey {}

impl StreamingDecryptingKey {
    #[allow(clippy::needless_pass_by_value)]
    fn new(
        key: UnboundCipherKey,
        mode: OperatingMode,
        context: DecryptionContext,
    ) -> Result<Self, Unspecified> {
        let algorithm = key.algorithm();
        if !algorithm.supports_mode(mode) {
            return Err(Unspecified);
        }
        // See comment in `StreamingEncryptingKey::new`.
        key.validate_key_material()?;
        let mut cipher_ctx = LcPtr::new(unsafe { EVP_CIPHER_CTX_new() })?;
        let cipher = mode.evp_cipher(key.algorithm);
        let key_bytes = key.key_bytes.as_ref();
        if key_bytes.len()
            != <usize>::try_from(unsafe { EVP_CIPHER_key_length(cipher.as_const_ptr()) }).unwrap()
        {
            return Err(Unspecified);
        }

        match &context {
            ctx @ DecryptionContext::Iv128(..) => {
                let iv = <&[u8]>::try_from(ctx)?;
                debug_assert_eq!(
                    iv.len(),
                    <usize>::try_from(unsafe { EVP_CIPHER_iv_length(cipher.as_const_ptr()) })
                        .unwrap()
                );
                evp_decrypt_init(&mut cipher_ctx, &cipher, key_bytes, Some(iv))?;
            }
            #[cfg(feature = "legacy-des")]
            ctx @ DecryptionContext::Iv64(..) => {
                let iv = <&[u8]>::try_from(ctx)?;
                debug_assert_eq!(
                    iv.len(),
                    <usize>::try_from(unsafe { EVP_CIPHER_iv_length(cipher.as_const_ptr()) })
                        .unwrap()
                );
                evp_decrypt_init(&mut cipher_ctx, &cipher, key_bytes, Some(iv))?;
            }
            DecryptionContext::None => {
                evp_decrypt_init(&mut cipher_ctx, &cipher, key_bytes, None)?;
            }
        }

        Ok(Self {
            algorithm,
            mode,
            cipher_ctx,
            output_generated: 0,
        })
    }

    fn update_internal<'a>(
        &mut self,
        input: &[u8],
        output: &'a mut [u8],
        min_outsize: usize,
    ) -> Result<BufferUpdate<'a>, Unspecified> {
        if output.len() < min_outsize {
            return Err(Unspecified);
        }
        let mut outlen: i32 = 0;
        let inlen: i32 = input.len().try_into()?;

        if 1 != unsafe {
            EVP_DecryptUpdate(
                self.cipher_ctx.as_mut_ptr(),
                output.as_mut_ptr(),
                &mut outlen,
                input.as_ptr(),
                inlen,
            )
        } {
            return Err(Unspecified);
        }
        let outlen: usize = outlen.try_into()?;
        debug_assert!(outlen <= min_outsize);
        self.output_generated += outlen;
        assert!(outlen <= output.len());

        Ok(BufferUpdate::new(output, outlen))
    }

    /// Updates the internal state of the key with the provided ciphertext `input`,
    /// potentially also writing bytes of plaintext to `output`.
    /// The number of bytes written to `output` can be up to `input.len()`
    /// plus the block length of the cipher algorithm (e.g., [`Algorithm::block_len`]) minus one.
    ///
    /// # Errors
    /// * Returns an error if the `output` buffer is smaller than the length of
    ///   the `input` plus the algorithm's block length minus one.
    /// * May return an error if the length of `input` plus the algorithm's block length is larger
    ///   than `i32::MAX`.
    pub fn update<'a>(
        &mut self,
        input: &[u8],
        output: &'a mut [u8],
    ) -> Result<BufferUpdate<'a>, Unspecified> {
        let min_outsize = input
            .len()
            .checked_add(self.algorithm().block_len())
            .ok_or(Unspecified)?
            .checked_sub(1)
            .ok_or(Unspecified)?;
        self.update_internal(input, output, min_outsize)
    }

    /// Updates the internal state of the key with the provided ciphertext `input`,
    /// potentially writing bytes of plaintext to `output`.
    ///
    /// This function has looser output buffer size requirements than [`Self::update`],
    /// calculating the minimum required size based on the total bytes of output generated
    /// and the cipher's block length. This is considered "less safe" because it's
    /// based on assumptions about the state of the underlying operations.
    ///
    /// The minimum output buffer size is calculated based on how many bytes are needed to
    /// reach the next block boundary after processing the input. If `next_total` is the sum
    /// of bytes already generated plus `input.len()`, then the minimum size is:
    /// `input.len() + ((block_len - (next_total % block_len)) % block_len)`
    ///
    /// # Errors
    /// Returns an error if the `output` buffer is smaller than the calculated minimum size,
    /// if the total output length overflows, or if the length of `input` is larger than
    /// `i32::MAX`.
    ///
    /// # Panics
    /// Panics if the number of bytes written by the cipher operation exceeds the output
    /// buffer length.
    pub fn less_safe_update<'a>(
        &mut self,
        input: &[u8],
        output: &'a mut [u8],
    ) -> Result<BufferUpdate<'a>, Unspecified> {
        let next_total = self
            .output_generated
            .checked_add(input.len())
            .ok_or(Unspecified)?;
        let extra_buffer_size = (self.algorithm().block_len
            - next_total.rem_euclid(self.algorithm().block_len))
        .rem_euclid(self.algorithm().block_len);
        let min_outsize = input
            .len()
            .checked_add(extra_buffer_size)
            .ok_or(Unspecified)?;
        self.update_internal(input, output, min_outsize)
    }

    /// Finishes the decryption operation, writing the remaining plaintext to
    /// `output`.
    /// The number of bytes written to `output` can be up to the block length of
    /// the cipher algorithm (e.g., [`Algorithm::block_len`]).
    ///
    /// # Errors
    /// * Returns an error if the `output` buffer is smaller than the algorithm's
    ///   block length.
    pub fn finish(mut self, output: &mut [u8]) -> Result<BufferUpdate<'_>, Unspecified> {
        if output.len() < self.algorithm().block_len() {
            return Err(Unspecified);
        }
        let mut outlen: i32 = 0;

        if 1 != indicator_check!(unsafe {
            EVP_DecryptFinal_ex(
                self.cipher_ctx.as_mut_ptr(),
                output.as_mut_ptr(),
                &mut outlen,
            )
        }) {
            return Err(Unspecified);
        }
        let outlen: usize = outlen.try_into()?;
        debug_assert!(outlen <= self.algorithm().block_len());
        Ok(BufferUpdate::new(output, outlen))
    }

    /// Returns the cipher operating mode.
    #[must_use]
    pub fn mode(&self) -> OperatingMode {
        self.mode
    }

    /// Returns the cipher algorithm
    #[must_use]
    pub fn algorithm(&self) -> &'static Algorithm {
        self.algorithm
    }

    /// Constructs a `StreamingDecryptingKey` for decrypting using the CTR cipher mode.
    /// The resulting plaintext will be the same length as the ciphertext.
    ///
    /// # Errors
    /// Returns an error on an internal failure. With `legacy-des` enabled, also
    /// returned if `key`'s algorithm does not support CTR mode (e.g.
    /// `DES_FOR_LEGACY_USE_ONLY`, `DES_EDE_FOR_LEGACY_USE_ONLY`,
    /// `DES_EDE3_FOR_LEGACY_USE_ONLY`).
    pub fn ctr(key: UnboundCipherKey, context: DecryptionContext) -> Result<Self, Unspecified> {
        Self::new(key, OperatingMode::CTR, context)
    }

    /// Constructs a `StreamingDecryptingKey` for decrypting using the CBC cipher mode.
    /// The resulting plaintext will be shorter than the ciphertext.
    ///
    /// # Errors
    /// Returns an error on an internal failure. With `legacy-des` enabled, also
    /// returned if `key` was constructed with `DES_FOR_LEGACY_USE_ONLY`,
    /// `DES_EDE_FOR_LEGACY_USE_ONLY` or `DES_EDE3_FOR_LEGACY_USE_ONLY` and the
    /// provided key material contains weak or semi-weak DES subkeys, or (for
    /// Triple DES) a degenerate subkey configuration (e.g. `K1 == K2` for 2TDEA,
    /// or any pairwise equality for 3TDEA).
    pub fn cbc_pkcs7(
        key: UnboundCipherKey,
        context: DecryptionContext,
    ) -> Result<Self, Unspecified> {
        Self::new(key, OperatingMode::CBC, context)
    }

    // Constructs a `StreamingDecryptingKey` for decrypting using the CFB128 cipher mode.
    /// The resulting plaintext will be the same length as the ciphertext.
    ///
    /// # Errors
    /// Returns an error on an internal failure. With `legacy-des` enabled, also
    /// returned if `key`'s algorithm does not support CFB128 mode (e.g.
    /// `DES_FOR_LEGACY_USE_ONLY`, `DES_EDE_FOR_LEGACY_USE_ONLY`,
    /// `DES_EDE3_FOR_LEGACY_USE_ONLY`).
    pub fn cfb128(key: UnboundCipherKey, context: DecryptionContext) -> Result<Self, Unspecified> {
        Self::new(key, OperatingMode::CFB128, context)
    }

    /// Constructs a `StreamingDecryptingKey` for decrypting using the ECB cipher mode.
    /// The resulting plaintext will be the same length as the ciphertext.
    ///
    /// # ☠️ ️️️DANGER ☠️
    /// Offered for computability purposes only. This is an extremely dangerous mode, and
    /// very likely not what you want to use.
    ///
    /// # Errors
    /// Returns an error on an internal failure. With `legacy-des` enabled, also
    /// returned if `key` was constructed with `DES_FOR_LEGACY_USE_ONLY`,
    /// `DES_EDE_FOR_LEGACY_USE_ONLY` or `DES_EDE3_FOR_LEGACY_USE_ONLY` and the
    /// provided key material contains weak or semi-weak DES subkeys, or (for
    /// Triple DES) a degenerate subkey configuration (e.g. `K1 == K2` for 2TDEA,
    /// or any pairwise equality for 3TDEA).
    pub fn ecb_pkcs7(
        key: UnboundCipherKey,
        context: DecryptionContext,
    ) -> Result<Self, Unspecified> {
        Self::new(key, OperatingMode::ECB, context)
    }
}

#[cfg(test)]
mod tests {
    use crate::cipher::{
        DecryptionContext, EncryptionContext, OperatingMode, StreamingDecryptingKey,
        StreamingEncryptingKey, UnboundCipherKey, AES_128, AES_256, AES_256_KEY_LEN,
    };
    use crate::iv::{FixedLength, IV_LEN_128_BIT};
    use crate::rand::{SecureRandom, SystemRandom};
    use crate::test::from_hex;
    use paste::*;

    /// Generic helper for step encryption that accepts a closure for the update operation.
    /// The closure receives: (key, input, output_buffer, out_idx, block_len, step)
    /// and returns the number of bytes written.
    fn step_encrypt_with_updater<F>(
        mut encrypting_key: StreamingEncryptingKey,
        plaintext: &[u8],
        step: usize,
        mut updater: F,
    ) -> (Box<[u8]>, DecryptionContext)
    where
        F: FnMut(&mut StreamingEncryptingKey, &[u8], &mut [u8], usize, usize, usize) -> usize,
    {
        let alg = encrypting_key.algorithm();
        let mode = encrypting_key.mode();
        let block_len = alg.block_len();
        let n = plaintext.len();
        let mut ciphertext = vec![0u8; n + block_len];

        let mut in_idx: usize = 0;
        let mut out_idx: usize = 0;
        loop {
            let mut in_end = in_idx + step;
            if in_end > n {
                in_end = n;
            }
            let written = updater(
                &mut encrypting_key,
                &plaintext[in_idx..in_end],
                &mut ciphertext,
                out_idx,
                block_len,
                step,
            );
            in_idx += step;
            out_idx += written;
            if in_idx >= n {
                break;
            }
        }
        let out_end = out_idx + block_len;
        let (decrypt_iv, output) = encrypting_key
            .finish(&mut ciphertext[out_idx..out_end])
            .unwrap();
        let outlen = output.written().len();
        ciphertext.truncate(out_idx + outlen);
        match mode {
            OperatingMode::CBC | OperatingMode::ECB => {
                assert!(ciphertext.len() > plaintext.len());
                assert!(ciphertext.len() <= plaintext.len() + block_len);
            }
            _ => {
                assert_eq!(ciphertext.len(), plaintext.len());
            }
        }

        (ciphertext.into_boxed_slice(), decrypt_iv)
    }

    /// Generic helper for step decryption that accepts a closure for the update operation.
    /// The closure receives: (key, input, output_buffer, out_idx, block_len, step)
    /// and returns the number of bytes written.
    fn step_decrypt_with_updater<F>(
        mut decrypting_key: StreamingDecryptingKey,
        ciphertext: &[u8],
        step: usize,
        mut updater: F,
    ) -> Box<[u8]>
    where
        F: FnMut(&mut StreamingDecryptingKey, &[u8], &mut [u8], usize, usize, usize) -> usize,
    {
        let alg = decrypting_key.algorithm();
        let mode = decrypting_key.mode();
        let block_len = alg.block_len();
        let n = ciphertext.len();
        let mut plaintext = vec![0u8; n + block_len];

        let mut in_idx: usize = 0;
        let mut out_idx: usize = 0;
        loop {
            let mut in_end = in_idx + step;
            if in_end > n {
                in_end = n;
            }
            let written = updater(
                &mut decrypting_key,
                &ciphertext[in_idx..in_end],
                &mut plaintext,
                out_idx,
                block_len,
                step,
            );
            in_idx += step;
            out_idx += written;
            if in_idx >= n {
                break;
            }
        }
        let out_end = out_idx + block_len;
        let output = decrypting_key
            .finish(&mut plaintext[out_idx..out_end])
            .unwrap();
        let outlen = output.written().len();
        plaintext.truncate(out_idx + outlen);
        match mode {
            OperatingMode::CBC | OperatingMode::ECB => {
                assert!(ciphertext.len() > plaintext.len());
                assert!(ciphertext.len() <= plaintext.len() + block_len);
            }
            _ => {
                assert_eq!(ciphertext.len(), plaintext.len());
            }
        }
        plaintext.into_boxed_slice()
    }

    fn step_encrypt(
        encrypting_key: StreamingEncryptingKey,
        plaintext: &[u8],
        step: usize,
    ) -> (Box<[u8]>, DecryptionContext) {
        step_encrypt_with_updater(
            encrypting_key,
            plaintext,
            step,
            |key, input, output, out_idx, block_len, _step| {
                let out_end = out_idx + input.len() + block_len - 1;
                let result = key.update(input, &mut output[out_idx..out_end]).unwrap();
                result.written().len()
            },
        )
    }

    fn step_decrypt(
        decrypting_key: StreamingDecryptingKey,
        ciphertext: &[u8],
        step: usize,
    ) -> Box<[u8]> {
        step_decrypt_with_updater(
            decrypting_key,
            ciphertext,
            step,
            |key, input, output, out_idx, block_len, _step| {
                let out_end = out_idx + input.len() + block_len - 1;
                let result = key.update(input, &mut output[out_idx..out_end]).unwrap();
                result.written().len()
            },
        )
    }

    fn step_encrypt_less_safe(
        encrypting_key: StreamingEncryptingKey,
        plaintext: &[u8],
        step: usize,
    ) -> (Box<[u8]>, DecryptionContext) {
        step_encrypt_with_updater(
            encrypting_key,
            plaintext,
            step,
            |key, input, output, out_idx, block_len, step| {
                let input_len = input.len();
                let next_total = out_idx + input_len;
                // Calculate the tighter minimum output buffer size
                let min_out_len = input_len + ((block_len - (next_total % block_len)) % block_len);
                if input_len % block_len == 0 && step % block_len == 0 {
                    // When input is provided one block at a time, no additional space should be needed.
                    assert_eq!(input_len, min_out_len);
                }
                let out_end = out_idx + min_out_len;
                let result = key
                    .less_safe_update(input, &mut output[out_idx..out_end])
                    .unwrap();
                result.written().len()
            },
        )
    }

    fn step_decrypt_less_safe(
        decrypting_key: StreamingDecryptingKey,
        ciphertext: &[u8],
        step: usize,
    ) -> Box<[u8]> {
        step_decrypt_with_updater(
            decrypting_key,
            ciphertext,
            step,
            |key, input, output, out_idx, block_len, step| {
                let input_len = input.len();
                let next_total = out_idx + input_len;
                // Calculate the tighter minimum output buffer size
                let min_out_len = input_len + ((block_len - (next_total % block_len)) % block_len);
                if input_len % block_len == 0 && step % block_len == 0 {
                    // When input is provided one block at a time, no additional space should be needed.
                    assert_eq!(input_len, min_out_len);
                }
                let out_end = out_idx + min_out_len;
                let result = key
                    .less_safe_update(input, &mut output[out_idx..out_end])
                    .unwrap();
                result.written().len()
            },
        )
    }

    macro_rules! helper_stream_step_encrypt_test {
        ($mode:ident) => {
            paste! {
                fn [<helper_test_ $mode _stream_encrypt_step_n_bytes>](
                    encrypting_key_creator: impl Fn() -> StreamingEncryptingKey,
                    decrypting_key_creator: impl Fn(DecryptionContext) -> StreamingDecryptingKey,
                    n: usize,
                    step: usize,
                ) {
                    let mut input = vec![0u8; n];
                    let random = SystemRandom::new();
                    random.fill(&mut input).unwrap();

                    let encrypting_key = encrypting_key_creator();

                    let (ciphertext, decrypt_iv) = step_encrypt(encrypting_key, &input, step);

                    let decrypting_key = decrypting_key_creator(decrypt_iv);

                    let plaintext = step_decrypt(decrypting_key, &ciphertext, step);

                    assert_eq!(input.as_slice(), &*plaintext);
                }
            }
        };
        ($mode:ident, less_safe) => {
            paste! {
                fn [<helper_test_ $mode _stream_encrypt_step_n_bytes_less_safe>](
                    encrypting_key_creator: impl Fn() -> StreamingEncryptingKey,
                    decrypting_key_creator: impl Fn(DecryptionContext) -> StreamingDecryptingKey,
                    n: usize,
                    step: usize,
                ) {
                    let mut input = vec![0u8; n];
                    let random = SystemRandom::new();
                    random.fill(&mut input).unwrap();

                    let encrypting_key = encrypting_key_creator();

                    let (ciphertext, decrypt_iv) = step_encrypt_less_safe(encrypting_key, &input, step);

                    let decrypting_key = decrypting_key_creator(decrypt_iv);

                    let plaintext = step_decrypt_less_safe(decrypting_key, &ciphertext, step);

                    assert_eq!(input.as_slice(), &*plaintext);
                }
            }
        };
    }

    helper_stream_step_encrypt_test!(cbc_pkcs7);
    helper_stream_step_encrypt_test!(ctr);
    helper_stream_step_encrypt_test!(cfb128);
    helper_stream_step_encrypt_test!(ecb_pkcs7);

    helper_stream_step_encrypt_test!(cbc_pkcs7, less_safe);
    helper_stream_step_encrypt_test!(ctr, less_safe);
    helper_stream_step_encrypt_test!(cfb128, less_safe);
    helper_stream_step_encrypt_test!(ecb_pkcs7, less_safe);

    #[test]
    fn test_step_cbc() {
        let random = SystemRandom::new();
        let mut key = [0u8; AES_256_KEY_LEN];
        random.fill(&mut key).unwrap();
        let key = key;

        let encrypting_key_creator = || {
            let key = UnboundCipherKey::new(&AES_256, &key.clone()).unwrap();
            StreamingEncryptingKey::cbc_pkcs7(key).unwrap()
        };
        let decrypting_key_creator = |decryption_ctx: DecryptionContext| {
            let key = UnboundCipherKey::new(&AES_256, &key.clone()).unwrap();
            StreamingDecryptingKey::cbc_pkcs7(key, decryption_ctx).unwrap()
        };

        for i in 13..=21 {
            for j in 124..=131 {
                helper_test_cbc_pkcs7_stream_encrypt_step_n_bytes(
                    encrypting_key_creator,
                    decrypting_key_creator,
                    j,
                    i,
                );
            }
            for j in 124..=131 {
                helper_test_cbc_pkcs7_stream_encrypt_step_n_bytes(
                    encrypting_key_creator,
                    decrypting_key_creator,
                    j,
                    j - i,
                );
            }
        }
        for j in 124..=131 {
            helper_test_cbc_pkcs7_stream_encrypt_step_n_bytes(
                encrypting_key_creator,
                decrypting_key_creator,
                j,
                j,
            );
            helper_test_cbc_pkcs7_stream_encrypt_step_n_bytes(
                encrypting_key_creator,
                decrypting_key_creator,
                j,
                256,
            );
            helper_test_cbc_pkcs7_stream_encrypt_step_n_bytes(
                encrypting_key_creator,
                decrypting_key_creator,
                j,
                1,
            );
        }
    }

    #[test]
    fn test_step_ctr() {
        let random = SystemRandom::new();
        let mut key = [0u8; AES_256_KEY_LEN];
        random.fill(&mut key).unwrap();

        let encrypting_key_creator = || {
            let key = UnboundCipherKey::new(&AES_256, &key.clone()).unwrap();
            StreamingEncryptingKey::ctr(key).unwrap()
        };
        let decrypting_key_creator = |decryption_ctx: DecryptionContext| {
            let key = UnboundCipherKey::new(&AES_256, &key.clone()).unwrap();
            StreamingDecryptingKey::ctr(key, decryption_ctx).unwrap()
        };

        for i in 13..=21 {
            for j in 124..=131 {
                helper_test_ctr_stream_encrypt_step_n_bytes(
                    encrypting_key_creator,
                    decrypting_key_creator,
                    j,
                    i,
                );
            }
            for j in 124..=131 {
                helper_test_ctr_stream_encrypt_step_n_bytes(
                    encrypting_key_creator,
                    decrypting_key_creator,
                    j,
                    j - i,
                );
            }
        }
        for j in 124..=131 {
            helper_test_ctr_stream_encrypt_step_n_bytes(
                encrypting_key_creator,
                decrypting_key_creator,
                j,
                j,
            );
            helper_test_ctr_stream_encrypt_step_n_bytes(
                encrypting_key_creator,
                decrypting_key_creator,
                j,
                256,
            );
            helper_test_ctr_stream_encrypt_step_n_bytes(
                encrypting_key_creator,
                decrypting_key_creator,
                j,
                1,
            );
        }
    }

    #[test]
    fn test_step_cfb128() {
        let random = SystemRandom::new();
        let mut key = [0u8; AES_256_KEY_LEN];
        random.fill(&mut key).unwrap();

        let encrypting_key_creator = || {
            let key = UnboundCipherKey::new(&AES_256, &key.clone()).unwrap();
            StreamingEncryptingKey::cfb128(key).unwrap()
        };
        let decrypting_key_creator = |decryption_ctx: DecryptionContext| {
            let key = UnboundCipherKey::new(&AES_256, &key.clone()).unwrap();
            StreamingDecryptingKey::cfb128(key, decryption_ctx).unwrap()
        };

        for i in 13..=21 {
            for j in 124..=131 {
                helper_test_cfb128_stream_encrypt_step_n_bytes(
                    encrypting_key_creator,
                    decrypting_key_creator,
                    j,
                    i,
                );
            }
            for j in 124..=131 {
                helper_test_cfb128_stream_encrypt_step_n_bytes(
                    encrypting_key_creator,
                    decrypting_key_creator,
                    j,
                    j - i,
                );
            }
        }
        for j in 124..=131 {
            helper_test_cfb128_stream_encrypt_step_n_bytes(
                encrypting_key_creator,
                decrypting_key_creator,
                j,
                j,
            );
            helper_test_cfb128_stream_encrypt_step_n_bytes(
                encrypting_key_creator,
                decrypting_key_creator,
                j,
                256,
            );
            helper_test_cfb128_stream_encrypt_step_n_bytes(
                encrypting_key_creator,
                decrypting_key_creator,
                j,
                1,
            );
        }
    }

    #[test]
    fn test_step_ecb_pkcs7() {
        let random = SystemRandom::new();
        let mut key = [0u8; AES_256_KEY_LEN];
        random.fill(&mut key).unwrap();

        let encrypting_key_creator = || {
            let key = UnboundCipherKey::new(&AES_256, &key.clone()).unwrap();
            StreamingEncryptingKey::ecb_pkcs7(key).unwrap()
        };
        let decrypting_key_creator = |decryption_ctx: DecryptionContext| {
            let key = UnboundCipherKey::new(&AES_256, &key.clone()).unwrap();
            StreamingDecryptingKey::ecb_pkcs7(key, decryption_ctx).unwrap()
        };

        for i in 13..=21 {
            for j in 124..=131 {
                helper_test_ecb_pkcs7_stream_encrypt_step_n_bytes(
                    encrypting_key_creator,
                    decrypting_key_creator,
                    j,
                    i,
                );
            }
            for j in 124..=131 {
                helper_test_ecb_pkcs7_stream_encrypt_step_n_bytes(
                    encrypting_key_creator,
                    decrypting_key_creator,
                    j,
                    j - i,
                );
            }
        }
        for j in 124..=131 {
            helper_test_ecb_pkcs7_stream_encrypt_step_n_bytes(
                encrypting_key_creator,
                decrypting_key_creator,
                j,
                j,
            );
            helper_test_ecb_pkcs7_stream_encrypt_step_n_bytes(
                encrypting_key_creator,
                decrypting_key_creator,
                j,
                256,
            );
            helper_test_ecb_pkcs7_stream_encrypt_step_n_bytes(
                encrypting_key_creator,
                decrypting_key_creator,
                j,
                1,
            );
        }
    }

    #[test]
    fn test_step_cbc_less_safe() {
        let random = SystemRandom::new();
        let mut key = [0u8; AES_256_KEY_LEN];
        random.fill(&mut key).unwrap();
        let key = key;

        let encrypting_key_creator = || {
            let key = UnboundCipherKey::new(&AES_256, &key.clone()).unwrap();
            StreamingEncryptingKey::cbc_pkcs7(key).unwrap()
        };
        let decrypting_key_creator = |decryption_ctx: DecryptionContext| {
            let key = UnboundCipherKey::new(&AES_256, &key.clone()).unwrap();
            StreamingDecryptingKey::cbc_pkcs7(key, decryption_ctx).unwrap()
        };

        for i in 13..=21 {
            for j in 124..=131 {
                helper_test_cbc_pkcs7_stream_encrypt_step_n_bytes_less_safe(
                    encrypting_key_creator,
                    decrypting_key_creator,
                    j,
                    i,
                );
            }
            for j in 124..=131 {
                helper_test_cbc_pkcs7_stream_encrypt_step_n_bytes_less_safe(
                    encrypting_key_creator,
                    decrypting_key_creator,
                    j,
                    j - i,
                );
            }
        }
        for j in 124..=131 {
            helper_test_cbc_pkcs7_stream_encrypt_step_n_bytes_less_safe(
                encrypting_key_creator,
                decrypting_key_creator,
                j,
                j,
            );
            helper_test_cbc_pkcs7_stream_encrypt_step_n_bytes_less_safe(
                encrypting_key_creator,
                decrypting_key_creator,
                j,
                256,
            );
            helper_test_cbc_pkcs7_stream_encrypt_step_n_bytes_less_safe(
                encrypting_key_creator,
                decrypting_key_creator,
                j,
                1,
            );
        }
    }

    #[test]
    fn test_step_ctr_less_safe() {
        let random = SystemRandom::new();
        let mut key = [0u8; AES_256_KEY_LEN];
        random.fill(&mut key).unwrap();

        let encrypting_key_creator = || {
            let key = UnboundCipherKey::new(&AES_256, &key.clone()).unwrap();
            StreamingEncryptingKey::ctr(key).unwrap()
        };
        let decrypting_key_creator = |decryption_ctx: DecryptionContext| {
            let key = UnboundCipherKey::new(&AES_256, &key.clone()).unwrap();
            StreamingDecryptingKey::ctr(key, decryption_ctx).unwrap()
        };

        for i in 13..=21 {
            for j in 124..=131 {
                helper_test_ctr_stream_encrypt_step_n_bytes_less_safe(
                    encrypting_key_creator,
                    decrypting_key_creator,
                    j,
                    i,
                );
            }
            for j in 124..=131 {
                helper_test_ctr_stream_encrypt_step_n_bytes_less_safe(
                    encrypting_key_creator,
                    decrypting_key_creator,
                    j,
                    j - i,
                );
            }
        }
        for j in 124..=131 {
            helper_test_ctr_stream_encrypt_step_n_bytes_less_safe(
                encrypting_key_creator,
                decrypting_key_creator,
                j,
                j,
            );
            helper_test_ctr_stream_encrypt_step_n_bytes_less_safe(
                encrypting_key_creator,
                decrypting_key_creator,
                j,
                256,
            );
            helper_test_ctr_stream_encrypt_step_n_bytes_less_safe(
                encrypting_key_creator,
                decrypting_key_creator,
                j,
                1,
            );
        }
    }

    #[test]
    fn test_step_cfb128_less_safe() {
        let random = SystemRandom::new();
        let mut key = [0u8; AES_256_KEY_LEN];
        random.fill(&mut key).unwrap();

        let encrypting_key_creator = || {
            let key = UnboundCipherKey::new(&AES_256, &key.clone()).unwrap();
            StreamingEncryptingKey::cfb128(key).unwrap()
        };
        let decrypting_key_creator = |decryption_ctx: DecryptionContext| {
            let key = UnboundCipherKey::new(&AES_256, &key.clone()).unwrap();
            StreamingDecryptingKey::cfb128(key, decryption_ctx).unwrap()
        };

        for i in 13..=21 {
            for j in 124..=131 {
                helper_test_cfb128_stream_encrypt_step_n_bytes_less_safe(
                    encrypting_key_creator,
                    decrypting_key_creator,
                    j,
                    i,
                );
            }
            for j in 124..=131 {
                helper_test_cfb128_stream_encrypt_step_n_bytes_less_safe(
                    encrypting_key_creator,
                    decrypting_key_creator,
                    j,
                    j - i,
                );
            }
        }
        for j in 124..=131 {
            helper_test_cfb128_stream_encrypt_step_n_bytes_less_safe(
                encrypting_key_creator,
                decrypting_key_creator,
                j,
                j,
            );
            helper_test_cfb128_stream_encrypt_step_n_bytes_less_safe(
                encrypting_key_creator,
                decrypting_key_creator,
                j,
                256,
            );
            helper_test_cfb128_stream_encrypt_step_n_bytes_less_safe(
                encrypting_key_creator,
                decrypting_key_creator,
                j,
                1,
            );
        }
    }

    #[test]
    fn test_step_ecb_pkcs7_less_safe() {
        let random = SystemRandom::new();
        let mut key = [0u8; AES_256_KEY_LEN];
        random.fill(&mut key).unwrap();

        let encrypting_key_creator = || {
            let key = UnboundCipherKey::new(&AES_256, &key.clone()).unwrap();
            StreamingEncryptingKey::ecb_pkcs7(key).unwrap()
        };
        let decrypting_key_creator = |decryption_ctx: DecryptionContext| {
            let key = UnboundCipherKey::new(&AES_256, &key.clone()).unwrap();
            StreamingDecryptingKey::ecb_pkcs7(key, decryption_ctx).unwrap()
        };

        for i in 13..=21 {
            for j in 124..=131 {
                helper_test_ecb_pkcs7_stream_encrypt_step_n_bytes_less_safe(
                    encrypting_key_creator,
                    decrypting_key_creator,
                    j,
                    i,
                );
            }
            for j in 124..=131 {
                helper_test_ecb_pkcs7_stream_encrypt_step_n_bytes_less_safe(
                    encrypting_key_creator,
                    decrypting_key_creator,
                    j,
                    j - i,
                );
            }
        }
        for j in 124..=131 {
            helper_test_ecb_pkcs7_stream_encrypt_step_n_bytes_less_safe(
                encrypting_key_creator,
                decrypting_key_creator,
                j,
                j,
            );
            helper_test_ecb_pkcs7_stream_encrypt_step_n_bytes_less_safe(
                encrypting_key_creator,
                decrypting_key_creator,
                j,
                256,
            );
            helper_test_ecb_pkcs7_stream_encrypt_step_n_bytes_less_safe(
                encrypting_key_creator,
                decrypting_key_creator,
                j,
                1,
            );
        }
    }

    macro_rules! streaming_cipher_kat {
        ($name:ident, $alg:expr, $mode:expr, $key:literal, $iv: literal, $plaintext:literal, $ciphertext:literal, $from_step:literal, $to_step:literal) => {
            #[test]
            fn $name() {
                let key = from_hex($key).unwrap();
                let input = from_hex($plaintext).unwrap();
                let expected_ciphertext = from_hex($ciphertext).unwrap();
                let iv = from_hex($iv).unwrap();

                for step in ($from_step..=$to_step) {
                    let ec = EncryptionContext::Iv128(
                        FixedLength::<IV_LEN_128_BIT>::try_from(iv.as_slice()).unwrap(),
                    );

                    let unbound_key = UnboundCipherKey::new($alg, &key).unwrap();

                    let encrypting_key =
                        StreamingEncryptingKey::new(unbound_key, $mode, ec).unwrap();

                    let (ciphertext, decrypt_ctx) = step_encrypt(encrypting_key, &input, step);

                    assert_eq!(expected_ciphertext.as_slice(), ciphertext.as_ref());

                    let unbound_key2 = UnboundCipherKey::new($alg, &key).unwrap();
                    let decrypting_key =
                        StreamingDecryptingKey::new(unbound_key2, $mode, decrypt_ctx).unwrap();

                    let plaintext = step_decrypt(decrypting_key, &ciphertext, step);
                    assert_eq!(input.as_slice(), plaintext.as_ref());
                }
            }
        };
        ($name:ident, $alg:expr, $mode:expr, $key:literal, $plaintext:literal, $ciphertext:literal, $from_step:literal, $to_step:literal) => {
            #[test]
            fn $name() {
                let key = from_hex($key).unwrap();
                let input = from_hex($plaintext).unwrap();
                let expected_ciphertext = from_hex($ciphertext).unwrap();

                for step in ($from_step..=$to_step) {
                    let unbound_key = UnboundCipherKey::new($alg, &key).unwrap();

                    let encrypting_key =
                        StreamingEncryptingKey::new(unbound_key, $mode, EncryptionContext::None)
                            .unwrap();

                    let (ciphertext, decrypt_ctx) = step_encrypt(encrypting_key, &input, step);

                    assert_eq!(expected_ciphertext.as_slice(), ciphertext.as_ref());

                    let unbound_key2 = UnboundCipherKey::new($alg, &key).unwrap();
                    let decrypting_key =
                        StreamingDecryptingKey::new(unbound_key2, $mode, decrypt_ctx).unwrap();

                    let plaintext = step_decrypt(decrypting_key, &ciphertext, step);
                    assert_eq!(input.as_slice(), plaintext.as_ref());
                }
            }
        };
    }

    streaming_cipher_kat!(
        test_iv_aes_128_ctr_16_bytes,
        &AES_128,
        OperatingMode::CTR,
        "000102030405060708090a0b0c0d0e0f",
        "00000000000000000000000000000000",
        "00112233445566778899aabbccddeeff",
        "c6b01904c3da3df5e7d62bd96d153686",
        2,
        9
    );
    streaming_cipher_kat!(
        test_iv_aes_256_ctr_15_bytes,
        &AES_256,
        OperatingMode::CTR,
        "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
        "00000000000000000000000000000000",
        "00112233445566778899aabbccddee",
        "f28122856e1cf9a7216a30d111f399",
        2,
        9
    );

    streaming_cipher_kat!(
        test_openssl_aes_128_ctr_15_bytes,
        &AES_128,
        OperatingMode::CTR,
        "244828580821c1652582c76e34d299f5",
        "093145d5af233f46072a5eb5adc11aa1",
        "3ee38cec171e6cf466bf0df98aa0e1",
        "bd7d928f60e3422d96b3f8cd614eb2",
        2,
        9
    );

    streaming_cipher_kat!(
        test_openssl_aes_256_ctr_15_bytes,
        &AES_256,
        OperatingMode::CTR,
        "0857db8240ea459bdf660b4cced66d1f2d3734ff2de7b81e92740e65e7cc6a1d",
        "f028ecb053f801102d11fccc9d303a27",
        "eca7285d19f3c20e295378460e8729",
        "b5098e5e788de6ac2f2098eb2fc6f8",
        2,
        9
    );

    streaming_cipher_kat!(
        test_iv_aes_128_cbc_16_bytes,
        &AES_128,
        OperatingMode::CBC,
        "000102030405060708090a0b0c0d0e0f",
        "00000000000000000000000000000000",
        "00112233445566778899aabbccddeeff",
        "69c4e0d86a7b0430d8cdb78070b4c55a9e978e6d16b086570ef794ef97984232",
        2,
        9
    );

    streaming_cipher_kat!(
        test_iv_aes_256_cbc_15_bytes,
        &AES_256,
        OperatingMode::CBC,
        "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
        "00000000000000000000000000000000",
        "00112233445566778899aabbccddee",
        "2ddfb635a651a43f582997966840ca0c",
        2,
        9
    );

    streaming_cipher_kat!(
        test_openssl_aes_128_cbc_15_bytes,
        &AES_128,
        OperatingMode::CBC,
        "053304bb3899e1d99db9d29343ea782d",
        "b5313560244a4822c46c2a0c9d0cf7fd",
        "a3e4c990356c01f320043c3d8d6f43",
        "ad96993f248bd6a29760ec7ccda95ee1",
        2,
        9
    );

    streaming_cipher_kat!(
        test_openssl_aes_128_cbc_16_bytes,
        &AES_128,
        OperatingMode::CBC,
        "95af71f1c63e4a1d0b0b1a27fb978283",
        "89e40797dca70197ff87d3dbb0ef2802",
        "aece7b5e3c3df1ffc9802d2dfe296dc7",
        "301b5dab49fb11e919d0d39970d06739301919743304f23f3cbc67d28564b25b",
        2,
        9
    );

    streaming_cipher_kat!(
        test_openssl_aes_256_cbc_15_bytes,
        &AES_256,
        OperatingMode::CBC,
        "d369e03e9752784917cc7bac1db7399598d9555e691861d9dd7b3292a693ef57",
        "1399bb66b2f6ad99a7f064140eaaa885",
        "7385f5784b85bf0a97768ddd896d6d",
        "4351082bac9b4593ae8848cc9dfb5a01",
        2,
        9
    );

    streaming_cipher_kat!(
        test_openssl_aes_256_cbc_16_bytes,
        &AES_256,
        OperatingMode::CBC,
        "d4a8206dcae01242f9db79a4ecfe277d0f7bb8ccbafd8f9809adb39f35aa9b41",
        "24f6076548fb9d93c8f7ed9f6e661ef9",
        "a39c1fdf77ea3e1f18178c0ec237c70a",
        "f1af484830a149ee0387b854d65fe87ca0e62efc1c8e6909d4b9ab8666470453",
        2,
        9
    );

    streaming_cipher_kat!(
        test_openssl_aes_128_cfb128_16_bytes,
        &AES_128,
        OperatingMode::CFB128,
        "5c353f739429bbd48b7e3f9a76facf4d",
        "7b2c7ce17a9b6a59a9e64253b98c8cd1",
        "add1bcebeaabe9423d4e916400e877c5",
        "8440ec442e4135a613ddb2ce26107e10",
        2,
        9
    );

    streaming_cipher_kat!(
        test_openssl_aes_128_cfb128_15_bytes,
        &AES_128,
        OperatingMode::CFB128,
        "e1f39d70ad378efc1ac318aa8ac4489f",
        "ec78c3d54fff2fe09678c7883024ddce",
        "b8c905004b2a92a323769f1b8dc1b2",
        "964c3e9bf8bf2a3cca02d8e2e75608",
        2,
        9
    );

    streaming_cipher_kat!(
        test_openssl_aes_256_cfb128_16_bytes,
        &AES_256,
        OperatingMode::CFB128,
        "0e8117d0984d6acb957a5d6ca526a12fa612ce5de2daadebd42c14d28a0a192e",
        "09147a153b230a40cd7bf4197ad0e825",
        "13f4540a4e06394148ade31a6f678787",
        "250e590e47b7613b7d0a53f684e970d6",
        2,
        9
    );

    streaming_cipher_kat!(
        test_openssl_aes_256_cfb128_15_bytes,
        &AES_256,
        OperatingMode::CFB128,
        "5cb17d8d5b9dbd81e4f1e0a2c82ebf36cf61156388fb7abf99d4526622858225",
        "13c77415ec24f3e2f784f228478a85be",
        "3efa583df4405aab61e18155aa7e0d",
        "c1f2ffe8aa5064199e8f4f1b388303",
        2,
        9
    );

    streaming_cipher_kat!(
        test_openssl_aes_128_ecb_pkcs7_16_bytes,
        &AES_128,
        OperatingMode::ECB,
        "a1b7cd124f9824a1532d8440f8136788",
        "388118e6848b0cea97401707a754d7a1",
        "19b7c7f5d9c2bda3f957e9e7d20847828d5eb5624bcbf221014063a87b38d133",
        2,
        9
    );

    streaming_cipher_kat!(
        test_openssl_aes_128_ecb_pkcs7_15_bytes,
        &AES_128,
        OperatingMode::ECB,
        "d10e12accb837aaffbb284448e53138c",
        "b21cfd1c9e6e7e6e912c82c7dd1aa8",
        "3d1168e61df34b51c6ab6745c20ee881",
        2,
        9
    );

    streaming_cipher_kat!(
        test_openssl_aes_256_ecb_pkcs7_16_bytes,
        &AES_256,
        OperatingMode::ECB,
        "0600f4ad4eda4bc8e3e99592abdfce7eb08fee0ccc801c5ccee26134bcaafbbd",
        "516b45cb1342239a549bd8c1d5998f98",
        "854c593555a213e4a862c6f66aa4a79631faca131eba6f163e5cd3940e9c0a57",
        2,
        9
    );

    streaming_cipher_kat!(
        test_openssl_aes_256_ecb_pkcs7_15_bytes,
        &AES_256,
        OperatingMode::ECB,
        "80f235756c8f70094ae1f99a95a599c27c4452a4b8412fd934e2b253f7098508",
        "2235590b90190d7a1dc2464a0205ad",
        "8547d8ac8dc6d9cebb2dc77a7034bb67",
        2,
        9
    );
}
