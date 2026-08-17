// Copyright 2018 Brian Smith.
// SPDX-License-Identifier: ISC
// Modifications copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use crate::aws_lc::{
    AES_cbc_encrypt, AES_cfb128_encrypt, AES_ctr128_encrypt, AES_ecb_encrypt, AES_DECRYPT,
    AES_ENCRYPT, AES_KEY,
};
use crate::cipher::block::Block;
use crate::error::Unspecified;
use crate::fips::indicator_check;
use zeroize::Zeroize;

use super::{DecryptionContext, EncryptionContext, OperatingMode, SymmetricCipherKey};

/// Length of an AES-128 key in bytes.
pub const AES_128_KEY_LEN: usize = 16;

/// Length of an AES-192 key in bytes.
pub const AES_192_KEY_LEN: usize = 24;

/// Length of an AES-256 key in bytes.
pub const AES_256_KEY_LEN: usize = 32;

/// The number of bytes for an AES-CBC initialization vector (IV)
pub const AES_CBC_IV_LEN: usize = 16;

/// The number of bytes for an AES-CTR initialization vector (IV)
pub const AES_CTR_IV_LEN: usize = 16;

/// The number of bytes for an AES-CFB initialization vector (IV)
pub const AES_CFB_IV_LEN: usize = 16;

pub const AES_BLOCK_LEN: usize = 16;

#[inline]
pub(crate) fn encrypt_block(aes_key: &AES_KEY, mut block: Block) -> Block {
    {
        let block_ref = block.as_mut();
        debug_assert_eq!(block_ref.len(), AES_BLOCK_LEN);
        aes_ecb_encrypt(aes_key, block_ref);
    }
    block
}

pub(super) fn encrypt_ctr_mode(
    key: &SymmetricCipherKey,
    context: EncryptionContext,
    in_out: &mut [u8],
) -> Result<DecryptionContext, Unspecified> {
    let (SymmetricCipherKey::Aes128 { enc_key, .. }
    | SymmetricCipherKey::Aes192 { enc_key, .. }
    | SymmetricCipherKey::Aes256 { enc_key, .. }) = &key
    else {
        unreachable!()
    };

    let mut iv = {
        let mut iv = [0u8; AES_CTR_IV_LEN];
        iv.copy_from_slice((&context).try_into()?);
        iv
    };

    let mut buffer = [0u8; AES_BLOCK_LEN];

    aes_ctr128_encrypt(enc_key, &mut iv, &mut buffer, in_out);
    iv.zeroize();

    Ok(context.into())
}

pub(super) fn decrypt_ctr_mode<'in_out>(
    key: &SymmetricCipherKey,
    context: DecryptionContext,
    in_out: &'in_out mut [u8],
) -> Result<&'in_out mut [u8], Unspecified> {
    // it's the same in CTR, just providing a nice named wrapper to match
    encrypt_ctr_mode(key, context.into(), in_out).map(|_| in_out)
}

pub(super) fn encrypt_cbc_mode(
    key: &SymmetricCipherKey,
    context: EncryptionContext,
    in_out: &mut [u8],
) -> Result<DecryptionContext, Unspecified> {
    let (SymmetricCipherKey::Aes128 { enc_key, .. }
    | SymmetricCipherKey::Aes192 { enc_key, .. }
    | SymmetricCipherKey::Aes256 { enc_key, .. }) = &key
    else {
        unreachable!()
    };

    let mut iv = {
        let mut iv = [0u8; AES_CBC_IV_LEN];
        iv.copy_from_slice((&context).try_into()?);
        iv
    };

    aes_cbc_encrypt(enc_key, &mut iv, in_out);
    iv.zeroize();

    Ok(context.into())
}

#[allow(clippy::needless_pass_by_value)]
pub(super) fn decrypt_cbc_mode<'in_out>(
    key: &SymmetricCipherKey,
    context: DecryptionContext,
    in_out: &'in_out mut [u8],
) -> Result<&'in_out mut [u8], Unspecified> {
    let (SymmetricCipherKey::Aes128 { dec_key, .. }
    | SymmetricCipherKey::Aes192 { dec_key, .. }
    | SymmetricCipherKey::Aes256 { dec_key, .. }) = &key
    else {
        unreachable!()
    };

    let mut iv = {
        let mut iv = [0u8; AES_CBC_IV_LEN];
        iv.copy_from_slice((&context).try_into()?);
        iv
    };

    aes_cbc_decrypt(dec_key, &mut iv, in_out);
    iv.zeroize();

    Ok(in_out)
}

#[allow(clippy::needless_pass_by_value)]
pub(super) fn encrypt_cfb_mode(
    key: &SymmetricCipherKey,
    mode: OperatingMode,
    context: EncryptionContext,
    in_out: &mut [u8],
) -> Result<DecryptionContext, Unspecified> {
    let (SymmetricCipherKey::Aes128 { enc_key, .. }
    | SymmetricCipherKey::Aes192 { enc_key, .. }
    | SymmetricCipherKey::Aes256 { enc_key, .. }) = &key
    else {
        unreachable!()
    };

    let mut iv = {
        let mut iv = [0u8; AES_CFB_IV_LEN];
        iv.copy_from_slice((&context).try_into()?);
        iv
    };

    let cfb_encrypt: fn(&AES_KEY, &mut [u8], &mut [u8]) = match mode {
        // TODO: Hopefully support CFB1, and CFB8
        OperatingMode::CFB128 => aes_cfb128_encrypt,
        _ => unreachable!(),
    };

    cfb_encrypt(enc_key, &mut iv, in_out);
    iv.zeroize();

    Ok(context.into())
}

#[allow(clippy::needless_pass_by_value)]
pub(super) fn decrypt_cfb_mode<'in_out>(
    key: &SymmetricCipherKey,
    mode: OperatingMode,
    context: DecryptionContext,
    in_out: &'in_out mut [u8],
) -> Result<&'in_out mut [u8], Unspecified> {
    let (SymmetricCipherKey::Aes128 { enc_key, .. }
    | SymmetricCipherKey::Aes192 { enc_key, .. }
    | SymmetricCipherKey::Aes256 { enc_key, .. }) = &key
    else {
        unreachable!()
    };

    let mut iv = {
        let mut iv = [0u8; AES_CFB_IV_LEN];
        iv.copy_from_slice((&context).try_into()?);
        iv
    };

    let cfb_decrypt: fn(&AES_KEY, &mut [u8], &mut [u8]) = match mode {
        // TODO: Hopefully support CFB1, and CFB8
        OperatingMode::CFB128 => aes_cfb128_decrypt,
        _ => unreachable!(),
    };

    cfb_decrypt(enc_key, &mut iv, in_out);

    iv.zeroize();

    Ok(in_out)
}

#[allow(clippy::needless_pass_by_value, clippy::unnecessary_wraps)]
pub(super) fn encrypt_ecb_mode(
    key: &SymmetricCipherKey,
    context: EncryptionContext,
    in_out: &mut [u8],
) -> Result<DecryptionContext, Unspecified> {
    if !matches!(context, EncryptionContext::None) {
        unreachable!();
    }

    let (SymmetricCipherKey::Aes128 { enc_key, .. }
    | SymmetricCipherKey::Aes192 { enc_key, .. }
    | SymmetricCipherKey::Aes256 { enc_key, .. }) = &key
    else {
        unreachable!()
    };

    let mut in_out_iter = in_out.chunks_exact_mut(AES_BLOCK_LEN);

    for block in in_out_iter.by_ref() {
        aes_ecb_encrypt(enc_key, block);
    }

    // This is a sanity check that should not happen. We validate in `encrypt` that in_out.len() % block_len == 0
    // for this mode.
    debug_assert!(in_out_iter.into_remainder().is_empty());

    Ok(context.into())
}

#[allow(clippy::needless_pass_by_value, clippy::unnecessary_wraps)]
pub(super) fn decrypt_ecb_mode<'in_out>(
    key: &SymmetricCipherKey,
    context: DecryptionContext,
    in_out: &'in_out mut [u8],
) -> Result<&'in_out mut [u8], Unspecified> {
    if !matches!(context, DecryptionContext::None) {
        unreachable!();
    }

    let (SymmetricCipherKey::Aes128 { dec_key, .. }
    | SymmetricCipherKey::Aes192 { dec_key, .. }
    | SymmetricCipherKey::Aes256 { dec_key, .. }) = &key
    else {
        unreachable!()
    };

    {
        let mut in_out_iter = in_out.chunks_exact_mut(AES_BLOCK_LEN);

        for block in in_out_iter.by_ref() {
            aes_ecb_decrypt(dec_key, block);
        }

        // This is a sanity check hat should not fail. We validate in `decrypt` that in_out.len() % block_len == 0 for
        // this mode.
        debug_assert!(in_out_iter.into_remainder().is_empty());
    }

    Ok(in_out)
}

fn aes_ecb_encrypt(key: &AES_KEY, in_out: &mut [u8]) {
    indicator_check!(unsafe {
        AES_ecb_encrypt(in_out.as_ptr(), in_out.as_mut_ptr(), key, AES_ENCRYPT);
    });
}

fn aes_ecb_decrypt(key: &AES_KEY, in_out: &mut [u8]) {
    indicator_check!(unsafe {
        AES_ecb_encrypt(in_out.as_ptr(), in_out.as_mut_ptr(), key, AES_DECRYPT);
    });
}

fn aes_ctr128_encrypt(key: &AES_KEY, iv: &mut [u8], block_buffer: &mut [u8], in_out: &mut [u8]) {
    let mut num: u32 = 0;

    indicator_check!(unsafe {
        AES_ctr128_encrypt(
            in_out.as_ptr(),
            in_out.as_mut_ptr(),
            in_out.len(),
            key,
            iv.as_mut_ptr(),
            block_buffer.as_mut_ptr(),
            &mut num,
        );
    });

    Zeroize::zeroize(block_buffer);
}

fn aes_cbc_encrypt(key: &AES_KEY, iv: &mut [u8], in_out: &mut [u8]) {
    indicator_check!(unsafe {
        AES_cbc_encrypt(
            in_out.as_ptr(),
            in_out.as_mut_ptr(),
            in_out.len(),
            key,
            iv.as_mut_ptr(),
            AES_ENCRYPT,
        );
    });
}

fn aes_cbc_decrypt(key: &AES_KEY, iv: &mut [u8], in_out: &mut [u8]) {
    indicator_check!(unsafe {
        AES_cbc_encrypt(
            in_out.as_ptr(),
            in_out.as_mut_ptr(),
            in_out.len(),
            key,
            iv.as_mut_ptr(),
            AES_DECRYPT,
        );
    });
}

fn aes_cfb128_encrypt(key: &AES_KEY, iv: &mut [u8], in_out: &mut [u8]) {
    let mut num: i32 = 0;
    indicator_check!(unsafe {
        AES_cfb128_encrypt(
            in_out.as_ptr(),
            in_out.as_mut_ptr(),
            in_out.len(),
            key,
            iv.as_mut_ptr(),
            &mut num,
            AES_ENCRYPT,
        );
    });
}

fn aes_cfb128_decrypt(key: &AES_KEY, iv: &mut [u8], in_out: &mut [u8]) {
    let mut num: i32 = 0;
    indicator_check!(unsafe {
        AES_cfb128_encrypt(
            in_out.as_ptr(),
            in_out.as_mut_ptr(),
            in_out.len(),
            key,
            iv.as_mut_ptr(),
            &mut num,
            AES_DECRYPT,
        );
    });
}
