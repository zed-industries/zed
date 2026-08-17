// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use crate::aws_lc::{
    DES_cblock, DES_ecb3_encrypt, DES_ecb_encrypt, DES_ede3_cbc_encrypt, DES_key_schedule,
    DES_ncbc_encrypt, DES_DECRYPT, DES_ENCRYPT,
};
use crate::error::Unspecified;
use crate::fips::indicator_check;
use zeroize::Zeroize;

use super::{DecryptionContext, EncryptionContext, SymmetricCipherKey};

/// Length of a single DES key in bytes.
pub const DES_KEY_LEN: usize = 8;

/// Length of a 2TDEA (DES-EDE) key in bytes.
pub const DES_EDE_KEY_LEN: usize = 16;

/// Length of a 3TDEA (DES-EDE3) key in bytes.
pub const DES_EDE3_KEY_LEN: usize = 24;

/// The number of bytes for a DES/3DES-CBC initialization vector (IV).
pub const DES_CBC_IV_LEN: usize = 8;

pub(crate) const DES_BLOCK_LEN: usize = 8;

// `#[repr(transparent)]` documents that `DesKey` has the same layout as the
// wrapped array, which the `Drop` impl in `key.rs` relies on when it zeroizes
// the key schedule bytes through a byte-slice view.
//
// The array is sized for 3TDEA and is used unchanged for single DES (slot 0
// populated, slots 1 and 2 zeroed) and 2TDEA (slots 0 and 1 populated, slot 2 =
// slot 0 per K3 == K1). Callers in this module match on `SymmetricCipherKey`
// and read only the slots that are meaningful for the variant.
#[repr(transparent)]
pub(crate) struct DesKey(pub(super) [DES_key_schedule; 3]);

pub(super) fn encrypt_cbc_mode(
    key: &SymmetricCipherKey,
    context: EncryptionContext,
    in_out: &mut [u8],
) -> Result<DecryptionContext, Unspecified> {
    let iv_bytes: &[u8] = (&context).try_into()?;
    let mut iv = [0u8; DES_CBC_IV_LEN];
    iv.copy_from_slice(iv_bytes);

    match key {
        SymmetricCipherKey::Des { key } => {
            des_cbc_encrypt(&key.0[0], &mut iv, in_out, DES_ENCRYPT);
        }
        SymmetricCipherKey::DesEde { key } | SymmetricCipherKey::DesEde3 { key } => {
            des3_cbc_encrypt(
                &key.0[0],
                &key.0[1],
                &key.0[2],
                &mut iv,
                in_out,
                DES_ENCRYPT,
            );
        }
        _ => unreachable!(),
    }

    iv.zeroize();
    Ok(context.into())
}

pub(super) fn decrypt_cbc_mode<'in_out>(
    key: &SymmetricCipherKey,
    context: DecryptionContext,
    in_out: &'in_out mut [u8],
) -> Result<&'in_out mut [u8], Unspecified> {
    let iv_bytes: &[u8] = (&context).try_into()?;
    let mut iv = [0u8; DES_CBC_IV_LEN];
    iv.copy_from_slice(iv_bytes);

    match key {
        SymmetricCipherKey::Des { key } => {
            des_cbc_encrypt(&key.0[0], &mut iv, in_out, DES_DECRYPT);
        }
        SymmetricCipherKey::DesEde { key } | SymmetricCipherKey::DesEde3 { key } => {
            des3_cbc_encrypt(
                &key.0[0],
                &key.0[1],
                &key.0[2],
                &mut iv,
                in_out,
                DES_DECRYPT,
            );
        }
        _ => unreachable!(),
    }

    iv.zeroize();
    Ok(in_out)
}

pub(super) fn encrypt_ecb_mode(
    key: &SymmetricCipherKey,
    context: EncryptionContext,
    in_out: &mut [u8],
) -> Result<DecryptionContext, Unspecified> {
    if !matches!(context, EncryptionContext::None) {
        unreachable!();
    }

    let mut in_out_iter = in_out.chunks_exact_mut(DES_BLOCK_LEN);
    match key {
        SymmetricCipherKey::Des { key } => {
            for block in in_out_iter.by_ref() {
                des_ecb_encrypt(&key.0[0], block, DES_ENCRYPT);
            }
        }
        SymmetricCipherKey::DesEde { key } | SymmetricCipherKey::DesEde3 { key } => {
            for block in in_out_iter.by_ref() {
                des3_ecb_encrypt(&key.0[0], &key.0[1], &key.0[2], block, DES_ENCRYPT);
            }
        }
        _ => unreachable!(),
    }
    // Sanity check: `encrypt` validates that `in_out.len() % block_len == 0`
    // for ECB mode before dispatching here.
    debug_assert!(in_out_iter.into_remainder().is_empty());

    Ok(context.into())
}

pub(super) fn decrypt_ecb_mode<'in_out>(
    key: &SymmetricCipherKey,
    context: DecryptionContext,
    in_out: &'in_out mut [u8],
) -> Result<&'in_out mut [u8], Unspecified> {
    if !matches!(context, DecryptionContext::None) {
        unreachable!();
    }

    // The inner scope ends the mutable borrow of `in_out` held by
    // `in_out_iter` before we return `Ok(in_out)` below. `into_remainder()`
    // would also consume the iterator and release the borrow, but it's inside
    // a `debug_assert!` and therefore not evaluated in release builds, so we
    // can't rely on it for that purpose. `encrypt_ecb_mode` doesn't need this
    // scope because it doesn't return `in_out`.
    {
        let mut in_out_iter = in_out.chunks_exact_mut(DES_BLOCK_LEN);
        match key {
            SymmetricCipherKey::Des { key } => {
                for block in in_out_iter.by_ref() {
                    des_ecb_encrypt(&key.0[0], block, DES_DECRYPT);
                }
            }
            SymmetricCipherKey::DesEde { key } | SymmetricCipherKey::DesEde3 { key } => {
                for block in in_out_iter.by_ref() {
                    des3_ecb_encrypt(&key.0[0], &key.0[1], &key.0[2], block, DES_DECRYPT);
                }
            }
            _ => unreachable!(),
        }
        // Sanity check: `decrypt` validates that `in_out.len() % block_len == 0`
        // for ECB mode before dispatching here.
        debug_assert!(in_out_iter.into_remainder().is_empty());
    }

    Ok(in_out)
}

fn des3_cbc_encrypt(
    ks1: &DES_key_schedule,
    ks2: &DES_key_schedule,
    ks3: &DES_key_schedule,
    iv: &mut [u8; DES_CBC_IV_LEN],
    in_out: &mut [u8],
    enc: i32,
) {
    // The caller (`encrypt`/`decrypt` in cipher.rs) validates block alignment
    // for CBC mode; assert here as a safety net.
    debug_assert_eq!(in_out.len() % DES_BLOCK_LEN, 0);

    // DES is not FIPS-approved; `indicator_check!` will observe that the
    // underlying call does not increment the approved-operation counter and
    // correctly mark the service indicator as unapproved in FIPS builds.
    //
    // SAFETY: `DES_ede3_cbc_encrypt` supports in-place operation (in == out).
    indicator_check!(unsafe {
        DES_ede3_cbc_encrypt(
            in_out.as_ptr(),
            in_out.as_mut_ptr(),
            in_out.len(),
            ks1,
            ks2,
            ks3,
            iv.as_mut_ptr() as *mut DES_cblock,
            enc,
        );
    });
}

fn des3_ecb_encrypt(
    ks1: &DES_key_schedule,
    ks2: &DES_key_schedule,
    ks3: &DES_key_schedule,
    block: &mut [u8],
    enc: i32,
) {
    let input_block = block.as_ptr() as *const DES_cblock;
    let output_block = block.as_mut_ptr() as *mut DES_cblock;
    // DES is not FIPS-approved; `indicator_check!` will observe that the
    // underlying call does not increment the approved-operation counter and
    // correctly mark the service indicator as unapproved in FIPS builds.
    //
    // SAFETY: `input_block` and `output_block` point to the same allocation;
    // `DES_ecb3_encrypt` supports in-place operation.
    indicator_check!(unsafe {
        DES_ecb3_encrypt(input_block, output_block, ks1, ks2, ks3, enc);
    });
}

fn des_cbc_encrypt(
    ks: &DES_key_schedule,
    iv: &mut [u8; DES_CBC_IV_LEN],
    in_out: &mut [u8],
    enc: i32,
) {
    // The caller (`encrypt`/`decrypt` in cipher.rs) validates block alignment
    // for CBC mode; assert here as a safety net.
    debug_assert_eq!(in_out.len() % DES_BLOCK_LEN, 0);

    // DES is not FIPS-approved; `indicator_check!` will observe that the
    // underlying call does not increment the approved-operation counter and
    // correctly mark the service indicator as unapproved in FIPS builds.
    //
    // SAFETY: `DES_ncbc_encrypt` supports in-place operation (in == out).
    indicator_check!(unsafe {
        DES_ncbc_encrypt(
            in_out.as_ptr(),
            in_out.as_mut_ptr(),
            in_out.len(),
            ks,
            iv.as_mut_ptr() as *mut DES_cblock,
            enc,
        );
    });
}

fn des_ecb_encrypt(ks: &DES_key_schedule, block: &mut [u8], enc: i32) {
    let input_block = block.as_ptr() as *const DES_cblock;
    let output_block = block.as_mut_ptr() as *mut DES_cblock;
    // DES is not FIPS-approved; `indicator_check!` will observe that the
    // underlying call does not increment the approved-operation counter and
    // correctly mark the service indicator as unapproved in FIPS builds.
    //
    // SAFETY: `input_block` and `output_block` point to the same allocation;
    // `DES_ecb_encrypt` supports in-place operation.
    indicator_check!(unsafe {
        DES_ecb_encrypt(input_block, output_block, ks, enc);
    });
}
