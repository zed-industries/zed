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

#ifndef AOM_AOM_DSP_SIMD_V64_INTRINSICS_H_
#define AOM_AOM_DSP_SIMD_V64_INTRINSICS_H_

#include <stdio.h>
#include <stdlib.h>

#include "aom_dsp/simd/v64_intrinsics_c.h"

/* Fallback to plain, unoptimised C. */

typedef c_v64 v64;

SIMD_INLINE uint32_t v64_low_u32(v64 a) { return c_v64_low_u32(a); }
SIMD_INLINE uint32_t v64_high_u32(v64 a) { return c_v64_high_u32(a); }
SIMD_INLINE int32_t v64_low_s32(v64 a) { return c_v64_low_s32(a); }
SIMD_INLINE int32_t v64_high_s32(v64 a) { return c_v64_high_s32(a); }
SIMD_INLINE v64 v64_from_32(uint32_t x, uint32_t y) {
  return c_v64_from_32(x, y);
}
SIMD_INLINE v64 v64_from_64(uint64_t x) { return c_v64_from_64(x); }
SIMD_INLINE uint64_t v64_u64(v64 x) { return c_v64_u64(x); }
SIMD_INLINE v64 v64_from_16(uint16_t a, uint16_t b, uint16_t c, uint16_t d) {
  return c_v64_from_16(a, b, c, d);
}

SIMD_INLINE uint32_t u32_load_unaligned(const void *p) {
  return c_u32_load_unaligned(p);
}
SIMD_INLINE uint32_t u32_load_aligned(const void *p) {
  return c_u32_load_aligned(p);
}
SIMD_INLINE void u32_store_unaligned(void *p, uint32_t a) {
  c_u32_store_unaligned(p, a);
}
SIMD_INLINE void u32_store_aligned(void *p, uint32_t a) {
  c_u32_store_aligned(p, a);
}

SIMD_INLINE v64 v64_load_unaligned(const void *p) {
  return c_v64_load_unaligned(p);
}
SIMD_INLINE v64 v64_load_aligned(const void *p) {
  return c_v64_load_aligned(p);
}

SIMD_INLINE void v64_store_unaligned(void *p, v64 a) {
  c_v64_store_unaligned(p, a);
}
SIMD_INLINE void v64_store_aligned(void *p, v64 a) {
  c_v64_store_aligned(p, a);
}

SIMD_INLINE v64 v64_align(v64 a, v64 b, unsigned int c) {
  return c_v64_align(a, b, c);
}

SIMD_INLINE v64 v64_zero(void) { return c_v64_zero(); }
SIMD_INLINE v64 v64_dup_8(uint8_t x) { return c_v64_dup_8(x); }
SIMD_INLINE v64 v64_dup_16(uint16_t x) { return c_v64_dup_16(x); }
SIMD_INLINE v64 v64_dup_32(uint32_t x) { return c_v64_dup_32(x); }

SIMD_INLINE v64 v64_add_8(v64 a, v64 b) { return c_v64_add_8(a, b); }
SIMD_INLINE v64 v64_add_16(v64 a, v64 b) { return c_v64_add_16(a, b); }
SIMD_INLINE v64 v64_sadd_u8(v64 a, v64 b) { return c_v64_sadd_u8(a, b); }
SIMD_INLINE v64 v64_sadd_s8(v64 a, v64 b) { return c_v64_sadd_s8(a, b); }
SIMD_INLINE v64 v64_sadd_s16(v64 a, v64 b) { return c_v64_sadd_s16(a, b); }
SIMD_INLINE v64 v64_add_32(v64 a, v64 b) { return c_v64_add_32(a, b); }
SIMD_INLINE v64 v64_sub_8(v64 a, v64 b) { return c_v64_sub_8(a, b); }
SIMD_INLINE v64 v64_ssub_u8(v64 a, v64 b) { return c_v64_ssub_u8(a, b); }
SIMD_INLINE v64 v64_ssub_s8(v64 a, v64 b) { return c_v64_ssub_s8(a, b); }
SIMD_INLINE v64 v64_sub_16(v64 a, v64 b) { return c_v64_sub_16(a, b); }
SIMD_INLINE v64 v64_ssub_s16(v64 a, v64 b) { return c_v64_ssub_s16(a, b); }
SIMD_INLINE v64 v64_ssub_u16(v64 a, v64 b) { return c_v64_ssub_u16(a, b); }
SIMD_INLINE v64 v64_sub_32(v64 a, v64 b) { return c_v64_sub_32(a, b); }
SIMD_INLINE v64 v64_abs_s16(v64 a) { return c_v64_abs_s16(a); }
SIMD_INLINE v64 v64_abs_s8(v64 a) { return c_v64_abs_s8(a); }

SIMD_INLINE v64 v64_ziplo_8(v64 a, v64 b) { return c_v64_ziplo_8(a, b); }
SIMD_INLINE v64 v64_ziphi_8(v64 a, v64 b) { return c_v64_ziphi_8(a, b); }
SIMD_INLINE v64 v64_ziplo_16(v64 a, v64 b) { return c_v64_ziplo_16(a, b); }
SIMD_INLINE v64 v64_ziphi_16(v64 a, v64 b) { return c_v64_ziphi_16(a, b); }
SIMD_INLINE v64 v64_ziplo_32(v64 a, v64 b) { return c_v64_ziplo_32(a, b); }
SIMD_INLINE v64 v64_ziphi_32(v64 a, v64 b) { return c_v64_ziphi_32(a, b); }
SIMD_INLINE v64 v64_unziplo_8(v64 a, v64 b) { return c_v64_unziplo_8(a, b); }
SIMD_INLINE v64 v64_unziphi_8(v64 a, v64 b) { return c_v64_unziphi_8(a, b); }
SIMD_INLINE v64 v64_unziplo_16(v64 a, v64 b) { return c_v64_unziplo_16(a, b); }
SIMD_INLINE v64 v64_unziphi_16(v64 a, v64 b) { return c_v64_unziphi_16(a, b); }
SIMD_INLINE v64 v64_unpacklo_u8_s16(v64 a) { return c_v64_unpacklo_u8_s16(a); }
SIMD_INLINE v64 v64_unpackhi_u8_s16(v64 a) { return c_v64_unpackhi_u8_s16(a); }
SIMD_INLINE v64 v64_unpacklo_s8_s16(v64 a) { return c_v64_unpacklo_s8_s16(a); }
SIMD_INLINE v64 v64_unpackhi_s8_s16(v64 a) { return c_v64_unpackhi_s8_s16(a); }
SIMD_INLINE v64 v64_pack_s32_s16(v64 a, v64 b) {
  return c_v64_pack_s32_s16(a, b);
}
SIMD_INLINE v64 v64_pack_s32_u16(v64 a, v64 b) {
  return c_v64_pack_s32_u16(a, b);
}
SIMD_INLINE v64 v64_pack_s16_u8(v64 a, v64 b) {
  return c_v64_pack_s16_u8(a, b);
}
SIMD_INLINE v64 v64_pack_s16_s8(v64 a, v64 b) {
  return c_v64_pack_s16_s8(a, b);
}
SIMD_INLINE v64 v64_unpacklo_u16_s32(v64 a) {
  return c_v64_unpacklo_u16_s32(a);
}
SIMD_INLINE v64 v64_unpacklo_s16_s32(v64 a) {
  return c_v64_unpacklo_s16_s32(a);
}
SIMD_INLINE v64 v64_unpackhi_u16_s32(v64 a) {
  return c_v64_unpackhi_u16_s32(a);
}
SIMD_INLINE v64 v64_unpackhi_s16_s32(v64 a) {
  return c_v64_unpackhi_s16_s32(a);
}
SIMD_INLINE v64 v64_shuffle_8(v64 a, v64 pattern) {
  return c_v64_shuffle_8(a, pattern);
}

SIMD_INLINE c_sad64_internal v64_sad_u8_init(void) {
  return c_v64_sad_u8_init();
}
SIMD_INLINE c_sad64_internal v64_sad_u8(c_sad64_internal s, v64 a, v64 b) {
  return c_v64_sad_u8(s, a, b);
}
SIMD_INLINE uint32_t v64_sad_u8_sum(c_sad64_internal s) {
  return c_v64_sad_u8_sum(s);
}
SIMD_INLINE c_ssd64_internal v64_ssd_u8_init(void) {
  return c_v64_ssd_u8_init();
}
SIMD_INLINE c_ssd64_internal v64_ssd_u8(c_ssd64_internal s, v64 a, v64 b) {
  return c_v64_ssd_u8(s, a, b);
}
SIMD_INLINE uint32_t v64_ssd_u8_sum(c_ssd64_internal s) {
  return c_v64_ssd_u8_sum(s);
}
SIMD_INLINE int64_t v64_dotp_su8(v64 a, v64 b) { return c_v64_dotp_su8(a, b); }
SIMD_INLINE int64_t v64_dotp_s16(v64 a, v64 b) { return c_v64_dotp_s16(a, b); }
SIMD_INLINE uint64_t v64_hadd_u8(v64 a) { return c_v64_hadd_u8(a); }
SIMD_INLINE int64_t v64_hadd_s16(v64 a) { return c_v64_hadd_s16(a); }

SIMD_INLINE v64 v64_or(v64 a, v64 b) { return c_v64_or(a, b); }
SIMD_INLINE v64 v64_xor(v64 a, v64 b) { return c_v64_xor(a, b); }
SIMD_INLINE v64 v64_and(v64 a, v64 b) { return c_v64_and(a, b); }
SIMD_INLINE v64 v64_andn(v64 a, v64 b) { return c_v64_andn(a, b); }

SIMD_INLINE v64 v64_mullo_s16(v64 a, v64 b) { return c_v64_mullo_s16(a, b); }
SIMD_INLINE v64 v64_mulhi_s16(v64 a, v64 b) { return c_v64_mulhi_s16(a, b); }
SIMD_INLINE v64 v64_mullo_s32(v64 a, v64 b) { return c_v64_mullo_s32(a, b); }
SIMD_INLINE v64 v64_madd_s16(v64 a, v64 b) { return c_v64_madd_s16(a, b); }
SIMD_INLINE v64 v64_madd_us8(v64 a, v64 b) { return c_v64_madd_us8(a, b); }

SIMD_INLINE v64 v64_avg_u8(v64 a, v64 b) { return c_v64_avg_u8(a, b); }
SIMD_INLINE v64 v64_rdavg_u8(v64 a, v64 b) { return c_v64_rdavg_u8(a, b); }
SIMD_INLINE v64 v64_rdavg_u16(v64 a, v64 b) { return c_v64_rdavg_u16(a, b); }
SIMD_INLINE v64 v64_avg_u16(v64 a, v64 b) { return c_v64_avg_u16(a, b); }
SIMD_INLINE v64 v64_min_u8(v64 a, v64 b) { return c_v64_min_u8(a, b); }
SIMD_INLINE v64 v64_max_u8(v64 a, v64 b) { return c_v64_max_u8(a, b); }
SIMD_INLINE v64 v64_min_s8(v64 a, v64 b) { return c_v64_min_s8(a, b); }
SIMD_INLINE v64 v64_max_s8(v64 a, v64 b) { return c_v64_max_s8(a, b); }
SIMD_INLINE v64 v64_min_s16(v64 a, v64 b) { return c_v64_min_s16(a, b); }
SIMD_INLINE v64 v64_max_s16(v64 a, v64 b) { return c_v64_max_s16(a, b); }

SIMD_INLINE v64 v64_cmpgt_s8(v64 a, v64 b) { return c_v64_cmpgt_s8(a, b); }
SIMD_INLINE v64 v64_cmplt_s8(v64 a, v64 b) { return c_v64_cmplt_s8(a, b); }
SIMD_INLINE v64 v64_cmpeq_8(v64 a, v64 b) { return c_v64_cmpeq_8(a, b); }
SIMD_INLINE v64 v64_cmpgt_s16(v64 a, v64 b) { return c_v64_cmpgt_s16(a, b); }
SIMD_INLINE v64 v64_cmplt_s16(v64 a, v64 b) { return c_v64_cmplt_s16(a, b); }
SIMD_INLINE v64 v64_cmpeq_16(v64 a, v64 b) { return c_v64_cmpeq_16(a, b); }

SIMD_INLINE v64 v64_shl_8(v64 a, unsigned int n) { return c_v64_shl_8(a, n); }
SIMD_INLINE v64 v64_shr_u8(v64 a, unsigned int n) { return c_v64_shr_u8(a, n); }
SIMD_INLINE v64 v64_shr_s8(v64 a, unsigned int n) { return c_v64_shr_s8(a, n); }
SIMD_INLINE v64 v64_shl_16(v64 a, unsigned int n) { return c_v64_shl_16(a, n); }
SIMD_INLINE v64 v64_shr_u16(v64 a, unsigned int n) {
  return c_v64_shr_u16(a, n);
}
SIMD_INLINE v64 v64_shr_s16(v64 a, unsigned int n) {
  return c_v64_shr_s16(a, n);
}
SIMD_INLINE v64 v64_shl_32(v64 a, unsigned int n) { return c_v64_shl_32(a, n); }
SIMD_INLINE v64 v64_shr_u32(v64 a, unsigned int n) {
  return c_v64_shr_u32(a, n);
}
SIMD_INLINE v64 v64_shr_s32(v64 a, unsigned int n) {
  return c_v64_shr_s32(a, n);
}
SIMD_INLINE v64 v64_shr_n_byte(v64 a, unsigned int n) {
  return c_v64_shr_n_byte(a, n);
}
SIMD_INLINE v64 v64_shl_n_byte(v64 a, unsigned int n) {
  return c_v64_shl_n_byte(a, n);
}
SIMD_INLINE v64 v64_shl_n_8(v64 a, unsigned int c) {
  return c_v64_shl_n_8(a, c);
}
SIMD_INLINE v64 v64_shr_n_u8(v64 a, unsigned int c) {
  return c_v64_shr_n_u8(a, c);
}
SIMD_INLINE v64 v64_shr_n_s8(v64 a, unsigned int c) {
  return c_v64_shr_n_s8(a, c);
}
SIMD_INLINE v64 v64_shl_n_16(v64 a, unsigned int c) {
  return c_v64_shl_n_16(a, c);
}
SIMD_INLINE v64 v64_shr_n_u16(v64 a, unsigned int c) {
  return c_v64_shr_n_u16(a, c);
}
SIMD_INLINE v64 v64_shr_n_s16(v64 a, unsigned int c) {
  return c_v64_shr_n_s16(a, c);
}
SIMD_INLINE v64 v64_shl_n_32(v64 a, unsigned int c) {
  return c_v64_shl_n_32(a, c);
}
SIMD_INLINE v64 v64_shr_n_u32(v64 a, unsigned int c) {
  return c_v64_shr_n_u32(a, c);
}
SIMD_INLINE v64 v64_shr_n_s32(v64 a, unsigned int c) {
  return c_v64_shr_n_s32(a, c);
}

#endif  // AOM_AOM_DSP_SIMD_V64_INTRINSICS_H_
