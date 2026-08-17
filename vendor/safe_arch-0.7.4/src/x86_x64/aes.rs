#![cfg(target_feature = "aes")]

use super::*;

/// Perform one round of an AES decryption flow on `a` using the `round_key`.
///
/// * **Intrinsic:** [`_mm_aesdec_si128`]
/// * **Assembly:** `aesdec xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "aes")))]
pub fn aes_decrypt_m128i(a: m128i, round_key: m128i) -> m128i {
  m128i(unsafe { _mm_aesdec_si128(a.0, round_key.0) })
}

/// Perform the last round of an AES decryption flow on `a` using the
/// `round_key`.
///
/// * **Intrinsic:** [`_mm_aesdeclast_si128`]
/// * **Assembly:** `aesdeclast xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "aes")))]
pub fn aes_decrypt_last_m128i(a: m128i, round_key: m128i) -> m128i {
  m128i(unsafe { _mm_aesdeclast_si128(a.0, round_key.0) })
}

/// Perform one round of an AES encryption flow on `a` using the `round_key`.
///
/// * **Intrinsic:** [`_mm_aesenc_si128`]
/// * **Assembly:** `aesenc xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "aes")))]
pub fn aes_encrypt_m128i(a: m128i, round_key: m128i) -> m128i {
  m128i(unsafe { _mm_aesenc_si128(a.0, round_key.0) })
}

/// Perform the last round of an AES encryption flow on `a` using the
/// `round_key`.
///
/// * **Intrinsic:** [`_mm_aesenclast_si128`]
/// * **Assembly:** `aesenclast xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "aes")))]
pub fn aes_encrypt_last_m128i(a: m128i, round_key: m128i) -> m128i {
  m128i(unsafe { _mm_aesenclast_si128(a.0, round_key.0) })
}

/// Perform the InvMixColumns transform on `a`.
///
/// * **Intrinsic:** [`_mm_aesimc_si128`]
/// * **Assembly:** `aesimc xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "aes")))]
pub fn aes_inv_mix_columns_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_aesimc_si128(a.0) })
}

/// Assist in expanding an AES cipher key.
///
/// This computes steps towards generating a round key for an encryption cipher
/// using data from `a` and an 8-bit round constant specified by the `IMM`
/// constant used.
///
/// * **Intrinsic:** [`_mm_aeskeygenassist_si128`]
/// * **Assembly:** `aeskeygenassist xmm, xmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "aes")))]
pub fn aes_key_gen_assist_m128i<const IMM: i32>(a: m128i) -> m128i {
  m128i(unsafe { _mm_aeskeygenassist_si128(a.0, IMM) })
}

