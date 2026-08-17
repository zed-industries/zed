// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#![allow(clippy::module_name_repetitions)]

use super::{EncryptionAlgorithmId, PrivateDecryptingKey, PublicEncryptingKey};
use crate::aws_lc::{
    EVP_PKEY_CTX_set0_rsa_oaep_label, EVP_PKEY_CTX_set_rsa_mgf1_md, EVP_PKEY_CTX_set_rsa_oaep_md,
    EVP_PKEY_CTX_set_rsa_padding, EVP_PKEY_decrypt, EVP_PKEY_decrypt_init, EVP_PKEY_encrypt,
    EVP_PKEY_encrypt_init, EVP_sha1, EVP_sha256, EVP_sha384, EVP_sha512, OPENSSL_malloc, EVP_MD,
    EVP_PKEY_CTX, RSA_PKCS1_OAEP_PADDING,
};
use crate::error::Unspecified;
use crate::fips::indicator_check;
use crate::ptr::{DetachableLcPtr, LcPtr};
use core::fmt::Debug;
use core::mem::size_of_val;
use core::ptr::null_mut;

/// RSA-OAEP with SHA1 Hash and SHA1 MGF1
pub const OAEP_SHA1_MGF1SHA1: OaepAlgorithm = OaepAlgorithm {
    id: EncryptionAlgorithmId::OaepSha1Mgf1sha1,
    oaep_hash_fn: EVP_sha1,
    mgf1_hash_fn: EVP_sha1,
};

/// RSA-OAEP with SHA256 Hash and SHA256 MGF1
pub const OAEP_SHA256_MGF1SHA256: OaepAlgorithm = OaepAlgorithm {
    id: EncryptionAlgorithmId::OaepSha256Mgf1sha256,
    oaep_hash_fn: EVP_sha256,
    mgf1_hash_fn: EVP_sha256,
};

/// RSA-OAEP with SHA384 Hash and SHA384  MGF1
pub const OAEP_SHA384_MGF1SHA384: OaepAlgorithm = OaepAlgorithm {
    id: EncryptionAlgorithmId::OaepSha384Mgf1sha384,
    oaep_hash_fn: EVP_sha384,
    mgf1_hash_fn: EVP_sha384,
};

/// RSA-OAEP with SHA512 Hash and SHA512 MGF1
pub const OAEP_SHA512_MGF1SHA512: OaepAlgorithm = OaepAlgorithm {
    id: EncryptionAlgorithmId::OaepSha512Mgf1sha512,
    oaep_hash_fn: EVP_sha512,
    mgf1_hash_fn: EVP_sha512,
};

type OaepHashFn = unsafe extern "C" fn() -> *const EVP_MD;
type Mgf1HashFn = unsafe extern "C" fn() -> *const EVP_MD;

/// An RSA-OAEP algorithm.
pub struct OaepAlgorithm {
    id: EncryptionAlgorithmId,
    oaep_hash_fn: OaepHashFn,
    mgf1_hash_fn: Mgf1HashFn,
}

impl OaepAlgorithm {
    /// Returns the `EncryptionAlgorithmId`.
    #[must_use]
    pub fn id(&self) -> EncryptionAlgorithmId {
        self.id
    }

    #[inline]
    fn oaep_hash_fn(&self) -> OaepHashFn {
        self.oaep_hash_fn
    }

    #[inline]
    fn mgf1_hash_fn(&self) -> Mgf1HashFn {
        self.mgf1_hash_fn
    }
}

impl Debug for OaepAlgorithm {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.id, f)
    }
}

/// An RSA-OAEP public key for encryption.
pub struct OaepPublicEncryptingKey {
    public_key: PublicEncryptingKey,
}

impl OaepPublicEncryptingKey {
    /// Constructs an `OaepPublicEncryptingKey` from a `PublicEncryptingKey`.
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
    /// For `OAEP_SHA1_MGF1SHA1`, `OAEP_SHA256_MGF1SHA256`, `OAEP_SHA384_MGF1SHA384`, `OAEP_SHA512_MGF1SHA512` The
    /// length of `output` must be greater then or equal to [`Self::ciphertext_size`].
    ///
    /// # Errors
    /// * `Unspecified` for any error that occurs while encrypting `plaintext`.
    pub fn encrypt<'ciphertext>(
        &self,
        algorithm: &'static OaepAlgorithm,
        plaintext: &[u8],
        ciphertext: &'ciphertext mut [u8],
        label: Option<&[u8]>,
    ) -> Result<&'ciphertext mut [u8], Unspecified> {
        let mut pkey_ctx = self.public_key.0.create_EVP_PKEY_CTX()?;

        if 1 != unsafe { EVP_PKEY_encrypt_init(pkey_ctx.as_mut_ptr()) } {
            return Err(Unspecified);
        }

        configure_oaep_crypto_operation(
            &mut pkey_ctx,
            algorithm.oaep_hash_fn(),
            algorithm.mgf1_hash_fn(),
            label,
        )?;

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

    /// Returns the max plaintext that could be decrypted using this key and with the provided algorithm.
    #[must_use]
    pub fn max_plaintext_size(&self, algorithm: &'static OaepAlgorithm) -> usize {
        #[allow(unreachable_patterns)]
        let hash_len: usize = match algorithm.id() {
            EncryptionAlgorithmId::OaepSha1Mgf1sha1 => 20,
            EncryptionAlgorithmId::OaepSha256Mgf1sha256 => 32,
            EncryptionAlgorithmId::OaepSha384Mgf1sha384 => 48,
            EncryptionAlgorithmId::OaepSha512Mgf1sha512 => 64,
            _ => unreachable!(),
        };

        // The RSA-OAEP algorithms we support use the hashing algorithm for the hash and mgf1 functions.
        self.key_size_bytes() - 2 * hash_len - 2
    }

    /// Returns the max ciphertext size that will be output by `Self::encrypt`.
    #[must_use]
    pub fn ciphertext_size(&self) -> usize {
        self.key_size_bytes()
    }
}

impl Debug for OaepPublicEncryptingKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("OaepPublicEncryptingKey")
            .finish_non_exhaustive()
    }
}

/// An RSA-OAEP private key for decryption.
pub struct OaepPrivateDecryptingKey {
    private_key: PrivateDecryptingKey,
}

impl OaepPrivateDecryptingKey {
    /// Constructs an `OaepPrivateDecryptingKey` from a `PrivateDecryptingKey`.
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
    /// For `OAEP_SHA1_MGF1SHA1`, `OAEP_SHA256_MGF1SHA256`, `OAEP_SHA384_MGF1SHA384`, `OAEP_SHA512_MGF1SHA512`. The
    /// length of `output` must be greater then or equal to [`Self::min_output_size`].
    ///
    /// # Errors
    /// * `Unspecified` for any error that occurs while decrypting `ciphertext`.
    pub fn decrypt<'plaintext>(
        &self,
        algorithm: &'static OaepAlgorithm,
        ciphertext: &[u8],
        plaintext: &'plaintext mut [u8],
        label: Option<&[u8]>,
    ) -> Result<&'plaintext mut [u8], Unspecified> {
        let mut pkey_ctx = self.private_key.0.create_EVP_PKEY_CTX()?;

        if 1 != unsafe { EVP_PKEY_decrypt_init(pkey_ctx.as_mut_ptr()) } {
            return Err(Unspecified);
        }

        configure_oaep_crypto_operation(
            &mut pkey_ctx,
            algorithm.oaep_hash_fn(),
            algorithm.mgf1_hash_fn(),
            label,
        )?;

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

impl Debug for OaepPrivateDecryptingKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("OaepPrivateDecryptingKey")
            .finish_non_exhaustive()
    }
}

fn configure_oaep_crypto_operation(
    evp_pkey_ctx: &mut LcPtr<EVP_PKEY_CTX>,
    oaep_hash_fn: OaepHashFn,
    mgf1_hash_fn: Mgf1HashFn,
    label: Option<&[u8]>,
) -> Result<(), Unspecified> {
    if 1 != unsafe {
        EVP_PKEY_CTX_set_rsa_padding(evp_pkey_ctx.as_mut_ptr(), RSA_PKCS1_OAEP_PADDING)
    } {
        return Err(Unspecified);
    }

    if 1 != unsafe { EVP_PKEY_CTX_set_rsa_oaep_md(evp_pkey_ctx.as_mut_ptr(), oaep_hash_fn()) } {
        return Err(Unspecified);
    }

    if 1 != unsafe { EVP_PKEY_CTX_set_rsa_mgf1_md(evp_pkey_ctx.as_mut_ptr(), mgf1_hash_fn()) } {
        return Err(Unspecified);
    }

    let label = label.unwrap_or(&[0u8; 0]);

    if label.is_empty() {
        // Safety: Don't pass zero-length slice pointers to C code :)
        if 1 != unsafe {
            EVP_PKEY_CTX_set0_rsa_oaep_label(evp_pkey_ctx.as_mut_ptr(), null_mut(), 0)
        } {
            return Err(Unspecified);
        }
        return Ok(());
    }

    // AWS-LC takes ownership of the label memory, and will call OPENSSL_free, so we are forced to copy it for now.
    let mut label_ptr =
        DetachableLcPtr::<u8>::new(unsafe { OPENSSL_malloc(size_of_val(label)) }.cast())?;

    {
        // memcpy the label data into the AWS-LC allocation
        let label_ptr =
            unsafe { core::slice::from_raw_parts_mut(label_ptr.as_mut_ptr(), label.len()) };
        label_ptr.copy_from_slice(label);
    }

    if 1 != unsafe {
        EVP_PKEY_CTX_set0_rsa_oaep_label(
            evp_pkey_ctx.as_mut_ptr(),
            label_ptr.as_mut_ptr(),
            label.len(),
        )
    } {
        return Err(Unspecified);
    }

    // AWS-LC owns the allocation now, so we detach it to avoid freeing it here when label_ptr goes out of scope.
    label_ptr.detach();

    Ok(())
}
