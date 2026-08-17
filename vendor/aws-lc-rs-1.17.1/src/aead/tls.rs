// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use super::aead_ctx::{self, AeadCtx};
use super::{Aad, Algorithm, AlgorithmID, Nonce, Tag, UnboundKey};
use crate::error::Unspecified;
use core::fmt::Debug;
use core::ops::RangeFrom;

/// The Transport Layer Security (TLS) protocol version.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
pub enum TlsProtocolId {
    /// TLS 1.2 (RFC 5246)
    TLS12,

    /// TLS 1.3 (RFC 8446)
    TLS13,
}

/// AEAD Encryption key used for TLS protocol record encryption.
///
/// This type encapsulates encryption operations for TLS AEAD algorithms.
/// It validates that the provides nonce values are monotonically increasing for each invocation.
///
/// The following algorithms are supported:
/// * `AES_128_GCM`
/// * `AES_256_GCM`
///
/// Prefer this type in place of `LessSafeKey`, `OpeningKey`, `SealingKey` for TLS protocol implementations.
#[allow(clippy::module_name_repetitions)]
pub struct TlsRecordSealingKey {
    // The TLS-specific AEAD seal constructions in AWS-LC maintain internal mutable state
    // (nonce counter). The seal methods take `&mut self` to prevent concurrent access.
    key: UnboundKey,
    protocol: TlsProtocolId,
}

impl TlsRecordSealingKey {
    /// New TLS record sealing key. Only supports `AES_128_GCM` and `AES_256_GCM`.
    ///
    /// # Errors
    /// * `Unspecified`: Returned if the length of `key_bytes` does not match the chosen algorithm,
    ///   or if an unsupported algorithm is provided.
    pub fn new(
        algorithm: &'static Algorithm,
        protocol: TlsProtocolId,
        key_bytes: &[u8],
    ) -> Result<Self, Unspecified> {
        let ctx = match (algorithm.id, protocol) {
            (AlgorithmID::AES_128_GCM, TlsProtocolId::TLS12) => AeadCtx::aes_128_gcm_tls12(
                key_bytes,
                algorithm.tag_len(),
                aead_ctx::AeadDirection::Seal,
            ),
            (AlgorithmID::AES_128_GCM, TlsProtocolId::TLS13) => AeadCtx::aes_128_gcm_tls13(
                key_bytes,
                algorithm.tag_len(),
                aead_ctx::AeadDirection::Seal,
            ),
            (AlgorithmID::AES_256_GCM, TlsProtocolId::TLS12) => AeadCtx::aes_256_gcm_tls12(
                key_bytes,
                algorithm.tag_len(),
                aead_ctx::AeadDirection::Seal,
            ),
            (AlgorithmID::AES_256_GCM, TlsProtocolId::TLS13) => AeadCtx::aes_256_gcm_tls13(
                key_bytes,
                algorithm.tag_len(),
                aead_ctx::AeadDirection::Seal,
            ),
            (
                AlgorithmID::AES_128_GCM_SIV
                | AlgorithmID::AES_192_GCM
                | AlgorithmID::AES_256_GCM_SIV
                | AlgorithmID::CHACHA20_POLY1305,
                _,
            ) => Err(Unspecified),
        }?;
        Ok(Self {
            key: UnboundKey::from(ctx),
            protocol,
        })
    }

    /// Accepts a `Nonce` and `Aad` construction that is unique for this key and
    /// TLS record sealing operation for the configured TLS protocol version.
    ///
    /// `nonce` must be unique and incremented per each sealing operation,
    /// otherwise an error is returned.
    ///
    /// # Errors
    /// `error::Unspecified` if encryption operation fails.
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn seal_in_place_append_tag<A, InOut>(
        &mut self,
        nonce: Nonce,
        aad: Aad<A>,
        in_out: &mut InOut,
    ) -> Result<(), Unspecified>
    where
        A: AsRef<[u8]>,
        InOut: AsMut<[u8]> + for<'in_out> Extend<&'in_out u8>,
    {
        self.key
            .seal_in_place_append_tag(Some(nonce), aad.as_ref(), in_out)
            .map(|_| ())
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
        &mut self,
        nonce: Nonce,
        aad: Aad<A>,
        in_out: &mut [u8],
    ) -> Result<Tag, Unspecified>
    where
        A: AsRef<[u8]>,
    {
        self.key
            .seal_in_place_separate_tag(Some(nonce), aad.as_ref(), in_out)
            .map(|(_, tag)| tag)
    }

    /// The key's AEAD algorithm.
    #[inline]
    #[must_use]
    pub fn algorithm(&self) -> &'static Algorithm {
        self.key.algorithm()
    }

    /// The key's associated `TlsProtocolId`.
    #[must_use]
    pub fn tls_protocol_id(&self) -> TlsProtocolId {
        self.protocol
    }
}

#[allow(clippy::missing_fields_in_debug)]
impl Debug for TlsRecordSealingKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TlsRecordSealingKey")
            .field("key", &self.key)
            .field("protocol", &self.protocol)
            .finish()
    }
}

/// AEAD Encryption key used for TLS protocol record encryption.
///
/// This type encapsulates decryption operations for TLS AEAD algorithms.
///
/// The following algorithms are supported:
/// * `AES_128_GCM`
/// * `AES_256_GCM`
///
/// Prefer this type in place of `LessSafeKey`, `OpeningKey`, `SealingKey` for TLS protocol implementations.
#[allow(clippy::module_name_repetitions)]
pub struct TlsRecordOpeningKey {
    // Unlike the seal path, the TLS-specific AEAD open operations in AWS-LC are stateless
    // and safe for concurrent use. The open methods take `&self`.
    key: UnboundKey,
    protocol: TlsProtocolId,
}

impl TlsRecordOpeningKey {
    /// New TLS record opening key. Only supports `AES_128_GCM` and `AES_256_GCM` Algorithms.
    ///
    /// # Errors
    /// * `Unspecified`: Returned if the length of `key_bytes` does not match the chosen algorithm,
    ///   or if an unsupported algorithm is provided.
    pub fn new(
        algorithm: &'static Algorithm,
        protocol: TlsProtocolId,
        key_bytes: &[u8],
    ) -> Result<Self, Unspecified> {
        let ctx = match (algorithm.id, protocol) {
            (AlgorithmID::AES_128_GCM, TlsProtocolId::TLS12) => AeadCtx::aes_128_gcm_tls12(
                key_bytes,
                algorithm.tag_len(),
                aead_ctx::AeadDirection::Open,
            ),
            (AlgorithmID::AES_128_GCM, TlsProtocolId::TLS13) => AeadCtx::aes_128_gcm_tls13(
                key_bytes,
                algorithm.tag_len(),
                aead_ctx::AeadDirection::Open,
            ),
            (AlgorithmID::AES_256_GCM, TlsProtocolId::TLS12) => AeadCtx::aes_256_gcm_tls12(
                key_bytes,
                algorithm.tag_len(),
                aead_ctx::AeadDirection::Open,
            ),
            (AlgorithmID::AES_256_GCM, TlsProtocolId::TLS13) => AeadCtx::aes_256_gcm_tls13(
                key_bytes,
                algorithm.tag_len(),
                aead_ctx::AeadDirection::Open,
            ),
            (
                AlgorithmID::AES_128_GCM_SIV
                | AlgorithmID::AES_192_GCM
                | AlgorithmID::AES_256_GCM_SIV
                | AlgorithmID::CHACHA20_POLY1305,
                _,
            ) => Err(Unspecified),
        }?;
        Ok(Self {
            key: UnboundKey::from(ctx),
            protocol,
        })
    }

    /// See [`super::OpeningKey::open_in_place()`] for details.
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

    /// See [`super::OpeningKey::open_within()`] for details.
    ///
    /// # Errors
    /// `error::Unspecified` when ciphertext is invalid.
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn open_within<'in_out, A>(
        &self,
        nonce: Nonce,
        aad: Aad<A>,
        in_out: &'in_out mut [u8],
        ciphertext_and_tag: RangeFrom<usize>,
    ) -> Result<&'in_out mut [u8], Unspecified>
    where
        A: AsRef<[u8]>,
    {
        self.key
            .open_within(nonce, aad.as_ref(), in_out, ciphertext_and_tag)
    }

    /// The key's AEAD algorithm.
    #[inline]
    #[must_use]
    pub fn algorithm(&self) -> &'static Algorithm {
        self.key.algorithm()
    }

    /// The key's associated `TlsProtocolId`.
    #[must_use]
    pub fn tls_protocol_id(&self) -> TlsProtocolId {
        self.protocol
    }
}

#[allow(clippy::missing_fields_in_debug)]
impl Debug for TlsRecordOpeningKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TlsRecordOpeningKey")
            .field("key", &self.key)
            .field("protocol", &self.protocol)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::{TlsProtocolId, TlsRecordOpeningKey, TlsRecordSealingKey};
    use crate::aead::{Aad, Nonce, AES_128_GCM, AES_256_GCM, CHACHA20_POLY1305};
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

    struct TlsNonceTestCase {
        nonce: &'static str,
        expect_err: bool,
    }

    const TLS_NONCE_TEST_CASES: &[TlsNonceTestCase] = &[
        TlsNonceTestCase {
            nonce: "9fab40177c900aad9fc28cc3",
            expect_err: false,
        },
        TlsNonceTestCase {
            nonce: "9fab40177c900aad9fc28cc4",
            expect_err: false,
        },
        TlsNonceTestCase {
            nonce: "9fab40177c900aad9fc28cc2",
            expect_err: true,
        },
    ];

    macro_rules! test_tls_aead {
        ($name:ident, $alg:expr, $proto:expr, $key:expr) => {
            paste! {
                #[test]
                fn [<test_ $name _tls_aead_unsupported>]() {
                    assert!(TlsRecordSealingKey::new($alg, $proto, $key).is_err());
                    assert!(TlsRecordOpeningKey::new($alg, $proto, $key).is_err());
                }
            }
        };
        ($name:ident, $alg:expr, $proto:expr, $key:expr, $expect_tag_len:expr, $expect_nonce_len:expr) => {
            paste! {
                #[test]
                fn [<test_ $name>]() {
                    let mut sealing_key =
                        TlsRecordSealingKey::new($alg, $proto, $key).unwrap();

                    let opening_key =
                        TlsRecordOpeningKey::new($alg, $proto, $key).unwrap();

                    for case in TLS_NONCE_TEST_CASES {
                        let plaintext = from_hex("00112233445566778899aabbccddeeff").unwrap();

                        assert_eq!($alg, sealing_key.algorithm());
                        assert_eq!(*$expect_tag_len, $alg.tag_len());
                        assert_eq!(*$expect_nonce_len, $alg.nonce_len());

                        let mut in_out = Vec::from(plaintext.as_slice());

                        let nonce = from_hex(case.nonce).unwrap();

                        let nonce_bytes = nonce.as_slice();

                        let result = sealing_key.seal_in_place_append_tag(
                            Nonce::try_assume_unique_for_key(nonce_bytes).unwrap(),
                            Aad::empty(),
                            &mut in_out,
                        );

                        match (result, case.expect_err) {
                            (Ok(()), true) => panic!("expected error for seal_in_place_append_tag"),
                            (Ok(()), false) => {}
                            (Err(_), true) => return,
                            (Err(e), false) => panic!("{e}"),
                        }

                        assert_ne!(plaintext, in_out[..plaintext.len()]);

                        // copy ciphertext with prefix, to exercise `open_within`
                        let mut offset_cipher_text = vec![ 1, 2, 3, 4 ];
                        offset_cipher_text.extend_from_slice(&in_out);

                        opening_key
                            .open_in_place(
                                Nonce::try_assume_unique_for_key(nonce_bytes).unwrap(),
                                Aad::empty(),
                                &mut in_out,
                            )
                            .unwrap();

                        assert_eq!(plaintext, in_out[..plaintext.len()]);

                        opening_key
                            .open_within(
                                         Nonce::try_assume_unique_for_key(nonce_bytes).unwrap(),
                                         Aad::empty(),
                                         &mut offset_cipher_text,
                                         4..)
                            .unwrap();
                        assert_eq!(plaintext, offset_cipher_text[..plaintext.len()]);
                    }
                }
            }
        };
    }

    test_tls_aead!(
        aes_128_gcm_tls12,
        &AES_128_GCM,
        TlsProtocolId::TLS12,
        TEST_128_BIT_KEY,
        &16,
        &12
    );
    test_tls_aead!(
        aes_128_gcm_tls13,
        &AES_128_GCM,
        TlsProtocolId::TLS13,
        TEST_128_BIT_KEY,
        &16,
        &12
    );
    test_tls_aead!(
        aes_256_gcm_tls12,
        &AES_256_GCM,
        TlsProtocolId::TLS12,
        TEST_256_BIT_KEY,
        &16,
        &12
    );
    test_tls_aead!(
        aes_256_gcm_tls13,
        &AES_256_GCM,
        TlsProtocolId::TLS13,
        TEST_256_BIT_KEY,
        &16,
        &12
    );
    test_tls_aead!(
        chacha20_poly1305_tls12,
        &CHACHA20_POLY1305,
        TlsProtocolId::TLS12,
        TEST_256_BIT_KEY
    );
    test_tls_aead!(
        chacha20_poly1305_tls13,
        &CHACHA20_POLY1305,
        TlsProtocolId::TLS13,
        TEST_256_BIT_KEY
    );
}
