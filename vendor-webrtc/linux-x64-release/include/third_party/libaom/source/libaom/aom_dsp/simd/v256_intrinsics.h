/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AOM_DSP_SIMD_V256_INTRINSICS_H_
#define AOM_AOM_DSP_SIMD_V256_INTRINSICS_H_

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "aom_dsp/simd/v256_intrinsics_c.h"
#include "aom_dsp/simd/v128_intrinsics.h"
#include "aom_dsp/simd/v64_intrinsics.h"

/* Fallback to plain, unoptimised C. */

typedef c_v256 v256;

SIMD_INLINE uint32_t v256_low_u32(v256 a) { return c_v256_low_u32(a); }
SIMD_INLINE v64 v256_low_v64(v256 a) { return c_v256_low_v64(a); }
SIMD_INLINE uint64_t v256_low_u64(v256 a) { return c_v256_low_u64(a); }
SIMD_INLINE v128 v256_low_v128(v256 a) { return c_v256_low_v128(a); }
SIMD_INLINE v128 v256_high_v128(v256 a) { return c_v256_high_v128(a); }
SIMD_INLINE v256 v256_from_v128(v128 hi, v128 lo) {
  return c_v256_from_v128(hi, lo);
}
SIMD_INLINE v256 v256_from_64(uint64_t a, uint64_t b, uint64_t c, uint64_t d) {
  return c_v256_from_64(a, b, c, d);
}
SIMD_INLINE v256 v256_from_v64(v64 a, v64 b, v64 c, v64 d) {
  return c_v256_from_v64(a, b, c, d);
}

SIMD_INLINE v256 v256_load_unaligned(const void *p) {
  return c_v256_load_unaligned(p);
}
SIMD_INLINE v256 v256_load_aligned(const void *p) {
  return c_v256_load_aligned(p);
}

SIMD_INLINE void v256_store_unaligned(void *p, v256 a) {
  c_v256_store_unaligned(p, a);
}
SIMD_INLINE void v256_store_aligned(void *p, v256 a) {
  c_v256_store_aligned(p, a);
}

SIMD_INLINE v256 v256_align(v256 a, v256 b, unsigned int c) {
  return c_v256_align(a, b, c);
}

SIMD_INLINE v256 v256_zero(void) { return c_v256_zero(); }
SIMD_INLINE v256 v256_dup_8(uint8_t x) { return c_v256_dup_8(x); }
SIMD_INLINE v256 v256_dup_16(uint16_t x) { return c_v256_dup_16(x); }
SIMD_INLINE v256 v256_dup_32(uint32_t x) { return c_v256_dup_32(x); }
SIMD_INLINE v256 v256_dup_64(uint64_t x) { return c_v256_dup_64(x); }

SIMD_INLINE c_sad256_internal v256_sad_u8_init(void) {
  return c_v256_sad_u8_init();
}
SIMD_INLINE c_sad256_internal v256_sad_u8(c_sad256_internal s, v256 a, v256 b) {
  return c_v256_sad_u8(s, a, b);
}
SIMD_INLINE uint32_t v256_sad_u8_sum(c_sad256_internal s) {
  return c_v256_sad_u8_sum(s);
}
SIMD_INLINE c_ssd256_internal v256_ssd_u8_init(void) {
  return c_v256_ssd_u8_init();
}
SIMD_INLINE c_ssd256_internal v256_ssd_u8(c_ssd256_internal s, v256 a, v256 b) {
  return c_v256_ssd_u8(s, a, b);
}
SIMD_INLINE uint32_t v256_ssd_u8_sum(c_ssd256_internal s) {
  return c_v256_ssd_u8_sum(s);
}

SIMD_INLINE c_ssd256_internal_s16 v256_ssd_s16_init(void) {
  return c_v256_ssd_s16_init();
}
SIMD_INLINE c_ssd256_internal_s16 v256_ssd_s16(c_ssd256_internal_s16 s, v256 a,
                                               v256 b) {
  return c_v256_ssd_s16(s, a, b);
}
SIMD_INLINE uint64_t v256_ssd_s16_sum(c_ssd256_internal_s16 s) {
  return c_v256_ssd_s16_sum(s);
}

SIMD_INLINE int64_t v256_dotp_su8(v256 a, v256 b) {
  return c_v256_dotp_su8(a, b);
}
SIMD_INLINE int64_t v256_dotp_s16(v256 a, v256 b) {
  return c_v256_dotp_s16(a, b);
}
SIMD_INLINE int64_t v256_dotp_s32(v256 a, v256 b) {
  return c_v256_dotp_s32(a, b);
}
SIMD_INLINE uint64_t v256_hadd_u8(v256 a) { return c_v256_hadd_u8(a); }

SIMD_INLINE v256 v256_or(v256 a, v256 b) { return c_v256_or(a, b); }
SIMD_INLINE v256 v256_xor(v256 a, v256 b) { return c_v256_xor(a, b); }
SIMD_INLINE v256 v256_and(v256 a, v256 b) { return c_v256_and(a, b); }
SIMD_INLINE v256 v256_andn(v256 a, v256 b) { return c_v256_andn(a, b); }

SIMD_INLINE v256 v256_add_8(v256 a, v256 b) { return c_v256_add_8(a, b); }
SIMD_INLINE v256 v256_add_16(v256 a, v256 b) { return c_v256_add_16(a, b); }
SIMD_INLINE v256 v256_sadd_s8(v256 a, v256 b) { return c_v256_sadd_s8(a, b); }
SIMD_INLINE v256 v256_sadd_u8(v256 a, v256 b) { return c_v256_sadd_u8(a, b); }
SIMD_INLINE v256 v256_sadd_s16(v256 a, v256 b) { return c_v256_sadd_s16(a, b); }
SIMD_INLINE v256 v256_add_32(v256 a, v256 b) { return c_v256_add_32(a, b); }
SIMD_INLINE v256 v256_add_64(v256 a, v256 b) { return c_v256_add_64(a, b); }
SIMD_INLINE v256 v256_sub_64(v256 a, v256 b) { return c_v256_sub_64(a, b); }
SIMD_INLINE v256 v256_padd_u8(v256 a) { return c_v256_padd_u8(a); }
SIMD_INLINE v256 v256_padd_s16(v256 a) { return c_v256_padd_s16(a); }
SIMD_INLINE v256 v256_sub_8(v256 a, v256 b) { return c_v256_sub_8(a, b); }
SIMD_INLINE v256 v256_ssub_u8(v256 a, v256 b) { return c_v256_ssub_u8(a, b); }
SIMD_INLINE v256 v256_ssub_s8(v256 a, v256 b) { return c_v256_ssub_s8(a, b); }
SIMD_INLINE v256 v256_sub_16(v256 a, v256 b) { return c_v256_sub_16(a, b); }
SIMD_INLINE v256 v256_ssub_s16(v256 a, v256 b) { return c_v256_ssub_s16(a, b); }
SIMD_INLINE v256 v256_ssub_u16(v256 a, v256 b) { return c_v256_ssub_u16(a, b); }
SIMD_INLINE v256 v256_sub_32(v256 a, v256 b) { return c_v256_sub_32(a, b); }
SIMD_INLINE v256 v256_abs_s16(v256 a) { return c_v256_abs_s16(a); }
SIMD_INLINE v256 v256_abs_s8(v256 a) { return c_v256_abs_s8(a); }

SIMD_INLINE v256 v256_mul_s16(v128 a, v128 b) { return c_v256_mul_s16(a, b); }
SIMD_INLINE v256 v256_mullo_s16(v256 a, v256 b) {
  return c_v256_mullo_s16(a, b);
}
SIMD_INLINE v256 v256_mulhi_s16(v256 a, v256 b) {
  return c_v256_mulhi_s16(a, b);
}
SIMD_INLINE v256 v256_mullo_s32(v256 a, v256 b) {
  return c_v256_mullo_s32(a, b);
}
SIMD_INLINE v256 v256_madd_s16(v256 a, v256 b) { return c_v256_madd_s16(a, b); }
SIMD_INLINE v256 v256_madd_us8(v256 a, v256 b) { return c_v256_madd_us8(a, b); }

SIMD_INLINE uint32_t v256_movemask_8(v256 a) { return c_v256_movemask_8(a); }
SIMD_INLINE v256 v256_blend_8(v256 a, v256 b, v256 c) {
  return c_v256_blend_8(a, b, c);
}

SIMD_INLINE v256 v256_avg_u8(v256 a, v256 b) { return c_v256_avg_u8(a, b); }
SIMD_INLINE v256 v256_rdavg_u8(v256 a, v256 b) { return c_v256_rdavg_u8(a, b); }
SIMD_INLINE v256 v256_rdavg_u16(v256 a, v256 b) {
  return c_v256_rdavg_u16(a, b);
}
SIMD_INLINE v256 v256_avg_u16(v256 a, v256 b) { return c_v256_avg_u16(a, b); }
SIMD_INLINE v256 v256_min_u8(v256 a, v256 b) { return c_v256_min_u8(a, b); }
SIMD_INLINE v256 v256_max_u8(v256 a, v256 b) { return c_v256_max_u8(a, b); }
SIMD_INLINE v256 v256_min_s8(v256 a, v256 b) { return c_v256_min_s8(a, b); }
SIMD_INLINE v256 v256_max_s8(v256 a, v256 b) { return c_v256_max_s8(a, b); }
SIMD_INLINE v256 v256_min_s16(v256 a, v256 b) { return c_v256_min_s16(a, b); }
SIMD_INLINE v256 v256_max_s16(v256 a, v256 b) { return c_v256_max_s16(a, b); }
SIMD_INLINE v256 v256_min_s32(v256 a, v256 b) { return c_v256_min_s32(a, b); }
SIMD_INLINE v256 v256_max_s32(v256 a, v256 b) { return c_v256_max_s32(a, b); }

SIMD_INLINE v256 v256_ziplo_8(v256 a, v256 b) { return c_v256_ziplo_8(a, b); }
SIMD_INLINE v256 v256_ziphi_8(v256 a, v256 b) { return c_v256_ziphi_8(a, b); }
SIMD_INLINE v256 v256_ziplo_16(v256 a, v256 b) { return c_v256_ziplo_16(a, b); }
SIMD_INLINE v256 v256_ziphi_16(v256 a, v256 b) { return c_v256_ziphi_16(a, b); }
SIMD_INLINE v256 v256_ziplo_32(v256 a, v256 b) { return c_v256_ziplo_32(a, b); }
SIMD_INLINE v256 v256_ziphi_32(v256 a, v256 b) { return c_v256_ziphi_32(a, b); }
SIMD_INLINE v256 v256_ziplo_64(v256 a, v256 b) { return c_v256_ziplo_64(a, b); }
SIMD_INLINE v256 v256_ziphi_64(v256 a, v256 b) { return c_v256_ziphi_64(a, b); }
SIMD_INLINE v256 v256_ziplo_128(v256 a, v256 b) {
  return c_v256_ziplo_128(a, b);
}
SIMD_INLINE v256 v256_ziphi_128(v256 a, v256 b) {
  return c_v256_ziphi_128(a, b);
}
SIMD_INLINE v256 v256_zip_8(v128 a, v128 b) { return c_v256_zip_8(a, b); }
SIMD_INLINE v256 v256_zip_16(v128 a, v128 b) { return c_v256_zip_16(a, b); }
SIMD_INLINE v256 v256_zip_32(v128 a, v128 b) { return c_v256_zip_32(a, b); }
SIMD_INLINE v256 v256_unziplo_8(v256 a, v256 b) {
  return c_v256_unziplo_8(a, b);
}
SIMD_INLINE v256 v256_unziphi_8(v256 a, v256 b) {
  return c_v256_unziphi_8(a, b);
}
SIMD_INLINE v256 v256_unziplo_16(v256 a, v256 b) {
  return c_v256_unziplo_16(a, b);
}
SIMD_INLINE v256 v256_unziphi_16(v256 a, v256 b) {
  return c_v256_unziphi_16(a, b);
}
SIMD_INLINE v256 v256_unziplo_32(v256 a, v256 b) {
  return c_v256_unziplo_32(a, b);
}
SIMD_INLINE v256 v256_unziphi_32(v256 a, v256 b) {
  return c_v256_unziphi_32(a, b);
}
SIMD_INLINE v256 v256_unziplo_64(v256 a, v256 b) {
  return c_v256_unziplo_64(a, b);
}
SIMD_INLINE v256 v256_unziphi_64(v256 a, v256 b) {
  return c_v256_unziphi_64(a, b);
}
SIMD_INLINE v256 v256_unpack_u8_s16(v128 a) { return c_v256_unpack_u8_s16(a); }
SIMD_INLINE v256 v256_unpacklo_u8_s16(v256 a) {
  return c_v256_unpacklo_u8_s16(a);
}
SIMD_INLINE v256 v256_unpackhi_u8_s16(v256 a) {
  return c_v256_unpackhi_u8_s16(a);
}
SIMD_INLINE v256 v256_unpack_s8_s16(v128 a) { return c_v256_unpack_s8_s16(a); }
SIMD_INLINE v256 v256_unpacklo_s8_s16(v256 a) {
  return c_v256_unpacklo_s8_s16(a);
}
SIMD_INLINE v256 v256_unpackhi_s8_s16(v256 a) {
  return c_v256_unpackhi_s8_s16(a);
}
SIMD_INLINE v256 v256_pack_s32_s16(v256 a, v256 b) {
  return c_v256_pack_s32_s16(a, b);
}
SIMD_INLINE v256 v256_pack_s32_u16(v256 a, v256 b) {
  return c_v256_pack_s32_u16(a, b);
}
SIMD_INLINE v256 v256_pack_s16_u8(v256 a, v256 b) {
  return c_v256_pack_s16_u8(a, b);
}
SIMD_INLINE v256 v256_pack_s16_s8(v256 a, v256 b) {
  return c_v256_pack_s16_s8(a, b);
}
SIMD_INLINE v256 v256_unpack_u16_s32(v128 a) {
  return c_v256_unpack_u16_s32(a);
}
SIMD_INLINE v256 v256_unpack_s16_s32(v128 a) {
  return c_v256_unpack_s16_s32(a);
}
SIMD_INLINE v256 v256_unpacklo_u16_s32(v256 a) {
  return c_v256_unpacklo_u16_s32(a);
}
SIMD_INLINE v256 v256_unpacklo_s16_s32(v256 a) {
  return c_v256_unpacklo_s16_s32(a);
}
SIMD_INLINE v256 v256_unpackhi_u16_s32(v256 a) {
  return c_v256_unpackhi_u16_s32(a);
}
SIMD_INLINE v256 v256_unpackhi_s16_s32(v256 a) {
  return c_v256_unpackhi_s16_s32(a);
}
SIMD_INLINE v256 v256_shuffle_8(v256 a, v256 pattern) {
  return c_v256_shuffle_8(a, pattern);
}
SIMD_INLINE v256 v256_wideshuffle_8(v256 a, v256 b, v256 pattern) {
  return c_v256_wideshuffle_8(a, b, pattern);
}
SIMD_INLINE v256 v256_pshuffle_8(v256 a, v256 pattern) {
  return c_v256_pshuffle_8(a, pattern);
}

SIMD_INLINE v256 v256_cmpgt_s8(v256 a, v256 b) { return c_v256_cmpgt_s8(a, b); }
SIMD_INLINE v256 v256_cmplt_s8(v256 a, v256 b) { return c_v256_cmplt_s8(a, b); }
SIMD_INLINE v256 v256_cmpeq_8(v256 a, v256 b) { return c_v256_cmpeq_8(a, b); }
SIMD_INLINE v256 v256_cmpgt_s16(v256 a, v256 b) {
  return c_v256_cmpgt_s16(a, b);
}
SIMD_INLINE v256 v256_cmplt_s16(v256 a, v256 b) {
  return c_v256_cmplt_s16(a, b);
}
SIMD_INLINE v256 v256_cmpeq_16(v256 a, v256 b) { return c_v256_cmpeq_16(a, b); }
SIMD_INLINE v256 v256_cmpeq_32(v256 a, v256 b) { return c_v256_cmpeq_32(a, b); }

SIMD_INLINE v256 v256_cmpgt_s32(v256 a, v256 b) {
  return c_v256_cmpgt_s32(a, b);
}
SIMD_INLINE v256 v256_cmplt_s32(v256 a, v256 b) {
  return c_v256_cmplt_s32(a, b);
}
SIMD_INLINE v256 v256_shl_8(v256 a, unsigned int c) {
  return c_v256_shl_8(a, c);
}
SIMD_INLINE v256 v256_shr_u8(v256 a, unsigned int c) {
  return c_v256_shr_u8(a, c);
}
SIMD_INLINE v256 v256_shr_s8(v256 a, unsigned int c) {
  return c_v256_shr_s8(a, c);
}
SIMD_INLINE v256 v256_shl_16(v256 a, unsigned int c) {
  return c_v256_shl_16(a, c);
}
SIMD_INLINE v256 v256_shr_u16(v256 a, unsigned int c) {
  return c_v256_shr_u16(a, c);
}
SIMD_INLINE v256 v256_shr_s16(v256 a, unsigned int c) {
  return c_v256_shr_s16(a, c);
}
SIMD_INLINE v256 v256_shl_32(v256 a, unsigned int c) {
  return c_v256_shl_32(a, c);
}
SIMD_INLINE v256 v256_shr_u32(v256 a, unsigned int c) {
  return c_v256_shr_u32(a, c);
}
SIMD_INLINE v256 v256_shr_s32(v256 a, unsigned int c) {
  return c_v256_shr_s32(a, c);
}
SIMD_INLINE v256 v256_shl_64(v256 a, unsigned int c) {
  return c_v256_shl_64(a, c);
}
SIMD_INLINE v256 v256_shr_u64(v256 a, unsigned int c) {
  return c_v256_shr_u64(a, c);
}
SIMD_INLINE v256 v256_shr_s64(v256 a, unsigned int c) {
  return c_v256_shr_s64(a, c);
}

SIMD_INLINE v256 v256_shr_n_byte(v256 a, unsigned int n) {
  return c_v256_shr_n_byte(a, n);
}
SIMD_INLINE v256 v256_shl_n_byte(v256 a, unsigned int n) {
  return c_v256_shl_n_byte(a, n);
}
SIMD_INLINE v256 v256_shl_n_8(v256 a, unsigned int n) {
  return c_v256_shl_n_8(a, n);
}
SIMD_INLINE v256 v256_shl_n_16(v256 a, unsigned int n) {
  return c_v256_shl_n_16(a, n);
}
SIMD_INLINE v256 v256_shl_n_32(v256 a, unsigned int n) {
  return c_v256_shl_n_32(a, n);
}
SIMD_INLINE v256 v256_shl_n_64(v256 a, unsigned int n) {
  return c_v256_shl_n_64(a, n);
}
SIMD_INLINE v256 v256_shr_n_u8(v256 a, unsigned int n) {
  return c_v256_shr_n_u8(a, n);
}
SIMD_INLINE v256 v256_shr_n_u16(v256 a, unsigned int n) {
  return c_v256_shr_n_u16(a, n);
}
SIMD_INLINE v256 v256_shr_n_u32(v256 a, unsigned int n) {
  return c_v256_shr_n_u32(a, n);
}
SIMD_INLINE v256 v256_shr_n_u64(v256 a, unsigned int n) {
  return c_v256_shr_n_u64(a, n);
}
SIMD_INLINE v256 v256_shr_n_s8(v256 a, unsigned int n) {
  return c_v256_shr_n_s8(a, n);
}
SIMD_INLINE v256 v256_shr_n_s16(v256 a, unsigned int n) {
  return c_v256_shr_n_s16(a, n);
}
SIMD_INLINE v256 v256_shr_n_s32(v256 a, unsigned int n) {
  return c_v256_shr_n_s32(a, n);
}
SIMD_INLINE v256 v256_shr_n_s64(v256 a, unsigned int n) {
  return c_v256_shr_n_s64(a, n);
}

SIMD_INLINE v256 v256_shr_n_word(v256 a, unsigned int n) {
  return c_v256_shr_n_word(a, n);
}
SIMD_INLINE v256 v256_shl_n_word(v256 a, unsigned int n) {
  return c_v256_shl_n_word(a, n);
}

typedef uint32_t sad256_internal_u16;
SIMD_INLINE sad256_internal_u16 v256_sad_u16_init(void) {
  return c_v256_sad_u16_init();
}
SIMD_INLINE sad256_internal_u16 v256_sad_u16(sad256_internal_u16 s, v256 a,
                                             v256 b) {
  return c_v256_sad_u16(s, a, b);
}
SIMD_INLINE uint32_t v256_sad_u16_sum(sad256_internal_u16 s) {
  return c_v256_sad_u16_sum(s);
}

#endif  // AOM_AOM_DSP_SIMD_V256_INTRINSICS_H_
