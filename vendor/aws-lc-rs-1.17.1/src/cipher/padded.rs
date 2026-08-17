// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC
use crate::cipher;
use crate::cipher::key::SymmetricCipherKey;
use crate::cipher::{
    Algorithm, DecryptionContext, EncryptionContext, OperatingMode, UnboundCipherKey,
    MAX_CIPHER_BLOCK_LEN,
};
use crate::error::Unspecified;
use core::fmt::Debug;

/// The cipher block padding strategy.
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum PaddingStrategy {
    /// ISO 10126 padding. For compatibility purposes only. Applies non-random PKCS7 padding.
    ISO10126,
    /// PKCS#7 Padding. ([See RFC 5652](https://datatracker.ietf.org/doc/html/rfc5652#section-6.3))
    PKCS7,
}

impl PaddingStrategy {
    fn add_padding<InOut>(self, block_len: usize, in_out: &mut InOut) -> Result<(), Unspecified>
    where
        InOut: AsMut<[u8]> + for<'in_out> Extend<&'in_out u8>,
    {
        match self {
            // PKCS7 padding can be unpadded as ISO 10126 padding
            PaddingStrategy::ISO10126 | PaddingStrategy::PKCS7 => {
                let mut padding_buffer = [0u8; MAX_CIPHER_BLOCK_LEN];

                let in_out_len = in_out.as_mut().len();
                // This implements PKCS#7 padding scheme, used by aws-lc if we were using EVP_CIPHER API's
                let remainder = in_out_len % block_len;
                let padding_size = block_len - remainder;
                let v: u8 = padding_size.try_into().map_err(|_| Unspecified)?;
                padding_buffer.fill(v);
                // Possible heap allocation here :(
                in_out.extend(padding_buffer[0..padding_size].iter());
            }
        }
        Ok(())
    }

    fn remove_padding(self, block_len: usize, in_out: &mut [u8]) -> Result<&mut [u8], Unspecified> {
        if in_out.is_empty() || in_out.len() < block_len {
            return Err(Unspecified);
        }
        match self {
            PaddingStrategy::ISO10126 => {
                let padding: u8 = in_out[in_out.len() - 1];
                if padding == 0 || padding as usize > block_len {
                    return Err(Unspecified);
                }

                // ISO 10126 padding is a random padding scheme, so we cannot verify the padding bytes
                let final_len = in_out.len() - padding as usize;
                Ok(&mut in_out[0..final_len])
            }
            PaddingStrategy::PKCS7 => {
                let block_size: u8 = block_len.try_into().map_err(|_| Unspecified)?;

                let padding: u8 = in_out[in_out.len() - 1];
                if padding == 0 || padding > block_size {
                    return Err(Unspecified);
                }

                for item in in_out.iter().skip(in_out.len() - padding as usize) {
                    if *item != padding {
                        return Err(Unspecified);
                    }
                }

                let final_len = in_out.len() - padding as usize;
                Ok(&mut in_out[0..final_len])
            }
        }
    }
}

/// A cipher encryption key that performs block padding.
pub struct PaddedBlockEncryptingKey {
    algorithm: &'static Algorithm,
    key: SymmetricCipherKey,
    mode: OperatingMode,
    padding: PaddingStrategy,
}

impl PaddedBlockEncryptingKey {
    /// Constructs a new `PaddedBlockEncryptingKey` cipher with chaining block cipher (CBC) mode.
    /// Plaintext data is padded following the PKCS#7 scheme.
    ///
    // # FIPS
    // Use this function with an `UnboundCipherKey` constructed with one of the following algorithms:
    // * `AES_128`
    // * `AES_256`
    //
    /// # Errors
    /// * [`Unspecified`]: Returned if there is an error constructing a `PaddedBlockEncryptingKey`.
    ///   With `legacy-des` enabled, also returned if `key` was constructed with
    ///   `DES_FOR_LEGACY_USE_ONLY`, `DES_EDE_FOR_LEGACY_USE_ONLY` or
    ///   `DES_EDE3_FOR_LEGACY_USE_ONLY` and the provided key material contains
    ///   weak or semi-weak DES subkeys, or (for Triple DES) a degenerate subkey
    ///   configuration (e.g. `K1 == K2` for 2TDEA, or any pairwise equality
    ///   for 3TDEA).
    pub fn cbc_pkcs7(key: UnboundCipherKey) -> Result<Self, Unspecified> {
        Self::new(key, OperatingMode::CBC, PaddingStrategy::PKCS7)
    }

    /// Constructs a new `PaddedBlockEncryptingKey` cipher with electronic code book (ECB) mode.
    /// Plaintext data is padded following the PKCS#7 scheme.
    ///
    /// # ☠️ ️️️DANGER ☠️
    /// Offered for computability purposes only. This is an extremely dangerous mode, and
    /// very likely not what you want to use.
    ///
    /// # Errors
    /// * [`Unspecified`]: Returned if there is an error constructing a `PaddedBlockEncryptingKey`.
    ///   With `legacy-des` enabled, also returned if `key` was constructed with
    ///   `DES_FOR_LEGACY_USE_ONLY`, `DES_EDE_FOR_LEGACY_USE_ONLY` or
    ///   `DES_EDE3_FOR_LEGACY_USE_ONLY` and the provided key material contains
    ///   weak or semi-weak DES subkeys, or (for Triple DES) a degenerate subkey
    ///   configuration (e.g. `K1 == K2` for 2TDEA, or any pairwise equality
    ///   for 3TDEA).
    pub fn ecb_pkcs7(key: UnboundCipherKey) -> Result<Self, Unspecified> {
        Self::new(key, OperatingMode::ECB, PaddingStrategy::PKCS7)
    }

    fn new(
        key: UnboundCipherKey,
        mode: OperatingMode,
        padding: PaddingStrategy,
    ) -> Result<PaddedBlockEncryptingKey, Unspecified> {
        let algorithm = key.algorithm();
        if !algorithm.supports_mode(mode) {
            return Err(Unspecified);
        }
        let key = key.try_into()?;
        Ok(Self {
            algorithm,
            key,
            mode,
            padding,
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

    /// Pads and encrypts data provided in `in_out` in-place.
    /// Returns a references to the encrypted data.
    ///
    /// # Errors
    /// * [`Unspecified`]: Returned if encryption fails.
    pub fn encrypt<InOut>(&self, in_out: &mut InOut) -> Result<DecryptionContext, Unspecified>
    where
        InOut: AsMut<[u8]> + for<'a> Extend<&'a u8>,
    {
        let context = self.algorithm.new_encryption_context(self.mode)?;
        self.less_safe_encrypt(in_out, context)
    }

    /// Pads and encrypts data provided in `in_out` in-place.
    /// Returns a references to the encryted data.
    ///
    /// # Errors
    /// * [`Unspecified`]: Returned if encryption fails.
    pub fn less_safe_encrypt<InOut>(
        &self,
        in_out: &mut InOut,
        context: EncryptionContext,
    ) -> Result<DecryptionContext, Unspecified>
    where
        InOut: AsMut<[u8]> + for<'a> Extend<&'a u8>,
    {
        if !self
            .algorithm()
            .is_valid_encryption_context(self.mode, &context)
        {
            return Err(Unspecified);
        }

        self.padding
            .add_padding(self.algorithm().block_len(), in_out)?;
        cipher::encrypt(
            self.algorithm(),
            &self.key,
            self.mode,
            in_out.as_mut(),
            context,
        )
    }
}

impl Debug for PaddedBlockEncryptingKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PaddedBlockEncryptingKey")
            .field("algorithm", &self.algorithm)
            .field("mode", &self.mode)
            .field("padding", &self.padding)
            .finish_non_exhaustive()
    }
}

/// A cipher decryption key that performs block padding.
pub struct PaddedBlockDecryptingKey {
    algorithm: &'static Algorithm,
    key: SymmetricCipherKey,
    mode: OperatingMode,
    padding: PaddingStrategy,
}

impl PaddedBlockDecryptingKey {
    /// Constructs a new `PaddedBlockDecryptingKey` cipher with chaining block cipher (CBC) mode.
    /// Decrypted data is unpadded following the PKCS#7 scheme.
    ///
    // # FIPS
    // Use this function with an `UnboundCipherKey` constructed with one of the following algorithms:
    // * `AES_128`
    // * `AES_256`
    //
    /// # Errors
    /// * [`Unspecified`]: Returned if there is an error constructing the `PaddedBlockDecryptingKey`.
    ///   With `legacy-des` enabled, also returned if `key` was constructed with
    ///   `DES_FOR_LEGACY_USE_ONLY`, `DES_EDE_FOR_LEGACY_USE_ONLY` or
    ///   `DES_EDE3_FOR_LEGACY_USE_ONLY` and the provided key material contains
    ///   weak or semi-weak DES subkeys, or (for Triple DES) a degenerate subkey
    ///   configuration (e.g. `K1 == K2` for 2TDEA, or any pairwise equality
    ///   for 3TDEA).
    pub fn cbc_pkcs7(key: UnboundCipherKey) -> Result<Self, Unspecified> {
        Self::new(key, OperatingMode::CBC, PaddingStrategy::PKCS7)
    }

    /// Constructs a new `PaddedBlockDecryptingKey` cipher with chaining block cipher (CBC) mode.
    /// Decrypted data is unpadded following the ISO 10126 scheme
    /// (compatible with PKCS#7 and ANSI X.923).
    ///
    /// Offered for computability purposes only.
    ///
    // # FIPS
    // Use this function with an `UnboundCipherKey` constructed with one of the following algorithms:
    // * `AES_128`
    // * `AES_256`
    //
    /// # Errors
    /// * [`Unspecified`]: Returned if there is an error constructing the `PaddedBlockDecryptingKey`.
    ///   With `legacy-des` enabled, also returned if `key` was constructed with
    ///   `DES_FOR_LEGACY_USE_ONLY`, `DES_EDE_FOR_LEGACY_USE_ONLY` or
    ///   `DES_EDE3_FOR_LEGACY_USE_ONLY` and the provided key material contains
    ///   weak or semi-weak DES subkeys, or (for Triple DES) a degenerate subkey
    ///   configuration (e.g. `K1 == K2` for 2TDEA, or any pairwise equality
    ///   for 3TDEA).
    pub fn cbc_iso10126(key: UnboundCipherKey) -> Result<Self, Unspecified> {
        Self::new(key, OperatingMode::CBC, PaddingStrategy::ISO10126)
    }

    /// Constructs a new `PaddedBlockDecryptingKey` cipher with electronic code book (ECB) mode.
    /// Decrypted data is unpadded following the PKCS#7 scheme.
    ///
    /// # ☠️ ️️️DANGER ☠️
    /// Offered for computability purposes only. This is an extremely dangerous mode, and
    /// very likely not what you want to use.
    ///
    // # FIPS
    // Use this function with an `UnboundCipherKey` constructed with one of the following algorithms:
    // * `AES_128`
    // * `AES_256`
    //
    /// # Errors
    /// * [`Unspecified`]: Returned if there is an error constructing the `PaddedBlockDecryptingKey`.
    ///   With `legacy-des` enabled, also returned if `key` was constructed with
    ///   `DES_FOR_LEGACY_USE_ONLY`, `DES_EDE_FOR_LEGACY_USE_ONLY` or
    ///   `DES_EDE3_FOR_LEGACY_USE_ONLY` and the provided key material contains
    ///   weak or semi-weak DES subkeys, or (for Triple DES) a degenerate subkey
    ///   configuration (e.g. `K1 == K2` for 2TDEA, or any pairwise equality
    ///   for 3TDEA).
    pub fn ecb_pkcs7(key: UnboundCipherKey) -> Result<Self, Unspecified> {
        Self::new(key, OperatingMode::ECB, PaddingStrategy::PKCS7)
    }

    fn new(
        key: UnboundCipherKey,
        mode: OperatingMode,
        padding: PaddingStrategy,
    ) -> Result<PaddedBlockDecryptingKey, Unspecified> {
        let algorithm = key.algorithm();
        if !algorithm.supports_mode(mode) {
            return Err(Unspecified);
        }
        let key = key.try_into()?;
        Ok(PaddedBlockDecryptingKey {
            algorithm,
            key,
            mode,
            padding,
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

    /// Decrypts and unpads data provided in `in_out` in-place.
    /// Returns a references to the decrypted data.
    ///
    /// # Errors
    /// * [`Unspecified`]: Returned if decryption fails.
    pub fn decrypt<'in_out>(
        &self,
        in_out: &'in_out mut [u8],
        context: DecryptionContext,
    ) -> Result<&'in_out mut [u8], Unspecified> {
        if !self
            .algorithm()
            .is_valid_decryption_context(self.mode, &context)
        {
            return Err(Unspecified);
        }

        let block_len = self.algorithm().block_len();
        let padding = self.padding;
        let mut in_out = cipher::decrypt(self.algorithm, &self.key, self.mode, in_out, context)?;
        in_out = padding.remove_padding(block_len, in_out)?;
        Ok(in_out)
    }
}

impl Debug for PaddedBlockDecryptingKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PaddedBlockDecryptingKey")
            .field("algorithm", &self.algorithm)
            .field("mode", &self.mode)
            .field("padding", &self.padding)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use crate::cipher::padded::PaddingStrategy;
    use crate::cipher::{
        Algorithm, EncryptionContext, OperatingMode, PaddedBlockDecryptingKey,
        PaddedBlockEncryptingKey, UnboundCipherKey, AES_128, AES_256,
    };
    use crate::iv::FixedLength;
    use crate::test::from_hex;

    fn helper_test_padded_cipher_n_bytes(
        key: &[u8],
        alg: &'static Algorithm,
        mode: OperatingMode,
        padding: PaddingStrategy,
        n: usize,
    ) {
        let mut input: Vec<u8> = Vec::with_capacity(n);
        for i in 0..n {
            let byte: u8 = i.try_into().unwrap();
            input.push(byte);
        }

        let cipher_key = UnboundCipherKey::new(alg, key).unwrap();
        let encrypting_key = PaddedBlockEncryptingKey::new(cipher_key, mode, padding).unwrap();

        let mut in_out = input.clone();
        let decrypt_iv = encrypting_key.encrypt(&mut in_out).unwrap();

        if n > 5 {
            // There's no more than a 1 in 2^48 chance that this will fail randomly
            assert_ne!(input.as_slice(), in_out);
        }

        let cipher_key2 = UnboundCipherKey::new(alg, key).unwrap();
        let decrypting_key = PaddedBlockDecryptingKey::new(cipher_key2, mode, padding).unwrap();

        let plaintext = decrypting_key.decrypt(&mut in_out, decrypt_iv).unwrap();
        assert_eq!(input.as_slice(), plaintext);
    }

    #[test]
    fn test_unpad_iso10126() {
        let mut input = from_hex("01020304050607fedcba9805").unwrap();
        let padding = PaddingStrategy::ISO10126;
        let block_len = 8;

        let unpadded = padding.remove_padding(block_len, &mut input).unwrap();
        assert_eq!(unpadded, &mut [1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_aes_128_cbc() {
        let key = from_hex("000102030405060708090a0b0c0d0e0f").unwrap();
        for i in 0..=50 {
            helper_test_padded_cipher_n_bytes(
                key.as_slice(),
                &AES_128,
                OperatingMode::CBC,
                PaddingStrategy::PKCS7,
                i,
            );
        }
    }

    #[test]
    fn test_aes_256_cbc() {
        let key =
            from_hex("000102030405060708090a0b0c0d0e0f000102030405060708090a0b0c0d0e0f").unwrap();
        for i in 0..=50 {
            helper_test_padded_cipher_n_bytes(
                key.as_slice(),
                &AES_256,
                OperatingMode::CBC,
                PaddingStrategy::PKCS7,
                i,
            );
        }
    }

    macro_rules! padded_cipher_kat {
        ($name:ident, $alg:expr, $mode:expr, $padding:expr, $key:literal, $iv: literal, $plaintext:literal, $ciphertext:literal) => {
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

                let encrypting_key =
                    PaddedBlockEncryptingKey::new(unbound_key, $mode, $padding).unwrap();

                let mut in_out = input.clone();

                let context = encrypting_key.less_safe_encrypt(&mut in_out, ec).unwrap();

                if ($padding == PaddingStrategy::ISO10126) {
                    // This padding scheme is technically non-deterministic in nature if the padding is more then one
                    // byte. So just validate the input length of in_out is no longer the plaintext.
                    assert_ne!(input, in_out[..input.len()]);
                } else {
                    assert_eq!(expected_ciphertext, in_out);
                }

                let unbound_key2 = UnboundCipherKey::new(alg, &key).unwrap();
                let decrypting_key =
                    PaddedBlockDecryptingKey::new(unbound_key2, $mode, $padding).unwrap();

                let plaintext = decrypting_key.decrypt(&mut in_out, context).unwrap();
                assert_eq!(input.as_slice(), plaintext);
            }
        };
    }

    padded_cipher_kat!(
        test_iv_aes_128_cbc_16_bytes,
        &AES_128,
        OperatingMode::CBC,
        PaddingStrategy::PKCS7,
        "000102030405060708090a0b0c0d0e0f",
        "00000000000000000000000000000000",
        "00112233445566778899aabbccddeeff",
        "69c4e0d86a7b0430d8cdb78070b4c55a9e978e6d16b086570ef794ef97984232"
    );

    padded_cipher_kat!(
        test_iv_aes_256_cbc_15_bytes,
        &AES_256,
        OperatingMode::CBC,
        PaddingStrategy::PKCS7,
        "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
        "00000000000000000000000000000000",
        "00112233445566778899aabbccddee",
        "2ddfb635a651a43f582997966840ca0c"
    );

    padded_cipher_kat!(
        test_openssl_aes_128_cbc_15_bytes,
        &AES_128,
        OperatingMode::CBC,
        PaddingStrategy::PKCS7,
        "053304bb3899e1d99db9d29343ea782d",
        "b5313560244a4822c46c2a0c9d0cf7fd",
        "a3e4c990356c01f320043c3d8d6f43",
        "ad96993f248bd6a29760ec7ccda95ee1"
    );

    padded_cipher_kat!(
        test_openssl_aes_128_cbc_iso10126_15_bytes,
        &AES_128,
        OperatingMode::CBC,
        PaddingStrategy::ISO10126,
        "053304bb3899e1d99db9d29343ea782d",
        "b5313560244a4822c46c2a0c9d0cf7fd",
        "a3e4c990356c01f320043c3d8d6f43",
        "ad96993f248bd6a29760ec7ccda95ee1"
    );

    padded_cipher_kat!(
        test_openssl_aes_128_cbc_iso10126_16_bytes,
        &AES_128,
        OperatingMode::CBC,
        PaddingStrategy::ISO10126,
        "053304bb3899e1d99db9d29343ea782d",
        "b83452fc9c80215a6ecdc505b5154c90",
        "736e65616b7920726163636f6f6e7321",
        "44563399c6bb2133e013161dc5bd4fa8ce83ef997ddb04bbbbe3632b68e9cde0"
    );

    padded_cipher_kat!(
        test_openssl_aes_128_cbc_16_bytes,
        &AES_128,
        OperatingMode::CBC,
        PaddingStrategy::PKCS7,
        "95af71f1c63e4a1d0b0b1a27fb978283",
        "89e40797dca70197ff87d3dbb0ef2802",
        "aece7b5e3c3df1ffc9802d2dfe296dc7",
        "301b5dab49fb11e919d0d39970d06739301919743304f23f3cbc67d28564b25b"
    );

    padded_cipher_kat!(
        test_openssl_aes_256_cbc_15_bytes,
        &AES_256,
        OperatingMode::CBC,
        PaddingStrategy::PKCS7,
        "d369e03e9752784917cc7bac1db7399598d9555e691861d9dd7b3292a693ef57",
        "1399bb66b2f6ad99a7f064140eaaa885",
        "7385f5784b85bf0a97768ddd896d6d",
        "4351082bac9b4593ae8848cc9dfb5a01"
    );

    padded_cipher_kat!(
        test_openssl_aes_256_cbc_16_bytes,
        &AES_256,
        OperatingMode::CBC,
        PaddingStrategy::PKCS7,
        "d4a8206dcae01242f9db79a4ecfe277d0f7bb8ccbafd8f9809adb39f35aa9b41",
        "24f6076548fb9d93c8f7ed9f6e661ef9",
        "a39c1fdf77ea3e1f18178c0ec237c70a",
        "f1af484830a149ee0387b854d65fe87ca0e62efc1c8e6909d4b9ab8666470453"
    );
}
