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

#ifndef AOM_AOM_DSP_SIMD_V128_INTRINSICS_C_H_
#define AOM_AOM_DSP_SIMD_V128_INTRINSICS_C_H_

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "config/aom_config.h"

#include "aom_dsp/simd/v64_intrinsics_c.h"

typedef union {
  uint8_t u8[16];
  uint16_t u16[8];
  uint32_t u32[4];
  uint64_t u64[2];
  int8_t s8[16];
  int16_t s16[8];
  int32_t s32[4];
  int64_t s64[2];
  c_v64 v64[2];
} c_v128;

SIMD_INLINE uint32_t c_v128_low_u32(c_v128 a) { return a.u32[0]; }

SIMD_INLINE c_v64 c_v128_low_v64(c_v128 a) { return a.v64[0]; }

SIMD_INLINE c_v64 c_v128_high_v64(c_v128 a) { return a.v64[1]; }

SIMD_INLINE c_v128 c_v128_from_64(uint64_t hi, uint64_t lo) {
  c_v128 t;
  t.u64[1] = hi;
  t.u64[0] = lo;
  return t;
}

SIMD_INLINE c_v128 c_v128_from_v64(c_v64 hi, c_v64 lo) {
  c_v128 t;
  t.v64[1] = hi;
  t.v64[0] = lo;
  return t;
}

SIMD_INLINE c_v128 c_v128_from_32(uint32_t a, uint32_t b, uint32_t c,
                                  uint32_t d) {
  c_v128 t;
  t.u32[3] = a;
  t.u32[2] = b;
  t.u32[1] = c;
  t.u32[0] = d;
  return t;
}

SIMD_INLINE c_v128 c_v128_load_unaligned(const void *p) {
  c_v128 t;
  memcpy(&t, p, 16);
  return t;
}

SIMD_INLINE c_v128 c_v128_load_aligned(const void *p) {
  if (SIMD_CHECK && (uintptr_t)p & 15) {
    fprintf(stderr, "Error: unaligned v128 load at %p\n", p);
    abort();
  }
  return c_v128_load_unaligned(p);
}

SIMD_INLINE void c_v128_store_unaligned(void *p, c_v128 a) {
  memcpy(p, &a, 16);
}

SIMD_INLINE void c_v128_store_aligned(void *p, c_v128 a) {
  if (SIMD_CHECK && (uintptr_t)p & 15) {
    fprintf(stderr, "Error: unaligned v128 store at %p\n", p);
    abort();
  }
  c_v128_store_unaligned(p, a);
}

SIMD_INLINE c_v128 c_v128_zero(void) {
  c_v128 t;
  t.u64[1] = t.u64[0] = 0;
  return t;
}

SIMD_INLINE c_v128 c_v128_dup_8(uint8_t x) {
  c_v128 t;
  t.v64[1] = t.v64[0] = c_v64_dup_8(x);
  return t;
}

SIMD_INLINE c_v128 c_v128_dup_16(uint16_t x) {
  c_v128 t;
  t.v64[1] = t.v64[0] = c_v64_dup_16(x);
  return t;
}

SIMD_INLINE c_v128 c_v128_dup_32(uint32_t x) {
  c_v128 t;
  t.v64[1] = t.v64[0] = c_v64_dup_32(x);
  return t;
}

SIMD_INLINE c_v128 c_v128_dup_64(uint64_t x) {
  c_v128 t;
  t.u64[1] = t.u64[0] = x;
  return t;
}

SIMD_INLINE int64_t c_v128_dotp_su8(c_v128 a, c_v128 b) {
  return c_v64_dotp_su8(a.v64[1], b.v64[1]) +
         c_v64_dotp_su8(a.v64[0], b.v64[0]);
}

SIMD_INLINE int64_t c_v128_dotp_s16(c_v128 a, c_v128 b) {
  return c_v64_dotp_s16(a.v64[1], b.v64[1]) +
         c_v64_dotp_s16(a.v64[0], b.v64[0]);
}

SIMD_INLINE int64_t c_v128_dotp_s32(c_v128 a, c_v128 b) {
  // 32 bit products, 64 bit sum
  return (int64_t)(int32_t)((int64_t)a.s32[3] * b.s32[3]) +
         (int64_t)(int32_t)((int64_t)a.s32[2] * b.s32[2]) +
         (int64_t)(int32_t)((int64_t)a.s32[1] * b.s32[1]) +
         (int64_t)(int32_t)((int64_t)a.s32[0] * b.s32[0]);
}

SIMD_INLINE uint64_t c_v128_hadd_u8(c_v128 a) {
  return c_v64_hadd_u8(a.v64[1]) + c_v64_hadd_u8(a.v64[0]);
}

typedef struct {
  uint32_t val;
  int count;
} c_sad128_internal;

SIMD_INLINE c_sad128_internal c_v128_sad_u8_init(void) {
  c_sad128_internal t;
  t.val = t.count = 0;
  return t;
}

/* Implementation dependent return value.  Result must be finalised with
 * v128_sad_u8_sum(). The result for more than 32 v128_sad_u8() calls is
 * undefined. */
SIMD_INLINE c_sad128_internal c_v128_sad_u8(c_sad128_internal s, c_v128 a,
                                            c_v128 b) {
  int c;
  for (c = 0; c < 16; c++)
    s.val += a.u8[c] > b.u8[c] ? a.u8[c] - b.u8[c] : b.u8[c] - a.u8[c];
  s.count++;
  if (SIMD_CHECK && s.count > 32) {
    fprintf(stderr,
            "Error: sad called 32 times returning an undefined result\n");
    abort();
  }
  return s;
}

SIMD_INLINE uint32_t c_v128_sad_u8_sum(c_sad128_internal s) { return s.val; }

typedef uint32_t c_ssd128_internal;

SIMD_INLINE c_ssd128_internal c_v128_ssd_u8_init(void) { return 0; }

/* Implementation dependent return value.  Result must be finalised with
 * v128_ssd_u8_sum(). */
SIMD_INLINE c_ssd128_internal c_v128_ssd_u8(c_ssd128_internal s, c_v128 a,
                                            c_v128 b) {
  int c;
  for (c = 0; c < 16; c++) s += (a.u8[c] - b.u8[c]) * (a.u8[c] - b.u8[c]);
  return s;
}

SIMD_INLINE uint32_t c_v128_ssd_u8_sum(c_ssd128_internal s) { return s; }

SIMD_INLINE c_v128 c_v128_or(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_or(a.v64[1], b.v64[1]),
                         c_v64_or(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_xor(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_xor(a.v64[1], b.v64[1]),
                         c_v64_xor(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_and(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_and(a.v64[1], b.v64[1]),
                         c_v64_and(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_andn(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_andn(a.v64[1], b.v64[1]),
                         c_v64_andn(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_add_8(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_add_8(a.v64[1], b.v64[1]),
                         c_v64_add_8(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_add_16(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_add_16(a.v64[1], b.v64[1]),
                         c_v64_add_16(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_sadd_u8(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_sadd_u8(a.v64[1], b.v64[1]),
                         c_v64_sadd_u8(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_sadd_s8(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_sadd_s8(a.v64[1], b.v64[1]),
                         c_v64_sadd_s8(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_sadd_s16(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_sadd_s16(a.v64[1], b.v64[1]),
                         c_v64_sadd_s16(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_add_32(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_add_32(a.v64[1], b.v64[1]),
                         c_v64_add_32(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_add_64(c_v128 a, c_v128 b) {
  // Two complement overflow (silences sanitizers)
  return c_v128_from_64(
      a.v64[1].u64 > ~b.v64[1].u64 ? a.v64[1].u64 - ~b.v64[1].u64 - 1
                                   : a.v64[1].u64 + b.v64[1].u64,
      a.v64[0].u64 > ~b.v64[0].u64 ? a.v64[0].u64 - ~b.v64[0].u64 - 1
                                   : a.v64[0].u64 + b.v64[0].u64);
}

SIMD_INLINE c_v128 c_v128_padd_s16(c_v128 a) {
  c_v128 t;
  t.s32[0] = (int32_t)a.s16[0] + (int32_t)a.s16[1];
  t.s32[1] = (int32_t)a.s16[2] + (int32_t)a.s16[3];
  t.s32[2] = (int32_t)a.s16[4] + (int32_t)a.s16[5];
  t.s32[3] = (int32_t)a.s16[6] + (int32_t)a.s16[7];
  return t;
}

SIMD_INLINE c_v128 c_v128_padd_u8(c_v128 a) {
  c_v128 t;
  t.u16[0] = (uint16_t)a.u8[0] + (uint16_t)a.u8[1];
  t.u16[1] = (uint16_t)a.u8[2] + (uint16_t)a.u8[3];
  t.u16[2] = (uint16_t)a.u8[4] + (uint16_t)a.u8[5];
  t.u16[3] = (uint16_t)a.u8[6] + (uint16_t)a.u8[7];
  t.u16[4] = (uint16_t)a.u8[8] + (uint16_t)a.u8[9];
  t.u16[5] = (uint16_t)a.u8[10] + (uint16_t)a.u8[11];
  t.u16[6] = (uint16_t)a.u8[12] + (uint16_t)a.u8[13];
  t.u16[7] = (uint16_t)a.u8[14] + (uint16_t)a.u8[15];
  return t;
}

SIMD_INLINE c_v128 c_v128_sub_8(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_sub_8(a.v64[1], b.v64[1]),
                         c_v64_sub_8(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_ssub_u8(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_ssub_u8(a.v64[1], b.v64[1]),
                         c_v64_ssub_u8(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_ssub_s8(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_ssub_s8(a.v64[1], b.v64[1]),
                         c_v64_ssub_s8(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_sub_16(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_sub_16(a.v64[1], b.v64[1]),
                         c_v64_sub_16(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_ssub_s16(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_ssub_s16(a.v64[1], b.v64[1]),
                         c_v64_ssub_s16(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_ssub_u16(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_ssub_u16(a.v64[1], b.v64[1]),
                         c_v64_ssub_u16(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_sub_32(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_sub_32(a.v64[1], b.v64[1]),
                         c_v64_sub_32(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_sub_64(c_v128 a, c_v128 b) {
  // Two complement underflow (silences sanitizers)
  return c_v128_from_64(
      a.v64[1].u64 < b.v64[1].u64 ? a.v64[1].u64 + ~b.v64[1].u64 + 1
                                  : a.v64[1].u64 - b.v64[1].u64,
      a.v64[0].u64 < b.v64[0].u64 ? a.v64[0].u64 + ~b.v64[0].u64 + 1
                                  : a.v64[0].u64 - b.v64[0].u64);
}

SIMD_INLINE c_v128 c_v128_abs_s16(c_v128 a) {
  return c_v128_from_v64(c_v64_abs_s16(a.v64[1]), c_v64_abs_s16(a.v64[0]));
}

SIMD_INLINE c_v128 c_v128_abs_s8(c_v128 a) {
  return c_v128_from_v64(c_v64_abs_s8(a.v64[1]), c_v64_abs_s8(a.v64[0]));
}

SIMD_INLINE c_v128 c_v128_mul_s16(c_v64 a, c_v64 b) {
  c_v64 lo_bits = c_v64_mullo_s16(a, b);
  c_v64 hi_bits = c_v64_mulhi_s16(a, b);
  return c_v128_from_v64(c_v64_ziphi_16(hi_bits, lo_bits),
                         c_v64_ziplo_16(hi_bits, lo_bits));
}

SIMD_INLINE c_v128 c_v128_mullo_s16(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_mullo_s16(a.v64[1], b.v64[1]),
                         c_v64_mullo_s16(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_mulhi_s16(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_mulhi_s16(a.v64[1], b.v64[1]),
                         c_v64_mulhi_s16(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_mullo_s32(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_mullo_s32(a.v64[1], b.v64[1]),
                         c_v64_mullo_s32(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_madd_s16(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_madd_s16(a.v64[1], b.v64[1]),
                         c_v64_madd_s16(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_madd_us8(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_madd_us8(a.v64[1], b.v64[1]),
                         c_v64_madd_us8(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_avg_u8(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_avg_u8(a.v64[1], b.v64[1]),
                         c_v64_avg_u8(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_rdavg_u8(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_rdavg_u8(a.v64[1], b.v64[1]),
                         c_v64_rdavg_u8(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_rdavg_u16(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_rdavg_u16(a.v64[1], b.v64[1]),
                         c_v64_rdavg_u16(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_avg_u16(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_avg_u16(a.v64[1], b.v64[1]),
                         c_v64_avg_u16(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_min_u8(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_min_u8(a.v64[1], b.v64[1]),
                         c_v64_min_u8(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_max_u8(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_max_u8(a.v64[1], b.v64[1]),
                         c_v64_max_u8(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_min_s8(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_min_s8(a.v64[1], b.v64[1]),
                         c_v64_min_s8(a.v64[0], b.v64[0]));
}

SIMD_INLINE uint32_t c_v128_movemask_8(c_v128 a) {
  return ((a.s8[15] < 0) << 15) | ((a.s8[14] < 0) << 14) |
         ((a.s8[13] < 0) << 13) | ((a.s8[12] < 0) << 12) |
         ((a.s8[11] < 0) << 11) | ((a.s8[10] < 0) << 10) |
         ((a.s8[9] < 0) << 9) | ((a.s8[8] < 0) << 8) | ((a.s8[7] < 0) << 7) |
         ((a.s8[6] < 0) << 6) | ((a.s8[5] < 0) << 5) | ((a.s8[4] < 0) << 4) |
         ((a.s8[3] < 0) << 3) | ((a.s8[2] < 0) << 2) | ((a.s8[1] < 0) << 1) |
         ((a.s8[0] < 0) << 0);
}

SIMD_INLINE c_v128 c_v128_blend_8(c_v128 a, c_v128 b, c_v128 c) {
  c_v128 t;
  for (int i = 0; i < 16; i++) t.u8[i] = c.s8[i] < 0 ? b.u8[i] : a.u8[i];
  return t;
}

SIMD_INLINE c_v128 c_v128_max_s8(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_max_s8(a.v64[1], b.v64[1]),
                         c_v64_max_s8(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_min_s16(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_min_s16(a.v64[1], b.v64[1]),
                         c_v64_min_s16(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_max_s16(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_max_s16(a.v64[1], b.v64[1]),
                         c_v64_max_s16(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_max_s32(c_v128 a, c_v128 b) {
  c_v128 t;
  int c;
  for (c = 0; c < 4; c++) t.s32[c] = a.s32[c] > b.s32[c] ? a.s32[c] : b.s32[c];
  return t;
}

SIMD_INLINE c_v128 c_v128_min_s32(c_v128 a, c_v128 b) {
  c_v128 t;
  int c;
  for (c = 0; c < 4; c++) t.s32[c] = a.s32[c] > b.s32[c] ? b.s32[c] : a.s32[c];
  return t;
}

SIMD_INLINE c_v128 c_v128_ziplo_8(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_ziphi_8(a.v64[0], b.v64[0]),
                         c_v64_ziplo_8(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_ziphi_8(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_ziphi_8(a.v64[1], b.v64[1]),
                         c_v64_ziplo_8(a.v64[1], b.v64[1]));
}

SIMD_INLINE c_v128 c_v128_ziplo_16(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_ziphi_16(a.v64[0], b.v64[0]),
                         c_v64_ziplo_16(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_ziphi_16(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_ziphi_16(a.v64[1], b.v64[1]),
                         c_v64_ziplo_16(a.v64[1], b.v64[1]));
}

SIMD_INLINE c_v128 c_v128_ziplo_32(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_ziphi_32(a.v64[0], b.v64[0]),
                         c_v64_ziplo_32(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_ziphi_32(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_ziphi_32(a.v64[1], b.v64[1]),
                         c_v64_ziplo_32(a.v64[1], b.v64[1]));
}

SIMD_INLINE c_v128 c_v128_ziplo_64(c_v128 a, c_v128 b) {
  return c_v128_from_v64(a.v64[0], b.v64[0]);
}

SIMD_INLINE c_v128 c_v128_ziphi_64(c_v128 a, c_v128 b) {
  return c_v128_from_v64(a.v64[1], b.v64[1]);
}

SIMD_INLINE c_v128 c_v128_zip_8(c_v64 a, c_v64 b) {
  return c_v128_from_v64(c_v64_ziphi_8(a, b), c_v64_ziplo_8(a, b));
}

SIMD_INLINE c_v128 c_v128_zip_16(c_v64 a, c_v64 b) {
  return c_v128_from_v64(c_v64_ziphi_16(a, b), c_v64_ziplo_16(a, b));
}

SIMD_INLINE c_v128 c_v128_zip_32(c_v64 a, c_v64 b) {
  return c_v128_from_v64(c_v64_ziphi_32(a, b), c_v64_ziplo_32(a, b));
}

SIMD_INLINE c_v128 _c_v128_unzip_8(c_v128 a, c_v128 b, int mode) {
  c_v128 t;
  if (mode) {
    t.u8[15] = b.u8[15];
    t.u8[14] = b.u8[13];
    t.u8[13] = b.u8[11];
    t.u8[12] = b.u8[9];
    t.u8[11] = b.u8[7];
    t.u8[10] = b.u8[5];
    t.u8[9] = b.u8[3];
    t.u8[8] = b.u8[1];
    t.u8[7] = a.u8[15];
    t.u8[6] = a.u8[13];
    t.u8[5] = a.u8[11];
    t.u8[4] = a.u8[9];
    t.u8[3] = a.u8[7];
    t.u8[2] = a.u8[5];
    t.u8[1] = a.u8[3];
    t.u8[0] = a.u8[1];
  } else {
    t.u8[15] = a.u8[14];
    t.u8[14] = a.u8[12];
    t.u8[13] = a.u8[10];
    t.u8[12] = a.u8[8];
    t.u8[11] = a.u8[6];
    t.u8[10] = a.u8[4];
    t.u8[9] = a.u8[2];
    t.u8[8] = a.u8[0];
    t.u8[7] = b.u8[14];
    t.u8[6] = b.u8[12];
    t.u8[5] = b.u8[10];
    t.u8[4] = b.u8[8];
    t.u8[3] = b.u8[6];
    t.u8[2] = b.u8[4];
    t.u8[1] = b.u8[2];
    t.u8[0] = b.u8[0];
  }
  return t;
}

SIMD_INLINE c_v128 c_v128_unziplo_8(c_v128 a, c_v128 b) {
  return CONFIG_BIG_ENDIAN ? _c_v128_unzip_8(a, b, 1)
                           : _c_v128_unzip_8(a, b, 0);
}

SIMD_INLINE c_v128 c_v128_unziphi_8(c_v128 a, c_v128 b) {
  return CONFIG_BIG_ENDIAN ? _c_v128_unzip_8(b, a, 0)
                           : _c_v128_unzip_8(b, a, 1);
}

SIMD_INLINE c_v128 _c_v128_unzip_16(c_v128 a, c_v128 b, int mode) {
  c_v128 t;
  if (mode) {
    t.u16[7] = b.u16[7];
    t.u16[6] = b.u16[5];
    t.u16[5] = b.u16[3];
    t.u16[4] = b.u16[1];
    t.u16[3] = a.u16[7];
    t.u16[2] = a.u16[5];
    t.u16[1] = a.u16[3];
    t.u16[0] = a.u16[1];
  } else {
    t.u16[7] = a.u16[6];
    t.u16[6] = a.u16[4];
    t.u16[5] = a.u16[2];
    t.u16[4] = a.u16[0];
    t.u16[3] = b.u16[6];
    t.u16[2] = b.u16[4];
    t.u16[1] = b.u16[2];
    t.u16[0] = b.u16[0];
  }
  return t;
}

SIMD_INLINE c_v128 c_v128_unziplo_16(c_v128 a, c_v128 b) {
  return CONFIG_BIG_ENDIAN ? _c_v128_unzip_16(a, b, 1)
                           : _c_v128_unzip_16(a, b, 0);
}

SIMD_INLINE c_v128 c_v128_unziphi_16(c_v128 a, c_v128 b) {
  return CONFIG_BIG_ENDIAN ? _c_v128_unzip_16(b, a, 0)
                           : _c_v128_unzip_16(b, a, 1);
}

SIMD_INLINE c_v128 _c_v128_unzip_32(c_v128 a, c_v128 b, int mode) {
  c_v128 t;
  if (mode) {
    t.u32[3] = b.u32[3];
    t.u32[2] = b.u32[1];
    t.u32[1] = a.u32[3];
    t.u32[0] = a.u32[1];
  } else {
    t.u32[3] = a.u32[2];
    t.u32[2] = a.u32[0];
    t.u32[1] = b.u32[2];
    t.u32[0] = b.u32[0];
  }
  return t;
}

SIMD_INLINE c_v128 c_v128_unziplo_32(c_v128 a, c_v128 b) {
  return CONFIG_BIG_ENDIAN ? _c_v128_unzip_32(a, b, 1)
                           : _c_v128_unzip_32(a, b, 0);
}

SIMD_INLINE c_v128 c_v128_unziphi_32(c_v128 a, c_v128 b) {
  return CONFIG_BIG_ENDIAN ? _c_v128_unzip_32(b, a, 0)
                           : _c_v128_unzip_32(b, a, 1);
}

SIMD_INLINE c_v128 c_v128_unpack_u8_s16(c_v64 a) {
  return c_v128_from_v64(c_v64_unpackhi_u8_s16(a), c_v64_unpacklo_u8_s16(a));
}

SIMD_INLINE c_v128 c_v128_unpacklo_u8_s16(c_v128 a) {
  return c_v128_from_v64(c_v64_unpackhi_u8_s16(a.v64[0]),
                         c_v64_unpacklo_u8_s16(a.v64[0]));
}

SIMD_INLINE c_v128 c_v128_unpackhi_u8_s16(c_v128 a) {
  return c_v128_from_v64(c_v64_unpackhi_u8_s16(a.v64[1]),
                         c_v64_unpacklo_u8_s16(a.v64[1]));
}

SIMD_INLINE c_v128 c_v128_unpack_s8_s16(c_v64 a) {
  return c_v128_from_v64(c_v64_unpackhi_s8_s16(a), c_v64_unpacklo_s8_s16(a));
}

SIMD_INLINE c_v128 c_v128_unpacklo_s8_s16(c_v128 a) {
  return c_v128_from_v64(c_v64_unpackhi_s8_s16(a.v64[0]),
                         c_v64_unpacklo_s8_s16(a.v64[0]));
}

SIMD_INLINE c_v128 c_v128_unpackhi_s8_s16(c_v128 a) {
  return c_v128_from_v64(c_v64_unpackhi_s8_s16(a.v64[1]),
                         c_v64_unpacklo_s8_s16(a.v64[1]));
}

SIMD_INLINE c_v128 c_v128_pack_s32_s16(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_pack_s32_s16(a.v64[1], a.v64[0]),
                         c_v64_pack_s32_s16(b.v64[1], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_pack_s32_u16(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_pack_s32_u16(a.v64[1], a.v64[0]),
                         c_v64_pack_s32_u16(b.v64[1], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_pack_s16_u8(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_pack_s16_u8(a.v64[1], a.v64[0]),
                         c_v64_pack_s16_u8(b.v64[1], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_pack_s16_s8(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_pack_s16_s8(a.v64[1], a.v64[0]),
                         c_v64_pack_s16_s8(b.v64[1], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_unpack_u16_s32(c_v64 a) {
  return c_v128_from_v64(c_v64_unpackhi_u16_s32(a), c_v64_unpacklo_u16_s32(a));
}

SIMD_INLINE c_v128 c_v128_unpack_s16_s32(c_v64 a) {
  return c_v128_from_v64(c_v64_unpackhi_s16_s32(a), c_v64_unpacklo_s16_s32(a));
}

SIMD_INLINE c_v128 c_v128_unpacklo_u16_s32(c_v128 a) {
  return c_v128_from_v64(c_v64_unpackhi_u16_s32(a.v64[0]),
                         c_v64_unpacklo_u16_s32(a.v64[0]));
}

SIMD_INLINE c_v128 c_v128_unpacklo_s16_s32(c_v128 a) {
  return c_v128_from_v64(c_v64_unpackhi_s16_s32(a.v64[0]),
                         c_v64_unpacklo_s16_s32(a.v64[0]));
}

SIMD_INLINE c_v128 c_v128_unpackhi_u16_s32(c_v128 a) {
  return c_v128_from_v64(c_v64_unpackhi_u16_s32(a.v64[1]),
                         c_v64_unpacklo_u16_s32(a.v64[1]));
}

SIMD_INLINE c_v128 c_v128_unpackhi_s16_s32(c_v128 a) {
  return c_v128_from_v64(c_v64_unpackhi_s16_s32(a.v64[1]),
                         c_v64_unpacklo_s16_s32(a.v64[1]));
}

SIMD_INLINE c_v128 c_v128_shuffle_8(c_v128 a, c_v128 pattern) {
  c_v128 t;
  int c;
  for (c = 0; c < 16; c++)
    t.u8[c] = a.u8[CONFIG_BIG_ENDIAN ? 15 - (pattern.u8[c] & 15)
                                     : pattern.u8[c] & 15];

  return t;
}

SIMD_INLINE c_v128 c_v128_cmpgt_s8(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_cmpgt_s8(a.v64[1], b.v64[1]),
                         c_v64_cmpgt_s8(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_cmplt_s8(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_cmplt_s8(a.v64[1], b.v64[1]),
                         c_v64_cmplt_s8(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_cmpeq_8(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_cmpeq_8(a.v64[1], b.v64[1]),
                         c_v64_cmpeq_8(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_cmpgt_s16(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_cmpgt_s16(a.v64[1], b.v64[1]),
                         c_v64_cmpgt_s16(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_cmplt_s16(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_cmplt_s16(a.v64[1], b.v64[1]),
                         c_v64_cmplt_s16(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_cmpeq_16(c_v128 a, c_v128 b) {
  return c_v128_from_v64(c_v64_cmpeq_16(a.v64[1], b.v64[1]),
                         c_v64_cmpeq_16(a.v64[0], b.v64[0]));
}

SIMD_INLINE c_v128 c_v128_cmpgt_s32(c_v128 a, c_v128 b) {
  c_v128 t;
  int c;
  for (c = 0; c < 4; c++) t.s32[c] = -(a.s32[c] > b.s32[c]);
  return t;
}

SIMD_INLINE c_v128 c_v128_cmplt_s32(c_v128 a, c_v128 b) {
  c_v128 t;
  int c;
  for (c = 0; c < 4; c++) t.s32[c] = -(a.s32[c] < b.s32[c]);
  return t;
}

SIMD_INLINE c_v128 c_v128_cmpeq_32(c_v128 a, c_v128 b) {
  c_v128 t;
  int c;
  for (c = 0; c < 4; c++) t.s32[c] = -(a.s32[c] == b.s32[c]);
  return t;
}

SIMD_INLINE c_v128 c_v128_shl_n_byte(c_v128 a, const unsigned int n) {
  if (n == 0) return a;
  if (n < 8)
    return c_v128_from_v64(c_v64_or(c_v64_shl_n_byte(a.v64[1], n),
                                    c_v64_shr_n_byte(a.v64[0], 8 - n)),
                           c_v64_shl_n_byte(a.v64[0], n));
  else
    return c_v128_from_v64(c_v64_shl_n_byte(a.v64[0], n - 8), c_v64_zero());
}

SIMD_INLINE c_v128 c_v128_shr_n_byte(c_v128 a, const unsigned int n) {
  if (n == 0) return a;
  if (n < 8)
    return c_v128_from_v64(c_v64_shr_n_byte(a.v64[1], n),
                           c_v64_or(c_v64_shr_n_byte(a.v64[0], n),
                                    c_v64_shl_n_byte(a.v64[1], 8 - n)));
  else
    return c_v128_from_v64(c_v64_zero(), c_v64_shr_n_byte(a.v64[1], n - 8));
}

SIMD_INLINE c_v128 c_v128_align(c_v128 a, c_v128 b, const unsigned int c) {
  if (SIMD_CHECK && c > 15) {
    fprintf(stderr, "Error: undefined alignment %d\n", c);
    abort();
  }
  return c ? c_v128_or(c_v128_shr_n_byte(b, c), c_v128_shl_n_byte(a, 16 - c))
           : b;
}

SIMD_INLINE c_v128 c_v128_shl_8(c_v128 a, const unsigned int c) {
  return c_v128_from_v64(c_v64_shl_8(a.v64[1], c), c_v64_shl_8(a.v64[0], c));
}

SIMD_INLINE c_v128 c_v128_shr_u8(c_v128 a, const unsigned int c) {
  return c_v128_from_v64(c_v64_shr_u8(a.v64[1], c), c_v64_shr_u8(a.v64[0], c));
}

SIMD_INLINE c_v128 c_v128_shr_s8(c_v128 a, const unsigned int c) {
  return c_v128_from_v64(c_v64_shr_s8(a.v64[1], c), c_v64_shr_s8(a.v64[0], c));
}

SIMD_INLINE c_v128 c_v128_shl_16(c_v128 a, const unsigned int c) {
  return c_v128_from_v64(c_v64_shl_16(a.v64[1], c), c_v64_shl_16(a.v64[0], c));
}

SIMD_INLINE c_v128 c_v128_shr_u16(c_v128 a, const unsigned int c) {
  return c_v128_from_v64(c_v64_shr_u16(a.v64[1], c),
                         c_v64_shr_u16(a.v64[0], c));
}

SIMD_INLINE c_v128 c_v128_shr_s16(c_v128 a, const unsigned int c) {
  return c_v128_from_v64(c_v64_shr_s16(a.v64[1], c),
                         c_v64_shr_s16(a.v64[0], c));
}

SIMD_INLINE c_v128 c_v128_shl_32(c_v128 a, const unsigned int c) {
  return c_v128_from_v64(c_v64_shl_32(a.v64[1], c), c_v64_shl_32(a.v64[0], c));
}

SIMD_INLINE c_v128 c_v128_shr_u32(c_v128 a, const unsigned int c) {
  return c_v128_from_v64(c_v64_shr_u32(a.v64[1], c),
                         c_v64_shr_u32(a.v64[0], c));
}

SIMD_INLINE c_v128 c_v128_shr_s32(c_v128 a, const unsigned int c) {
  return c_v128_from_v64(c_v64_shr_s32(a.v64[1], c),
                         c_v64_shr_s32(a.v64[0], c));
}

SIMD_INLINE c_v128 c_v128_shl_64(c_v128 a, const unsigned int c) {
  a.v64[1].u64 <<= c;
  a.v64[0].u64 <<= c;
  return c_v128_from_v64(a.v64[1], a.v64[0]);
}

SIMD_INLINE c_v128 c_v128_shr_u64(c_v128 a, const unsigned int c) {
  a.v64[1].u64 >>= c;
  a.v64[0].u64 >>= c;
  return c_v128_from_v64(a.v64[1], a.v64[0]);
}

SIMD_INLINE c_v128 c_v128_shr_s64(c_v128 a, const unsigned int c) {
  a.v64[1].s64 >>= c;
  a.v64[0].s64 >>= c;
  return c_v128_from_v64(a.v64[1], a.v64[0]);
}

SIMD_INLINE c_v128 c_v128_shl_n_8(c_v128 a, const unsigned int n) {
  return c_v128_shl_8(a, n);
}

SIMD_INLINE c_v128 c_v128_shl_n_16(c_v128 a, const unsigned int n) {
  return c_v128_shl_16(a, n);
}

SIMD_INLINE c_v128 c_v128_shl_n_32(c_v128 a, const unsigned int n) {
  return c_v128_shl_32(a, n);
}

SIMD_INLINE c_v128 c_v128_shl_n_64(c_v128 a, const unsigned int n) {
  return c_v128_shl_64(a, n);
}

SIMD_INLINE c_v128 c_v128_shr_n_u8(c_v128 a, const unsigned int n) {
  return c_v128_shr_u8(a, n);
}

SIMD_INLINE c_v128 c_v128_shr_n_u16(c_v128 a, const unsigned int n) {
  return c_v128_shr_u16(a, n);
}

SIMD_INLINE c_v128 c_v128_shr_n_u32(c_v128 a, const unsigned int n) {
  return c_v128_shr_u32(a, n);
}

SIMD_INLINE c_v128 c_v128_shr_n_u64(c_v128 a, const unsigned int n) {
  return c_v128_shr_u64(a, n);
}

SIMD_INLINE c_v128 c_v128_shr_n_s8(c_v128 a, const unsigned int n) {
  return c_v128_shr_s8(a, n);
}

SIMD_INLINE c_v128 c_v128_shr_n_s16(c_v128 a, const unsigned int n) {
  return c_v128_shr_s16(a, n);
}

SIMD_INLINE c_v128 c_v128_shr_n_s32(c_v128 a, const unsigned int n) {
  return c_v128_shr_s32(a, n);
}

SIMD_INLINE c_v128 c_v128_shr_n_s64(c_v128 a, const unsigned int n) {
  return c_v128_shr_s64(a, n);
}

typedef uint32_t c_sad128_internal_u16;

SIMD_INLINE c_sad128_internal_u16 c_v128_sad_u16_init(void) { return 0; }

/* Implementation dependent return value.  Result must be finalised with
 * v128_sad_u16_sum(). */
SIMD_INLINE c_sad128_internal_u16 c_v128_sad_u16(c_sad128_internal_u16 s,
                                                 c_v128 a, c_v128 b) {
  int c;
  for (c = 0; c < 8; c++)
    s += a.u16[c] > b.u16[c] ? a.u16[c] - b.u16[c] : b.u16[c] - a.u16[c];
  return s;
}

SIMD_INLINE uint32_t c_v128_sad_u16_sum(c_sad128_internal_u16 s) { return s; }

typedef uint64_t c_ssd128_internal_s16;

SIMD_INLINE c_ssd128_internal_s16 c_v128_ssd_s16_init(void) { return 0; }

/* Implementation dependent return value.  Result must be finalised with
 * v128_ssd_s16_sum(). */
SIMD_INLINE c_ssd128_internal_s16 c_v128_ssd_s16(c_ssd128_internal_s16 s,
                                                 c_v128 a, c_v128 b) {
  int c;
  for (c = 0; c < 8; c++)
    s += (int32_t)(int16_t)(a.s16[c] - b.s16[c]) *
         (int32_t)(int16_t)(a.s16[c] - b.s16[c]);
  return s;
}

SIMD_INLINE uint64_t c_v128_ssd_s16_sum(c_ssd128_internal_s16 s) { return s; }

#endif  // AOM_AOM_DSP_SIMD_V128_INTRINSICS_C_H_
