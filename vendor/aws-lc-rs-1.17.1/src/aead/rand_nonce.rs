// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use crate::error::Unspecified;
use core::fmt::Debug;

use super::aead_ctx::AeadCtx;
use super::{
    Aad, Algorithm, AlgorithmID, Nonce, Tag, UnboundKey, AES_128_GCM_SIV, AES_256_GCM_SIV,
};

/// AEAD Cipher key using a randomized nonce.
///
/// `RandomizedNonceKey` handles generation random nonce values.
///
/// The following algorithms are supported:
/// * `AES_128_GCM`
/// * `AES_256_GCM`
/// * `AES_128_GCM_SIV`
/// * `AES_256_GCM_SIV`
///
/// Prefer this type in place of `LessSafeKey`, `OpeningKey`, `SealingKey`.
pub struct RandomizedNonceKey {
    key: UnboundKey,
    algorithm: &'static Algorithm,
}

impl RandomizedNonceKey {
    /// New Random Nonce Sequence
    /// # Errors
    pub fn new(algorithm: &'static Algorithm, key_bytes: &[u8]) -> Result<Self, Unspecified> {
        let ctx = match algorithm.id {
            AlgorithmID::AES_128_GCM => AeadCtx::aes_128_gcm_randnonce(
                key_bytes,
                algorithm.tag_len(),
                algorithm.nonce_len(),
            ),
            AlgorithmID::AES_256_GCM => AeadCtx::aes_256_gcm_randnonce(
                key_bytes,
                algorithm.tag_len(),
                algorithm.nonce_len(),
            ),
            AlgorithmID::AES_128_GCM_SIV => {
                AeadCtx::aes_128_gcm_siv(key_bytes, algorithm.tag_len())
            }
            AlgorithmID::AES_256_GCM_SIV => {
                AeadCtx::aes_256_gcm_siv(key_bytes, algorithm.tag_len())
            }
            AlgorithmID::AES_192_GCM | AlgorithmID::CHACHA20_POLY1305 => return Err(Unspecified),
        }?;
        Ok(Self {
            key: UnboundKey::from(ctx),
            algorithm,
        })
    }

    /// Authenticates and decrypts (“opens”) data in place.
    //
    // aad is the additional authenticated data (AAD), if any.
    //
    // On input, in_out must be the ciphertext followed by the tag. When open_in_place() returns Ok(plaintext),
    // the input ciphertext has been overwritten by the plaintext; plaintext will refer to the plaintext without the tag.
    ///
    /// # Errors
    /// `error::Unspecified` when ciphertext is invalid.
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn open_in_place<'in_out, A>(
        &self,
        nonce: Nonce,
        aad: Aad<A>,
        in_out: &'in_out mut [u8],
    ) -> Result<&'in_out mut [u8], Unspecified>
    where
        A: AsRef<[u8]>,
    {
        self.key.open_within(nonce, aad.as_ref(), in_out, 0..)
    }

    /// Encrypts and signs (“seals”) data in place, appending the tag to the
    /// resulting ciphertext.
    ///
    /// `key.seal_in_place_append_tag(aad, in_out)` is equivalent to:
    ///
    /// ```skip
    /// key.seal_in_place_separate_tag(aad, in_out.as_mut())
    ///     .map(|tag| in_out.extend(tag.as_ref()))
    /// ```
    ///
    /// The Nonce used for the operation is randomly generated, and returned to the caller.
    ///
    /// # Errors
    /// `error::Unspecified` if encryption operation fails.
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn seal_in_place_append_tag<'a, A, InOut>(
        &self,
        aad: Aad<A>,
        in_out: &'a mut InOut,
    ) -> Result<Nonce, Unspecified>
    where
        A: AsRef<[u8]>,
        InOut: AsMut<[u8]> + for<'in_out> Extend<&'in_out u8>,
    {
        let nonce = if self.algorithm == &AES_128_GCM_SIV || self.algorithm == &AES_256_GCM_SIV {
            let mut nonce = vec![0u8; self.algorithm.nonce_len()];
            crate::rand::fill(&mut nonce[..])?;
            Some(Nonce::try_assume_unique_for_key(nonce.as_slice())?)
        } else {
            None
        };
        self.key
            .seal_in_place_append_tag(nonce, aad.as_ref(), in_out)
    }

    /// Encrypts and signs (“seals”) data in place.
    ///
    /// `aad` is the additional authenticated data (AAD), if any. This is
    /// authenticated but not encrypted. The type `A` could be a byte slice
    /// `&[u8]`, a byte array `[u8; N]` for some constant `N`, `Vec<u8>`, etc.
    /// If there is no AAD then use `Aad::empty()`.
    ///
    /// The plaintext is given as the input value of `in_out`. `seal_in_place()`
    /// will overwrite the plaintext with the ciphertext and return the tag.
    /// For most protocols, the caller must append the tag to the ciphertext.
    /// The tag will be `self.algorithm.tag_len()` bytes long.
    ///
    /// The Nonce used for the operation is randomly generated, and returned to the caller.
    ///
    /// # Errors
    /// `error::Unspecified` if encryption operation fails.
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn seal_in_place_separate_tag<A>(
        &self,
        aad: Aad<A>,
        in_out: &mut [u8],
    ) -> Result<(Nonce, Tag), Unspecified>
    where
        A: AsRef<[u8]>,
    {
        let nonce = if self.algorithm == &AES_128_GCM_SIV || self.algorithm == &AES_256_GCM_SIV {
            let mut nonce = vec![0u8; self.algorithm.nonce_len()];
            crate::rand::fill(&mut nonce[..])?;
            Some(Nonce::try_assume_unique_for_key(nonce.as_slice())?)
        } else {
            None
        };
        self.key
            .seal_in_place_separate_tag(nonce, aad.as_ref(), in_out)
    }

    /// The key's AEAD algorithm.
    #[inline]
    #[must_use]
    pub fn algorithm(&self) -> &'static Algorithm {
        self.algorithm
    }
}

#[allow(clippy::missing_fields_in_debug)]
impl Debug for RandomizedNonceKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RandomizedNonceKey")
            .field("algorithm", &self.algorithm)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::{Aad, RandomizedNonceKey};
    use crate::aead::{
        AES_128_GCM, AES_128_GCM_SIV, AES_256_GCM, AES_256_GCM_SIV, CHACHA20_POLY1305,
    };
    use crate::test::from_hex;
    use paste::paste;

    const TEST_128_BIT_KEY: &[u8] = &[
        0xb0, 0x37, 0x9f, 0xf8, 0xfb, 0x8e, 0xa6, 0x31, 0xf4, 0x1c, 0xe6, 0x3e, 0xb5, 0xc5, 0x20,
        0x7c,
    ];

    const TEST_256_BIT_KEY: &[u8] = &[
        0x56, 0xd8, 0x96, 0x68, 0xbd, 0x96, 0xeb, 0xff, 0x5e, 0xa2, 0x0b, 0x34, 0xf2, 0x79, 0x84,
        0x6e, 0x2b, 0x13, 0x01, 0x3d, 0xab, 0x1d, 0xa4, 0x07, 0x5a, 0x16, 0xd5, 0x0b, 0x53, 0xb0,
        0xcc, 0x88,
    ];

    macro_rules! test_randnonce {
        ($name:ident, $alg:expr, $key:expr) => {
            paste! {
                #[test]
                fn [<test_ $name _randnonce_unsupported>]() {
                    assert!(RandomizedNonceKey::new($alg, $key).is_err());
                }
            }
        };
        ($name:ident, $alg:expr, $key:expr, $expect_tag_len:expr, $expect_nonce_len:expr) => {
            paste! {
                #[test]
                fn [<test_ $name _randnonce>]() {
                    let plaintext = from_hex("00112233445566778899aabbccddeeff").unwrap();
                    let rand_nonce_key =
                        RandomizedNonceKey::new($alg, $key).unwrap();

                    assert_eq!($alg, rand_nonce_key.algorithm());
                    assert_eq!(*$expect_tag_len, $alg.tag_len());
                    assert_eq!(*$expect_nonce_len, $alg.nonce_len());

                    let mut in_out = Vec::from(plaintext.as_slice());

                    let nonce = rand_nonce_key
                        .seal_in_place_append_tag(Aad::empty(), &mut in_out)
                        .unwrap();

                    assert_ne!(plaintext, in_out[..plaintext.len()]);

                    rand_nonce_key
                        .open_in_place(nonce, Aad::empty(), &mut in_out)
                        .unwrap();

                    assert_eq!(plaintext, in_out[..plaintext.len()]);

                    let mut in_out = Vec::from(plaintext.as_slice());

                    let (nonce, tag) = rand_nonce_key
                        .seal_in_place_separate_tag(Aad::empty(), &mut in_out)
                        .unwrap();

                    assert_ne!(plaintext, in_out[..plaintext.len()]);

                    in_out.extend(tag.as_ref());

                    rand_nonce_key
                        .open_in_place(nonce, Aad::empty(), &mut in_out)
                        .unwrap();

                    assert_eq!(plaintext, in_out[..plaintext.len()]);
                }
            }
        };
    }

    test_randnonce!(aes_128_gcm, &AES_128_GCM, TEST_128_BIT_KEY, &16, &12);
    test_randnonce!(aes_256_gcm, &AES_256_GCM, TEST_256_BIT_KEY, &16, &12);
    test_randnonce!(
        aes_128_gcm_siv,
        &AES_128_GCM_SIV,
        TEST_128_BIT_KEY,
        &16,
        &12
    );
    test_randnonce!(
        aes_256_gcm_siv,
        &AES_256_GCM_SIV,
        TEST_256_BIT_KEY,
        &16,
        &12
    );

    test_randnonce!(chacha20_poly1305, &CHACHA20_POLY1305, TEST_256_BIT_KEY);
}
