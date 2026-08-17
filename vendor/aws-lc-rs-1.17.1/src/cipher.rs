// Copyright 2018 Brian Smith.
// SPDX-License-Identifier: ISC
// Modifications copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

//! Block and Stream Ciphers for Encryption and Decryption.
//!
//! # 🛑 Read Before Using
//!
//! This module provides access to block and stream cipher algorithms.
//! The modes provided here only provide confidentiality, but **do not**
//! provide integrity or authentication verification of ciphertext.
//!
//! These algorithms are provided solely for applications requiring them
//! in order to maintain backwards compatibility in legacy applications.
//!
//! If you are developing new applications requiring data encryption see
//! the algorithms provided in [`aead`](crate::aead).
//!
//! # Examples
//!
//! ## Encryption Modes
//!
//! ### AES-128 CBC
//!
//! ```rust
//! # use std::error::Error;
//! #
//! # fn main() -> Result<(), Box<dyn Error>> {
//! use aws_lc_rs::cipher::{
//!     PaddedBlockDecryptingKey, PaddedBlockEncryptingKey, UnboundCipherKey, AES_128,
//! };
//! use std::io::Read;
//!
//! let original_message = "This is a secret message!".as_bytes();
//! let mut in_out_buffer = Vec::from(original_message);
//!
//! let key_bytes: &[u8] = &[
//!     0xff, 0x0b, 0xe5, 0x84, 0x64, 0x0b, 0x00, 0xc8, 0x90, 0x7a, 0x4b, 0xbf, 0x82, 0x7c, 0xb6,
//!     0xd1,
//! ];
//!
//! let key = UnboundCipherKey::new(&AES_128, key_bytes)?;
//! let mut encrypting_key = PaddedBlockEncryptingKey::cbc_pkcs7(key)?;
//! let context = encrypting_key.encrypt(&mut in_out_buffer)?;
//!
//! let key = UnboundCipherKey::new(&AES_128, key_bytes)?;
//! let mut decrypting_key = PaddedBlockDecryptingKey::cbc_pkcs7(key)?;
//! let plaintext = decrypting_key.decrypt(&mut in_out_buffer, context)?;
//! assert_eq!(original_message, plaintext);
//! #
//! #
//! # Ok(())
//! # }
//! ```
//!
//! ### AES-128 CTR
//!
//! ```rust
//! # use std::error::Error;
//! #
//! # fn main() -> Result<(), Box<dyn Error>> {
//! use aws_lc_rs::cipher::{DecryptingKey, EncryptingKey, UnboundCipherKey, AES_128};
//!
//! let original_message = "This is a secret message!".as_bytes();
//! let mut in_out_buffer = Vec::from(original_message);
//!
//! let key_bytes: &[u8] = &[
//!     0xff, 0x0b, 0xe5, 0x84, 0x64, 0x0b, 0x00, 0xc8, 0x90, 0x7a, 0x4b, 0xbf, 0x82, 0x7c, 0xb6,
//!     0xd1,
//! ];
//!
//! let key = UnboundCipherKey::new(&AES_128, key_bytes)?;
//! let mut encrypting_key = EncryptingKey::ctr(key)?;
//! let context = encrypting_key.encrypt(&mut in_out_buffer)?;
//!
//! let key = UnboundCipherKey::new(&AES_128, key_bytes)?;
//! let mut decrypting_key = DecryptingKey::ctr(key)?;
//! let plaintext = decrypting_key.decrypt(&mut in_out_buffer, context)?;
//! assert_eq!(original_message, plaintext);
//! #
//! # Ok(())
//! # }
//! ```
//!
//! ### AES-128 CBC Streaming Cipher
//!
//! ```rust
//! # use std::error::Error;
//! #
//! # fn main() -> Result<(), Box<dyn Error>> {
//! use aws_lc_rs::cipher::{
//!     StreamingDecryptingKey, StreamingEncryptingKey, UnboundCipherKey, AES_128,
//! };
//! let original_message = "This is a secret message!".as_bytes();
//! let key_bytes: &[u8] = &[
//!     0xff, 0x0b, 0xe5, 0x84, 0x64, 0x0b, 0x00, 0xc8, 0x90, 0x7a, 0x4b, 0xbf, 0x82, 0x7c,
//!     0xb6, 0xd1,
//! ];
//! // Prepare ciphertext buffer
//! let mut ciphertext_buffer = vec![0u8; original_message.len() + AES_128.block_len()];
//! let ciphertext_slice = ciphertext_buffer.as_mut_slice();
//!
//! // Create StreamingEncryptingKey
//! let key = UnboundCipherKey::new(&AES_128, key_bytes).unwrap();
//! let mut encrypting_key = StreamingEncryptingKey::cbc_pkcs7(key).unwrap();
//!
//! // Encrypt
//! let mut first_update = encrypting_key
//!                            .update(original_message, ciphertext_slice)
//!                            .unwrap();
//! let first_update_len = first_update.written().len();
//! let (context, final_update) = encrypting_key.finish(first_update.remainder_mut()).unwrap();
//! let ciphertext_len = first_update_len + final_update.written().len();
//! let ciphertext = &ciphertext_slice[0..ciphertext_len];
//!
//! // Prepare plaintext buffer
//! let mut plaintext_buffer = vec![0u8; ciphertext_len + AES_128.block_len()];
//! let plaintext_slice = plaintext_buffer.as_mut_slice();
//!
//! // Create StreamingDecryptingKey
//! let key = UnboundCipherKey::new(&AES_128, key_bytes).unwrap();
//! let mut decrypting_key = StreamingDecryptingKey::cbc_pkcs7(key, context).unwrap();
//!
//! // Decrypt
//! let mut first_update = decrypting_key.update(ciphertext, plaintext_slice).unwrap();
//! let first_update_len = first_update.written().len();
//! let final_update = decrypting_key.finish(first_update.remainder_mut()).unwrap();
//! let plaintext_len = first_update_len + final_update.written().len();
//! let plaintext = &plaintext_slice[0..plaintext_len];
//!
//! assert_eq!(original_message, plaintext);
//! #
//! # Ok(())
//! # }
//! ```
//!
//! ### AES-128 CFB 128-bit mode
//!
//! ```rust
//! # use std::error::Error;
//! #
//! # fn main() -> Result<(), Box<dyn Error>> {
//! use aws_lc_rs::cipher::{DecryptingKey, EncryptingKey, UnboundCipherKey, AES_128};
//!
//! let original_message = "This is a secret message!".as_bytes();
//! let mut in_out_buffer = Vec::from(original_message);
//!
//! let key_bytes: &[u8] = &[
//!     0xff, 0x0b, 0xe5, 0x84, 0x64, 0x0b, 0x00, 0xc8, 0x90, 0x7a, 0x4b, 0xbf, 0x82, 0x7c, 0xb6,
//!     0xd1,
//! ];
//!
//! let key = UnboundCipherKey::new(&AES_128, key_bytes)?;
//! let mut encrypting_key = EncryptingKey::cfb128(key)?;
//! let context = encrypting_key.encrypt(&mut in_out_buffer)?;
//!
//! let key = UnboundCipherKey::new(&AES_128, key_bytes)?;
//! let mut decrypting_key = DecryptingKey::cfb128(key)?;
//! let plaintext = decrypting_key.decrypt(&mut in_out_buffer, context)?;
//! assert_eq!(original_message, plaintext);
//! #
//! # Ok(())
//! # }
//! ```
//!
//! ## Constructing a `DecryptionContext` for decryption.
//!
//! ```rust
//! # use std::error::Error;
//! # fn main() -> Result<(), Box<dyn Error>> {
//! use aws_lc_rs::cipher::{DecryptingKey, DecryptionContext, UnboundCipherKey, AES_128};
//! use aws_lc_rs::iv::{FixedLength, IV_LEN_128_BIT};
//!
//! let context = DecryptionContext::Iv128(FixedLength::<IV_LEN_128_BIT>::from(&[
//!     0x8d, 0xdb, 0x7d, 0xf1, 0x56, 0xf5, 0x1c, 0xde, 0x63, 0xe3, 0x4a, 0x34, 0xb0, 0xdf, 0x28,
//!     0xf0,
//! ]));
//!
//! let ciphertext: &[u8] = &[
//!     0x79, 0x8c, 0x04, 0x58, 0xcf, 0x98, 0xb1, 0xe9, 0x97, 0x6b, 0xa1, 0xce,
//! ];
//!
//! let mut in_out_buffer = Vec::from(ciphertext);
//!
//! let key = UnboundCipherKey::new(
//!     &AES_128,
//!     &[
//!         0x5b, 0xfc, 0xe7, 0x5e, 0x57, 0xc5, 0x4d, 0xda, 0x2d, 0xd4, 0x7e, 0x07, 0x0a, 0xef,
//!         0x43, 0x29,
//!     ],
//! )?;
//! let mut decrypting_key = DecryptingKey::ctr(key)?;
//! let plaintext = decrypting_key.decrypt(&mut in_out_buffer, context)?;
//! assert_eq!("Hello World!".as_bytes(), plaintext);
//!
//! # Ok(())
//! # }
//! ```
//!
//! ## Feature flags
//!
//! ### `legacy-des`
//!
//! Enables single DES and Triple DES (`DES_FOR_LEGACY_USE_ONLY`,
//! `DES_EDE_FOR_LEGACY_USE_ONLY`, `DES_EDE3_FOR_LEGACY_USE_ONLY`) for legacy
//! interoperability. Only CBC and ECB modes are supported. All exposed items
//! are `#[deprecated]`. See the crate-level [feature flag
//! documentation](crate#legacy-des) for details and security caveats.
//!
//! ## Getting an immutable reference to the IV slice.
//!
//! `TryFrom<&DecryptionContext>` is implemented for `&[u8]` allowing immutable references
//! to IV bytes returned from cipher encryption operations. Note this is implemented as a `TryFrom` as it
//! may fail for future enum variants that aren't representable as a single slice.
//!
//! ```rust
//! # use std::error::Error;
//! # fn main() -> Result<(), Box<dyn Error>> {
//! # use aws_lc_rs::cipher::DecryptionContext;
//! # use aws_lc_rs::iv::FixedLength;
//! # let x: DecryptionContext = DecryptionContext::Iv128(FixedLength::from([0u8; 16]));
//! // x is type `DecryptionContext`
//! let iv: &[u8] = (&x).try_into()?;
//! # Ok(())
//! # }
//! ```

#![allow(clippy::module_name_repetitions)]

pub(crate) mod aes;
pub(crate) mod block;
pub(crate) mod chacha;
#[cfg(feature = "legacy-des")]
pub(crate) mod des;
pub(crate) mod key;
mod padded;
mod streaming;

pub use padded::{PaddedBlockDecryptingKey, PaddedBlockEncryptingKey};
pub use streaming::{BufferUpdate, StreamingDecryptingKey, StreamingEncryptingKey};

use crate::aws_lc::{
    EVP_aes_128_cbc, EVP_aes_128_cfb128, EVP_aes_128_ctr, EVP_aes_128_ecb, EVP_aes_192_cbc,
    EVP_aes_192_cfb128, EVP_aes_192_ctr, EVP_aes_192_ecb, EVP_aes_256_cbc, EVP_aes_256_cfb128,
    EVP_aes_256_ctr, EVP_aes_256_ecb, EVP_CIPHER,
};
#[cfg(feature = "legacy-des")]
use crate::aws_lc::{
    EVP_des_cbc, EVP_des_ecb, EVP_des_ede, EVP_des_ede3_cbc, EVP_des_ede3_ecb, EVP_des_ede_cbc,
};
use crate::buffer::Buffer;
use crate::error::{KeyRejected, Unspecified};
use crate::hkdf;
use crate::hkdf::KeyType;
#[cfg(feature = "legacy-des")]
use crate::iv::IV_LEN_64_BIT;
use crate::iv::{FixedLength, IV_LEN_128_BIT};
use crate::ptr::ConstPointer;
use core::fmt::Debug;
use key::SymmetricCipherKey;

/// The number of bytes in an AES 128-bit key
pub use crate::cipher::aes::AES_128_KEY_LEN;

/// The number of bytes in an AES 192-bit key
pub use crate::cipher::aes::AES_192_KEY_LEN;

/// The number of bytes in an AES 256-bit key
pub use crate::cipher::aes::AES_256_KEY_LEN;

/// The number of bytes in a single DES key.
///
/// Single DES provides only 56 bits of effective security and is retained
/// for interoperability only.
#[cfg(feature = "legacy-des")]
#[cfg_attr(aws_lc_rs_docsrs, doc(cfg(feature = "legacy-des")))]
#[deprecated(
    note = "Single DES is a legacy algorithm retained for interoperability only. Prefer an AES-based algorithm."
)]
pub use crate::cipher::des::DES_KEY_LEN;

/// The number of bytes in a 2TDEA (DES-EDE, 2-key Triple DES) key.
///
/// 2-key Triple DES is a legacy algorithm and has been disallowed for encryption
/// by NIST SP 800-131A Rev. 2 since 2015. It is retained for interoperability only.
#[cfg(feature = "legacy-des")]
#[cfg_attr(aws_lc_rs_docsrs, doc(cfg(feature = "legacy-des")))]
#[deprecated(
    note = "2-key Triple DES is a legacy algorithm retained for interoperability only; NIST SP 800-131A Rev. 2 disallows its use for encryption. Prefer an AES-based algorithm."
)]
pub use crate::cipher::des::DES_EDE_KEY_LEN;

/// The number of bytes in a 3TDEA (DES-EDE3, 3-key Triple DES) key.
///
/// 3DES is a legacy algorithm and has been disallowed for encryption by
/// NIST SP 800-131A Rev. 2 after 2023. It is retained for interoperability only.
#[cfg(feature = "legacy-des")]
#[cfg_attr(aws_lc_rs_docsrs, doc(cfg(feature = "legacy-des")))]
#[deprecated(
    note = "3DES is a legacy algorithm retained for interoperability only; NIST SP 800-131A Rev. 2 disallows its use for encryption. Prefer an AES-based algorithm."
)]
pub use crate::cipher::des::DES_EDE3_KEY_LEN;

const MAX_CIPHER_KEY_LEN: usize = AES_256_KEY_LEN;

/// The number of bytes for an AES-CBC initialization vector (IV)
pub use crate::cipher::aes::AES_CBC_IV_LEN;

/// The number of bytes for an AES-CTR initialization vector (IV)
pub use crate::cipher::aes::AES_CTR_IV_LEN;

/// The number of bytes for an AES-CFB initialization vector (IV)
pub use crate::cipher::aes::AES_CFB_IV_LEN;

/// The number of bytes for a DES/3DES-CBC initialization vector (IV).
///
/// DES and 3DES are legacy algorithms and have been disallowed for encryption by
/// NIST SP 800-131A Rev. 2 after 2023. They are retained for interoperability only.
#[cfg(feature = "legacy-des")]
#[cfg_attr(aws_lc_rs_docsrs, doc(cfg(feature = "legacy-des")))]
#[deprecated(
    note = "DES/3DES are legacy algorithms retained for interoperability only; NIST SP 800-131A Rev. 2 disallows their use for encryption. Prefer an AES-based algorithm."
)]
pub use crate::cipher::des::DES_CBC_IV_LEN;

use crate::cipher::aes::AES_BLOCK_LEN;
#[cfg(feature = "legacy-des")]
use crate::cipher::des::DES_BLOCK_LEN;

const MAX_CIPHER_BLOCK_LEN: usize = AES_BLOCK_LEN;

/// The cipher operating mode.
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OperatingMode {
    /// Cipher block chaining (CBC) mode.
    CBC,

    /// Counter (CTR) mode.
    CTR,

    /// CFB 128-bit mode.
    CFB128,

    /// Electronic Code Book (ECB) mode.
    ECB,
}

impl OperatingMode {
    fn evp_cipher(&self, algorithm: &Algorithm) -> ConstPointer<'_, EVP_CIPHER> {
        let alg = match (self, algorithm.id) {
            (OperatingMode::CBC, AlgorithmId::Aes128) => unsafe { EVP_aes_128_cbc() },
            (OperatingMode::CTR, AlgorithmId::Aes128) => unsafe { EVP_aes_128_ctr() },
            (OperatingMode::CFB128, AlgorithmId::Aes128) => unsafe { EVP_aes_128_cfb128() },
            (OperatingMode::ECB, AlgorithmId::Aes128) => unsafe { EVP_aes_128_ecb() },
            (OperatingMode::CBC, AlgorithmId::Aes192) => unsafe { EVP_aes_192_cbc() },
            (OperatingMode::CTR, AlgorithmId::Aes192) => unsafe { EVP_aes_192_ctr() },
            (OperatingMode::CFB128, AlgorithmId::Aes192) => unsafe { EVP_aes_192_cfb128() },
            (OperatingMode::ECB, AlgorithmId::Aes192) => unsafe { EVP_aes_192_ecb() },
            (OperatingMode::CBC, AlgorithmId::Aes256) => unsafe { EVP_aes_256_cbc() },
            (OperatingMode::CTR, AlgorithmId::Aes256) => unsafe { EVP_aes_256_ctr() },
            (OperatingMode::CFB128, AlgorithmId::Aes256) => unsafe { EVP_aes_256_cfb128() },
            (OperatingMode::ECB, AlgorithmId::Aes256) => unsafe { EVP_aes_256_ecb() },
            #[cfg(feature = "legacy-des")]
            #[allow(deprecated)]
            (OperatingMode::CBC, AlgorithmId::DesForLegacyUseOnly) => unsafe { EVP_des_cbc() },
            #[cfg(feature = "legacy-des")]
            #[allow(deprecated)]
            (OperatingMode::ECB, AlgorithmId::DesForLegacyUseOnly) => unsafe { EVP_des_ecb() },
            #[cfg(feature = "legacy-des")]
            #[allow(deprecated)]
            (OperatingMode::CBC, AlgorithmId::DesEdeForLegacyUseOnly) => unsafe {
                EVP_des_ede_cbc()
            },
            #[cfg(feature = "legacy-des")]
            #[allow(deprecated)]
            (OperatingMode::ECB, AlgorithmId::DesEdeForLegacyUseOnly) => unsafe { EVP_des_ede() },
            #[cfg(feature = "legacy-des")]
            #[allow(deprecated)]
            (OperatingMode::CBC, AlgorithmId::DesEde3ForLegacyUseOnly) => unsafe {
                EVP_des_ede3_cbc()
            },
            #[cfg(feature = "legacy-des")]
            #[allow(deprecated)]
            (OperatingMode::ECB, AlgorithmId::DesEde3ForLegacyUseOnly) => unsafe {
                EVP_des_ede3_ecb()
            },
            #[cfg(feature = "legacy-des")]
            #[allow(deprecated)]
            (OperatingMode::CTR, AlgorithmId::DesForLegacyUseOnly)
            | (OperatingMode::CTR, AlgorithmId::DesEdeForLegacyUseOnly)
            | (OperatingMode::CTR, AlgorithmId::DesEde3ForLegacyUseOnly)
            | (OperatingMode::CFB128, AlgorithmId::DesForLegacyUseOnly)
            | (OperatingMode::CFB128, AlgorithmId::DesEdeForLegacyUseOnly)
            | (OperatingMode::CFB128, AlgorithmId::DesEde3ForLegacyUseOnly) => {
                // `supports_mode()` rejects these combinations at key-construction
                // time, so this branch is unreachable in practice.
                unreachable!("DES does not support CTR or CFB128 modes")
            }
        };
        unsafe { ConstPointer::new_static(alg).unwrap() }
    }
}

macro_rules! define_cipher_context {
    ($name:ident, $other:ident) => {
        /// The contextual data used to encrypt or decrypt data.
        #[non_exhaustive]
        pub enum $name {
            /// A 128-bit Initialization Vector.
            Iv128(FixedLength<IV_LEN_128_BIT>),

            /// A 64-bit Initialization Vector.
            ///
            /// Used by DES and 3DES in CBC mode only. ECB mode uses
            /// [`Self::None`] regardless of the cipher's block size, and
            /// no other cipher in this crate produces or consumes a 64-bit IV.
            #[cfg(feature = "legacy-des")]
            #[cfg_attr(aws_lc_rs_docsrs, doc(cfg(feature = "legacy-des")))]
            Iv64(FixedLength<IV_LEN_64_BIT>),

            /// No Cipher Context
            None,
        }

        impl<'a> TryFrom<&'a $name> for &'a [u8] {
            type Error = Unspecified;

            fn try_from(value: &'a $name) -> Result<Self, Unspecified> {
                match value {
                    $name::Iv128(iv) => Ok(iv.as_ref()),
                    #[cfg(feature = "legacy-des")]
                    $name::Iv64(iv) => Ok(iv.as_ref()),
                    _ => Err(Unspecified),
                }
            }
        }

        impl Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {
                    Self::Iv128(_) => write!(f, "Iv128"),
                    #[cfg(feature = "legacy-des")]
                    Self::Iv64(_) => write!(f, "Iv64"),
                    Self::None => write!(f, "None"),
                }
            }
        }

        impl From<$other> for $name {
            fn from(value: $other) -> Self {
                match value {
                    $other::Iv128(iv) => $name::Iv128(iv),
                    #[cfg(feature = "legacy-des")]
                    $other::Iv64(iv) => $name::Iv64(iv),
                    $other::None => $name::None,
                }
            }
        }
    };
}

define_cipher_context!(EncryptionContext, DecryptionContext);
define_cipher_context!(DecryptionContext, EncryptionContext);

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// Cipher algorithm identifier.
pub enum AlgorithmId {
    /// AES 128-bit
    Aes128,

    /// AES 256-bit
    Aes256,

    /// AES 192-bit
    Aes192,

    /// Single DES (for legacy use only).
    ///
    /// Single DES provides only 56 bits of effective security and has been
    /// considered insecure for decades.
    #[cfg(feature = "legacy-des")]
    #[cfg_attr(aws_lc_rs_docsrs, doc(cfg(feature = "legacy-des")))]
    #[deprecated(
        note = "Single DES is a legacy algorithm retained for interoperability only. Prefer an AES-based algorithm."
    )]
    DesForLegacyUseOnly,

    /// 2TDEA (for legacy use only).
    ///
    /// 2-key Triple DES has been disallowed for encryption by
    /// NIST SP 800-131A Rev. 2 since 2015.
    #[cfg(feature = "legacy-des")]
    #[cfg_attr(aws_lc_rs_docsrs, doc(cfg(feature = "legacy-des")))]
    #[deprecated(
        note = "2-key Triple DES is a legacy algorithm retained for interoperability only; NIST SP 800-131A Rev. 2 disallows its use for encryption. Prefer an AES-based algorithm."
    )]
    DesEdeForLegacyUseOnly,

    /// 3TDEA (for legacy use only).
    ///
    /// 3DES has been disallowed for encryption by NIST SP 800-131A Rev. 2
    /// after 2023.
    #[cfg(feature = "legacy-des")]
    #[cfg_attr(aws_lc_rs_docsrs, doc(cfg(feature = "legacy-des")))]
    #[deprecated(
        note = "3DES is a legacy algorithm retained for interoperability only; NIST SP 800-131A Rev. 2 disallows its use for encryption. Prefer an AES-based algorithm."
    )]
    DesEde3ForLegacyUseOnly,
}

/// A cipher algorithm.
#[derive(Debug, PartialEq, Eq)]
pub struct Algorithm {
    id: AlgorithmId,
    key_len: usize,
    block_len: usize,
}

/// AES 128-bit cipher
pub const AES_128: Algorithm = Algorithm {
    id: AlgorithmId::Aes128,
    key_len: AES_128_KEY_LEN,
    block_len: AES_BLOCK_LEN,
};

/// AES 192-bit cipher
pub const AES_192: Algorithm = Algorithm {
    id: AlgorithmId::Aes192,
    key_len: AES_192_KEY_LEN,
    block_len: AES_BLOCK_LEN,
};

/// AES 256-bit cipher
pub const AES_256: Algorithm = Algorithm {
    id: AlgorithmId::Aes256,
    key_len: AES_256_KEY_LEN,
    block_len: AES_BLOCK_LEN,
};

/// Single DES cipher, for legacy interoperability only.
///
/// Single DES provides only 56 bits of effective security and has been
/// considered insecure for decades. It is retained here solely for
/// interoperability with legacy systems. New designs must use an AES-based
/// algorithm.
///
/// Only CBC and ECB operating modes are supported. Weak and semi-weak DES
/// keys are rejected.
#[cfg(feature = "legacy-des")]
#[cfg_attr(aws_lc_rs_docsrs, doc(cfg(feature = "legacy-des")))]
#[allow(deprecated)]
#[deprecated(
    note = "Single DES is a legacy algorithm retained for interoperability only. Prefer an AES-based algorithm."
)]
pub const DES_FOR_LEGACY_USE_ONLY: Algorithm = Algorithm {
    id: AlgorithmId::DesForLegacyUseOnly,
    key_len: DES_KEY_LEN,
    block_len: DES_BLOCK_LEN,
};

/// 2TDEA (DES-EDE, 2-key Triple DES) cipher, for legacy interoperability only.
///
/// 2-key Triple DES is a legacy algorithm and has been disallowed for encryption
/// by [NIST SP 800-131A Rev. 2](https://csrc.nist.gov/publications/detail/sp/800-131a/rev-2/final)
/// since 2015 — several years earlier than 3-key Triple DES. It is retained here
/// solely for interoperability. New designs must use an AES-based algorithm.
///
/// Only CBC and ECB operating modes are supported. Key material is validated
/// beyond the length check: weak and semi-weak DES subkeys are rejected, and
/// K1 must differ from K2 (otherwise the cipher degenerates to single-DES).
/// This validation is deferred — constructing an [`UnboundCipherKey`] only
/// checks length; the weak-key and pairwise-distinctness checks fire when the
/// [`UnboundCipherKey`] is passed to an [`EncryptingKey`], [`DecryptingKey`],
/// [`PaddedBlockEncryptingKey`], [`PaddedBlockDecryptingKey`],
/// [`StreamingEncryptingKey`], or [`StreamingDecryptingKey`] constructor.
#[cfg(feature = "legacy-des")]
#[cfg_attr(aws_lc_rs_docsrs, doc(cfg(feature = "legacy-des")))]
#[allow(deprecated)]
#[deprecated(
    note = "2-key Triple DES is a legacy algorithm retained for interoperability only; NIST SP 800-131A Rev. 2 disallows its use for encryption. Prefer an AES-based algorithm."
)]
pub const DES_EDE_FOR_LEGACY_USE_ONLY: Algorithm = Algorithm {
    id: AlgorithmId::DesEdeForLegacyUseOnly,
    key_len: DES_EDE_KEY_LEN,
    block_len: DES_BLOCK_LEN,
};

/// 3TDEA (DES-EDE3, 3-key Triple DES) cipher, for legacy interoperability only.
///
/// 3DES is a legacy algorithm and has been disallowed for encryption by
/// [NIST SP 800-131A Rev. 2](https://csrc.nist.gov/publications/detail/sp/800-131a/rev-2/final)
/// after 2023. It is retained here solely for interoperability. New designs
/// must use an AES-based algorithm.
///
/// Only CBC and ECB operating modes are supported. Key material is validated
/// beyond the length check: weak and semi-weak DES subkeys are rejected, and
/// K1, K2, and K3 must all be pairwise distinct (otherwise 3TDEA degenerates
/// to either single-DES or 2TDEA). This validation is deferred —
/// constructing an [`UnboundCipherKey`] only checks length; the weak-key and
/// pairwise-distinctness checks fire when the [`UnboundCipherKey`] is passed
/// to an [`EncryptingKey`], [`DecryptingKey`], [`PaddedBlockEncryptingKey`],
/// [`PaddedBlockDecryptingKey`], [`StreamingEncryptingKey`], or
/// [`StreamingDecryptingKey`] constructor.
///
/// Callers who specifically need 2-key Triple DES (2TDEA) must use
/// [`DES_EDE_FOR_LEGACY_USE_ONLY`] (16-byte key) rather than encoding 2TDEA
/// as a 24-byte `K1 ‖ K2 ‖ K1` key here.
///
/// # Example: 3DES-CBC interop round-trip
///
/// ```rust
/// # #[cfg(feature = "legacy-des")]
/// # #[allow(deprecated)]
/// # fn main() -> Result<(), aws_lc_rs::error::Unspecified> {
/// use aws_lc_rs::cipher::{
///     DecryptingKey, EncryptingKey, UnboundCipherKey, DES_EDE3_FOR_LEGACY_USE_ONLY,
/// };
///
/// let key_bytes: [u8; 24] = [
///     0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
///     0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01,
///     0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23,
/// ];
/// let plaintext = *b"8-byte..16-byte.";
///
/// let key = UnboundCipherKey::new(&DES_EDE3_FOR_LEGACY_USE_ONLY, &key_bytes)?;
/// let mut buf = plaintext.to_vec();
/// let ctx = EncryptingKey::cbc(key)?.encrypt(&mut buf)?;
///
/// let key = UnboundCipherKey::new(&DES_EDE3_FOR_LEGACY_USE_ONLY, &key_bytes)?;
/// let recovered = DecryptingKey::cbc(key)?.decrypt(&mut buf, ctx)?;
/// assert_eq!(&plaintext[..], recovered);
/// # Ok(())
/// # }
/// # #[cfg(not(feature = "legacy-des"))]
/// # fn main() {}
/// ```
#[cfg(feature = "legacy-des")]
#[cfg_attr(aws_lc_rs_docsrs, doc(cfg(feature = "legacy-des")))]
#[allow(deprecated)]
#[deprecated(
    note = "3DES is a legacy algorithm retained for interoperability only; NIST SP 800-131A Rev. 2 disallows its use for encryption. Prefer an AES-based algorithm."
)]
pub const DES_EDE3_FOR_LEGACY_USE_ONLY: Algorithm = Algorithm {
    id: AlgorithmId::DesEde3ForLegacyUseOnly,
    key_len: DES_EDE3_KEY_LEN,
    block_len: DES_BLOCK_LEN,
};

impl Algorithm {
    fn id(&self) -> &AlgorithmId {
        &self.id
    }

    /// The block length of this cipher algorithm.
    #[must_use]
    pub const fn block_len(&self) -> usize {
        self.block_len
    }

    fn new_encryption_context(
        &self,
        mode: OperatingMode,
    ) -> Result<EncryptionContext, Unspecified> {
        match self.id {
            // TODO: Hopefully support CFB1, and CFB8
            AlgorithmId::Aes128 | AlgorithmId::Aes192 | AlgorithmId::Aes256 => match mode {
                OperatingMode::CBC | OperatingMode::CTR | OperatingMode::CFB128 => {
                    Ok(EncryptionContext::Iv128(FixedLength::new()?))
                }
                OperatingMode::ECB => Ok(EncryptionContext::None),
            },
            #[cfg(feature = "legacy-des")]
            #[allow(deprecated)]
            AlgorithmId::DesForLegacyUseOnly
            | AlgorithmId::DesEdeForLegacyUseOnly
            | AlgorithmId::DesEde3ForLegacyUseOnly => match mode {
                OperatingMode::CBC => Ok(EncryptionContext::Iv64(FixedLength::new()?)),
                OperatingMode::ECB => Ok(EncryptionContext::None),
                _ => Err(Unspecified),
            },
        }
    }

    fn is_valid_encryption_context(&self, mode: OperatingMode, input: &EncryptionContext) -> bool {
        match self.id {
            // TODO: Hopefully support CFB1, and CFB8
            AlgorithmId::Aes128 | AlgorithmId::Aes192 | AlgorithmId::Aes256 => match mode {
                OperatingMode::CBC | OperatingMode::CTR | OperatingMode::CFB128 => {
                    matches!(input, EncryptionContext::Iv128(_))
                }
                OperatingMode::ECB => {
                    matches!(input, EncryptionContext::None)
                }
            },
            #[cfg(feature = "legacy-des")]
            #[allow(deprecated)]
            AlgorithmId::DesForLegacyUseOnly
            | AlgorithmId::DesEdeForLegacyUseOnly
            | AlgorithmId::DesEde3ForLegacyUseOnly => match mode {
                OperatingMode::CBC => matches!(input, EncryptionContext::Iv64(_)),
                OperatingMode::ECB => matches!(input, EncryptionContext::None),
                _ => false,
            },
        }
    }

    fn is_valid_decryption_context(&self, mode: OperatingMode, input: &DecryptionContext) -> bool {
        // TODO: Hopefully support CFB1, and CFB8
        match self.id {
            AlgorithmId::Aes128 | AlgorithmId::Aes192 | AlgorithmId::Aes256 => match mode {
                OperatingMode::CBC | OperatingMode::CTR | OperatingMode::CFB128 => {
                    matches!(input, DecryptionContext::Iv128(_))
                }
                OperatingMode::ECB => {
                    matches!(input, DecryptionContext::None)
                }
            },
            #[cfg(feature = "legacy-des")]
            #[allow(deprecated)]
            AlgorithmId::DesForLegacyUseOnly
            | AlgorithmId::DesEdeForLegacyUseOnly
            | AlgorithmId::DesEde3ForLegacyUseOnly => match mode {
                OperatingMode::CBC => matches!(input, DecryptionContext::Iv64(_)),
                OperatingMode::ECB => matches!(input, DecryptionContext::None),
                _ => false,
            },
        }
    }

    /// Returns `true` if the algorithm supports the given operating mode.
    ///
    /// Used to reject invalid `(algorithm, mode)` combinations (e.g. DES with
    /// CTR or CFB128) at key-construction time rather than deferring the
    /// failure to the first encrypt/decrypt call.
    fn supports_mode(&self, mode: OperatingMode) -> bool {
        match self.id {
            AlgorithmId::Aes128 | AlgorithmId::Aes192 | AlgorithmId::Aes256 => matches!(
                mode,
                OperatingMode::CBC
                    | OperatingMode::CTR
                    | OperatingMode::CFB128
                    | OperatingMode::ECB
            ),
            #[cfg(feature = "legacy-des")]
            #[allow(deprecated)]
            AlgorithmId::DesForLegacyUseOnly
            | AlgorithmId::DesEdeForLegacyUseOnly
            | AlgorithmId::DesEde3ForLegacyUseOnly => {
                matches!(mode, OperatingMode::CBC | OperatingMode::ECB)
            }
        }
    }
}

#[allow(clippy::missing_fields_in_debug)]
impl Debug for UnboundCipherKey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        f.debug_struct("UnboundCipherKey")
            .field("algorithm", &self.algorithm)
            .finish()
    }
}

impl From<hkdf::Okm<'_, &'static Algorithm>> for UnboundCipherKey {
    fn from(okm: hkdf::Okm<&'static Algorithm>) -> Self {
        let mut key_bytes = [0; MAX_CIPHER_KEY_LEN];
        let key_bytes = &mut key_bytes[..okm.len().key_len];
        let algorithm = *okm.len();
        okm.fill(key_bytes).unwrap();
        Self::new(algorithm, key_bytes).unwrap()
    }
}

impl KeyType for &'static Algorithm {
    fn len(&self) -> usize {
        self.key_len
    }
}

/// A key bound to a particular cipher algorithm.
pub struct UnboundCipherKey {
    algorithm: &'static Algorithm,
    key_bytes: Buffer<'static, &'static [u8]>,
}

impl UnboundCipherKey {
    /// Constructs an [`UnboundCipherKey`].
    ///
    /// # Errors
    ///
    /// * [`Unspecified`] if `key_bytes.len()` does not match the length required by `algorithm`.
    pub fn new(algorithm: &'static Algorithm, key_bytes: &[u8]) -> Result<Self, Unspecified> {
        if key_bytes.len() != algorithm.key_len {
            return Err(Unspecified);
        }
        let key_bytes = Buffer::new(key_bytes.to_vec());
        Ok(UnboundCipherKey {
            algorithm,
            key_bytes,
        })
    }

    #[inline]
    #[must_use]
    /// Returns the algorithm associated with this key.
    pub fn algorithm(&self) -> &'static Algorithm {
        self.algorithm
    }

    /// Validates the key material against algorithm-specific constraints beyond
    /// the length check performed by [`UnboundCipherKey::new`].
    ///
    /// For DES algorithms (behind `legacy-des`), this rejects weak DES subkeys
    /// and degenerate key configurations (e.g. K1 == K2 for 2TDEA). For other
    /// algorithms this is a no-op since the only constraint is key length.
    ///
    /// The streaming cipher constructors call this because they pass raw key
    /// bytes directly to the EVP API rather than going through
    /// [`SymmetricCipherKey`] construction, which would otherwise perform these
    /// checks. This is necessary because the EVP layer internally uses
    /// `DES_set_key_unchecked`, which skips weak-key detection.
    #[allow(clippy::unused_self)]
    #[allow(clippy::unnecessary_wraps)]
    fn validate_key_material(&self) -> Result<(), KeyRejected> {
        #[cfg(feature = "legacy-des")]
        #[allow(deprecated)]
        match self.algorithm.id() {
            AlgorithmId::DesForLegacyUseOnly => {
                let _ = SymmetricCipherKey::prepare_des(self.key_bytes.as_ref())?;
            }
            AlgorithmId::DesEdeForLegacyUseOnly => {
                let _ = SymmetricCipherKey::prepare_des_ede(self.key_bytes.as_ref())?;
            }
            AlgorithmId::DesEde3ForLegacyUseOnly => {
                let _ = SymmetricCipherKey::prepare_des_ede3(self.key_bytes.as_ref())?;
            }
            _ => {}
        }
        Ok(())
    }
}

impl TryInto<SymmetricCipherKey> for UnboundCipherKey {
    type Error = KeyRejected;

    fn try_into(self) -> Result<SymmetricCipherKey, Self::Error> {
        match self.algorithm.id() {
            AlgorithmId::Aes128 => SymmetricCipherKey::aes128(self.key_bytes.as_ref()),
            AlgorithmId::Aes192 => SymmetricCipherKey::aes192(self.key_bytes.as_ref()),
            AlgorithmId::Aes256 => SymmetricCipherKey::aes256(self.key_bytes.as_ref()),
            #[cfg(feature = "legacy-des")]
            #[allow(deprecated)]
            AlgorithmId::DesForLegacyUseOnly => SymmetricCipherKey::des(self.key_bytes.as_ref()),
            #[cfg(feature = "legacy-des")]
            #[allow(deprecated)]
            AlgorithmId::DesEdeForLegacyUseOnly => {
                SymmetricCipherKey::des_ede(self.key_bytes.as_ref())
            }
            #[cfg(feature = "legacy-des")]
            #[allow(deprecated)]
            AlgorithmId::DesEde3ForLegacyUseOnly => {
                SymmetricCipherKey::des_ede3(self.key_bytes.as_ref())
            }
        }
    }
}

/// A cipher encryption key that does not perform block padding.
pub struct EncryptingKey {
    algorithm: &'static Algorithm,
    key: SymmetricCipherKey,
    mode: OperatingMode,
}

impl EncryptingKey {
    /// Constructs an `EncryptingKey` operating in counter (CTR) mode using the provided key.
    ///
    // # FIPS
    // Use this function with an `UnboundCipherKey` constructed with one of the following algorithms:
    // * `AES_128`
    // * `AES_256`
    //
    /// # Errors
    /// * [`Unspecified`]: Returned if there is an error constructing the `EncryptingKey`.
    ///   With `legacy-des` enabled, also returned if `key`'s algorithm does not
    ///   support CTR mode (e.g. `DES_FOR_LEGACY_USE_ONLY`,
    ///   `DES_EDE_FOR_LEGACY_USE_ONLY`, `DES_EDE3_FOR_LEGACY_USE_ONLY`).
    pub fn ctr(key: UnboundCipherKey) -> Result<Self, Unspecified> {
        Self::new(key, OperatingMode::CTR)
    }

    /// Constructs an `EncryptingKey` operating in cipher feedback 128-bit mode (CFB128) using the provided key.
    ///
    // # FIPS
    // Use this function with an `UnboundCipherKey` constructed with one of the following algorithms:
    // * `AES_128`
    // * `AES_256`
    //
    /// # Errors
    /// * [`Unspecified`]: Returned if there is an error constructing the `EncryptingKey`.
    ///   With `legacy-des` enabled, also returned if `key`'s algorithm does not
    ///   support CFB128 mode (e.g. `DES_FOR_LEGACY_USE_ONLY`,
    ///   `DES_EDE_FOR_LEGACY_USE_ONLY`, `DES_EDE3_FOR_LEGACY_USE_ONLY`).
    pub fn cfb128(key: UnboundCipherKey) -> Result<Self, Unspecified> {
        Self::new(key, OperatingMode::CFB128)
    }

    /// Constructs an `EncryptingKey` operating in cipher block chaining (CBC) mode using the provided key.
    ///
    /// # ☠️ ️️️DANGER ☠️
    /// Offered for compatibility purposes only. This is an extremely dangerous mode, and
    /// very likely not what you want to use.
    ///
    // # FIPS
    // Use this function with an `UnboundCipherKey` constructed with one of the following algorithms:
    // * `AES_128`
    // * `AES_256`
    //
    // `DES_FOR_LEGACY_USE_ONLY`, `DES_EDE_FOR_LEGACY_USE_ONLY` and
    // `DES_EDE3_FOR_LEGACY_USE_ONLY` are never FIPS-approved.
    //
    /// # Errors
    /// * [`Unspecified`]: Returned if there is an error constructing the `EncryptingKey`.
    ///   With `legacy-des` enabled, also returned if `key` was constructed with
    ///   `DES_FOR_LEGACY_USE_ONLY`, `DES_EDE_FOR_LEGACY_USE_ONLY` or
    ///   `DES_EDE3_FOR_LEGACY_USE_ONLY` and the provided key material contains
    ///   weak or semi-weak DES subkeys, or (for Triple DES) a degenerate subkey
    ///   configuration (e.g. `K1 == K2` for 2TDEA, or any pairwise equality
    ///   for 3TDEA).
    pub fn cbc(key: UnboundCipherKey) -> Result<Self, Unspecified> {
        Self::new(key, OperatingMode::CBC)
    }

    /// Constructs an `EncryptingKey` operating in electronic code book mode (ECB) using the provided key.
    ///
    /// # ☠️ ️️️DANGER ☠️
    /// Offered for compatibility purposes only. This is an extremely dangerous mode, and
    /// very likely not what you want to use.
    ///
    // # FIPS
    // Use this function with an `UnboundCipherKey` constructed with one of the following algorithms:
    // * `AES_128`
    // * `AES_256`
    //
    // `DES_FOR_LEGACY_USE_ONLY`, `DES_EDE_FOR_LEGACY_USE_ONLY` and
    // `DES_EDE3_FOR_LEGACY_USE_ONLY` are never FIPS-approved.
    //
    /// # Errors
    /// * [`Unspecified`]: Returned if there is an error constructing the `EncryptingKey`.
    ///   With `legacy-des` enabled, also returned if `key` was constructed with
    ///   `DES_FOR_LEGACY_USE_ONLY`, `DES_EDE_FOR_LEGACY_USE_ONLY` or
    ///   `DES_EDE3_FOR_LEGACY_USE_ONLY` and the provided key material contains
    ///   weak or semi-weak DES subkeys, or (for Triple DES) a degenerate subkey
    ///   configuration (e.g. `K1 == K2` for 2TDEA, or any pairwise equality
    ///   for 3TDEA).
    pub fn ecb(key: UnboundCipherKey) -> Result<Self, Unspecified> {
        Self::new(key, OperatingMode::ECB)
    }

    #[allow(clippy::unnecessary_wraps)]
    fn new(key: UnboundCipherKey, mode: OperatingMode) -> Result<Self, Unspecified> {
        let algorithm = key.algorithm();
        if !algorithm.supports_mode(mode) {
            return Err(Unspecified);
        }
        let key = key.try_into()?;
        Ok(Self {
            algorithm,
            key,
            mode,
        })
    }

    /// Returns the cipher algorithm.
    #[must_use]
    pub fn algorithm(&self) -> &Algorithm {
        self.algorithm
    }

    /// Returns the cipher operating mode.
    #[must_use]
    pub fn mode(&self) -> OperatingMode {
        self.mode
    }

    /// Encrypts the data provided in `in_out` in-place.
    /// Returns a [`DecryptionContext`] with the randomly generated IV that was used to encrypt
    /// the data provided.
    ///
    /// If `EncryptingKey` is operating in `OperatingMode::ECB`, then `in_out.len()` must be a multiple
    /// of the block length.
    ///
    /// # Errors
    /// * [`Unspecified`]: Returned if cipher mode requires input to be a multiple of the block length,
    ///   and `in_out.len()` is not. Otherwise, returned if encryption fails.
    pub fn encrypt(&self, in_out: &mut [u8]) -> Result<DecryptionContext, Unspecified> {
        let context = self.algorithm.new_encryption_context(self.mode)?;
        self.less_safe_encrypt(in_out, context)
    }

    /// Encrypts the data provided in `in_out` in-place using the provided `EncryptionContext`.
    /// This is considered "less safe" because the caller could potentially construct
    /// a `EncryptionContext` from a previously used IV (initialization vector).
    /// Returns a [`DecryptionContext`] produced from the provided `EncryptionContext`.
    ///
    /// If `EncryptingKey` is operating in `OperatingMode::ECB`, then `in_out.len()` must be a multiple
    /// of the block length.
    ///
    /// # Errors
    /// * [`Unspecified`]: Returned if cipher mode requires input to be a multiple of the block length,
    ///   and `in_out.len()` is not. Otherwise returned if encryption fails.
    pub fn less_safe_encrypt(
        &self,
        in_out: &mut [u8],
        context: EncryptionContext,
    ) -> Result<DecryptionContext, Unspecified> {
        if !self
            .algorithm()
            .is_valid_encryption_context(self.mode, &context)
        {
            return Err(Unspecified);
        }
        encrypt(self.algorithm(), &self.key, self.mode, in_out, context)
    }
}

impl Debug for EncryptingKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EncryptingKey")
            .field("algorithm", self.algorithm)
            .field("mode", &self.mode)
            .finish_non_exhaustive()
    }
}

/// A cipher decryption key that does not perform block padding.
pub struct DecryptingKey {
    algorithm: &'static Algorithm,
    key: SymmetricCipherKey,
    mode: OperatingMode,
}

impl DecryptingKey {
    /// Constructs a cipher decrypting key operating in counter (CTR) mode using the provided key and context.
    ///
    // # FIPS
    // Use this function with an `UnboundCipherKey` constructed with one of the following algorithms:
    // * `AES_128`
    // * `AES_256`
    //
    /// # Errors
    /// * [`Unspecified`]: Returned if there is an error during decryption.
    ///   With `legacy-des` enabled, also returned if `key`'s algorithm does not
    ///   support CTR mode (e.g. `DES_FOR_LEGACY_USE_ONLY`,
    ///   `DES_EDE_FOR_LEGACY_USE_ONLY`, `DES_EDE3_FOR_LEGACY_USE_ONLY`).
    pub fn ctr(key: UnboundCipherKey) -> Result<DecryptingKey, Unspecified> {
        Self::new(key, OperatingMode::CTR)
    }

    /// Constructs a cipher decrypting key operating in cipher feedback 128-bit mode (CFB128) using the provided key and context.
    ///
    // # FIPS
    // Use this function with an `UnboundCipherKey` constructed with one of the following algorithms:
    // * `AES_128`
    // * `AES_256`
    //
    /// # Errors
    /// * [`Unspecified`]: Returned if there is an error during decryption.
    ///   With `legacy-des` enabled, also returned if `key`'s algorithm does not
    ///   support CFB128 mode (e.g. `DES_FOR_LEGACY_USE_ONLY`,
    ///   `DES_EDE_FOR_LEGACY_USE_ONLY`, `DES_EDE3_FOR_LEGACY_USE_ONLY`).
    pub fn cfb128(key: UnboundCipherKey) -> Result<Self, Unspecified> {
        Self::new(key, OperatingMode::CFB128)
    }

    /// Constructs an `DecryptingKey` operating in cipher block chaining (CBC) mode using the provided key and context.
    ///
    /// # ☠️ ️️️DANGER ☠️
    /// Offered for compatibility purposes only. This is an extremely dangerous mode, and
    /// very likely not what you want to use.
    ///
    // # FIPS
    // Use this function with an `UnboundCipherKey` constructed with one of the following algorithms:
    // * `AES_128`
    // * `AES_256`
    //
    // `DES_FOR_LEGACY_USE_ONLY`, `DES_EDE_FOR_LEGACY_USE_ONLY` and
    // `DES_EDE3_FOR_LEGACY_USE_ONLY` are never FIPS-approved.
    //
    /// # Errors
    /// * [`Unspecified`]: Returned if there is an error during decryption.
    ///   With `legacy-des` enabled, also returned if `key` was constructed with
    ///   `DES_FOR_LEGACY_USE_ONLY`, `DES_EDE_FOR_LEGACY_USE_ONLY` or
    ///   `DES_EDE3_FOR_LEGACY_USE_ONLY` and the provided key material contains
    ///   weak or semi-weak DES subkeys, or (for Triple DES) a degenerate subkey
    ///   configuration (e.g. `K1 == K2` for 2TDEA, or any pairwise equality
    ///   for 3TDEA).
    pub fn cbc(key: UnboundCipherKey) -> Result<DecryptingKey, Unspecified> {
        Self::new(key, OperatingMode::CBC)
    }

    /// Constructs an `DecryptingKey` operating in electronic code book (ECB) mode using the provided key.
    ///
    /// # ☠️ ️️️DANGER ☠️
    /// Offered for compatibility purposes only. This is an extremely dangerous mode, and
    /// very likely not what you want to use.
    ///
    // # FIPS
    // Use this function with an `UnboundCipherKey` constructed with one of the following algorithms:
    // * `AES_128`
    // * `AES_256`
    //
    // `DES_FOR_LEGACY_USE_ONLY`, `DES_EDE_FOR_LEGACY_USE_ONLY` and
    // `DES_EDE3_FOR_LEGACY_USE_ONLY` are never FIPS-approved.
    //
    /// # Errors
    /// * [`Unspecified`]: Returned if there is an error constructing the `DecryptingKey`.
    ///   With `legacy-des` enabled, also returned if `key` was constructed with
    ///   `DES_FOR_LEGACY_USE_ONLY`, `DES_EDE_FOR_LEGACY_USE_ONLY` or
    ///   `DES_EDE3_FOR_LEGACY_USE_ONLY` and the provided key material contains
    ///   weak or semi-weak DES subkeys, or (for Triple DES) a degenerate subkey
    ///   configuration (e.g. `K1 == K2` for 2TDEA, or any pairwise equality
    ///   for 3TDEA).
    pub fn ecb(key: UnboundCipherKey) -> Result<Self, Unspecified> {
        Self::new(key, OperatingMode::ECB)
    }

    #[allow(clippy::unnecessary_wraps)]
    fn new(key: UnboundCipherKey, mode: OperatingMode) -> Result<Self, Unspecified> {
        let algorithm = key.algorithm();
        if !algorithm.supports_mode(mode) {
            return Err(Unspecified);
        }
        let key = key.try_into()?;
        Ok(Self {
            algorithm,
            key,
            mode,
        })
    }

    /// Returns the cipher algorithm.
    #[must_use]
    pub fn algorithm(&self) -> &Algorithm {
        self.algorithm
    }

    /// Returns the cipher operating mode.
    #[must_use]
    pub fn mode(&self) -> OperatingMode {
        self.mode
    }

    /// Decrypts the data provided in `in_out` in-place.
    /// Returns a references to the decrypted data.
    ///
    /// If `DecryptingKey` is operating in `OperatingMode::ECB`, then `in_out.len()` must be a multiple
    /// of the block length.
    ///
    /// # Errors
    /// * [`Unspecified`]: Returned if cipher mode requires input to be a multiple of the block length,
    ///   and `in_out.len()` is not. Also returned if decryption fails.
    pub fn decrypt<'in_out>(
        &self,
        in_out: &'in_out mut [u8],
        context: DecryptionContext,
    ) -> Result<&'in_out mut [u8], Unspecified> {
        decrypt(self.algorithm, &self.key, self.mode, in_out, context)
    }
}

impl Debug for DecryptingKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DecryptingKey")
            .field("algorithm", &self.algorithm)
            .field("mode", &self.mode)
            .finish_non_exhaustive()
    }
}

fn encrypt(
    algorithm: &Algorithm,
    key: &SymmetricCipherKey,
    mode: OperatingMode,
    in_out: &mut [u8],
    context: EncryptionContext,
) -> Result<DecryptionContext, Unspecified> {
    let block_len = algorithm.block_len();

    match mode {
        OperatingMode::CBC | OperatingMode::ECB if in_out.len() % block_len != 0 => {
            return Err(Unspecified);
        }
        _ => {}
    }

    match mode {
        OperatingMode::CBC => match algorithm.id() {
            AlgorithmId::Aes128 | AlgorithmId::Aes192 | AlgorithmId::Aes256 => {
                aes::encrypt_cbc_mode(key, context, in_out)
            }
            #[cfg(feature = "legacy-des")]
            #[allow(deprecated)]
            AlgorithmId::DesForLegacyUseOnly
            | AlgorithmId::DesEdeForLegacyUseOnly
            | AlgorithmId::DesEde3ForLegacyUseOnly => des::encrypt_cbc_mode(key, context, in_out),
        },
        OperatingMode::CTR => match algorithm.id() {
            AlgorithmId::Aes128 | AlgorithmId::Aes192 | AlgorithmId::Aes256 => {
                aes::encrypt_ctr_mode(key, context, in_out)
            }
            #[cfg(feature = "legacy-des")]
            #[allow(deprecated)]
            AlgorithmId::DesForLegacyUseOnly
            | AlgorithmId::DesEdeForLegacyUseOnly
            | AlgorithmId::DesEde3ForLegacyUseOnly => {
                unreachable!("DES does not support CTR mode.")
            }
        },
        // TODO: Hopefully support CFB1, and CFB8
        OperatingMode::CFB128 => match algorithm.id() {
            AlgorithmId::Aes128 | AlgorithmId::Aes192 | AlgorithmId::Aes256 => {
                aes::encrypt_cfb_mode(key, mode, context, in_out)
            }
            #[cfg(feature = "legacy-des")]
            #[allow(deprecated)]
            AlgorithmId::DesForLegacyUseOnly
            | AlgorithmId::DesEdeForLegacyUseOnly
            | AlgorithmId::DesEde3ForLegacyUseOnly => {
                unreachable!("DES does not support CFB128 mode.")
            }
        },
        OperatingMode::ECB => match algorithm.id() {
            AlgorithmId::Aes128 | AlgorithmId::Aes192 | AlgorithmId::Aes256 => {
                aes::encrypt_ecb_mode(key, context, in_out)
            }
            #[cfg(feature = "legacy-des")]
            #[allow(deprecated)]
            AlgorithmId::DesForLegacyUseOnly
            | AlgorithmId::DesEdeForLegacyUseOnly
            | AlgorithmId::DesEde3ForLegacyUseOnly => des::encrypt_ecb_mode(key, context, in_out),
        },
    }
}

fn decrypt<'in_out>(
    algorithm: &'static Algorithm,
    key: &SymmetricCipherKey,
    mode: OperatingMode,
    in_out: &'in_out mut [u8],
    context: DecryptionContext,
) -> Result<&'in_out mut [u8], Unspecified> {
    let block_len = algorithm.block_len();

    match mode {
        OperatingMode::CBC | OperatingMode::ECB if in_out.len() % block_len != 0 => {
            return Err(Unspecified);
        }
        _ => {}
    }

    match mode {
        OperatingMode::CBC => match algorithm.id() {
            AlgorithmId::Aes128 | AlgorithmId::Aes192 | AlgorithmId::Aes256 => {
                aes::decrypt_cbc_mode(key, context, in_out)
            }
            #[cfg(feature = "legacy-des")]
            #[allow(deprecated)]
            AlgorithmId::DesForLegacyUseOnly
            | AlgorithmId::DesEdeForLegacyUseOnly
            | AlgorithmId::DesEde3ForLegacyUseOnly => des::decrypt_cbc_mode(key, context, in_out),
        },
        OperatingMode::CTR => match algorithm.id() {
            AlgorithmId::Aes128 | AlgorithmId::Aes192 | AlgorithmId::Aes256 => {
                aes::decrypt_ctr_mode(key, context, in_out)
            }
            #[cfg(feature = "legacy-des")]
            #[allow(deprecated)]
            AlgorithmId::DesForLegacyUseOnly
            | AlgorithmId::DesEdeForLegacyUseOnly
            | AlgorithmId::DesEde3ForLegacyUseOnly => {
                unreachable!("DES does not support CTR mode.")
            }
        },
        // TODO: Hopefully support CFB1, and CFB8
        OperatingMode::CFB128 => match algorithm.id() {
            AlgorithmId::Aes128 | AlgorithmId::Aes192 | AlgorithmId::Aes256 => {
                aes::decrypt_cfb_mode(key, mode, context, in_out)
            }
            #[cfg(feature = "legacy-des")]
            #[allow(deprecated)]
            AlgorithmId::DesForLegacyUseOnly
            | AlgorithmId::DesEdeForLegacyUseOnly
            | AlgorithmId::DesEde3ForLegacyUseOnly => {
                unreachable!("DES does not support CFB128 mode.")
            }
        },
        OperatingMode::ECB => match algorithm.id() {
            AlgorithmId::Aes128 | AlgorithmId::Aes192 | AlgorithmId::Aes256 => {
                aes::decrypt_ecb_mode(key, context, in_out)
            }
            #[cfg(feature = "legacy-des")]
            #[allow(deprecated)]
            AlgorithmId::DesForLegacyUseOnly
            | AlgorithmId::DesEdeForLegacyUseOnly
            | AlgorithmId::DesEde3ForLegacyUseOnly => des::decrypt_ecb_mode(key, context, in_out),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::from_hex;

    #[cfg(feature = "fips")]
    mod fips;

    #[test]
    fn test_debug() {
        {
            let aes_128_key_bytes = from_hex("000102030405060708090a0b0c0d0e0f").unwrap();
            let cipher_key = UnboundCipherKey::new(&AES_128, aes_128_key_bytes.as_slice()).unwrap();
            assert_eq!("UnboundCipherKey { algorithm: Algorithm { id: Aes128, key_len: 16, block_len: 16 } }", format!("{cipher_key:?}"));
        }

        {
            let aes_256_key_bytes =
                from_hex("000102030405060708090a0b0c0d0e0f000102030405060708090a0b0c0d0e0f")
                    .unwrap();
            let cipher_key = UnboundCipherKey::new(&AES_256, aes_256_key_bytes.as_slice()).unwrap();
            assert_eq!("UnboundCipherKey { algorithm: Algorithm { id: Aes256, key_len: 32, block_len: 16 } }", format!("{cipher_key:?}"));
        }

        {
            let key_bytes = &[0u8; 16];
            let key = PaddedBlockEncryptingKey::cbc_pkcs7(
                UnboundCipherKey::new(&AES_128, key_bytes).unwrap(),
            )
            .unwrap();
            assert_eq!("PaddedBlockEncryptingKey { algorithm: Algorithm { id: Aes128, key_len: 16, block_len: 16 }, mode: CBC, padding: PKCS7, .. }", format!("{key:?}"));
            let mut data = vec![0u8; 16];
            let context = key.encrypt(&mut data).unwrap();
            assert_eq!("Iv128", format!("{context:?}"));
            let key = PaddedBlockDecryptingKey::cbc_pkcs7(
                UnboundCipherKey::new(&AES_128, key_bytes).unwrap(),
            )
            .unwrap();
            assert_eq!("PaddedBlockDecryptingKey { algorithm: Algorithm { id: Aes128, key_len: 16, block_len: 16 }, mode: CBC, padding: PKCS7, .. }", format!("{key:?}"));
        }

        {
            let key_bytes = &[0u8; 16];
            let key =
                EncryptingKey::ctr(UnboundCipherKey::new(&AES_128, key_bytes).unwrap()).unwrap();
            assert_eq!("EncryptingKey { algorithm: Algorithm { id: Aes128, key_len: 16, block_len: 16 }, mode: CTR, .. }", format!("{key:?}"));
            let mut data = vec![0u8; 16];
            let context = key.encrypt(&mut data).unwrap();
            assert_eq!("Iv128", format!("{context:?}"));
            let key =
                DecryptingKey::ctr(UnboundCipherKey::new(&AES_128, key_bytes).unwrap()).unwrap();
            assert_eq!("DecryptingKey { algorithm: Algorithm { id: Aes128, key_len: 16, block_len: 16 }, mode: CTR, .. }", format!("{key:?}"));
        }
    }

    fn helper_test_cipher_n_bytes(
        key: &[u8],
        alg: &'static Algorithm,
        mode: OperatingMode,
        n: usize,
    ) {
        let mut input: Vec<u8> = Vec::with_capacity(n);
        for i in 0..n {
            let byte: u8 = i.try_into().unwrap();
            input.push(byte);
        }

        let cipher_key = UnboundCipherKey::new(alg, key).unwrap();
        let encrypting_key = EncryptingKey::new(cipher_key, mode).unwrap();

        let mut in_out = input.clone();
        let decrypt_iv = encrypting_key.encrypt(&mut in_out).unwrap();

        if n > 5 {
            // There's no more than a 1 in 2^48 chance that this will fail randomly
            assert_ne!(input.as_slice(), in_out);
        }

        let cipher_key2 = UnboundCipherKey::new(alg, key).unwrap();
        let decrypting_key = DecryptingKey::new(cipher_key2, mode).unwrap();

        let plaintext = decrypting_key.decrypt(&mut in_out, decrypt_iv).unwrap();
        assert_eq!(input.as_slice(), plaintext);
    }

    #[test]
    fn test_aes_128_ctr() {
        let key = from_hex("000102030405060708090a0b0c0d0e0f").unwrap();
        for i in 0..=50 {
            helper_test_cipher_n_bytes(key.as_slice(), &AES_128, OperatingMode::CTR, i);
        }
    }

    #[test]
    fn test_aes_128_cfb128() {
        let key = from_hex("000102030405060708090a0b0c0d0e0f").unwrap();
        for i in 0..=50 {
            helper_test_cipher_n_bytes(key.as_slice(), &AES_128, OperatingMode::CFB128, i);
        }
    }

    #[test]
    fn test_aes_256_cfb128() {
        let key =
            from_hex("000102030405060708090a0b0c0d0e0f000102030405060708090a0b0c0d0e0f").unwrap();
        for i in 0..=50 {
            helper_test_cipher_n_bytes(key.as_slice(), &AES_256, OperatingMode::CFB128, i);
        }
    }

    #[test]
    fn test_aes_256_ctr() {
        let key =
            from_hex("000102030405060708090a0b0c0d0e0f000102030405060708090a0b0c0d0e0f").unwrap();
        for i in 0..=50 {
            helper_test_cipher_n_bytes(key.as_slice(), &AES_256, OperatingMode::CTR, i);
        }
    }

    #[test]
    fn test_aes_128_cbc() {
        let key = from_hex("000102030405060708090a0b0c0d0e0f").unwrap();
        // CBC mode requires input to be a multiple of block size (16 bytes)
        for i in 0..=3 {
            let size = i * 16; // Test with 0, 16, 32, 48 bytes
            helper_test_cipher_n_bytes(key.as_slice(), &AES_128, OperatingMode::CBC, size);
        }
    }

    #[test]
    fn test_aes_256_cbc() {
        let key =
            from_hex("000102030405060708090a0b0c0d0e0f000102030405060708090a0b0c0d0e0f").unwrap();
        // CBC mode requires input to be a multiple of block size (16 bytes)
        for i in 0..=3 {
            let size = i * 16; // Test with 0, 16, 32, 48 bytes
            helper_test_cipher_n_bytes(key.as_slice(), &AES_256, OperatingMode::CBC, size);
        }
    }

    #[test]
    fn test_aes_128_ecb() {
        let key = from_hex("000102030405060708090a0b0c0d0e0f").unwrap();
        _ = key;
    }

    macro_rules! cipher_kat {
        ($name:ident, $alg:expr, $mode:expr, $key:literal, $iv: literal, $plaintext:literal, $ciphertext:literal) => {
            #[test]
            fn $name() {
                let key = from_hex($key).unwrap();
                let input = from_hex($plaintext).unwrap();
                let expected_ciphertext = from_hex($ciphertext).unwrap();
                let mut iv = from_hex($iv).unwrap();
                let iv = {
                    let slice = iv.as_mut_slice();
                    let mut iv = [0u8; $iv.len() / 2];
                    {
                        let x = iv.as_mut_slice();
                        x.copy_from_slice(slice);
                    }
                    iv
                };

                let ec = EncryptionContext::Iv128(FixedLength::from(iv));

                let alg = $alg;

                let unbound_key = UnboundCipherKey::new(alg, &key).unwrap();

                let encrypting_key = EncryptingKey::new(unbound_key, $mode).unwrap();

                let mut in_out = input.clone();

                let context = encrypting_key.less_safe_encrypt(&mut in_out, ec).unwrap();

                assert_eq!(expected_ciphertext, in_out);

                let unbound_key2 = UnboundCipherKey::new(alg, &key).unwrap();
                let decrypting_key = DecryptingKey::new(unbound_key2, $mode).unwrap();

                let plaintext = decrypting_key.decrypt(&mut in_out, context).unwrap();
                assert_eq!(input.as_slice(), plaintext);
            }
        };
        ($name:ident, $alg:expr, $mode:expr, $key:literal, $plaintext:literal, $ciphertext:literal) => {
            #[test]
            fn $name() {
                let key = from_hex($key).unwrap();
                let input = from_hex($plaintext).unwrap();
                let expected_ciphertext = from_hex($ciphertext).unwrap();

                let alg = $alg;

                let unbound_key = UnboundCipherKey::new(alg, &key).unwrap();

                let encrypting_key = EncryptingKey::new(unbound_key, $mode).unwrap();

                let mut in_out = input.clone();

                let context = encrypting_key
                    .less_safe_encrypt(&mut in_out, EncryptionContext::None)
                    .unwrap();

                assert_eq!(expected_ciphertext, in_out);

                let unbound_key2 = UnboundCipherKey::new(alg, &key).unwrap();
                let decrypting_key = DecryptingKey::new(unbound_key2, $mode).unwrap();

                let plaintext = decrypting_key.decrypt(&mut in_out, context).unwrap();
                assert_eq!(input.as_slice(), plaintext);
            }
        };
    }

    cipher_kat!(
        test_iv_aes_128_ctr_16_bytes,
        &AES_128,
        OperatingMode::CTR,
        "000102030405060708090a0b0c0d0e0f",
        "00000000000000000000000000000000",
        "00112233445566778899aabbccddeeff",
        "c6b01904c3da3df5e7d62bd96d153686"
    );

    cipher_kat!(
        test_iv_aes_256_ctr_15_bytes,
        &AES_256,
        OperatingMode::CTR,
        "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
        "00000000000000000000000000000000",
        "00112233445566778899aabbccddee",
        "f28122856e1cf9a7216a30d111f399"
    );

    cipher_kat!(
        test_openssl_aes_128_ctr_15_bytes,
        &AES_128,
        OperatingMode::CTR,
        "244828580821c1652582c76e34d299f5",
        "093145d5af233f46072a5eb5adc11aa1",
        "3ee38cec171e6cf466bf0df98aa0e1",
        "bd7d928f60e3422d96b3f8cd614eb2"
    );

    cipher_kat!(
        test_openssl_aes_256_ctr_15_bytes,
        &AES_256,
        OperatingMode::CTR,
        "0857db8240ea459bdf660b4cced66d1f2d3734ff2de7b81e92740e65e7cc6a1d",
        "f028ecb053f801102d11fccc9d303a27",
        "eca7285d19f3c20e295378460e8729",
        "b5098e5e788de6ac2f2098eb2fc6f8"
    );

    cipher_kat!(
        test_sp800_38a_cfb128_aes128,
        &AES_128,
        OperatingMode::CFB128,
        "2b7e151628aed2a6abf7158809cf4f3c",
        "000102030405060708090a0b0c0d0e0f",
        "6bc1bee22e409f96e93d7e117393172aae2d8a571e03ac9c9eb76fac45af8e5130c81c46a35ce411e5fbc1191a0a52eff69f2445df4f9b17ad2b417be66c3710",
        "3b3fd92eb72dad20333449f8e83cfb4ac8a64537a0b3a93fcde3cdad9f1ce58b26751f67a3cbb140b1808cf187a4f4dfc04b05357c5d1c0eeac4c66f9ff7f2e6"
    );

    cipher_kat!(
        test_sp800_38a_cfb128_aes256,
        &AES_256,
        OperatingMode::CFB128,
        "603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4",
        "000102030405060708090a0b0c0d0e0f",
        "6bc1bee22e409f96e93d7e117393172aae2d8a571e03ac9c9eb76fac45af8e5130c81c46a35ce411e5fbc1191a0a52eff69f2445df4f9b17ad2b417be66c3710",
        "dc7e84bfda79164b7ecd8486985d386039ffed143b28b1c832113c6331e5407bdf10132415e54b92a13ed0a8267ae2f975a385741ab9cef82031623d55b1e471"
    );

    cipher_kat!(
        test_sp800_38a_ecb_aes128,
        &AES_128,
        OperatingMode::ECB,
        "2b7e151628aed2a6abf7158809cf4f3c",
        "6bc1bee22e409f96e93d7e117393172aae2d8a571e03ac9c9eb76fac45af8e5130c81c46a35ce411e5fbc1191a0a52eff69f2445df4f9b17ad2b417be66c3710",
        "3ad77bb40d7a3660a89ecaf32466ef97f5d3d58503b9699de785895a96fdbaaf43b1cd7f598ece23881b00e3ed0306887b0c785e27e8ad3f8223207104725dd4"
    );

    cipher_kat!(
        test_sp800_38a_ecb_aes256,
        &AES_256,
        OperatingMode::ECB,
        "603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4",
        "6bc1bee22e409f96e93d7e117393172aae2d8a571e03ac9c9eb76fac45af8e5130c81c46a35ce411e5fbc1191a0a52eff69f2445df4f9b17ad2b417be66c3710",
        "f3eed1bdb5d2a03c064b5a7e3db181f8591ccb10d410ed26dc5ba74a31362870b6ed21b99ca6f4f9f153e7b1beafed1d23304b7a39f9f3ff067d8d8f9e24ecc7"
    );

    cipher_kat!(
        test_sp800_38a_cbc_aes128,
        &AES_128,
        OperatingMode::CBC,
        "2b7e151628aed2a6abf7158809cf4f3c",
        "000102030405060708090a0b0c0d0e0f",
        "6bc1bee22e409f96e93d7e117393172a",
        "7649abac8119b246cee98e9b12e9197d"
    );

    cipher_kat!(
        test_sp800_38a_cbc_aes256,
        &AES_256,
        OperatingMode::CBC,
        "603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4",
        "000102030405060708090a0b0c0d0e0f",
        "6bc1bee22e409f96e93d7e117393172a",
        "f58c4c04d6e5f1ba779eabfb5f7bfbd6"
    );

    cipher_kat!(
        test_sp800_38a_cbc_aes128_multi_block,
        &AES_128,
        OperatingMode::CBC,
        "2b7e151628aed2a6abf7158809cf4f3c",
        "000102030405060708090a0b0c0d0e0f",
        "6bc1bee22e409f96e93d7e117393172aae2d8a571e03ac9c9eb76fac45af8e5130c81c46a35ce411e5fbc1191a0a52eff69f2445df4f9b17ad2b417be66c3710",
        "7649abac8119b246cee98e9b12e9197d5086cb9b507219ee95db113a917678b273bed6b8e3c1743b7116e69e222295163ff1caa1681fac09120eca307586e1a7"
    );

    cipher_kat!(
        test_sp800_38a_cbc_aes256_multi_block,
        &AES_256,
        OperatingMode::CBC,
        "603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4",
        "000102030405060708090a0b0c0d0e0f",
        "6bc1bee22e409f96e93d7e117393172aae2d8a571e03ac9c9eb76fac45af8e5130c81c46a35ce411e5fbc1191a0a52eff69f2445df4f9b17ad2b417be66c3710",
        "f58c4c04d6e5f1ba779eabfb5f7bfbd69cfc4e967edb808d679f777bc6702c7d39f23369a9d9bacfa530e26304231461b2eb05e2c39be9fcda6c19078c6a9d1b"
    );

    #[cfg(feature = "legacy-des")]
    #[test]
    #[allow(deprecated)]
    fn test_des_rejects_weak_key() {
        let weak_keys: [&str; 4] = [
            "0101010101010101",
            "fefefefefefefefe",
            "e0e0e0e0f1f1f1f1",
            "1f1f1f1f0e0e0e0e",
        ];
        for weak in &weak_keys {
            let key = from_hex(weak).unwrap();
            assert!(
                UnboundCipherKey::new(&DES_FOR_LEGACY_USE_ONLY, &key)
                    .and_then(EncryptingKey::cbc)
                    .is_err(),
                "expected weak DES key {weak} to be rejected"
            );
        }
    }

    #[cfg(feature = "legacy-des")]
    #[test]
    #[allow(deprecated)]
    fn test_des_rejects_semi_weak_key() {
        let semi_weak_keys: [&str; 12] = [
            "01fe01fe01fe01fe",
            "fe01fe01fe01fe01",
            "1fe01fe00ef10ef1",
            "e01fe01ff10ef10e",
            "01e001e001f101f1",
            "e001e001f101f101",
            "1ffe1ffe0efe0efe",
            "fe1ffe1ffe0efe0e",
            "011f011f010e010e",
            "1f011f010e010e01",
            "e0fee0fef1fef1fe",
            "fee0fee0fef1fef1",
        ];
        for semi_weak in &semi_weak_keys {
            let key = from_hex(semi_weak).unwrap();
            assert!(
                UnboundCipherKey::new(&DES_FOR_LEGACY_USE_ONLY, &key)
                    .and_then(EncryptingKey::cbc)
                    .is_err(),
                "expected semi-weak DES key {semi_weak} to be rejected"
            );
        }
    }

    #[cfg(feature = "legacy-des")]
    #[test]
    #[allow(deprecated)]
    fn test_des_ede_rejects_k1_equal_k2() {
        // SP 800-67 §3.1 requires K1 != K2 for 2-Key TDEA. If K1 == K2 then
        // 2TDEA collapses to single-DES (56-bit effective security).
        let weak_key = from_hex("0123456789abcdef0123456789abcdef").unwrap();
        let result = UnboundCipherKey::new(&DES_EDE_FOR_LEGACY_USE_ONLY, &weak_key)
            .and_then(EncryptingKey::cbc);
        assert!(result.is_err(), "expected K1 == K2 to be rejected");

        // Sanity check: differing K1 and K2 of the same length is accepted.
        let good_key = from_hex("0123456789abcdef23456789abcdef01").unwrap();
        UnboundCipherKey::new(&DES_EDE_FOR_LEGACY_USE_ONLY, &good_key)
            .and_then(EncryptingKey::cbc)
            .expect("valid 2TDEA key should be accepted");
    }

    #[cfg(feature = "legacy-des")]
    #[test]
    #[allow(deprecated)]
    fn test_des_ede_rejects_weak_subkey() {
        // Each of DES_set_key's four weak keys must be rejected when used as
        // either subkey of a 2TDEA key. The weak keys (with correct odd
        // parity) are 0101..01, FEFE..FE, E0E0..E0F1F1..F1, and
        // 1F1F..1F0E0E..0E; see openssl/des.h.
        let weak_keys: [&str; 4] = [
            "0101010101010101",
            "fefefefefefefefe",
            "e0e0e0e0f1f1f1f1",
            "1f1f1f1f0e0e0e0e",
        ];
        let distinct = "23456789abcdef01";
        for weak in &weak_keys {
            // Weak K1, non-weak K2.
            let key = from_hex(&format!("{}{}", weak, distinct)).unwrap();
            assert!(
                UnboundCipherKey::new(&DES_EDE_FOR_LEGACY_USE_ONLY, &key)
                    .and_then(EncryptingKey::cbc)
                    .is_err(),
                "expected weak DES K1 = {weak} to be rejected in 2TDEA"
            );
            // Non-weak K1, weak K2.
            let key = from_hex(&format!("{}{}", distinct, weak)).unwrap();
            assert!(
                UnboundCipherKey::new(&DES_EDE_FOR_LEGACY_USE_ONLY, &key)
                    .and_then(EncryptingKey::cbc)
                    .is_err(),
                "expected weak DES K2 = {weak} to be rejected in 2TDEA"
            );
        }
    }

    #[cfg(feature = "legacy-des")]
    #[test]
    #[allow(deprecated)]
    fn test_des_ede3_rejects_degenerate_keys() {
        // SP 800-67 §2 (Keying Option 1) requires K1, K2, and K3 to be pairwise
        // independent. We enforce that all three subkeys are distinct so the cipher
        // cannot degenerate to single-DES (K1==K2 or K2==K3) or to 2TDEA (K1==K3).
        // Callers who want 2TDEA must use `DES_EDE_FOR_LEGACY_USE_ONLY` with a 16-byte key.
        let k1_eq_k2 = from_hex("0123456789abcdef0123456789abcdef456789abcdef0123").unwrap();
        assert!(
            UnboundCipherKey::new(&DES_EDE3_FOR_LEGACY_USE_ONLY, &k1_eq_k2)
                .and_then(EncryptingKey::cbc)
                .is_err(),
            "expected K1 == K2 to be rejected for 3TDEA"
        );

        let k2_eq_k3 = from_hex("0123456789abcdef23456789abcdef0123456789abcdef01").unwrap();
        assert!(
            UnboundCipherKey::new(&DES_EDE3_FOR_LEGACY_USE_ONLY, &k2_eq_k3)
                .and_then(EncryptingKey::cbc)
                .is_err(),
            "expected K2 == K3 to be rejected for 3TDEA"
        );

        let k1_eq_k3 = from_hex("0123456789abcdef23456789abcdef010123456789abcdef").unwrap();
        assert!(
            UnboundCipherKey::new(&DES_EDE3_FOR_LEGACY_USE_ONLY, &k1_eq_k3)
                .and_then(EncryptingKey::cbc)
                .is_err(),
            "expected K1 == K3 (2TDEA-in-3TDEA form) to be rejected for 3TDEA"
        );

        // All three distinct is accepted.
        let good_key = from_hex("0123456789abcdef23456789abcdef01456789abcdef0123").unwrap();
        UnboundCipherKey::new(&DES_EDE3_FOR_LEGACY_USE_ONLY, &good_key)
            .and_then(EncryptingKey::cbc)
            .expect("valid 3TDEA key should be accepted");
    }

    #[cfg(feature = "legacy-des")]
    #[test]
    #[allow(deprecated)]
    fn test_des_ede3_rejects_weak_subkey() {
        // Each of `DES_set_key`'s four weak keys must be rejected when used
        // as any subkey of a 3TDEA key. The weak keys (with correct odd
        // parity) are 0101..01, FEFE..FE, E0E0..E0F1F1..F1, and
        // 1F1F..1F0E0E..0E; see openssl/des.h.
        let weak_keys: [&str; 4] = [
            "0101010101010101",
            "fefefefefefefefe",
            "e0e0e0e0f1f1f1f1",
            "1f1f1f1f0e0e0e0e",
        ];
        let fillers = ["23456789abcdef01", "456789abcdef0123"];

        for weak in &weak_keys {
            for position in 0..3 {
                // Place the weak key at `position` and fill the other two
                // slots with distinct non-weak subkeys so that the K1 != K2
                // and K2 != K3 checks do not short-circuit the weak-key
                // rejection we are trying to exercise.
                let mut parts = [""; 3];
                parts[position] = *weak;
                parts[(position + 1) % 3] = fillers[0];
                parts[(position + 2) % 3] = fillers[1];
                let key = from_hex(&format!("{}{}{}", parts[0], parts[1], parts[2])).unwrap();
                assert!(
                    UnboundCipherKey::new(&DES_EDE3_FOR_LEGACY_USE_ONLY, &key)
                        .and_then(EncryptingKey::cbc)
                        .is_err(),
                    "expected weak DES subkey {weak} at position {position} to be rejected"
                );
            }
        }
    }

    #[cfg(feature = "legacy-des")]
    #[test]
    #[allow(deprecated)]
    fn test_des_ede_rejects_semi_weak_subkey() {
        // DES_set_key also rejects semi-weak keys (return value -2).
        // There are 6 semi-weak key pairs (12 keys total); each member of a
        // pair encrypts what the other decrypts, making the cipher an
        // involuntary self-inverse. The semi-weak keys are listed in
        // openssl/des.h. We verify that using any one of them as either
        // subkey of a 2TDEA key is rejected.
        // The 12 DES semi-weak keys (6 pairs); each key in a pair is its own
        // partner's inverse. These are exactly the keys for which DES_set_key
        // returns -2 (along with the 4 weak keys already tested separately).
        // Values taken from OpenSSL/AWS-LC des.h.
        let semi_weak_keys: [&str; 12] = [
            "01fe01fe01fe01fe",
            "fe01fe01fe01fe01",
            "1fe01fe00ef10ef1",
            "e01fe01ff10ef10e",
            "01e001e001f101f1",
            "e001e001f101f101",
            "1ffe1ffe0efe0efe",
            "fe1ffe1ffe0efe0e",
            "011f011f010e010e",
            "1f011f010e010e01",
            "e0fee0fef1fef1fe",
            "fee0fee0fef1fef1",
        ];
        let distinct = "0123456789abcdef";
        for semi_weak in &semi_weak_keys {
            let key = from_hex(&format!("{}{}", semi_weak, distinct)).unwrap();
            assert!(
                UnboundCipherKey::new(&DES_EDE_FOR_LEGACY_USE_ONLY, &key)
                    .and_then(EncryptingKey::cbc)
                    .is_err(),
                "expected semi-weak DES K1 = {semi_weak} to be rejected in 2TDEA"
            );
            let key = from_hex(&format!("{}{}", distinct, semi_weak)).unwrap();
            assert!(
                UnboundCipherKey::new(&DES_EDE_FOR_LEGACY_USE_ONLY, &key)
                    .and_then(EncryptingKey::cbc)
                    .is_err(),
                "expected semi-weak DES K2 = {semi_weak} to be rejected in 2TDEA"
            );
        }
    }

    #[cfg(feature = "legacy-des")]
    #[test]
    #[allow(deprecated)]
    fn test_des_ede3_rejects_semi_weak_subkey() {
        // Same 12 semi-weak keys as above; verify rejection at each of the
        // three subkey positions in a 3TDEA key.
        let semi_weak_keys: [&str; 12] = [
            "01fe01fe01fe01fe",
            "fe01fe01fe01fe01",
            "1fe01fe00ef10ef1",
            "e01fe01ff10ef10e",
            "01e001e001f101f1",
            "e001e001f101f101",
            "1ffe1ffe0efe0efe",
            "fe1ffe1ffe0efe0e",
            "011f011f010e010e",
            "1f011f010e010e01",
            "e0fee0fef1fef1fe",
            "fee0fee0fef1fef1",
        ];
        let fillers = ["0123456789abcdef", "23456789abcdef01"];

        for semi_weak in &semi_weak_keys {
            for position in 0..3 {
                let mut parts = [""; 3];
                parts[position] = *semi_weak;
                parts[(position + 1) % 3] = fillers[0];
                parts[(position + 2) % 3] = fillers[1];
                let key = from_hex(&format!("{}{}{}", parts[0], parts[1], parts[2])).unwrap();
                assert!(
                    UnboundCipherKey::new(&DES_EDE3_FOR_LEGACY_USE_ONLY, &key)
                        .and_then(EncryptingKey::cbc)
                        .is_err(),
                    "expected semi-weak DES subkey {semi_weak} at position {position} to be rejected in 3TDEA"
                );
            }
        }
    }

    #[cfg(feature = "legacy-des")]
    #[test]
    #[allow(deprecated)]
    fn test_des_rejects_unsupported_modes() {
        // DES/2TDEA/3TDEA only support CBC and ECB in this crate; CTR and CFB128
        // must be rejected at construction time rather than at encrypt time.
        let key1 = from_hex("0123456789abcdef").unwrap();
        assert!(
            UnboundCipherKey::new(&DES_FOR_LEGACY_USE_ONLY, &key1)
                .and_then(EncryptingKey::ctr)
                .is_err(),
            "DES + CTR (encrypt) should be rejected"
        );
        assert!(
            UnboundCipherKey::new(&DES_FOR_LEGACY_USE_ONLY, &key1)
                .and_then(EncryptingKey::cfb128)
                .is_err(),
            "DES + CFB128 (encrypt) should be rejected"
        );
        assert!(
            UnboundCipherKey::new(&DES_FOR_LEGACY_USE_ONLY, &key1)
                .and_then(DecryptingKey::ctr)
                .is_err(),
            "DES + CTR (decrypt) should be rejected"
        );
        assert!(
            UnboundCipherKey::new(&DES_FOR_LEGACY_USE_ONLY, &key1)
                .and_then(DecryptingKey::cfb128)
                .is_err(),
            "DES + CFB128 (decrypt) should be rejected"
        );

        let key2 = from_hex("0123456789abcdef23456789abcdef01").unwrap();
        assert!(
            UnboundCipherKey::new(&DES_EDE_FOR_LEGACY_USE_ONLY, &key2)
                .and_then(EncryptingKey::ctr)
                .is_err(),
            "2TDEA + CTR (encrypt) should be rejected"
        );
        assert!(
            UnboundCipherKey::new(&DES_EDE_FOR_LEGACY_USE_ONLY, &key2)
                .and_then(EncryptingKey::cfb128)
                .is_err(),
            "2TDEA + CFB128 (encrypt) should be rejected"
        );
        assert!(
            UnboundCipherKey::new(&DES_EDE_FOR_LEGACY_USE_ONLY, &key2)
                .and_then(DecryptingKey::ctr)
                .is_err(),
            "2TDEA + CTR (decrypt) should be rejected"
        );
        assert!(
            UnboundCipherKey::new(&DES_EDE_FOR_LEGACY_USE_ONLY, &key2)
                .and_then(DecryptingKey::cfb128)
                .is_err(),
            "2TDEA + CFB128 (decrypt) should be rejected"
        );

        let key3 = from_hex("0123456789abcdef23456789abcdef01456789abcdef0123").unwrap();
        assert!(
            UnboundCipherKey::new(&DES_EDE3_FOR_LEGACY_USE_ONLY, &key3)
                .and_then(EncryptingKey::ctr)
                .is_err(),
            "3TDEA + CTR (encrypt) should be rejected"
        );
        assert!(
            UnboundCipherKey::new(&DES_EDE3_FOR_LEGACY_USE_ONLY, &key3)
                .and_then(EncryptingKey::cfb128)
                .is_err(),
            "3TDEA + CFB128 (encrypt) should be rejected"
        );
        assert!(
            UnboundCipherKey::new(&DES_EDE3_FOR_LEGACY_USE_ONLY, &key3)
                .and_then(DecryptingKey::ctr)
                .is_err(),
            "3TDEA + CTR (decrypt) should be rejected"
        );
        assert!(
            UnboundCipherKey::new(&DES_EDE3_FOR_LEGACY_USE_ONLY, &key3)
                .and_then(DecryptingKey::cfb128)
                .is_err(),
            "3TDEA + CFB128 (decrypt) should be rejected"
        );

        // Streaming constructors must also reject unsupported modes.
        assert!(
            UnboundCipherKey::new(&DES_FOR_LEGACY_USE_ONLY, &key1)
                .and_then(StreamingEncryptingKey::ctr)
                .is_err(),
            "DES + CTR (streaming encrypt) should be rejected"
        );
        assert!(
            UnboundCipherKey::new(&DES_FOR_LEGACY_USE_ONLY, &key1)
                .and_then(StreamingEncryptingKey::cfb128)
                .is_err(),
            "DES + CFB128 (streaming encrypt) should be rejected"
        );
        assert!(
            UnboundCipherKey::new(&DES_FOR_LEGACY_USE_ONLY, &key1)
                .and_then(|k| StreamingDecryptingKey::ctr(k, DecryptionContext::None))
                .is_err(),
            "DES + CTR (streaming decrypt) should be rejected"
        );
        assert!(
            UnboundCipherKey::new(&DES_FOR_LEGACY_USE_ONLY, &key1)
                .and_then(|k| StreamingDecryptingKey::cfb128(k, DecryptionContext::None))
                .is_err(),
            "DES + CFB128 (streaming decrypt) should be rejected"
        );

        assert!(
            UnboundCipherKey::new(&DES_EDE_FOR_LEGACY_USE_ONLY, &key2)
                .and_then(StreamingEncryptingKey::ctr)
                .is_err(),
            "2TDEA + CTR (streaming encrypt) should be rejected"
        );
        assert!(
            UnboundCipherKey::new(&DES_EDE_FOR_LEGACY_USE_ONLY, &key2)
                .and_then(StreamingEncryptingKey::cfb128)
                .is_err(),
            "2TDEA + CFB128 (streaming encrypt) should be rejected"
        );
        assert!(
            UnboundCipherKey::new(&DES_EDE_FOR_LEGACY_USE_ONLY, &key2)
                .and_then(|k| StreamingDecryptingKey::ctr(k, DecryptionContext::None))
                .is_err(),
            "2TDEA + CTR (streaming decrypt) should be rejected"
        );
        assert!(
            UnboundCipherKey::new(&DES_EDE_FOR_LEGACY_USE_ONLY, &key2)
                .and_then(|k| StreamingDecryptingKey::cfb128(k, DecryptionContext::None))
                .is_err(),
            "2TDEA + CFB128 (streaming decrypt) should be rejected"
        );

        assert!(
            UnboundCipherKey::new(&DES_EDE3_FOR_LEGACY_USE_ONLY, &key3)
                .and_then(StreamingEncryptingKey::ctr)
                .is_err(),
            "3TDEA + CTR (streaming encrypt) should be rejected"
        );
        assert!(
            UnboundCipherKey::new(&DES_EDE3_FOR_LEGACY_USE_ONLY, &key3)
                .and_then(StreamingEncryptingKey::cfb128)
                .is_err(),
            "3TDEA + CFB128 (streaming encrypt) should be rejected"
        );
        assert!(
            UnboundCipherKey::new(&DES_EDE3_FOR_LEGACY_USE_ONLY, &key3)
                .and_then(|k| StreamingDecryptingKey::ctr(k, DecryptionContext::None))
                .is_err(),
            "3TDEA + CTR (streaming decrypt) should be rejected"
        );
        assert!(
            UnboundCipherKey::new(&DES_EDE3_FOR_LEGACY_USE_ONLY, &key3)
                .and_then(|k| StreamingDecryptingKey::cfb128(k, DecryptionContext::None))
                .is_err(),
            "3TDEA + CFB128 (streaming decrypt) should be rejected"
        );
    }

    #[cfg(feature = "legacy-des")]
    #[test]
    #[allow(deprecated)]
    fn test_des_cbc() {
        let key = from_hex("0123456789abcdef").unwrap();
        for i in 0..=3 {
            let size = i * 8;
            helper_test_cipher_n_bytes(
                key.as_slice(),
                &DES_FOR_LEGACY_USE_ONLY,
                OperatingMode::CBC,
                size,
            );
        }
    }

    #[cfg(feature = "legacy-des")]
    #[test]
    #[allow(deprecated)]
    fn test_des_ecb() {
        let key = from_hex("0123456789abcdef").unwrap();
        for i in 0..=3 {
            let size = i * 8;
            helper_test_cipher_n_bytes(
                key.as_slice(),
                &DES_FOR_LEGACY_USE_ONLY,
                OperatingMode::ECB,
                size,
            );
        }
    }

    #[cfg(feature = "legacy-des")]
    #[test]
    #[allow(deprecated)]
    fn test_des_ede_cbc() {
        let key = from_hex("0123456789abcdef23456789abcdef01").unwrap();
        for i in 0..=3 {
            let size = i * 8;
            helper_test_cipher_n_bytes(
                key.as_slice(),
                &DES_EDE_FOR_LEGACY_USE_ONLY,
                OperatingMode::CBC,
                size,
            );
        }
    }

    #[cfg(feature = "legacy-des")]
    #[test]
    #[allow(deprecated)]
    fn test_des_ede_ecb() {
        let key = from_hex("0123456789abcdef23456789abcdef01").unwrap();
        for i in 0..=3 {
            let size = i * 8;
            helper_test_cipher_n_bytes(
                key.as_slice(),
                &DES_EDE_FOR_LEGACY_USE_ONLY,
                OperatingMode::ECB,
                size,
            );
        }
    }

    #[cfg(feature = "legacy-des")]
    #[test]
    #[allow(deprecated)]
    fn test_des_ede3_cbc() {
        let key = from_hex("0123456789abcdef23456789abcdef01456789abcdef0123").unwrap();
        for i in 0..=3 {
            let size = i * 8;
            helper_test_cipher_n_bytes(
                key.as_slice(),
                &DES_EDE3_FOR_LEGACY_USE_ONLY,
                OperatingMode::CBC,
                size,
            );
        }
    }

    #[cfg(feature = "legacy-des")]
    #[test]
    #[allow(deprecated)]
    fn test_des_ede3_ecb() {
        let key = from_hex("0123456789abcdef23456789abcdef01456789abcdef0123").unwrap();
        for i in 0..=3 {
            let size = i * 8;
            helper_test_cipher_n_bytes(
                key.as_slice(),
                &DES_EDE3_FOR_LEGACY_USE_ONLY,
                OperatingMode::ECB,
                size,
            );
        }
    }

    #[cfg(feature = "legacy-des")]
    macro_rules! des_cipher_kat {
        ($name:ident, $alg:expr, $mode:expr, $key:literal, $iv:literal, $plaintext:literal, $ciphertext:literal) => {
            #[test]
            #[allow(deprecated)]
            fn $name() {
                let key = from_hex($key).unwrap();
                let input = from_hex($plaintext).unwrap();
                let expected_ciphertext = from_hex($ciphertext).unwrap();

                let ec = if $iv.len() == 0 {
                    EncryptionContext::None
                } else {
                    let iv_arr: [u8; 8] = from_hex($iv).unwrap().try_into().unwrap();
                    EncryptionContext::Iv64(FixedLength::from(iv_arr))
                };

                let unbound_key = UnboundCipherKey::new($alg, &key).unwrap();
                let encrypting_key = EncryptingKey::new(unbound_key, $mode).unwrap();
                let mut in_out = input.clone();
                let context = encrypting_key.less_safe_encrypt(&mut in_out, ec).unwrap();
                assert_eq!(expected_ciphertext, in_out);

                let unbound_key2 = UnboundCipherKey::new($alg, &key).unwrap();
                let decrypting_key = DecryptingKey::new(unbound_key2, $mode).unwrap();
                let plaintext = decrypting_key.decrypt(&mut in_out, context).unwrap();
                assert_eq!(input.as_slice(), plaintext);
            }
        };
    }

    #[cfg(feature = "legacy-des")]
    des_cipher_kat!(
        test_des_cbc_kat,
        &DES_FOR_LEGACY_USE_ONLY,
        OperatingMode::CBC,
        "0123456789abcdef",
        "f69f2445df4f9b17",
        "6bc1bee22e409f96e93d7e117393172aae2d8a571e03ac9c9eb76fac45af8e51",
        "2d121f90fcf68631ed788fcfe31a62dffa784891fa512ef928ea856d934257c6"
    );

    #[cfg(feature = "legacy-des")]
    des_cipher_kat!(
        test_des_ecb_kat,
        &DES_FOR_LEGACY_USE_ONLY,
        OperatingMode::ECB,
        "0123456789abcdef",
        "",
        "6bc1bee22e409f96e93d7e117393172aae2d8a571e03ac9c9eb76fac45af8e51",
        "7277a00dc1c1c36b2256338b7a9f36ef6e511022cfebec489a4a53a1c7d22e5b"
    );

    #[cfg(feature = "legacy-des")]
    des_cipher_kat!(
        test_sp800_67_des_ede_cbc,
        &DES_EDE_FOR_LEGACY_USE_ONLY,
        OperatingMode::CBC,
        "0123456789abcdef23456789abcdef01",
        "f69f2445df4f9b17",
        "6bc1bee22e409f96e93d7e117393172aae2d8a571e03ac9c9eb76fac45af8e51",
        "7401CE1EAB6D003CAFF84BF47B36CC2154F0238F9FFECD8F6ACF118392B45581"
    );

    #[cfg(feature = "legacy-des")]
    des_cipher_kat!(
        test_sp800_67_des_ede_ecb,
        &DES_EDE_FOR_LEGACY_USE_ONLY,
        OperatingMode::ECB,
        "0123456789abcdef23456789abcdef01",
        "",
        "6bc1bee22e409f96e93d7e117393172aae2d8a571e03ac9c9eb76fac45af8e51",
        "06EDE3D82884090AFF322C19F0518486730576972A666E58B6C88CF107340D3D"
    );

    #[cfg(feature = "legacy-des")]
    des_cipher_kat!(
        test_sp800_67_des_ede3_cbc,
        &DES_EDE3_FOR_LEGACY_USE_ONLY,
        OperatingMode::CBC,
        "0123456789abcdef23456789abcdef01456789abcdef0123",
        "f69f2445df4f9b17",
        "6bc1bee22e409f96e93d7e117393172aae2d8a571e03ac9c9eb76fac45af8e51",
        "2079c3d53aa763e193b79e2569ab5262516570481f25b50f73c0bda85c8e0da7"
    );

    #[cfg(feature = "legacy-des")]
    des_cipher_kat!(
        test_sp800_67_des_ede3_ecb,
        &DES_EDE3_FOR_LEGACY_USE_ONLY,
        OperatingMode::ECB,
        "0123456789abcdef23456789abcdef01456789abcdef0123",
        "",
        "6bc1bee22e409f96e93d7e117393172aae2d8a571e03ac9c9eb76fac45af8e51",
        "714772f339841d34267fcc4bd2949cc3ee11c22a576a303876183f99c0b6de87"
    );
}
