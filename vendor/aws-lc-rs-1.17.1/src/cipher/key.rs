// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use crate::aws_lc::{AES_set_decrypt_key, AES_set_encrypt_key, AES_KEY};
#[cfg(feature = "legacy-des")]
use crate::aws_lc::{DES_cblock, DES_key_schedule, DES_set_key};
use crate::cipher::block::Block;
use crate::cipher::chacha::ChaCha20Key;
#[cfg(feature = "legacy-des")]
use crate::cipher::des::{DesKey, DES_EDE3_KEY_LEN, DES_EDE_KEY_LEN, DES_KEY_LEN};
use crate::cipher::{AES_128_KEY_LEN, AES_192_KEY_LEN, AES_256_KEY_LEN};
#[cfg(feature = "legacy-des")]
use crate::constant_time;
use crate::error::{KeyRejected, Unspecified};
use core::mem::{size_of, MaybeUninit};
use core::ptr::copy_nonoverlapping;
// TODO: Uncomment when MSRV >= 1.64
// use core::ffi::c_uint;
use std::os::raw::c_uint;
use zeroize::Zeroize;

pub(crate) enum SymmetricCipherKey {
    Aes128 {
        enc_key: AES_KEY,
        dec_key: AES_KEY,
    },
    Aes192 {
        enc_key: AES_KEY,
        dec_key: AES_KEY,
    },
    Aes256 {
        enc_key: AES_KEY,
        dec_key: AES_KEY,
    },
    ChaCha20 {
        raw_key: ChaCha20Key,
    },
    #[cfg(feature = "legacy-des")]
    Des {
        key: DesKey,
    },
    #[cfg(feature = "legacy-des")]
    DesEde {
        key: DesKey,
    },
    #[cfg(feature = "legacy-des")]
    DesEde3 {
        key: DesKey,
    },
}

unsafe impl Send for SymmetricCipherKey {}

// The AES_KEY value is only used as a `*const AES_KEY` in calls to `AES_encrypt`.
unsafe impl Sync for SymmetricCipherKey {}

impl Drop for SymmetricCipherKey {
    fn drop(&mut self) {
        // Aes128Key, Aes256Key and ChaCha20Key implement Drop separately.
        match self {
            SymmetricCipherKey::Aes128 { enc_key, dec_key }
            | SymmetricCipherKey::Aes192 { enc_key, dec_key }
            | SymmetricCipherKey::Aes256 { enc_key, dec_key } => unsafe {
                let enc_bytes: &mut [u8; size_of::<AES_KEY>()] = (enc_key as *mut AES_KEY)
                    .cast::<[u8; size_of::<AES_KEY>()]>()
                    .as_mut()
                    .unwrap();
                enc_bytes.zeroize();
                let dec_bytes: &mut [u8; size_of::<AES_KEY>()] = (dec_key as *mut AES_KEY)
                    .cast::<[u8; size_of::<AES_KEY>()]>()
                    .as_mut()
                    .unwrap();
                dec_bytes.zeroize();
            },
            SymmetricCipherKey::ChaCha20 { .. } => {}
            #[cfg(feature = "legacy-des")]
            SymmetricCipherKey::Des { key }
            | SymmetricCipherKey::DesEde { key }
            | SymmetricCipherKey::DesEde3 { key } => unsafe {
                let key_bytes: &mut [u8; size_of::<DesKey>()] = (key as *mut DesKey)
                    .cast::<[u8; size_of::<DesKey>()]>()
                    .as_mut()
                    .unwrap();
                key_bytes.zeroize();
            },
        }
    }
}

impl SymmetricCipherKey {
    fn aes(key_bytes: &[u8]) -> Result<(AES_KEY, AES_KEY), Unspecified> {
        let mut enc_key = MaybeUninit::<AES_KEY>::uninit();
        let mut dec_key = MaybeUninit::<AES_KEY>::uninit();
        #[allow(clippy::cast_possible_truncation)]
        if unsafe {
            0 != AES_set_encrypt_key(
                key_bytes.as_ptr(),
                (key_bytes.len() * 8) as c_uint,
                enc_key.as_mut_ptr(),
            )
        } {
            return Err(Unspecified);
        }

        #[allow(clippy::cast_possible_truncation)]
        if unsafe {
            0 != AES_set_decrypt_key(
                key_bytes.as_ptr(),
                (key_bytes.len() * 8) as c_uint,
                dec_key.as_mut_ptr(),
            )
        } {
            return Err(Unspecified);
        }
        unsafe { Ok((enc_key.assume_init(), dec_key.assume_init())) }
    }

    pub(crate) fn aes128(key_bytes: &[u8]) -> Result<Self, KeyRejected> {
        if key_bytes.len() != AES_128_KEY_LEN {
            return Err(KeyRejected::unspecified());
        }
        let (enc_key, dec_key) = SymmetricCipherKey::aes(key_bytes)?;
        Ok(SymmetricCipherKey::Aes128 { enc_key, dec_key })
    }

    pub(crate) fn aes192(key_bytes: &[u8]) -> Result<Self, KeyRejected> {
        if key_bytes.len() != AES_192_KEY_LEN {
            return Err(KeyRejected::unspecified());
        }
        let (enc_key, dec_key) = SymmetricCipherKey::aes(key_bytes)?;
        Ok(SymmetricCipherKey::Aes192 { enc_key, dec_key })
    }

    pub(crate) fn aes256(key_bytes: &[u8]) -> Result<Self, KeyRejected> {
        if key_bytes.len() != AES_256_KEY_LEN {
            return Err(KeyRejected::unspecified());
        }
        let (enc_key, dec_key) = SymmetricCipherKey::aes(key_bytes)?;
        Ok(SymmetricCipherKey::Aes256 { enc_key, dec_key })
    }

    pub(crate) fn chacha20(key_bytes: &[u8]) -> Result<Self, KeyRejected> {
        if key_bytes.len() != 32 {
            return Err(KeyRejected::unspecified());
        }
        let mut kb = MaybeUninit::<[u8; 32]>::uninit();
        unsafe {
            copy_nonoverlapping(key_bytes.as_ptr(), kb.as_mut_ptr().cast(), 32);
            Ok(SymmetricCipherKey::ChaCha20 {
                raw_key: ChaCha20Key(kb.assume_init()),
            })
        }
    }

    /// Validates single DES key material and computes the key schedule.
    ///
    /// Returns the schedule as `[ks, zeroed, zeroed]` (only slot 0 is used).
    /// This is the shared validation/preparation step used by both
    /// [`SymmetricCipherKey::des`] and
    /// [`UnboundCipherKey::validate_key_material`].
    #[cfg(feature = "legacy-des")]
    pub(crate) fn prepare_des(key_bytes: &[u8]) -> Result<[DES_key_schedule; 3], KeyRejected> {
        if key_bytes.len() != DES_KEY_LEN {
            return Err(KeyRejected::unspecified());
        }
        let k: &[u8; 8] = key_bytes.try_into().expect("length already checked");
        let ks = Self::des_set_key(k)?;
        let zero = MaybeUninit::<DES_key_schedule>::zeroed();
        unsafe { Ok([ks, zero.assume_init(), zero.assume_init()]) }
    }

    #[cfg(feature = "legacy-des")]
    pub(crate) fn des(key_bytes: &[u8]) -> Result<Self, KeyRejected> {
        Ok(SymmetricCipherKey::Des {
            key: DesKey(Self::prepare_des(key_bytes)?),
        })
    }

    /// Validates 2TDEA key material and computes the three DES key schedules.
    ///
    /// Returns the schedules as `[ks1, ks2, ks1]` (K3 = K1 for 2TDEA).
    /// This is the shared validation/preparation step used by both
    /// [`SymmetricCipherKey::des_ede`] and
    /// [`UnboundCipherKey::validate_key_material`].
    #[cfg(feature = "legacy-des")]
    pub(crate) fn prepare_des_ede(key_bytes: &[u8]) -> Result<[DES_key_schedule; 3], KeyRejected> {
        if key_bytes.len() != DES_EDE_KEY_LEN {
            return Err(KeyRejected::unspecified());
        }
        // `as_chunks` is only stable since Rust 1.88.0, so use explicit slicing
        // instead to stay within the crate's MSRV.
        let first_key: &[u8; 8] = key_bytes[0..8].try_into().expect("length already checked");
        let second_key: &[u8; 8] = key_bytes[8..16].try_into().expect("length already checked");

        // SP 800-67 §3.1 requires K1 != K2 for 2-Key TDEA; if they are equal
        // the cipher degenerates to single-DES (56-bit effective security).
        if constant_time::verify_slices_are_equal(first_key, second_key).is_ok() {
            return Err(KeyRejected::inconsistent_components());
        }

        // 2TDEA is defined as E_K1(D_K2(E_K1(.))), so K3 is always a copy of
        // K1. `DES_key_schedule` is `Copy`, so we only need to run the key
        // schedule for K1 once.
        let ks1 = Self::des_set_key(first_key)?;
        let ks2 = Self::des_set_key(second_key)?;
        Ok([ks1, ks2, ks1])
    }

    #[cfg(feature = "legacy-des")]
    pub(crate) fn des_ede(key_bytes: &[u8]) -> Result<Self, KeyRejected> {
        Ok(SymmetricCipherKey::DesEde {
            key: DesKey(Self::prepare_des_ede(key_bytes)?),
        })
    }

    #[cfg(feature = "legacy-des")]
    fn des_set_key(key_bytes: &[u8; 8]) -> Result<DES_key_schedule, KeyRejected> {
        let mut ks = MaybeUninit::<DES_key_schedule>::uninit();
        // DES_set_key return values (see openssl/des.h):
        //   0  => key is not weak and has odd parity (preferred)
        //  -1  => key parity is not odd (schedule is still valid; most callers
        //         do not maintain DES parity bits, so we accept this)
        //  -2  => key is a weak or semi-weak DES key; reject it
        // Any other non-zero value is treated as a failure.
        match unsafe { DES_set_key(key_bytes.as_ptr() as *const DES_cblock, ks.as_mut_ptr()) } {
            0 | -1 => Ok(unsafe { ks.assume_init() }),
            _ => Err(KeyRejected::unspecified()),
        }
    }

    /// Validates 3TDEA key material and computes the three DES key schedules.
    ///
    /// This is the shared validation/preparation step used by both
    /// [`SymmetricCipherKey::des_ede3`] and
    /// [`UnboundCipherKey::validate_key_material`].
    #[cfg(feature = "legacy-des")]
    pub(crate) fn prepare_des_ede3(key_bytes: &[u8]) -> Result<[DES_key_schedule; 3], KeyRejected> {
        if key_bytes.len() != DES_EDE3_KEY_LEN {
            return Err(KeyRejected::unspecified());
        }
        // `as_chunks` is only stable since Rust 1.88.0, so use explicit slicing
        // instead to stay within the crate's MSRV.
        let first_key: &[u8; 8] = key_bytes[0..8].try_into().expect("length already checked");
        let second_key: &[u8; 8] = key_bytes[8..16].try_into().expect("length already checked");
        let third_key: &[u8; 8] = key_bytes[16..24]
            .try_into()
            .expect("length already checked");

        // SP 800-67 §2 (Keying Option 1) requires K1, K2 and K3 to be
        // pairwise independent. We enforce that all three subkeys are distinct
        // so the cipher cannot degenerate to single-DES (K1==K2 or K2==K3)
        // or to 2TDEA (K1==K3). Callers who want 2TDEA must use
        // `DES_EDE_FOR_LEGACY_USE_ONLY` with a 16-byte key instead of
        // encoding 2TDEA as a 24-byte 3TDEA key.
        let mut any_equal: u8 = 0;
        for (a, b) in [
            (first_key, second_key),
            (second_key, third_key),
            (first_key, third_key),
        ] {
            any_equal |= u8::from(constant_time::verify_slices_are_equal(a, b).is_ok());
            any_equal = core::hint::black_box(any_equal);
        }
        if any_equal != 0 {
            return Err(KeyRejected::inconsistent_components());
        }

        Ok([
            Self::des_set_key(first_key)?,
            Self::des_set_key(second_key)?,
            Self::des_set_key(third_key)?,
        ])
    }

    #[cfg(feature = "legacy-des")]
    pub(crate) fn des_ede3(key_bytes: &[u8]) -> Result<Self, KeyRejected> {
        Ok(SymmetricCipherKey::DesEde3 {
            key: DesKey(Self::prepare_des_ede3(key_bytes)?),
        })
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) fn encrypt_block(&self, block: Block) -> Block {
        match self {
            SymmetricCipherKey::Aes128 { enc_key, .. }
            | SymmetricCipherKey::Aes192 { enc_key, .. }
            | SymmetricCipherKey::Aes256 { enc_key, .. } => {
                super::aes::encrypt_block(enc_key, block)
            }
            SymmetricCipherKey::ChaCha20 { .. } => {
                panic!("Unsupported algorithm!")
            }
            #[cfg(feature = "legacy-des")]
            SymmetricCipherKey::Des { .. }
            | SymmetricCipherKey::DesEde { .. }
            | SymmetricCipherKey::DesEde3 { .. } => {
                panic!("Unsupported algorithm!")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cipher::block::{Block, BLOCK_LEN};
    use crate::cipher::key::SymmetricCipherKey;
    use crate::test::from_hex;

    #[test]
    fn test_encrypt_block_aes_128() {
        let key = from_hex("000102030405060708090a0b0c0d0e0f").unwrap();
        let input = from_hex("00112233445566778899aabbccddeeff").unwrap();
        let expected_result = from_hex("69c4e0d86a7b0430d8cdb78070b4c55a").unwrap();
        let input_block: [u8; BLOCK_LEN] = <[u8; BLOCK_LEN]>::try_from(input).unwrap();

        let aes128 = SymmetricCipherKey::aes128(key.as_slice()).unwrap();
        let result = aes128.encrypt_block(Block::from(input_block));

        assert_eq!(expected_result.as_slice(), result.as_ref());
    }

    #[test]
    fn test_encrypt_block_aes_256() {
        let key =
            from_hex("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f").unwrap();
        let input = from_hex("00112233445566778899aabbccddeeff").unwrap();
        let expected_result = from_hex("8ea2b7ca516745bfeafc49904b496089").unwrap();
        let input_block: [u8; BLOCK_LEN] = <[u8; BLOCK_LEN]>::try_from(input).unwrap();

        let aes128 = SymmetricCipherKey::aes256(key.as_slice()).unwrap();
        let result = aes128.encrypt_block(Block::from(input_block));

        assert_eq!(expected_result.as_slice(), result.as_ref());
    }
}
