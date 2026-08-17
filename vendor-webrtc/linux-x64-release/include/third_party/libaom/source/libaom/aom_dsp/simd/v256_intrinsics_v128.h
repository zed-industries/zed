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

#ifndef AOM_AOM_DSP_SIMD_V256_INTRINSICS_V128_H_
#define AOM_AOM_DSP_SIMD_V256_INTRINSICS_V128_H_

#include "config/aom_config.h"

#if HAVE_NEON
#error "Do not use this file for Neon"
#endif

#if HAVE_SSE2
#include "aom_dsp/simd/v128_intrinsics_x86.h"
#else
#include "aom_dsp/simd/v128_intrinsics.h"
#endif

typedef struct {
  v128 val[2];
} v256;

SIMD_INLINE uint32_t v256_low_u32(v256 a) { return v128_low_u32(a.val[0]); }

SIMD_INLINE v64 v256_low_v64(v256 a) { return v128_low_v64(a.val[0]); }

SIMD_INLINE uint64_t v256_low_u64(v256 a) { return v64_u64(v256_low_v64(a)); }

SIMD_INLINE v128 v256_low_v128(v256 a) { return a.val[0]; }

SIMD_INLINE v128 v256_high_v128(v256 a) { return a.val[1]; }

SIMD_INLINE v256 v256_from_v128(v128 hi, v128 lo) {
  v256 t;
  t.val[1] = hi;
  t.val[0] = lo;
  return t;
}

SIMD_INLINE v256 v256_from_64(uint64_t a, uint64_t b, uint64_t c, uint64_t d) {
  return v256_from_v128(v128_from_64(a, b), v128_from_64(c, d));
}

SIMD_INLINE v256 v256_from_v64(v64 a, v64 b, v64 c, v64 d) {
  return v256_from_v128(v128_from_v64(a, b), v128_from_v64(c, d));
}

SIMD_INLINE v256 v256_load_unaligned(const void *p) {
  return v256_from_v128(v128_load_unaligned((uint8_t *)p + 16),
                        v128_load_unaligned(p));
}

SIMD_INLINE v256 v256_load_aligned(const void *p) {
  return v256_from_v128(v128_load_aligned((uint8_t *)p + 16),
                        v128_load_aligned(p));
}

SIMD_INLINE void v256_store_unaligned(void *p, v256 a) {
  v128_store_unaligned(p, a.val[0]);
  v128_store_unaligned((uint8_t *)p + 16, a.val[1]);
}

SIMD_INLINE void v256_store_aligned(void *p, v256 a) {
  v128_store_aligned(p, a.val[0]);
  v128_store_aligned((uint8_t *)p + 16, a.val[1]);
}

SIMD_INLINE v256 v256_zero(void) {
  return v256_from_v128(v128_zero(), v128_zero());
}

SIMD_INLINE v256 v256_dup_8(uint8_t x) {
  v128 t = v128_dup_8(x);
  return v256_from_v128(t, t);
}

SIMD_INLINE v256 v256_dup_16(uint16_t x) {
  v128 t = v128_dup_16(x);
  return v256_from_v128(t, t);
}

SIMD_INLINE v256 v256_dup_32(uint32_t x) {
  v128 t = v128_dup_32(x);
  return v256_from_v128(t, t);
}

SIMD_INLINE v256 v256_dup_64(uint64_t x) {
  v128 t = v128_dup_64(x);
  return v256_from_v128(t, t);
}

SIMD_INLINE int64_t v256_dotp_su8(v256 a, v256 b) {
  return v128_dotp_su8(a.val[1], b.val[1]) + v128_dotp_su8(a.val[0], b.val[0]);
}

SIMD_INLINE int64_t v256_dotp_s16(v256 a, v256 b) {
  return v128_dotp_s16(a.val[1], b.val[1]) + v128_dotp_s16(a.val[0], b.val[0]);
}

SIMD_INLINE int64_t v256_dotp_s32(v256 a, v256 b) {
  return v128_dotp_s32(a.val[1], b.val[1]) + v128_dotp_s32(a.val[0], b.val[0]);
}

SIMD_INLINE uint64_t v256_hadd_u8(v256 a) {
  return v128_hadd_u8(a.val[1]) + v128_hadd_u8(a.val[0]);
}

typedef struct {
  sad128_internal val[2];
} sad256_internal;

SIMD_INLINE sad256_internal v256_sad_u8_init(void) {
  sad256_internal t;
  t.val[1] = v128_sad_u8_init();
  t.val[0] = v128_sad_u8_init();
  return t;
}

/* Implementation dependent return value.  Result must be finalised with
   v256_sad_u8_sum().
   The result for more than 16 v256_sad_u8() calls is undefined. */
SIMD_INLINE sad256_internal v256_sad_u8(sad256_internal s, v256 a, v256 b) {
  sad256_internal t;
  t.val[1] = v128_sad_u8(s.val[1], a.val[1], b.val[1]);
  t.val[0] = v128_sad_u8(s.val[0], a.val[0], b.val[0]);
  return t;
}

SIMD_INLINE uint32_t v256_sad_u8_sum(sad256_internal s) {
  return v128_sad_u8_sum(s.val[1]) + v128_sad_u8_sum(s.val[0]);
}

typedef struct {
  ssd128_internal val[2];
} ssd256_internal;

SIMD_INLINE ssd256_internal v256_ssd_u8_init(void) {
  ssd256_internal t;
  t.val[1] = v128_ssd_u8_init();
  t.val[0] = v128_ssd_u8_init();
  return t;
}

/* Implementation dependent return value.  Result must be finalised with
 * v256_ssd_u8_sum(). */
SIMD_INLINE ssd256_internal v256_ssd_u8(ssd256_internal s, v256 a, v256 b) {
  ssd256_internal t;
  t.val[1] = v128_ssd_u8(s.val[1], a.val[1], b.val[1]);
  t.val[0] = v128_ssd_u8(s.val[0], a.val[0], b.val[0]);
  return t;
}

SIMD_INLINE uint32_t v256_ssd_u8_sum(ssd256_internal s) {
  return v128_ssd_u8_sum(s.val[1]) + v128_ssd_u8_sum(s.val[0]);
}

SIMD_INLINE v256 v256_or(v256 a, v256 b) {
  return v256_from_v128(v128_or(a.val[1], b.val[1]),
                        v128_or(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_xor(v256 a, v256 b) {
  return v256_from_v128(v128_xor(a.val[1], b.val[1]),
                        v128_xor(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_and(v256 a, v256 b) {
  return v256_from_v128(v128_and(a.val[1], b.val[1]),
                        v128_and(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_andn(v256 a, v256 b) {
  return v256_from_v128(v128_andn(a.val[1], b.val[1]),
                        v128_andn(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_add_8(v256 a, v256 b) {
  return v256_from_v128(v128_add_8(a.val[1], b.val[1]),
                        v128_add_8(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_add_16(v256 a, v256 b) {
  return v256_from_v128(v128_add_16(a.val[1], b.val[1]),
                        v128_add_16(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_sadd_s8(v256 a, v256 b) {
  return v256_from_v128(v128_sadd_s8(a.val[1], b.val[1]),
                        v128_sadd_s8(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_sadd_u8(v256 a, v256 b) {
  return v256_from_v128(v128_sadd_u8(a.val[1], b.val[1]),
                        v128_sadd_u8(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_sadd_s16(v256 a, v256 b) {
  return v256_from_v128(v128_sadd_s16(a.val[1], b.val[1]),
                        v128_sadd_s16(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_add_32(v256 a, v256 b) {
  return v256_from_v128(v128_add_32(a.val[1], b.val[1]),
                        v128_add_32(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_add_64(v256 a, v256 b) {
  return v256_from_v128(v128_add_64(a.val[1], b.val[1]),
                        v128_add_64(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_padd_u8(v256 a) {
  return v256_from_v128(v128_padd_u8(a.val[1]), v128_padd_u8(a.val[0]));
}

SIMD_INLINE v256 v256_padd_s16(v256 a) {
  return v256_from_v128(v128_padd_s16(a.val[1]), v128_padd_s16(a.val[0]));
}

SIMD_INLINE v256 v256_sub_8(v256 a, v256 b) {
  return v256_from_v128(v128_sub_8(a.val[1], b.val[1]),
                        v128_sub_8(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_ssub_u8(v256 a, v256 b) {
  return v256_from_v128(v128_ssub_u8(a.val[1], b.val[1]),
                        v128_ssub_u8(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_ssub_s8(v256 a, v256 b) {
  return v256_from_v128(v128_ssub_s8(a.val[1], b.val[1]),
                        v128_ssub_s8(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_sub_16(v256 a, v256 b) {
  return v256_from_v128(v128_sub_16(a.val[1], b.val[1]),
                        v128_sub_16(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_ssub_s16(v256 a, v256 b) {
  return v256_from_v128(v128_ssub_s16(a.val[1], b.val[1]),
                        v128_ssub_s16(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_ssub_u16(v256 a, v256 b) {
  return v256_from_v128(v128_ssub_u16(a.val[1], b.val[1]),
                        v128_ssub_u16(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_sub_32(v256 a, v256 b) {
  return v256_from_v128(v128_sub_32(a.val[1], b.val[1]),
                        v128_sub_32(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_sub_64(v256 a, v256 b) {
  return v256_from_v128(v128_sub_64(a.val[1], b.val[1]),
                        v128_sub_64(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_abs_s16(v256 a) {
  return v256_from_v128(v128_abs_s16(a.val[1]), v128_abs_s16(a.val[0]));
}

SIMD_INLINE v256 v256_abs_s8(v256 a) {
  return v256_from_v128(v128_abs_s8(a.val[1]), v128_abs_s8(a.val[0]));
}

SIMD_INLINE v256 v256_mul_s16(v128 a, v128 b) {
  v128 lo_bits = v128_mullo_s16(a, b);
  v128 hi_bits = v128_mulhi_s16(a, b);
  return v256_from_v128(v128_ziphi_16(hi_bits, lo_bits),
                        v128_ziplo_16(hi_bits, lo_bits));
}

SIMD_INLINE v256 v256_mullo_s16(v256 a, v256 b) {
  return v256_from_v128(v128_mullo_s16(a.val[1], b.val[1]),
                        v128_mullo_s16(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_mulhi_s16(v256 a, v256 b) {
  return v256_from_v128(v128_mulhi_s16(a.val[1], b.val[1]),
                        v128_mulhi_s16(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_mullo_s32(v256 a, v256 b) {
  return v256_from_v128(v128_mullo_s32(a.val[1], b.val[1]),
                        v128_mullo_s32(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_madd_s16(v256 a, v256 b) {
  return v256_from_v128(v128_madd_s16(a.val[1], b.val[1]),
                        v128_madd_s16(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_madd_us8(v256 a, v256 b) {
  return v256_from_v128(v128_madd_us8(a.val[1], b.val[1]),
                        v128_madd_us8(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_avg_u8(v256 a, v256 b) {
  return v256_from_v128(v128_avg_u8(a.val[1], b.val[1]),
                        v128_avg_u8(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_rdavg_u8(v256 a, v256 b) {
  return v256_from_v128(v128_rdavg_u8(a.val[1], b.val[1]),
                        v128_rdavg_u8(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_rdavg_u16(v256 a, v256 b) {
  return v256_from_v128(v128_rdavg_u16(a.val[1], b.val[1]),
                        v128_rdavg_u16(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_avg_u16(v256 a, v256 b) {
  return v256_from_v128(v128_avg_u16(a.val[1], b.val[1]),
                        v128_avg_u16(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_min_u8(v256 a, v256 b) {
  return v256_from_v128(v128_min_u8(a.val[1], b.val[1]),
                        v128_min_u8(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_max_u8(v256 a, v256 b) {
  return v256_from_v128(v128_max_u8(a.val[1], b.val[1]),
                        v128_max_u8(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_min_s8(v256 a, v256 b) {
  return v256_from_v128(v128_min_s8(a.val[1], b.val[1]),
                        v128_min_s8(a.val[0], b.val[0]));
}

SIMD_INLINE uint32_t v256_movemask_8(v256 a) {
  return (v128_movemask_8(v256_high_v128(a)) << 16) |
         v128_movemask_8(v256_low_v128(a));
}

SIMD_INLINE v256 v256_blend_8(v256 a, v256 b, v256 c) {
  return v256_from_v128(v128_blend_8(a.val[1], b.val[1], c.val[1]),
                        v128_blend_8(a.val[0], b.val[0], c.val[0]));
}

SIMD_INLINE v256 v256_max_s8(v256 a, v256 b) {
  return v256_from_v128(v128_max_s8(a.val[1], b.val[1]),
                        v128_max_s8(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_min_s16(v256 a, v256 b) {
  return v256_from_v128(v128_min_s16(a.val[1], b.val[1]),
                        v128_min_s16(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_max_s16(v256 a, v256 b) {
  return v256_from_v128(v128_max_s16(a.val[1], b.val[1]),
                        v128_max_s16(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_min_s32(v256 a, v256 b) {
  return v256_from_v128(v128_min_s32(a.val[1], b.val[1]),
                        v128_min_s32(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_max_s32(v256 a, v256 b) {
  return v256_from_v128(v128_max_s32(a.val[1], b.val[1]),
                        v128_max_s32(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_ziplo_8(v256 a, v256 b) {
  return v256_from_v128(v128_ziphi_8(a.val[0], b.val[0]),
                        v128_ziplo_8(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_ziphi_8(v256 a, v256 b) {
  return v256_from_v128(v128_ziphi_8(a.val[1], b.val[1]),
                        v128_ziplo_8(a.val[1], b.val[1]));
}

SIMD_INLINE v256 v256_ziplo_16(v256 a, v256 b) {
  return v256_from_v128(v128_ziphi_16(a.val[0], b.val[0]),
                        v128_ziplo_16(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_ziphi_16(v256 a, v256 b) {
  return v256_from_v128(v128_ziphi_16(a.val[1], b.val[1]),
                        v128_ziplo_16(a.val[1], b.val[1]));
}

SIMD_INLINE v256 v256_ziplo_32(v256 a, v256 b) {
  return v256_from_v128(v128_ziphi_32(a.val[0], b.val[0]),
                        v128_ziplo_32(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_ziphi_32(v256 a, v256 b) {
  return v256_from_v128(v128_ziphi_32(a.val[1], b.val[1]),
                        v128_ziplo_32(a.val[1], b.val[1]));
}

SIMD_INLINE v256 v256_ziplo_64(v256 a, v256 b) {
  return v256_from_v128(v128_ziphi_64(a.val[0], b.val[0]),
                        v128_ziplo_64(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_ziphi_64(v256 a, v256 b) {
  return v256_from_v128(v128_ziphi_64(a.val[1], b.val[1]),
                        v128_ziplo_64(a.val[1], b.val[1]));
}

SIMD_INLINE v256 v256_ziplo_128(v256 a, v256 b) {
  return v256_from_v128(a.val[0], b.val[0]);
}

SIMD_INLINE v256 v256_ziphi_128(v256 a, v256 b) {
  return v256_from_v128(a.val[1], b.val[1]);
}

SIMD_INLINE v256 v256_zip_8(v128 a, v128 b) {
  return v256_from_v128(v128_ziphi_8(a, b), v128_ziplo_8(a, b));
}

SIMD_INLINE v256 v256_zip_16(v128 a, v128 b) {
  return v256_from_v128(v128_ziphi_16(a, b), v128_ziplo_16(a, b));
}

SIMD_INLINE v256 v256_zip_32(v128 a, v128 b) {
  return v256_from_v128(v128_ziphi_32(a, b), v128_ziplo_32(a, b));
}

SIMD_INLINE v256 v256_unziplo_8(v256 a, v256 b) {
  return v256_from_v128(v128_unziplo_8(a.val[1], a.val[0]),
                        v128_unziplo_8(b.val[1], b.val[0]));
}

SIMD_INLINE v256 v256_unziphi_8(v256 a, v256 b) {
  return v256_from_v128(v128_unziphi_8(a.val[1], a.val[0]),
                        v128_unziphi_8(b.val[1], b.val[0]));
}

SIMD_INLINE v256 v256_unziplo_16(v256 a, v256 b) {
  return v256_from_v128(v128_unziplo_16(a.val[1], a.val[0]),
                        v128_unziplo_16(b.val[1], b.val[0]));
}

SIMD_INLINE v256 v256_unziphi_16(v256 a, v256 b) {
  return v256_from_v128(v128_unziphi_16(a.val[1], a.val[0]),
                        v128_unziphi_16(b.val[1], b.val[0]));
}

SIMD_INLINE v256 v256_unziplo_32(v256 a, v256 b) {
  return v256_from_v128(v128_unziplo_32(a.val[1], a.val[0]),
                        v128_unziplo_32(b.val[1], b.val[0]));
}

SIMD_INLINE v256 v256_unziphi_32(v256 a, v256 b) {
  return v256_from_v128(v128_unziphi_32(a.val[1], a.val[0]),
                        v128_unziphi_32(b.val[1], b.val[0]));
}

SIMD_INLINE v256 v256_unziplo_64(v256 a, v256 b) {
#if HAVE_SSE2
  return v256_from_v128(
      _mm_castpd_si128(_mm_shuffle_pd(_mm_castsi128_pd(a.val[0]),
                                      _mm_castsi128_pd(a.val[1]), 0)),
      _mm_castpd_si128(_mm_shuffle_pd(_mm_castsi128_pd(b.val[0]),
                                      _mm_castsi128_pd(b.val[1]), 0)));
#else
  return v256_from_v64(v128_low_v64(a.val[1]), v128_low_v64(a.val[0]),
                       v128_low_v64(b.val[1]), v128_low_v64(b.val[0]));
#endif
}

SIMD_INLINE v256 v256_unziphi_64(v256 a, v256 b) {
#if HAVE_SSE2
  return v256_from_v128(
      _mm_castpd_si128(_mm_shuffle_pd(_mm_castsi128_pd(a.val[0]),
                                      _mm_castsi128_pd(a.val[1]), 3)),
      _mm_castpd_si128(_mm_shuffle_pd(_mm_castsi128_pd(b.val[0]),
                                      _mm_castsi128_pd(b.val[1]), 3)));
#else
  return v256_from_v64(v128_high_v64(a.val[1]), v128_high_v64(a.val[0]),
                       v128_high_v64(b.val[1]), v128_high_v64(b.val[0]));
#endif
}

SIMD_INLINE v256 v256_unpack_u8_s16(v128 a) {
  return v256_from_v128(v128_unpackhi_u8_s16(a), v128_unpacklo_u8_s16(a));
}

SIMD_INLINE v256 v256_unpacklo_u8_s16(v256 a) {
  return v256_from_v128(v128_unpackhi_u8_s16(a.val[0]),
                        v128_unpacklo_u8_s16(a.val[0]));
}

SIMD_INLINE v256 v256_unpackhi_u8_s16(v256 a) {
  return v256_from_v128(v128_unpackhi_u8_s16(a.val[1]),
                        v128_unpacklo_u8_s16(a.val[1]));
}

SIMD_INLINE v256 v256_unpack_s8_s16(v128 a) {
  return v256_from_v128(v128_unpackhi_s8_s16(a), v128_unpacklo_s8_s16(a));
}

SIMD_INLINE v256 v256_unpacklo_s8_s16(v256 a) {
  return v256_from_v128(v128_unpackhi_s8_s16(a.val[0]),
                        v128_unpacklo_s8_s16(a.val[0]));
}

SIMD_INLINE v256 v256_unpackhi_s8_s16(v256 a) {
  return v256_from_v128(v128_unpackhi_s8_s16(a.val[1]),
                        v128_unpacklo_s8_s16(a.val[1]));
}

SIMD_INLINE v256 v256_pack_s32_s16(v256 a, v256 b) {
  return v256_from_v128(v128_pack_s32_s16(a.val[1], a.val[0]),
                        v128_pack_s32_s16(b.val[1], b.val[0]));
}

SIMD_INLINE v256 v256_pack_s32_u16(v256 a, v256 b) {
  return v256_from_v128(v128_pack_s32_u16(a.val[1], a.val[0]),
                        v128_pack_s32_u16(b.val[1], b.val[0]));
}

SIMD_INLINE v256 v256_pack_s16_u8(v256 a, v256 b) {
  return v256_from_v128(v128_pack_s16_u8(a.val[1], a.val[0]),
                        v128_pack_s16_u8(b.val[1], b.val[0]));
}

SIMD_INLINE v256 v256_pack_s16_s8(v256 a, v256 b) {
  return v256_from_v128(v128_pack_s16_s8(a.val[1], a.val[0]),
                        v128_pack_s16_s8(b.val[1], b.val[0]));
}

SIMD_INLINE v256 v256_unpack_u16_s32(v128 a) {
  return v256_from_v128(v128_unpackhi_u16_s32(a), v128_unpacklo_u16_s32(a));
}

SIMD_INLINE v256 v256_unpack_s16_s32(v128 a) {
  return v256_from_v128(v128_unpackhi_s16_s32(a), v128_unpacklo_s16_s32(a));
}

SIMD_INLINE v256 v256_unpacklo_u16_s32(v256 a) {
  return v256_from_v128(v128_unpackhi_u16_s32(a.val[0]),
                        v128_unpacklo_u16_s32(a.val[0]));
}

SIMD_INLINE v256 v256_unpacklo_s16_s32(v256 a) {
  return v256_from_v128(v128_unpackhi_s16_s32(a.val[0]),
                        v128_unpacklo_s16_s32(a.val[0]));
}

SIMD_INLINE v256 v256_unpackhi_u16_s32(v256 a) {
  return v256_from_v128(v128_unpackhi_u16_s32(a.val[1]),
                        v128_unpacklo_u16_s32(a.val[1]));
}

SIMD_INLINE v256 v256_unpackhi_s16_s32(v256 a) {
  return v256_from_v128(v128_unpackhi_s16_s32(a.val[1]),
                        v128_unpacklo_s16_s32(a.val[1]));
}

SIMD_INLINE v256 v256_cmpgt_s8(v256 a, v256 b) {
  return v256_from_v128(v128_cmpgt_s8(a.val[1], b.val[1]),
                        v128_cmpgt_s8(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_cmplt_s8(v256 a, v256 b) {
  return v256_from_v128(v128_cmplt_s8(a.val[1], b.val[1]),
                        v128_cmplt_s8(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_cmpeq_8(v256 a, v256 b) {
  return v256_from_v128(v128_cmpeq_8(a.val[1], b.val[1]),
                        v128_cmpeq_8(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_cmpgt_s16(v256 a, v256 b) {
  return v256_from_v128(v128_cmpgt_s16(a.val[1], b.val[1]),
                        v128_cmpgt_s16(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_cmplt_s16(v256 a, v256 b) {
  return v256_from_v128(v128_cmplt_s16(a.val[1], b.val[1]),
                        v128_cmplt_s16(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_cmpeq_16(v256 a, v256 b) {
  return v256_from_v128(v128_cmpeq_16(a.val[1], b.val[1]),
                        v128_cmpeq_16(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_cmpgt_s32(v256 a, v256 b) {
  return v256_from_v128(v128_cmpgt_s32(a.val[1], b.val[1]),
                        v128_cmpgt_s32(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_cmplt_s32(v256 a, v256 b) {
  return v256_from_v128(v128_cmplt_s32(a.val[1], b.val[1]),
                        v128_cmplt_s32(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_cmpeq_32(v256 a, v256 b) {
  return v256_from_v128(v128_cmpeq_32(a.val[1], b.val[1]),
                        v128_cmpeq_32(a.val[0], b.val[0]));
}

SIMD_INLINE v256 v256_shuffle_8(v256 x, v256 pattern) {
  v128 c16 = v128_dup_8(16);
  v128 maskhi = v128_cmplt_s8(pattern.val[1], c16);
  v128 masklo = v128_cmplt_s8(pattern.val[0], c16);
  return v256_from_v128(
      v128_blend_8(v128_shuffle_8(x.val[1], v128_sub_8(pattern.val[1], c16)),
                   v128_shuffle_8(x.val[0], pattern.val[1]), maskhi),
      v128_blend_8(v128_shuffle_8(x.val[1], v128_sub_8(pattern.val[0], c16)),
                   v128_shuffle_8(x.val[0], pattern.val[0]), masklo));
}

SIMD_INLINE v256 v256_wideshuffle_8(v256 x, v256 y, v256 pattern) {
  v128 c16 = v128_dup_8(16);
  v128 c32 = v128_dup_8(32);
  v128 c48 = v128_dup_8(48);
  v128 maskhi16 = v128_cmpgt_s8(c16, pattern.val[1]);
  v128 masklo16 = v128_cmpgt_s8(c16, pattern.val[0]);
  v128 maskhi48 = v128_cmpgt_s8(c48, pattern.val[1]);
  v128 masklo48 = v128_cmpgt_s8(c48, pattern.val[0]);
  v256 r1 = v256_from_v128(
      v128_blend_8(v128_shuffle_8(x.val[1], v128_sub_8(pattern.val[1], c48)),
                   v128_shuffle_8(x.val[0], v128_sub_8(pattern.val[1], c32)),
                   maskhi48),
      v128_blend_8(v128_shuffle_8(x.val[1], v128_sub_8(pattern.val[0], c48)),
                   v128_shuffle_8(x.val[0], v128_sub_8(pattern.val[0], c32)),
                   masklo48));
  v256 r2 = v256_from_v128(
      v128_blend_8(v128_shuffle_8(y.val[1], v128_sub_8(pattern.val[1], c16)),
                   v128_shuffle_8(y.val[0], pattern.val[1]), maskhi16),
      v128_blend_8(v128_shuffle_8(y.val[1], v128_sub_8(pattern.val[0], c16)),
                   v128_shuffle_8(y.val[0], pattern.val[0]), masklo16));
  return v256_blend_8(r1, r2, v256_cmpgt_s8(v256_from_v128(c32, c32), pattern));
}

SIMD_INLINE v256 v256_pshuffle_8(v256 a, v256 pattern) {
  return v256_from_v128(
      v128_shuffle_8(v256_high_v128(a), v256_high_v128(pattern)),
      v128_shuffle_8(v256_low_v128(a), v256_low_v128(pattern)));
}

SIMD_INLINE v256 v256_shl_8(v256 a, const unsigned int c) {
  return v256_from_v128(v128_shl_8(a.val[1], c), v128_shl_8(a.val[0], c));
}

SIMD_INLINE v256 v256_shr_u8(v256 a, const unsigned int c) {
  return v256_from_v128(v128_shr_u8(a.val[1], c), v128_shr_u8(a.val[0], c));
}

SIMD_INLINE v256 v256_shr_s8(v256 a, const unsigned int c) {
  return v256_from_v128(v128_shr_s8(a.val[1], c), v128_shr_s8(a.val[0], c));
}

SIMD_INLINE v256 v256_shl_16(v256 a, const unsigned int c) {
  return v256_from_v128(v128_shl_16(a.val[1], c), v128_shl_16(a.val[0], c));
}

SIMD_INLINE v256 v256_shr_u16(v256 a, const unsigned int c) {
  return v256_from_v128(v128_shr_u16(a.val[1], c), v128_shr_u16(a.val[0], c));
}

SIMD_INLINE v256 v256_shr_s16(v256 a, const unsigned int c) {
  return v256_from_v128(v128_shr_s16(a.val[1], c), v128_shr_s16(a.val[0], c));
}

SIMD_INLINE v256 v256_shl_32(v256 a, const unsigned int c) {
  return v256_from_v128(v128_shl_32(a.val[1], c), v128_shl_32(a.val[0], c));
}

SIMD_INLINE v256 v256_shr_u32(v256 a, const unsigned int c) {
  return v256_from_v128(v128_shr_u32(a.val[1], c), v128_shr_u32(a.val[0], c));
}

SIMD_INLINE v256 v256_shr_s32(v256 a, const unsigned int c) {
  return v256_from_v128(v128_shr_s32(a.val[1], c), v128_shr_s32(a.val[0], c));
}

SIMD_INLINE v256 v256_shl_64(v256 a, const unsigned int c) {
  return v256_from_v128(v128_shl_64(a.val[1], c), v128_shl_64(a.val[0], c));
}

SIMD_INLINE v256 v256_shr_u64(v256 a, const unsigned int c) {
  return v256_from_v128(v128_shr_u64(a.val[1], c), v128_shr_u64(a.val[0], c));
}

SIMD_INLINE v256 v256_shr_s64(v256 a, const unsigned int c) {
  return v256_from_v128(v128_shr_s64(a.val[1], c), v128_shr_s64(a.val[0], c));
}

/* These intrinsics require immediate values, so we must use #defines
   to enforce that. */
#define v256_shl_n_byte(a, n)                                              \
  ((n) < 16 ? v256_from_v128(v128_or(v128_shl_n_byte(a.val[1], n),         \
                                     v128_shr_n_byte(a.val[0], 16 - (n))), \
                             v128_shl_n_byte(a.val[0], (n)))               \
            : v256_from_v128(                                              \
                  (n) > 16 ? v128_shl_n_byte(a.val[0], (n)-16) : a.val[0], \
                  v128_zero()))

#define v256_shr_n_byte(a, n)                                                \
  (n == 0                                                                    \
       ? a                                                                   \
       : ((n) < 16                                                           \
              ? v256_from_v128(v128_shr_n_byte(a.val[1], n),                 \
                               v128_or(v128_shr_n_byte(a.val[0], n),         \
                                       v128_shl_n_byte(a.val[1], 16 - (n)))) \
              : v256_from_v128(                                              \
                    v128_zero(),                                             \
                    (n) > 16 ? v128_shr_n_byte(a.val[1], (n)-16) : a.val[1])))

#define v256_align(a, b, c) \
  ((c) ? v256_or(v256_shr_n_byte(b, c), v256_shl_n_byte(a, 32 - (c))) : b)

#define v256_shl_n_8(a, n) \
  v256_from_v128(v128_shl_n_8(a.val[1], n), v128_shl_n_8(a.val[0], n))
#define v256_shl_n_16(a, n) \
  v256_from_v128(v128_shl_n_16(a.val[1], n), v128_shl_n_16(a.val[0], n))
#define v256_shl_n_32(a, n) \
  v256_from_v128(v128_shl_n_32(a.val[1], n), v128_shl_n_32(a.val[0], n))
#define v256_shl_n_64(a, n) \
  v256_from_v128(v128_shl_n_64(a.val[1], n), v128_shl_n_64(a.val[0], n))
#define v256_shr_n_u8(a, n) \
  v256_from_v128(v128_shr_n_u8(a.val[1], n), v128_shr_n_u8(a.val[0], n))
#define v256_shr_n_u16(a, n) \
  v256_from_v128(v128_shr_n_u16(a.val[1], n), v128_shr_n_u16(a.val[0], n))
#define v256_shr_n_u32(a, n) \
  v256_from_v128(v128_shr_n_u32(a.val[1], n), v128_shr_n_u32(a.val[0], n))
#define v256_shr_n_u64(a, n) \
  v256_from_v128(v128_shr_n_u64(a.val[1], n), v128_shr_n_u64(a.val[0], n))
#define v256_shr_n_s8(a, n) \
  v256_from_v128(v128_shr_n_s8(a.val[1], n), v128_shr_n_s8(a.val[0], n))
#define v256_shr_n_s16(a, n) \
  v256_from_v128(v128_shr_n_s16(a.val[1], n), v128_shr_n_s16(a.val[0], n))
#define v256_shr_n_s32(a, n) \
  v256_from_v128(v128_shr_n_s32(a.val[1], n), v128_shr_n_s32(a.val[0], n))
#define v256_shr_n_s64(a, n) \
  v256_from_v128(v128_shr_n_s64(a.val[1], n), v128_shr_n_s64(a.val[0], n))

#define v256_shr_n_word(a, n) v256_shr_n_byte(a, 2 * (n))
#define v256_shl_n_word(a, n) v256_shl_n_byte(a, 2 * (n))

typedef struct {
  sad128_internal_u16 val[2];
} sad256_internal_u16;

SIMD_INLINE sad256_internal_u16 v256_sad_u16_init(void) {
  sad256_internal_u16 t;
  t.val[1] = v128_sad_u16_init();
  t.val[0] = v128_sad_u16_init();
  return t;
}

/* Implementation dependent return value.  Result must be finalised with
   v256_sad_u16_sum().
   The result for more than 16 v256_sad_u16() calls is undefined. */
SIMD_INLINE sad256_internal_u16 v256_sad_u16(sad256_internal_u16 s, v256 a,
                                             v256 b) {
  sad256_internal_u16 t;
  t.val[1] = v128_sad_u16(s.val[1], a.val[1], b.val[1]);
  t.val[0] = v128_sad_u16(s.val[0], a.val[0], b.val[0]);
  return t;
}

SIMD_INLINE uint32_t v256_sad_u16_sum(sad256_internal_u16 s) {
  return v128_sad_u16_sum(s.val[1]) + v128_sad_u16_sum(s.val[0]);
}

typedef struct {
  ssd128_internal_s16 val[2];
} ssd256_internal_s16;

SIMD_INLINE ssd256_internal_s16 v256_ssd_s16_init(void) {
  ssd256_internal_s16 t;
  t.val[1] = v128_ssd_s16_init();
  t.val[0] = v128_ssd_s16_init();
  return t;
}

/* Implementation dependent return value.  Result must be finalised with
 * v256_ssd_s16_sum(). */
SIMD_INLINE ssd256_internal_s16 v256_ssd_s16(ssd256_internal_s16 s, v256 a,
                                             v256 b) {
  ssd256_internal_s16 t;
  t.val[1] = v128_ssd_s16(s.val[1], a.val[1], b.val[1]);
  t.val[0] = v128_ssd_s16(s.val[0], a.val[0], b.val[0]);
  return t;
}

SIMD_INLINE uint64_t v256_ssd_s16_sum(ssd256_internal_s16 s) {
  return v128_ssd_s16_sum(s.val[1]) + v128_ssd_s16_sum(s.val[0]);
}

#endif  // AOM_AOM_DSP_SIMD_V256_INTRINSICS_V128_H_
