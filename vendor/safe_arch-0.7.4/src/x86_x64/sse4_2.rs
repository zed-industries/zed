#![cfg(target_feature = "sse4.2")]

use super::*;

/// Lanewise `a > b` with lanes as `i64`.
///
/// All bits 1 for true (`-1`), all bit 0 for false (`0`).
///
/// * **Intrinsic:** [`_mm_cmpgt_epi64`]
/// * **Assembly:** `pcmpgtq xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.2")))]
pub fn cmp_gt_mask_i64_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_cmpgt_epi64(a.0, b.0) })
}

/// Accumulates the `u8` into a running CRC32 value.
///
/// * **Intrinsic:** [`_mm_crc32_u8`]
/// * **Assembly:** `crc32 r32, r8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.2")))]
pub fn crc32_u8(crc: u32, v: u8) -> u32 {
  unsafe { _mm_crc32_u8(crc, v) }
}

/// Accumulates the `u16` into a running CRC32 value.
///
/// * **Intrinsic:** [`_mm_crc32_u16`]
/// * **Assembly:** `crc32 r32, r16`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.2")))]
pub fn crc32_u16(crc: u32, v: u16) -> u32 {
  unsafe { _mm_crc32_u16(crc, v) }
}

/// Accumulates the `u32` into a running CRC32 value.
///
/// * **Intrinsic:** [`_mm_crc32_u32`]
/// * **Assembly:** `crc32 r32, r32`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.2")))]
pub fn crc32_u32(crc: u32, v: u32) -> u32 {
  unsafe { _mm_crc32_u32(crc, v) }
}

/// Accumulates the `u64` into a running CRC32 value.
///
/// **Note:** Has a different return type from the other crc32 functions.
///
/// * **Intrinsic:** [`_mm_crc32_u64`]
/// * **Assembly:** `crc32 r64, r64`
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.2")))]
pub fn crc32_u64(crc: u64, v: u64) -> u64 {
  unsafe { _mm_crc32_u64(crc, v) }
}

/// string segment elements are u8 values
pub const STR_CMP_U8: i32 = _SIDD_UBYTE_OPS;
/// string segment elements are u16 values
pub const STR_CMP_U16: i32 = _SIDD_UWORD_OPS;
/// string segment elements are i8 values
pub const STR_CMP_I8: i32 = _SIDD_SBYTE_OPS;
/// string segment elements are i16 values
pub const STR_CMP_I16: i32 = _SIDD_SWORD_OPS;

/// Matches when _any_ haystack character equals _any_ needle character,
/// regardless of position.
pub const STR_CMP_EQ_ANY: i32 = _SIDD_CMP_EQUAL_ANY;
/// Interprets consecutive pairs of characters in the needle as `(low..=high)`
/// ranges to compare each haystack character to.
pub const STR_CMP_RANGES: i32 = _SIDD_CMP_RANGES;
/// Matches when a character position in the needle is equal to the character at
/// the same position in the haystack.
pub const STR_CMP_EQ_EACH: i32 = _SIDD_CMP_EQUAL_EACH;
/// Matches when the complete needle string is a substring somewhere in the
/// haystack.
pub const STR_CMP_EQ_ORDERED: i32 = _SIDD_CMP_EQUAL_ORDERED;

/// Return the index of the first match found.
pub const STR_CMP_FIRST_MATCH: i32 = _SIDD_LEAST_SIGNIFICANT;
/// Return the index of the last match found.
pub const STR_CMP_LAST_MATCH: i32 = _SIDD_MOST_SIGNIFICANT;

/// Return the bitwise mask of matches.
pub const STR_CMP_BIT_MASK: i32 = _SIDD_BIT_MASK;
/// Return the lanewise mask of matches.
pub const STR_CMP_UNIT_MASK: i32 = _SIDD_UNIT_MASK;

/// Search for `needle` in `haystack, with implicit string length.
///
/// In the constant you need to provide (combine with `|`):
/// * A comparison unit: `STR_CMP_U8`, `STR_CMP_U16`, `STR_CMP_I8`, or
///   `STR_CMP_I16`.
/// * A comparison op: `STR_CMP_EQ_ANY`, `STR_CMP_RANGES`, `STR_CMP_EQ_EACH`, or
///   `STR_CMP_EQ_ORDERED`.
/// * The desired output: `STR_CMP_FIRST_MATCH` or `STR_CMP_LAST_MATCH`.
///
/// The first 0 unit is a null terminator for the string. If the string has no 0
/// units then the string ends at the end of the register.
///
/// If there's no match the output is the length of the haystack.
///
/// * **Intrinsic:** [`_mm_cmpistri`]
/// * **Assembly:** `pcmpistri xmm, xmm, imm8`
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.2")))]
pub fn search_implicit_str_for_index<const IMM: i32>(
  needle: m128i, haystack: m128i,
) -> i32 {
  unsafe { _mm_cmpistri(needle.0, haystack.0, IMM) }
}

/// Search for `needle` in `haystack, with explicit string length.
///
/// In the constant you need to provide (combine with `|`):
/// * A comparison unit: `STR_CMP_U8`, `STR_CMP_U16`, `STR_CMP_I8`, or
///   `STR_CMP_I16`.
/// * A comparison op: `STR_CMP_EQ_ANY`, `STR_CMP_RANGES`, `STR_CMP_EQ_EACH`, or
///   `STR_CMP_EQ_ORDERED`.
/// * The desired output: `STR_CMP_FIRST_MATCH` or `STR_CMP_LAST_MATCH`.
///
/// If there's no match the output is the length of the haystack.
///
/// * **Intrinsic:** [`_mm_cmpestri`]
/// * **Assembly:** `pcmpestri xmm, xmm, imm8`
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.2")))]
pub fn search_explicit_str_for_index<const IMM: i32>(
  needle: m128i, needle_len: i32, haystack: m128i, haystack_len: i32,
) -> i32 {
  unsafe { _mm_cmpestri(needle.0, needle_len, haystack.0, haystack_len, IMM) }
}

/// Search for `needle` in `haystack, with implicit string length.
///
/// In the constant you need to provide (combine with `|`):
/// * A comparison unit: `STR_CMP_U8`, `STR_CMP_U16`, `STR_CMP_I8`, or
///   `STR_CMP_I16`.
/// * A comparison op: `STR_CMP_EQ_ANY`, `STR_CMP_RANGES`, `STR_CMP_EQ_EACH`, or
///   `STR_CMP_EQ_ORDERED`.
/// * The desired out mask style: `STR_CMP_BIT_MASK` or `STR_CMP_UNIT_MASK`.
///
/// The first 0 unit is a null terminator for the string. If the string has no 0
/// units then the string ends at the end of the register.
///
/// * **Intrinsic:** [`_mm_cmpistrm`]
/// * **Assembly:** `pcmpistrm xmm, xmm, imm8`
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.2")))]
pub fn search_implicit_str_for_mask<const IMM: i32>(
  needle: m128i, haystack: m128i,
) -> m128i {
  m128i(unsafe { _mm_cmpistrm(needle.0, haystack.0, IMM) })
}

/// Search for `needle` in `haystack, with explicit string length.
///
/// In the constant you need to provide (combine with `|`):
/// * A comparison unit: `STR_CMP_U8`, `STR_CMP_U16`, `STR_CMP_I8`, or
///   `STR_CMP_I16`.
/// * A comparison op: `STR_CMP_EQ_ANY`, `STR_CMP_RANGES`, `STR_CMP_EQ_EACH`, or
///   `STR_CMP_EQ_ORDERED`.
/// * The desired out mask style: `STR_CMP_BIT_MASK` or `STR_CMP_UNIT_MASK`.
///
/// If there's no match the output is the length of the haystack.
///
/// * **Intrinsic:** [`_mm_cmpestrm`]
/// * **Assembly:** `pcmpestrm xmm, xmm, imm8`
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.2")))]
pub fn search_explicit_str_for_mask<const IMM: i32>(
  needle: m128i, needle_len: i32, haystack: m128i, haystack_len: i32,
) -> m128i {
  m128i(unsafe {
    _mm_cmpestrm(needle.0, needle_len, haystack.0, haystack_len, IMM)
  })
}

