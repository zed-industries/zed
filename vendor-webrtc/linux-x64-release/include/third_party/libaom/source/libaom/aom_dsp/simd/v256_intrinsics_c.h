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

#ifndef AOM_AOM_DSP_SIMD_V256_INTRINSICS_C_H_
#define AOM_AOM_DSP_SIMD_V256_INTRINSICS_C_H_

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "config/aom_config.h"

#include "aom_dsp/simd/v128_intrinsics_c.h"

typedef union {
  uint8_t u8[32];
  uint16_t u16[16];
  uint32_t u32[8];
  uint64_t u64[4];
  int8_t s8[32];
  int16_t s16[16];
  int32_t s32[8];
  int64_t s64[4];
  c_v64 v64[4];
  c_v128 v128[2];
} c_v256;

SIMD_INLINE uint32_t c_v256_low_u32(c_v256 a) { return a.u32[0]; }

SIMD_INLINE c_v64 c_v256_low_v64(c_v256 a) { return a.v64[0]; }

SIMD_INLINE uint64_t c_v256_low_u64(c_v256 a) { return a.u64[0]; }

SIMD_INLINE c_v128 c_v256_low_v128(c_v256 a) { return a.v128[0]; }

SIMD_INLINE c_v128 c_v256_high_v128(c_v256 a) { return a.v128[1]; }

SIMD_INLINE c_v256 c_v256_from_v128(c_v128 hi, c_v128 lo) {
  c_v256 t;
  t.v128[1] = hi;
  t.v128[0] = lo;
  return t;
}

SIMD_INLINE c_v256 c_v256_from_64(uint64_t a, uint64_t b, uint64_t c,
                                  uint64_t d) {
  c_v256 t;
  t.u64[3] = a;
  t.u64[2] = b;
  t.u64[1] = c;
  t.u64[0] = d;
  return t;
}

SIMD_INLINE c_v256 c_v256_from_v64(c_v64 a, c_v64 b, c_v64 c, c_v64 d) {
  c_v256 t;
  t.u64[3] = a.u64;
  t.u64[2] = b.u64;
  t.u64[1] = c.u64;
  t.u64[0] = d.u64;
  return t;
}

SIMD_INLINE c_v256 c_v256_load_unaligned(const void *p) {
  c_v256 t;
  memcpy(&t, p, 32);
  return t;
}

SIMD_INLINE c_v256 c_v256_load_aligned(const void *p) {
  if (SIMD_CHECK && (uintptr_t)p & 31) {
    fprintf(stderr, "Error: unaligned v256 load at %p\n", p);
    abort();
  }
  return c_v256_load_unaligned(p);
}

SIMD_INLINE void c_v256_store_unaligned(void *p, c_v256 a) {
  memcpy(p, &a, 32);
}

SIMD_INLINE void c_v256_store_aligned(void *p, c_v256 a) {
  if (SIMD_CHECK && (uintptr_t)p & 31) {
    fprintf(stderr, "Error: unaligned v256 store at %p\n", p);
    abort();
  }
  c_v256_store_unaligned(p, a);
}

SIMD_INLINE c_v256 c_v256_zero(void) {
  c_v256 t;
  t.u64[3] = t.u64[2] = t.u64[1] = t.u64[0] = 0;
  return t;
}

SIMD_INLINE c_v256 c_v256_dup_8(uint8_t x) {
  c_v256 t;
  t.v64[3] = t.v64[2] = t.v64[1] = t.v64[0] = c_v64_dup_8(x);
  return t;
}

SIMD_INLINE c_v256 c_v256_dup_16(uint16_t x) {
  c_v256 t;
  t.v64[3] = t.v64[2] = t.v64[1] = t.v64[0] = c_v64_dup_16(x);
  return t;
}

SIMD_INLINE c_v256 c_v256_dup_32(uint32_t x) {
  c_v256 t;
  t.v64[3] = t.v64[2] = t.v64[1] = t.v64[0] = c_v64_dup_32(x);
  return t;
}

SIMD_INLINE c_v256 c_v256_dup_64(uint64_t x) {
  c_v256 t;
  t.u64[3] = t.u64[2] = t.u64[1] = t.u64[0] = x;
  return t;
}

SIMD_INLINE int64_t c_v256_dotp_su8(c_v256 a, c_v256 b) {
  return c_v128_dotp_su8(a.v128[1], b.v128[1]) +
         c_v128_dotp_su8(a.v128[0], b.v128[0]);
}

SIMD_INLINE int64_t c_v256_dotp_s16(c_v256 a, c_v256 b) {
  return c_v128_dotp_s16(a.v128[1], b.v128[1]) +
         c_v128_dotp_s16(a.v128[0], b.v128[0]);
}

SIMD_INLINE int64_t c_v256_dotp_s32(c_v256 a, c_v256 b) {
  return c_v128_dotp_s32(a.v128[1], b.v128[1]) +
         c_v128_dotp_s32(a.v128[0], b.v128[0]);
}

SIMD_INLINE uint64_t c_v256_hadd_u8(c_v256 a) {
  return c_v128_hadd_u8(a.v128[1]) + c_v128_hadd_u8(a.v128[0]);
}

typedef struct {
  uint32_t val;
  int count;
} c_sad256_internal;

SIMD_INLINE c_sad256_internal c_v256_sad_u8_init(void) {
  c_sad256_internal t;
  t.val = t.count = 0;
  return t;
}

/* Implementation dependent return value.  Result must be finalised with
   v256_sad_u8_sum().
   The result for more than 16 v256_sad_u8() calls is undefined. */
SIMD_INLINE c_sad256_internal c_v256_sad_u8(c_sad256_internal s, c_v256 a,
                                            c_v256 b) {
  int c;
  for (c = 0; c < 32; c++)
    s.val += a.u8[c] > b.u8[c] ? a.u8[c] - b.u8[c] : b.u8[c] - a.u8[c];
  s.count++;
  if (SIMD_CHECK && s.count > 32) {
    fprintf(stderr,
            "Error: sad called 32 times returning an undefined result\n");
    abort();
  }
  return s;
}

SIMD_INLINE uint32_t c_v256_sad_u8_sum(c_sad256_internal s) { return s.val; }

typedef uint32_t c_ssd256_internal;

SIMD_INLINE c_ssd256_internal c_v256_ssd_u8_init(void) { return 0; }

/* Implementation dependent return value.  Result must be finalised with
 * v256_ssd_u8_sum(). */
SIMD_INLINE c_ssd256_internal c_v256_ssd_u8(c_ssd256_internal s, c_v256 a,
                                            c_v256 b) {
  int c;
  for (c = 0; c < 32; c++) s += (a.u8[c] - b.u8[c]) * (a.u8[c] - b.u8[c]);
  return s;
}

SIMD_INLINE uint32_t c_v256_ssd_u8_sum(c_ssd256_internal s) { return s; }

SIMD_INLINE c_v256 c_v256_or(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_or(a.v128[1], b.v128[1]),
                          c_v128_or(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_xor(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_xor(a.v128[1], b.v128[1]),
                          c_v128_xor(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_and(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_and(a.v128[1], b.v128[1]),
                          c_v128_and(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_andn(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_andn(a.v128[1], b.v128[1]),
                          c_v128_andn(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_add_8(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_add_8(a.v128[1], b.v128[1]),
                          c_v128_add_8(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_add_16(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_add_16(a.v128[1], b.v128[1]),
                          c_v128_add_16(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_sadd_s8(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_sadd_s8(a.v128[1], b.v128[1]),
                          c_v128_sadd_s8(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_sadd_u8(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_sadd_u8(a.v128[1], b.v128[1]),
                          c_v128_sadd_u8(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_sadd_s16(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_sadd_s16(a.v128[1], b.v128[1]),
                          c_v128_sadd_s16(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_add_32(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_add_32(a.v128[1], b.v128[1]),
                          c_v128_add_32(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_add_64(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_add_64(a.v128[1], b.v128[1]),
                          c_v128_add_64(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_sub_64(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_sub_64(a.v128[1], b.v128[1]),
                          c_v128_sub_64(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_padd_u8(c_v256 a) {
  c_v256 t;
  for (int i = 0; i < 16; i++)
    t.u16[i] = (uint16_t)a.u8[i * 2] + (uint16_t)a.u8[i * 2 + 1];
  return t;
}

SIMD_INLINE c_v256 c_v256_padd_s16(c_v256 a) {
  c_v256 t;
  t.s32[0] = (int32_t)a.s16[0] + (int32_t)a.s16[1];
  t.s32[1] = (int32_t)a.s16[2] + (int32_t)a.s16[3];
  t.s32[2] = (int32_t)a.s16[4] + (int32_t)a.s16[5];
  t.s32[3] = (int32_t)a.s16[6] + (int32_t)a.s16[7];
  t.s32[4] = (int32_t)a.s16[8] + (int32_t)a.s16[9];
  t.s32[5] = (int32_t)a.s16[10] + (int32_t)a.s16[11];
  t.s32[6] = (int32_t)a.s16[12] + (int32_t)a.s16[13];
  t.s32[7] = (int32_t)a.s16[14] + (int32_t)a.s16[15];
  return t;
}

SIMD_INLINE c_v256 c_v256_sub_8(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_sub_8(a.v128[1], b.v128[1]),
                          c_v128_sub_8(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_ssub_u8(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_ssub_u8(a.v128[1], b.v128[1]),
                          c_v128_ssub_u8(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_ssub_s8(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_ssub_s8(a.v128[1], b.v128[1]),
                          c_v128_ssub_s8(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_sub_16(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_sub_16(a.v128[1], b.v128[1]),
                          c_v128_sub_16(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_ssub_s16(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_ssub_s16(a.v128[1], b.v128[1]),
                          c_v128_ssub_s16(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_ssub_u16(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_ssub_u16(a.v128[1], b.v128[1]),
                          c_v128_ssub_u16(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_sub_32(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_sub_32(a.v128[1], b.v128[1]),
                          c_v128_sub_32(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_abs_s16(c_v256 a) {
  return c_v256_from_v128(c_v128_abs_s16(a.v128[1]), c_v128_abs_s16(a.v128[0]));
}

SIMD_INLINE c_v256 c_v256_abs_s8(c_v256 a) {
  return c_v256_from_v128(c_v128_abs_s8(a.v128[1]), c_v128_abs_s8(a.v128[0]));
}

SIMD_INLINE c_v256 c_v256_mul_s16(c_v128 a, c_v128 b) {
  c_v128 lo_bits = c_v128_mullo_s16(a, b);
  c_v128 hi_bits = c_v128_mulhi_s16(a, b);
  return c_v256_from_v128(c_v128_ziphi_16(hi_bits, lo_bits),
                          c_v128_ziplo_16(hi_bits, lo_bits));
}

SIMD_INLINE c_v256 c_v256_mullo_s16(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_mullo_s16(a.v128[1], b.v128[1]),
                          c_v128_mullo_s16(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_mulhi_s16(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_mulhi_s16(a.v128[1], b.v128[1]),
                          c_v128_mulhi_s16(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_mullo_s32(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_mullo_s32(a.v128[1], b.v128[1]),
                          c_v128_mullo_s32(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_madd_s16(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_madd_s16(a.v128[1], b.v128[1]),
                          c_v128_madd_s16(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_madd_us8(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_madd_us8(a.v128[1], b.v128[1]),
                          c_v128_madd_us8(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_avg_u8(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_avg_u8(a.v128[1], b.v128[1]),
                          c_v128_avg_u8(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_rdavg_u8(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_rdavg_u8(a.v128[1], b.v128[1]),
                          c_v128_rdavg_u8(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_rdavg_u16(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_rdavg_u16(a.v128[1], b.v128[1]),
                          c_v128_rdavg_u16(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_avg_u16(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_avg_u16(a.v128[1], b.v128[1]),
                          c_v128_avg_u16(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_min_u8(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_min_u8(a.v128[1], b.v128[1]),
                          c_v128_min_u8(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_max_u8(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_max_u8(a.v128[1], b.v128[1]),
                          c_v128_max_u8(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_min_s8(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_min_s8(a.v128[1], b.v128[1]),
                          c_v128_min_s8(a.v128[0], b.v128[0]));
}

SIMD_INLINE uint32_t c_v256_movemask_8(c_v256 a) {
  return ((uint32_t)(a.s8[31] < 0) << 31) | ((a.s8[30] < 0) << 30) |
         ((a.s8[29] < 0) << 29) | ((a.s8[28] < 0) << 28) |
         ((a.s8[27] < 0) << 27) | ((a.s8[26] < 0) << 26) |
         ((a.s8[25] < 0) << 25) | ((a.s8[24] < 0) << 24) |
         ((a.s8[23] < 0) << 23) | ((a.s8[22] < 0) << 22) |
         ((a.s8[21] < 0) << 21) | ((a.s8[20] < 0) << 20) |
         ((a.s8[19] < 0) << 19) | ((a.s8[18] < 0) << 18) |
         ((a.s8[17] < 0) << 17) | ((a.s8[16] < 0) << 16) |
         ((a.s8[15] < 0) << 15) | ((a.s8[14] < 0) << 14) |
         ((a.s8[13] < 0) << 13) | ((a.s8[12] < 0) << 12) |
         ((a.s8[11] < 0) << 11) | ((a.s8[10] < 0) << 10) |
         ((a.s8[9] < 0) << 9) | ((a.s8[8] < 0) << 8) | ((a.s8[7] < 0) << 7) |
         ((a.s8[6] < 0) << 6) | ((a.s8[5] < 0) << 5) | ((a.s8[4] < 0) << 4) |
         ((a.s8[3] < 0) << 3) | ((a.s8[2] < 0) << 2) | ((a.s8[1] < 0) << 1) |
         ((a.s8[0] < 0) << 0);
}

SIMD_INLINE c_v256 c_v256_blend_8(c_v256 a, c_v256 b, c_v256 c) {
  c_v256 t;
  for (int i = 0; i < 32; i++) t.u8[i] = c.s8[i] < 0 ? b.u8[i] : a.u8[i];
  return t;
}

SIMD_INLINE c_v256 c_v256_max_s8(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_max_s8(a.v128[1], b.v128[1]),
                          c_v128_max_s8(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_min_s16(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_min_s16(a.v128[1], b.v128[1]),
                          c_v128_min_s16(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_max_s16(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_max_s16(a.v128[1], b.v128[1]),
                          c_v128_max_s16(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_min_s32(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_min_s32(a.v128[1], b.v128[1]),
                          c_v128_min_s32(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_max_s32(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_max_s32(a.v128[1], b.v128[1]),
                          c_v128_max_s32(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_ziplo_8(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_ziphi_8(a.v128[0], b.v128[0]),
                          c_v128_ziplo_8(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_ziphi_8(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_ziphi_8(a.v128[1], b.v128[1]),
                          c_v128_ziplo_8(a.v128[1], b.v128[1]));
}

SIMD_INLINE c_v256 c_v256_ziplo_16(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_ziphi_16(a.v128[0], b.v128[0]),
                          c_v128_ziplo_16(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_ziphi_16(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_ziphi_16(a.v128[1], b.v128[1]),
                          c_v128_ziplo_16(a.v128[1], b.v128[1]));
}

SIMD_INLINE c_v256 c_v256_ziplo_32(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_ziphi_32(a.v128[0], b.v128[0]),
                          c_v128_ziplo_32(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_ziphi_32(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_ziphi_32(a.v128[1], b.v128[1]),
                          c_v128_ziplo_32(a.v128[1], b.v128[1]));
}

SIMD_INLINE c_v256 c_v256_ziplo_64(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_ziphi_64(a.v128[0], b.v128[0]),
                          c_v128_ziplo_64(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_ziphi_64(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_ziphi_64(a.v128[1], b.v128[1]),
                          c_v128_ziplo_64(a.v128[1], b.v128[1]));
}

SIMD_INLINE c_v256 c_v256_ziplo_128(c_v256 a, c_v256 b) {
  return c_v256_from_v128(a.v128[0], b.v128[0]);
}

SIMD_INLINE c_v256 c_v256_ziphi_128(c_v256 a, c_v256 b) {
  return c_v256_from_v128(a.v128[1], b.v128[1]);
}

SIMD_INLINE c_v256 c_v256_zip_8(c_v128 a, c_v128 b) {
  return c_v256_from_v128(c_v128_ziphi_8(a, b), c_v128_ziplo_8(a, b));
}

SIMD_INLINE c_v256 c_v256_zip_16(c_v128 a, c_v128 b) {
  return c_v256_from_v128(c_v128_ziphi_16(a, b), c_v128_ziplo_16(a, b));
}

SIMD_INLINE c_v256 c_v256_zip_32(c_v128 a, c_v128 b) {
  return c_v256_from_v128(c_v128_ziphi_32(a, b), c_v128_ziplo_32(a, b));
}

SIMD_INLINE c_v256 _c_v256_unzip_8(c_v256 a, c_v256 b, int mode) {
  c_v256 t;
  int i;
  if (mode) {
    for (i = 0; i < 16; i++) {
      t.u8[i] = a.u8[i * 2 + 1];
      t.u8[i + 16] = b.u8[i * 2 + 1];
    }
  } else {
    for (i = 0; i < 16; i++) {
      t.u8[i] = b.u8[i * 2];
      t.u8[i + 16] = a.u8[i * 2];
    }
  }
  return t;
}

SIMD_INLINE c_v256 c_v256_unziplo_8(c_v256 a, c_v256 b) {
  return CONFIG_BIG_ENDIAN ? _c_v256_unzip_8(a, b, 1)
                           : _c_v256_unzip_8(a, b, 0);
}

SIMD_INLINE c_v256 c_v256_unziphi_8(c_v256 a, c_v256 b) {
  return CONFIG_BIG_ENDIAN ? _c_v256_unzip_8(b, a, 0)
                           : _c_v256_unzip_8(b, a, 1);
}

SIMD_INLINE c_v256 _c_v256_unzip_16(c_v256 a, c_v256 b, int mode) {
  c_v256 t;
  int i;
  if (mode) {
    for (i = 0; i < 8; i++) {
      t.u16[i] = a.u16[i * 2 + 1];
      t.u16[i + 8] = b.u16[i * 2 + 1];
    }
  } else {
    for (i = 0; i < 8; i++) {
      t.u16[i] = b.u16[i * 2];
      t.u16[i + 8] = a.u16[i * 2];
    }
  }
  return t;
}

SIMD_INLINE c_v256 c_v256_unziplo_16(c_v256 a, c_v256 b) {
  return CONFIG_BIG_ENDIAN ? _c_v256_unzip_16(a, b, 1)
                           : _c_v256_unzip_16(a, b, 0);
}

SIMD_INLINE c_v256 c_v256_unziphi_16(c_v256 a, c_v256 b) {
  return CONFIG_BIG_ENDIAN ? _c_v256_unzip_16(b, a, 0)
                           : _c_v256_unzip_16(b, a, 1);
}

SIMD_INLINE c_v256 _c_v256_unzip_32(c_v256 a, c_v256 b, int mode) {
  c_v256 t;
  if (mode) {
    t.u32[7] = b.u32[7];
    t.u32[6] = b.u32[5];
    t.u32[5] = b.u32[3];
    t.u32[4] = b.u32[1];
    t.u32[3] = a.u32[7];
    t.u32[2] = a.u32[5];
    t.u32[1] = a.u32[3];
    t.u32[0] = a.u32[1];
  } else {
    t.u32[7] = a.u32[6];
    t.u32[6] = a.u32[4];
    t.u32[5] = a.u32[2];
    t.u32[4] = a.u32[0];
    t.u32[3] = b.u32[6];
    t.u32[2] = b.u32[4];
    t.u32[1] = b.u32[2];
    t.u32[0] = b.u32[0];
  }
  return t;
}

SIMD_INLINE c_v256 c_v256_unziplo_32(c_v256 a, c_v256 b) {
  return CONFIG_BIG_ENDIAN ? _c_v256_unzip_32(a, b, 1)
                           : _c_v256_unzip_32(a, b, 0);
}

SIMD_INLINE c_v256 c_v256_unziphi_32(c_v256 a, c_v256 b) {
  return CONFIG_BIG_ENDIAN ? _c_v256_unzip_32(b, a, 0)
                           : _c_v256_unzip_32(b, a, 1);
}

SIMD_INLINE c_v256 _c_v256_unzip_64(c_v256 a, c_v256 b, int mode) {
  c_v256 t;
  if (mode) {
    t.u64[3] = b.u64[3];
    t.u64[2] = b.u64[1];
    t.u64[1] = a.u64[3];
    t.u64[0] = a.u64[1];
  } else {
    t.u64[3] = a.u64[2];
    t.u64[2] = a.u64[0];
    t.u64[1] = b.u64[2];
    t.u64[0] = b.u64[0];
  }
  return t;
}

SIMD_INLINE c_v256 c_v256_unziplo_64(c_v256 a, c_v256 b) {
  return CONFIG_BIG_ENDIAN ? _c_v256_unzip_64(a, b, 1)
                           : _c_v256_unzip_64(a, b, 0);
}

SIMD_INLINE c_v256 c_v256_unziphi_64(c_v256 a, c_v256 b) {
  return CONFIG_BIG_ENDIAN ? _c_v256_unzip_64(b, a, 0)
                           : _c_v256_unzip_64(b, a, 1);
}

SIMD_INLINE c_v256 c_v256_unpack_u8_s16(c_v128 a) {
  return c_v256_from_v128(c_v128_unpackhi_u8_s16(a), c_v128_unpacklo_u8_s16(a));
}

SIMD_INLINE c_v256 c_v256_unpacklo_u8_s16(c_v256 a) {
  return c_v256_from_v128(c_v128_unpackhi_u8_s16(a.v128[0]),
                          c_v128_unpacklo_u8_s16(a.v128[0]));
}

SIMD_INLINE c_v256 c_v256_unpackhi_u8_s16(c_v256 a) {
  return c_v256_from_v128(c_v128_unpackhi_u8_s16(a.v128[1]),
                          c_v128_unpacklo_u8_s16(a.v128[1]));
}

SIMD_INLINE c_v256 c_v256_unpack_s8_s16(c_v128 a) {
  return c_v256_from_v128(c_v128_unpackhi_s8_s16(a), c_v128_unpacklo_s8_s16(a));
}

SIMD_INLINE c_v256 c_v256_unpacklo_s8_s16(c_v256 a) {
  return c_v256_from_v128(c_v128_unpackhi_s8_s16(a.v128[0]),
                          c_v128_unpacklo_s8_s16(a.v128[0]));
}

SIMD_INLINE c_v256 c_v256_unpackhi_s8_s16(c_v256 a) {
  return c_v256_from_v128(c_v128_unpackhi_s8_s16(a.v128[1]),
                          c_v128_unpacklo_s8_s16(a.v128[1]));
}

SIMD_INLINE c_v256 c_v256_pack_s32_s16(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_pack_s32_s16(a.v128[1], a.v128[0]),
                          c_v128_pack_s32_s16(b.v128[1], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_pack_s32_u16(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_pack_s32_u16(a.v128[1], a.v128[0]),
                          c_v128_pack_s32_u16(b.v128[1], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_pack_s16_u8(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_pack_s16_u8(a.v128[1], a.v128[0]),
                          c_v128_pack_s16_u8(b.v128[1], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_pack_s16_s8(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_pack_s16_s8(a.v128[1], a.v128[0]),
                          c_v128_pack_s16_s8(b.v128[1], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_unpack_u16_s32(c_v128 a) {
  return c_v256_from_v128(c_v128_unpackhi_u16_s32(a),
                          c_v128_unpacklo_u16_s32(a));
}

SIMD_INLINE c_v256 c_v256_unpack_s16_s32(c_v128 a) {
  return c_v256_from_v128(c_v128_unpackhi_s16_s32(a),
                          c_v128_unpacklo_s16_s32(a));
}

SIMD_INLINE c_v256 c_v256_unpacklo_u16_s32(c_v256 a) {
  return c_v256_from_v128(c_v128_unpackhi_u16_s32(a.v128[0]),
                          c_v128_unpacklo_u16_s32(a.v128[0]));
}

SIMD_INLINE c_v256 c_v256_unpacklo_s16_s32(c_v256 a) {
  return c_v256_from_v128(c_v128_unpackhi_s16_s32(a.v128[0]),
                          c_v128_unpacklo_s16_s32(a.v128[0]));
}

SIMD_INLINE c_v256 c_v256_unpackhi_u16_s32(c_v256 a) {
  return c_v256_from_v128(c_v128_unpackhi_u16_s32(a.v128[1]),
                          c_v128_unpacklo_u16_s32(a.v128[1]));
}

SIMD_INLINE c_v256 c_v256_unpackhi_s16_s32(c_v256 a) {
  return c_v256_from_v128(c_v128_unpackhi_s16_s32(a.v128[1]),
                          c_v128_unpacklo_s16_s32(a.v128[1]));
}

SIMD_INLINE c_v256 c_v256_shuffle_8(c_v256 a, c_v256 pattern) {
  c_v256 t;
  int c;
  for (c = 0; c < 32; c++)
    t.u8[c] = a.u8[CONFIG_BIG_ENDIAN ? 31 - (pattern.u8[c] & 31)
                                     : pattern.u8[c] & 31];

  return t;
}

SIMD_INLINE c_v256 c_v256_wideshuffle_8(c_v256 a, c_v256 b, c_v256 pattern) {
  c_v256 t;
  int c;
  for (c = 0; c < 32; c++)
    t.u8[c] = (pattern.u8[c] < 32
                   ? b.u8
                   : a.u8)[CONFIG_BIG_ENDIAN ? 31 - (pattern.u8[c] & 31)
                                             : pattern.u8[c] & 31];
  return t;
}

// Pairwise / dual-lane shuffle: shuffle two 128 bit lates.
SIMD_INLINE c_v256 c_v256_pshuffle_8(c_v256 a, c_v256 pattern) {
  return c_v256_from_v128(
      c_v128_shuffle_8(c_v256_high_v128(a), c_v256_high_v128(pattern)),
      c_v128_shuffle_8(c_v256_low_v128(a), c_v256_low_v128(pattern)));
}

SIMD_INLINE c_v256 c_v256_cmpgt_s8(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_cmpgt_s8(a.v128[1], b.v128[1]),
                          c_v128_cmpgt_s8(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_cmplt_s8(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_cmplt_s8(a.v128[1], b.v128[1]),
                          c_v128_cmplt_s8(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_cmpeq_8(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_cmpeq_8(a.v128[1], b.v128[1]),
                          c_v128_cmpeq_8(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_cmpgt_s16(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_cmpgt_s16(a.v128[1], b.v128[1]),
                          c_v128_cmpgt_s16(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_cmplt_s16(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_cmplt_s16(a.v128[1], b.v128[1]),
                          c_v128_cmplt_s16(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_cmpeq_16(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_cmpeq_16(a.v128[1], b.v128[1]),
                          c_v128_cmpeq_16(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_cmpgt_s32(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_cmpgt_s32(a.v128[1], b.v128[1]),
                          c_v128_cmpgt_s32(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_cmplt_s32(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_cmplt_s32(a.v128[1], b.v128[1]),
                          c_v128_cmplt_s32(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_cmpeq_32(c_v256 a, c_v256 b) {
  return c_v256_from_v128(c_v128_cmpeq_32(a.v128[1], b.v128[1]),
                          c_v128_cmpeq_32(a.v128[0], b.v128[0]));
}

SIMD_INLINE c_v256 c_v256_shl_n_byte(c_v256 a, unsigned int n) {
  if (n == 0) return a;
  if (n < 16)
    return c_v256_from_v128(c_v128_or(c_v128_shl_n_byte(a.v128[1], n),
                                      c_v128_shr_n_byte(a.v128[0], 16 - n)),
                            c_v128_shl_n_byte(a.v128[0], n));
  else if (n > 16)
    return c_v256_from_v128(c_v128_shl_n_byte(a.v128[0], n - 16),
                            c_v128_zero());
  else
    return c_v256_from_v128(c_v256_low_v128(a), c_v128_zero());
}

SIMD_INLINE c_v256 c_v256_shr_n_byte(c_v256 a, unsigned int n) {
  if (n == 0) return a;
  if (n < 16)
    return c_v256_from_v128(c_v128_shr_n_byte(a.v128[1], n),
                            c_v128_or(c_v128_shr_n_byte(a.v128[0], n),
                                      c_v128_shl_n_byte(a.v128[1], 16 - n)));
  else if (n > 16)
    return c_v256_from_v128(c_v128_zero(),
                            c_v128_shr_n_byte(a.v128[1], n - 16));
  else
    return c_v256_from_v128(c_v128_zero(), c_v256_high_v128(a));
}

SIMD_INLINE c_v256 c_v256_align(c_v256 a, c_v256 b, unsigned int c) {
  if (SIMD_CHECK && c > 31) {
    fprintf(stderr, "Error: undefined alignment %d\n", c);
    abort();
  }
  return c ? c_v256_or(c_v256_shr_n_byte(b, c), c_v256_shl_n_byte(a, 32 - c))
           : b;
}

SIMD_INLINE c_v256 c_v256_shl_8(c_v256 a, unsigned int c) {
  return c_v256_from_v128(c_v128_shl_8(a.v128[1], c),
                          c_v128_shl_8(a.v128[0], c));
}

SIMD_INLINE c_v256 c_v256_shr_u8(c_v256 a, unsigned int c) {
  return c_v256_from_v128(c_v128_shr_u8(a.v128[1], c),
                          c_v128_shr_u8(a.v128[0], c));
}

SIMD_INLINE c_v256 c_v256_shr_s8(c_v256 a, unsigned int c) {
  return c_v256_from_v128(c_v128_shr_s8(a.v128[1], c),
                          c_v128_shr_s8(a.v128[0], c));
}

SIMD_INLINE c_v256 c_v256_shl_16(c_v256 a, unsigned int c) {
  return c_v256_from_v128(c_v128_shl_16(a.v128[1], c),
                          c_v128_shl_16(a.v128[0], c));
}

SIMD_INLINE c_v256 c_v256_shr_u16(c_v256 a, unsigned int c) {
  return c_v256_from_v128(c_v128_shr_u16(a.v128[1], c),
                          c_v128_shr_u16(a.v128[0], c));
}

SIMD_INLINE c_v256 c_v256_shr_s16(c_v256 a, unsigned int c) {
  return c_v256_from_v128(c_v128_shr_s16(a.v128[1], c),
                          c_v128_shr_s16(a.v128[0], c));
}

SIMD_INLINE c_v256 c_v256_shl_32(c_v256 a, unsigned int c) {
  return c_v256_from_v128(c_v128_shl_32(a.v128[1], c),
                          c_v128_shl_32(a.v128[0], c));
}

SIMD_INLINE c_v256 c_v256_shr_u32(c_v256 a, unsigned int c) {
  return c_v256_from_v128(c_v128_shr_u32(a.v128[1], c),
                          c_v128_shr_u32(a.v128[0], c));
}

SIMD_INLINE c_v256 c_v256_shr_s32(c_v256 a, unsigned int c) {
  return c_v256_from_v128(c_v128_shr_s32(a.v128[1], c),
                          c_v128_shr_s32(a.v128[0], c));
}

SIMD_INLINE c_v256 c_v256_shr_s64(c_v256 a, unsigned int n) {
  c_v256 t;
  if (SIMD_CHECK && n > 63) {
    fprintf(stderr, "Error: undefined s64 shift right %d\n", n);
    abort();
  }
  t.s64[3] = a.s64[3] >> n;
  t.s64[2] = a.s64[2] >> n;
  t.s64[1] = a.s64[1] >> n;
  t.s64[0] = a.s64[0] >> n;
  return t;
}

SIMD_INLINE c_v256 c_v256_shr_u64(c_v256 a, unsigned int n) {
  c_v256 t;
  if (SIMD_CHECK && n > 63) {
    fprintf(stderr, "Error: undefined s64 shift right %d\n", n);
    abort();
  }
  t.u64[3] = a.u64[3] >> n;
  t.u64[2] = a.u64[2] >> n;
  t.u64[1] = a.u64[1] >> n;
  t.u64[0] = a.u64[0] >> n;
  return t;
}

SIMD_INLINE c_v256 c_v256_shl_64(c_v256 a, unsigned int n) {
  c_v256 t;
  if (SIMD_CHECK && n > 63) {
    fprintf(stderr, "Error: undefined s64 shift right %d\n", n);
    abort();
  }
  t.u64[3] = a.u64[3] << n;
  t.u64[2] = a.u64[2] << n;
  t.u64[1] = a.u64[1] << n;
  t.u64[0] = a.u64[0] << n;
  return t;
}

SIMD_INLINE c_v256 c_v256_shl_n_8(c_v256 a, unsigned int n) {
  return c_v256_shl_8(a, n);
}

SIMD_INLINE c_v256 c_v256_shl_n_16(c_v256 a, unsigned int n) {
  return c_v256_shl_16(a, n);
}

SIMD_INLINE c_v256 c_v256_shl_n_32(c_v256 a, unsigned int n) {
  return c_v256_shl_32(a, n);
}

SIMD_INLINE c_v256 c_v256_shl_n_64(c_v256 a, unsigned int n) {
  return c_v256_shl_64(a, n);
}

SIMD_INLINE c_v256 c_v256_shr_n_u8(c_v256 a, unsigned int n) {
  return c_v256_shr_u8(a, n);
}

SIMD_INLINE c_v256 c_v256_shr_n_u16(c_v256 a, unsigned int n) {
  return c_v256_shr_u16(a, n);
}

SIMD_INLINE c_v256 c_v256_shr_n_u32(c_v256 a, unsigned int n) {
  return c_v256_shr_u32(a, n);
}

SIMD_INLINE c_v256 c_v256_shr_n_u64(c_v256 a, unsigned int n) {
  return c_v256_shr_u64(a, n);
}

SIMD_INLINE c_v256 c_v256_shr_n_s8(c_v256 a, unsigned int n) {
  return c_v256_shr_s8(a, n);
}

SIMD_INLINE c_v256 c_v256_shr_n_s16(c_v256 a, unsigned int n) {
  return c_v256_shr_s16(a, n);
}

SIMD_INLINE c_v256 c_v256_shr_n_s32(c_v256 a, unsigned int n) {
  return c_v256_shr_s32(a, n);
}

SIMD_INLINE c_v256 c_v256_shr_n_s64(c_v256 a, unsigned int n) {
  return c_v256_shr_s64(a, n);
}

SIMD_INLINE c_v256 c_v256_shr_n_word(c_v256 a, const unsigned int n) {
  return c_v256_shr_n_byte(a, 2 * n);
}
SIMD_INLINE c_v256 c_v256_shl_n_word(c_v256 a, const unsigned int n) {
  return c_v256_shl_n_byte(a, 2 * n);
}

typedef uint32_t c_sad256_internal_u16;

SIMD_INLINE c_sad256_internal_u16 c_v256_sad_u16_init(void) { return 0; }

/* Implementation dependent return value.  Result must be finalised with
   v256_sad_u16_sum(). */
SIMD_INLINE c_sad256_internal_u16 c_v256_sad_u16(c_sad256_internal_u16 s,
                                                 c_v256 a, c_v256 b) {
  int c;
  for (c = 0; c < 16; c++)
    s += a.u16[c] > b.u16[c] ? a.u16[c] - b.u16[c] : b.u16[c] - a.u16[c];
  return s;
}

SIMD_INLINE uint32_t c_v256_sad_u16_sum(c_sad256_internal_u16 s) { return s; }

typedef uint64_t c_ssd256_internal_s16;

SIMD_INLINE c_ssd256_internal_s16 c_v256_ssd_s16_init(void) { return 0; }

/* Implementation dependent return value.  Result must be finalised with
 * v256_ssd_s16_sum(). */
SIMD_INLINE c_ssd256_internal_s16 c_v256_ssd_s16(c_ssd256_internal_s16 s,
                                                 c_v256 a, c_v256 b) {
  int c;
  for (c = 0; c < 16; c++)
    s += (int32_t)(int16_t)(a.s16[c] - b.s16[c]) *
         (int32_t)(int16_t)(a.s16[c] - b.s16[c]);
  return s;
}

SIMD_INLINE uint64_t c_v256_ssd_s16_sum(c_ssd256_internal_s16 s) { return s; }

#endif  // AOM_AOM_DSP_SIMD_V256_INTRINSICS_C_H_
