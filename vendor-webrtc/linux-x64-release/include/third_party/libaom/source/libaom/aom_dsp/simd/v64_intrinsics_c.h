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

#ifndef AOM_AOM_DSP_SIMD_V64_INTRINSICS_C_H_
#define AOM_AOM_DSP_SIMD_V64_INTRINSICS_C_H_

/* Note: This implements the intrinsics in plain, unoptimised C.
   Intended for reference, porting or debugging. */

#include <stdio.h>
#include <stdlib.h>

#include "config/aom_config.h"

typedef union {
  uint8_t u8[8];
  uint16_t u16[4];
  uint32_t u32[2];
  uint64_t u64;
  int8_t s8[8];
  int16_t s16[4];
  int32_t s32[2];
  int64_t s64;
} c_v64;

SIMD_INLINE uint32_t c_v64_low_u32(c_v64 a) {
  return a.u32[!!CONFIG_BIG_ENDIAN];
}

SIMD_INLINE uint32_t c_v64_high_u32(c_v64 a) {
  return a.u32[!CONFIG_BIG_ENDIAN];
}

SIMD_INLINE int32_t c_v64_low_s32(c_v64 a) {
  return a.s32[!!CONFIG_BIG_ENDIAN];
}

SIMD_INLINE int32_t c_v64_high_s32(c_v64 a) {
  return a.s32[!CONFIG_BIG_ENDIAN];
}

SIMD_INLINE c_v64 c_v64_from_32(uint32_t x, uint32_t y) {
  c_v64 t;
  t.u32[!CONFIG_BIG_ENDIAN] = x;
  t.u32[!!CONFIG_BIG_ENDIAN] = y;
  return t;
}

SIMD_INLINE c_v64 c_v64_from_64(uint64_t x) {
  c_v64 t;
  t.u64 = x;
  return t;
}

SIMD_INLINE uint64_t c_v64_u64(c_v64 x) { return x.u64; }

SIMD_INLINE c_v64 c_v64_from_16(uint16_t a, uint16_t b, uint16_t c,
                                uint16_t d) {
  c_v64 t;
  if (CONFIG_BIG_ENDIAN) {
    t.u16[0] = a;
    t.u16[1] = b;
    t.u16[2] = c;
    t.u16[3] = d;
  } else {
    t.u16[3] = a;
    t.u16[2] = b;
    t.u16[1] = c;
    t.u16[0] = d;
  }
  return t;
}

SIMD_INLINE uint32_t c_u32_load_unaligned(const void *p) {
  uint32_t t;
  uint8_t *pp = (uint8_t *)p;
  uint8_t *q = (uint8_t *)&t;
  int c;
  for (c = 0; c < 4; c++) q[c] = pp[c];
  return t;
}

SIMD_INLINE void c_u32_store_unaligned(void *p, uint32_t a) {
  uint8_t *pp = (uint8_t *)p;
  uint8_t *q = (uint8_t *)&a;
  int c;
  for (c = 0; c < 4; c++) pp[c] = q[c];
}

SIMD_INLINE uint32_t c_u32_load_aligned(const void *p) {
  if (SIMD_CHECK && (uintptr_t)p & 3) {
    fprintf(stderr, "Error: Unaligned u32 load at %p\n", p);
    abort();
  }
  return c_u32_load_unaligned(p);
}

SIMD_INLINE void c_u32_store_aligned(void *p, uint32_t a) {
  if (SIMD_CHECK && (uintptr_t)p & 3) {
    fprintf(stderr, "Error: Unaligned u32 store at %p\n", p);
    abort();
  }
  c_u32_store_unaligned(p, a);
}

SIMD_INLINE c_v64 c_v64_load_unaligned(const void *p) {
  c_v64 t;
  uint8_t *pp = (uint8_t *)p;
  uint8_t *q = (uint8_t *)&t;
  int c;
  for (c = 0; c < 8; c++) q[c] = pp[c];
  return t;
}

SIMD_INLINE c_v64 c_v64_load_aligned(const void *p) {
  if (SIMD_CHECK && (uintptr_t)p & 7) {
    fprintf(stderr, "Error: Unaligned c_v64 load at %p\n", p);
    abort();
  }
  return c_v64_load_unaligned(p);
}

SIMD_INLINE void c_v64_store_unaligned(void *p, c_v64 a) {
  uint8_t *q = (uint8_t *)p;
  uint8_t *r = (uint8_t *)&a;
  int c;
  for (c = 0; c < 8; c++) q[c] = r[c];
}

SIMD_INLINE void c_v64_store_aligned(void *p, c_v64 a) {
  if (SIMD_CHECK && (uintptr_t)p & 7) {
    fprintf(stderr, "Error: Unaligned c_v64 store at %p\n", p);
    abort();
  }
  c_v64_store_unaligned(p, a);
}

SIMD_INLINE c_v64 c_v64_zero(void) {
  c_v64 t;
  t.u64 = 0;
  return t;
}

SIMD_INLINE c_v64 c_v64_dup_8(uint8_t x) {
  c_v64 t;
  t.u8[0] = t.u8[1] = t.u8[2] = t.u8[3] = t.u8[4] = t.u8[5] = t.u8[6] =
      t.u8[7] = x;
  return t;
}

SIMD_INLINE c_v64 c_v64_dup_16(uint16_t x) {
  c_v64 t;
  t.u16[0] = t.u16[1] = t.u16[2] = t.u16[3] = x;
  return t;
}

SIMD_INLINE c_v64 c_v64_dup_32(uint32_t x) {
  c_v64 t;
  t.u32[0] = t.u32[1] = x;
  return t;
}

SIMD_INLINE c_v64 c_v64_add_8(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 8; c++) t.u8[c] = (uint8_t)(a.u8[c] + b.u8[c]);
  return t;
}

SIMD_INLINE c_v64 c_v64_add_16(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 4; c++) t.u16[c] = (uint16_t)(a.u16[c] + b.u16[c]);
  return t;
}

SIMD_INLINE c_v64 c_v64_sadd_u8(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 8; c++)
    t.u8[c] = SIMD_CLAMP((int16_t)a.u8[c] + (int16_t)b.u8[c], 0, 255);
  return t;
}

SIMD_INLINE c_v64 c_v64_sadd_s8(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 8; c++)
    t.s8[c] = SIMD_CLAMP((int16_t)a.s8[c] + (int16_t)b.s8[c], -128, 127);
  return t;
}

SIMD_INLINE c_v64 c_v64_sadd_s16(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 4; c++)
    t.s16[c] = SIMD_CLAMP((int32_t)a.s16[c] + (int32_t)b.s16[c], -32768, 32767);
  return t;
}

SIMD_INLINE c_v64 c_v64_add_32(c_v64 a, c_v64 b) {
  c_v64 t;
  t.u32[0] = (uint32_t)((uint64_t)a.u32[0] + b.u32[0]);
  t.u32[1] = (uint32_t)((uint64_t)a.u32[1] + b.u32[1]);
  return t;
}

SIMD_INLINE c_v64 c_v64_sub_8(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 8; c++) t.u8[c] = (uint8_t)(a.u8[c] - b.u8[c]);
  return t;
}

SIMD_INLINE c_v64 c_v64_ssub_u8(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 8; c++) t.u8[c] = a.u8[c] < b.u8[c] ? 0 : a.u8[c] - b.u8[c];
  return t;
}

SIMD_INLINE c_v64 c_v64_ssub_s8(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 8; c++) {
    int16_t d = (int16_t)a.s8[c] - (int16_t)b.s8[c];
    t.s8[c] = SIMD_CLAMP(d, -128, 127);
  }
  return t;
}

SIMD_INLINE c_v64 c_v64_sub_16(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 4; c++) t.u16[c] = (uint16_t)(a.u16[c] - b.u16[c]);
  return t;
}

SIMD_INLINE c_v64 c_v64_ssub_s16(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 4; c++)
    t.s16[c] = SIMD_CLAMP((int32_t)a.s16[c] - (int32_t)b.s16[c], -32768, 32767);
  return t;
}

SIMD_INLINE c_v64 c_v64_ssub_u16(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 4; c++)
    t.u16[c] =
        (int32_t)a.u16[c] - (int32_t)b.u16[c] < 0 ? 0 : a.u16[c] - b.u16[c];
  return t;
}

SIMD_INLINE c_v64 c_v64_sub_32(c_v64 a, c_v64 b) {
  c_v64 t;
  t.u32[0] = (uint32_t)((int64_t)a.u32[0] - b.u32[0]);
  t.u32[1] = (uint32_t)((int64_t)a.u32[1] - b.u32[1]);
  return t;
}

SIMD_INLINE c_v64 c_v64_abs_s16(c_v64 a) {
  c_v64 t;
  int c;
  for (c = 0; c < 4; c++)
    t.u16[c] = (uint16_t)((int16_t)a.u16[c] > 0 ? a.u16[c] : -a.u16[c]);
  return t;
}

SIMD_INLINE c_v64 c_v64_abs_s8(c_v64 a) {
  c_v64 t;
  int c;
  for (c = 0; c < 8; c++)
    t.u8[c] = (uint8_t)((int8_t)a.u8[c] > 0 ? a.u8[c] : -a.u8[c]);
  return t;
}

SIMD_INLINE c_v64 _c_v64_zip_8(c_v64 a, c_v64 b, int mode) {
  c_v64 t;
  if (mode) {
    t.u8[7] = a.u8[7];
    t.u8[6] = b.u8[7];
    t.u8[5] = a.u8[6];
    t.u8[4] = b.u8[6];
    t.u8[3] = a.u8[5];
    t.u8[2] = b.u8[5];
    t.u8[1] = a.u8[4];
    t.u8[0] = b.u8[4];
  } else {
    t.u8[7] = a.u8[3];
    t.u8[6] = b.u8[3];
    t.u8[5] = a.u8[2];
    t.u8[4] = b.u8[2];
    t.u8[3] = a.u8[1];
    t.u8[2] = b.u8[1];
    t.u8[1] = a.u8[0];
    t.u8[0] = b.u8[0];
  }
  return t;
}

SIMD_INLINE c_v64 c_v64_ziplo_8(c_v64 a, c_v64 b) {
  return CONFIG_BIG_ENDIAN ? _c_v64_zip_8(b, a, 1) : _c_v64_zip_8(a, b, 0);
}

SIMD_INLINE c_v64 c_v64_ziphi_8(c_v64 a, c_v64 b) {
  return CONFIG_BIG_ENDIAN ? _c_v64_zip_8(b, a, 0) : _c_v64_zip_8(a, b, 1);
}

SIMD_INLINE c_v64 _c_v64_zip_16(c_v64 a, c_v64 b, int mode) {
  c_v64 t;
  if (mode) {
    t.u16[3] = a.u16[3];
    t.u16[2] = b.u16[3];
    t.u16[1] = a.u16[2];
    t.u16[0] = b.u16[2];
  } else {
    t.u16[3] = a.u16[1];
    t.u16[2] = b.u16[1];
    t.u16[1] = a.u16[0];
    t.u16[0] = b.u16[0];
  }
  return t;
}

SIMD_INLINE c_v64 c_v64_ziplo_16(c_v64 a, c_v64 b) {
  return CONFIG_BIG_ENDIAN ? _c_v64_zip_16(b, a, 1) : _c_v64_zip_16(a, b, 0);
}

SIMD_INLINE c_v64 c_v64_ziphi_16(c_v64 a, c_v64 b) {
  return CONFIG_BIG_ENDIAN ? _c_v64_zip_16(b, a, 0) : _c_v64_zip_16(a, b, 1);
}

SIMD_INLINE c_v64 _c_v64_zip_32(c_v64 a, c_v64 b, int mode) {
  c_v64 t;
  if (mode) {
    t.u32[1] = a.u32[1];
    t.u32[0] = b.u32[1];
  } else {
    t.u32[1] = a.u32[0];
    t.u32[0] = b.u32[0];
  }
  return t;
}

SIMD_INLINE c_v64 c_v64_ziplo_32(c_v64 a, c_v64 b) {
  return CONFIG_BIG_ENDIAN ? _c_v64_zip_32(b, a, 1) : _c_v64_zip_32(a, b, 0);
}

SIMD_INLINE c_v64 c_v64_ziphi_32(c_v64 a, c_v64 b) {
  return CONFIG_BIG_ENDIAN ? _c_v64_zip_32(b, a, 0) : _c_v64_zip_32(a, b, 1);
}

SIMD_INLINE c_v64 _c_v64_unzip_8(c_v64 a, c_v64 b, int mode) {
  c_v64 t;
  if (mode) {
    t.u8[7] = b.u8[7];
    t.u8[6] = b.u8[5];
    t.u8[5] = b.u8[3];
    t.u8[4] = b.u8[1];
    t.u8[3] = a.u8[7];
    t.u8[2] = a.u8[5];
    t.u8[1] = a.u8[3];
    t.u8[0] = a.u8[1];
  } else {
    t.u8[7] = a.u8[6];
    t.u8[6] = a.u8[4];
    t.u8[5] = a.u8[2];
    t.u8[4] = a.u8[0];
    t.u8[3] = b.u8[6];
    t.u8[2] = b.u8[4];
    t.u8[1] = b.u8[2];
    t.u8[0] = b.u8[0];
  }
  return t;
}

SIMD_INLINE c_v64 c_v64_unziplo_8(c_v64 a, c_v64 b) {
  return CONFIG_BIG_ENDIAN ? _c_v64_unzip_8(a, b, 1) : _c_v64_unzip_8(a, b, 0);
}

SIMD_INLINE c_v64 c_v64_unziphi_8(c_v64 a, c_v64 b) {
  return CONFIG_BIG_ENDIAN ? _c_v64_unzip_8(b, a, 0) : _c_v64_unzip_8(b, a, 1);
}

SIMD_INLINE c_v64 _c_v64_unzip_16(c_v64 a, c_v64 b, int mode) {
  c_v64 t;
  if (mode) {
    t.u16[3] = b.u16[3];
    t.u16[2] = b.u16[1];
    t.u16[1] = a.u16[3];
    t.u16[0] = a.u16[1];
  } else {
    t.u16[3] = a.u16[2];
    t.u16[2] = a.u16[0];
    t.u16[1] = b.u16[2];
    t.u16[0] = b.u16[0];
  }
  return t;
}

SIMD_INLINE c_v64 c_v64_unziplo_16(c_v64 a, c_v64 b) {
  return CONFIG_BIG_ENDIAN ? _c_v64_unzip_16(a, b, 1)
                           : _c_v64_unzip_16(a, b, 0);
}

SIMD_INLINE c_v64 c_v64_unziphi_16(c_v64 a, c_v64 b) {
  return CONFIG_BIG_ENDIAN ? _c_v64_unzip_16(b, a, 0)
                           : _c_v64_unzip_16(b, a, 1);
}

SIMD_INLINE c_v64 c_v64_unpacklo_u8_s16(c_v64 a) {
  c_v64 t;
  int endian = !!CONFIG_BIG_ENDIAN * 4;
  t.s16[3] = (int16_t)a.u8[3 + endian];
  t.s16[2] = (int16_t)a.u8[2 + endian];
  t.s16[1] = (int16_t)a.u8[1 + endian];
  t.s16[0] = (int16_t)a.u8[0 + endian];
  return t;
}

SIMD_INLINE c_v64 c_v64_unpackhi_u8_s16(c_v64 a) {
  c_v64 t;
  int endian = !!CONFIG_BIG_ENDIAN * 4;
  t.s16[3] = (int16_t)a.u8[7 - endian];
  t.s16[2] = (int16_t)a.u8[6 - endian];
  t.s16[1] = (int16_t)a.u8[5 - endian];
  t.s16[0] = (int16_t)a.u8[4 - endian];
  return t;
}

SIMD_INLINE c_v64 c_v64_unpacklo_s8_s16(c_v64 a) {
  c_v64 t;
  int endian = !!CONFIG_BIG_ENDIAN * 4;
  t.s16[3] = (int16_t)a.s8[3 + endian];
  t.s16[2] = (int16_t)a.s8[2 + endian];
  t.s16[1] = (int16_t)a.s8[1 + endian];
  t.s16[0] = (int16_t)a.s8[0 + endian];
  return t;
}

SIMD_INLINE c_v64 c_v64_unpackhi_s8_s16(c_v64 a) {
  c_v64 t;
  int endian = !!CONFIG_BIG_ENDIAN * 4;
  t.s16[3] = (int16_t)a.s8[7 - endian];
  t.s16[2] = (int16_t)a.s8[6 - endian];
  t.s16[1] = (int16_t)a.s8[5 - endian];
  t.s16[0] = (int16_t)a.s8[4 - endian];
  return t;
}

SIMD_INLINE c_v64 c_v64_pack_s32_s16(c_v64 a, c_v64 b) {
  c_v64 t;
  if (CONFIG_BIG_ENDIAN) {
    c_v64 u = a;
    a = b;
    b = u;
  }
  t.s16[3] = SIMD_CLAMP(a.s32[1], -32768, 32767);
  t.s16[2] = SIMD_CLAMP(a.s32[0], -32768, 32767);
  t.s16[1] = SIMD_CLAMP(b.s32[1], -32768, 32767);
  t.s16[0] = SIMD_CLAMP(b.s32[0], -32768, 32767);
  return t;
}

SIMD_INLINE c_v64 c_v64_pack_s32_u16(c_v64 a, c_v64 b) {
  c_v64 t;
  if (CONFIG_BIG_ENDIAN) {
    c_v64 u = a;
    a = b;
    b = u;
  }
  t.u16[3] = SIMD_CLAMP(a.s32[1], 0, 65535);
  t.u16[2] = SIMD_CLAMP(a.s32[0], 0, 65535);
  t.u16[1] = SIMD_CLAMP(b.s32[1], 0, 65535);
  t.u16[0] = SIMD_CLAMP(b.s32[0], 0, 65535);
  return t;
}

SIMD_INLINE c_v64 c_v64_pack_s16_u8(c_v64 a, c_v64 b) {
  c_v64 t;
  if (CONFIG_BIG_ENDIAN) {
    c_v64 u = a;
    a = b;
    b = u;
  }
  t.u8[7] = SIMD_CLAMP(a.s16[3], 0, 255);
  t.u8[6] = SIMD_CLAMP(a.s16[2], 0, 255);
  t.u8[5] = SIMD_CLAMP(a.s16[1], 0, 255);
  t.u8[4] = SIMD_CLAMP(a.s16[0], 0, 255);
  t.u8[3] = SIMD_CLAMP(b.s16[3], 0, 255);
  t.u8[2] = SIMD_CLAMP(b.s16[2], 0, 255);
  t.u8[1] = SIMD_CLAMP(b.s16[1], 0, 255);
  t.u8[0] = SIMD_CLAMP(b.s16[0], 0, 255);
  return t;
}

SIMD_INLINE c_v64 c_v64_pack_s16_s8(c_v64 a, c_v64 b) {
  c_v64 t;
  if (CONFIG_BIG_ENDIAN) {
    c_v64 u = a;
    a = b;
    b = u;
  }
  t.s8[7] = SIMD_CLAMP(a.s16[3], -128, 127);
  t.s8[6] = SIMD_CLAMP(a.s16[2], -128, 127);
  t.s8[5] = SIMD_CLAMP(a.s16[1], -128, 127);
  t.s8[4] = SIMD_CLAMP(a.s16[0], -128, 127);
  t.s8[3] = SIMD_CLAMP(b.s16[3], -128, 127);
  t.s8[2] = SIMD_CLAMP(b.s16[2], -128, 127);
  t.s8[1] = SIMD_CLAMP(b.s16[1], -128, 127);
  t.s8[0] = SIMD_CLAMP(b.s16[0], -128, 127);
  return t;
}

SIMD_INLINE c_v64 c_v64_unpacklo_u16_s32(c_v64 a) {
  c_v64 t;
  t.s32[1] = a.u16[1 + !!CONFIG_BIG_ENDIAN * 2];
  t.s32[0] = a.u16[0 + !!CONFIG_BIG_ENDIAN * 2];
  return t;
}

SIMD_INLINE c_v64 c_v64_unpacklo_s16_s32(c_v64 a) {
  c_v64 t;
  t.s32[1] = a.s16[1 + !!CONFIG_BIG_ENDIAN * 2];
  t.s32[0] = a.s16[0 + !!CONFIG_BIG_ENDIAN * 2];
  return t;
}

SIMD_INLINE c_v64 c_v64_unpackhi_u16_s32(c_v64 a) {
  c_v64 t;
  t.s32[1] = a.u16[3 - !!CONFIG_BIG_ENDIAN * 2];
  t.s32[0] = a.u16[2 - !!CONFIG_BIG_ENDIAN * 2];
  return t;
}

SIMD_INLINE c_v64 c_v64_unpackhi_s16_s32(c_v64 a) {
  c_v64 t;
  t.s32[1] = a.s16[3 - !!CONFIG_BIG_ENDIAN * 2];
  t.s32[0] = a.s16[2 - !!CONFIG_BIG_ENDIAN * 2];
  return t;
}

SIMD_INLINE c_v64 c_v64_shuffle_8(c_v64 a, c_v64 pattern) {
  c_v64 t;
  int c;
  for (c = 0; c < 8; c++) {
    if (SIMD_CHECK && (pattern.u8[c] & ~7)) {
      fprintf(stderr, "Error: Undefined v64_shuffle_8 index %d/%d\n",
              pattern.u8[c], c);
      abort();
    }
    t.u8[c] =
        a.u8[CONFIG_BIG_ENDIAN ? 7 - (pattern.u8[c] & 7) : pattern.u8[c] & 7];
  }
  return t;
}

SIMD_INLINE int64_t c_v64_dotp_su8(c_v64 a, c_v64 b) {
  return a.s8[7] * b.u8[7] + a.s8[6] * b.u8[6] + a.s8[5] * b.u8[5] +
         a.s8[4] * b.u8[4] + a.s8[3] * b.u8[3] + a.s8[2] * b.u8[2] +
         a.s8[1] * b.u8[1] + a.s8[0] * b.u8[0];
}

SIMD_INLINE int64_t c_v64_dotp_s16(c_v64 a, c_v64 b) {
  return (int64_t)(a.s16[3] * b.s16[3] + a.s16[2] * b.s16[2]) +
         (int64_t)(a.s16[1] * b.s16[1] + a.s16[0] * b.s16[0]);
}

SIMD_INLINE uint64_t c_v64_hadd_u8(c_v64 a) {
  return a.u8[7] + a.u8[6] + a.u8[5] + a.u8[4] + a.u8[3] + a.u8[2] + a.u8[1] +
         a.u8[0];
}

SIMD_INLINE int64_t c_v64_hadd_s16(c_v64 a) {
  return a.s16[3] + a.s16[2] + a.s16[1] + a.s16[0];
}

typedef struct {
  uint32_t val;
  int count;
} c_sad64_internal;

SIMD_INLINE c_sad64_internal c_v64_sad_u8_init(void) {
  c_sad64_internal t;
  t.val = t.count = 0;
  return t;
}

/* Implementation dependent return value.  Result must be finalised with
   v64_sad_u8_sum(). The result for more than 32 v64_sad_u8() calls is
   undefined. */
SIMD_INLINE c_sad64_internal c_v64_sad_u8(c_sad64_internal s, c_v64 a,
                                          c_v64 b) {
  int c;
  for (c = 0; c < 8; c++)
    s.val += a.u8[c] > b.u8[c] ? a.u8[c] - b.u8[c] : b.u8[c] - a.u8[c];
  s.count++;
  if (SIMD_CHECK && s.count > 32) {
    fprintf(stderr,
            "Error: sad called 32 times returning an undefined result\n");
    abort();
  }
  return s;
}

SIMD_INLINE uint32_t c_v64_sad_u8_sum(c_sad64_internal s) { return s.val; }

typedef uint32_t c_ssd64_internal;

/* Implementation dependent return value.  Result must be finalised with
 * v64_ssd_u8_sum(). */
SIMD_INLINE c_ssd64_internal c_v64_ssd_u8_init(void) { return 0; }

SIMD_INLINE c_ssd64_internal c_v64_ssd_u8(c_ssd64_internal s, c_v64 a,
                                          c_v64 b) {
  int c;
  for (c = 0; c < 8; c++) s += (a.u8[c] - b.u8[c]) * (a.u8[c] - b.u8[c]);
  return s;
}

SIMD_INLINE uint32_t c_v64_ssd_u8_sum(c_ssd64_internal s) { return s; }

SIMD_INLINE c_v64 c_v64_or(c_v64 a, c_v64 b) {
  c_v64 t;
  t.u64 = a.u64 | b.u64;
  return t;
}

SIMD_INLINE c_v64 c_v64_xor(c_v64 a, c_v64 b) {
  c_v64 t;
  t.u64 = a.u64 ^ b.u64;
  return t;
}

SIMD_INLINE c_v64 c_v64_and(c_v64 a, c_v64 b) {
  c_v64 t;
  t.u64 = a.u64 & b.u64;
  return t;
}

SIMD_INLINE c_v64 c_v64_andn(c_v64 a, c_v64 b) {
  c_v64 t;
  t.u64 = a.u64 & ~b.u64;
  return t;
}

SIMD_INLINE c_v64 c_v64_mullo_s16(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 4; c++) t.s16[c] = (int16_t)(a.s16[c] * b.s16[c]);
  return t;
}

SIMD_INLINE c_v64 c_v64_mulhi_s16(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 4; c++) t.s16[c] = (a.s16[c] * b.s16[c]) >> 16;
  return t;
}

SIMD_INLINE c_v64 c_v64_mullo_s32(c_v64 a, c_v64 b) {
  c_v64 t;
  t.s32[0] = (int32_t)((int64_t)a.s32[0] * b.s32[0]);
  t.s32[1] = (int32_t)((int64_t)a.s32[1] * b.s32[1]);
  return t;
}

SIMD_INLINE c_v64 c_v64_madd_s16(c_v64 a, c_v64 b) {
  c_v64 t;
  t.s32[0] = a.s16[0] * b.s16[0] + a.s16[1] * b.s16[1];
  t.s32[1] = a.s16[2] * b.s16[2] + a.s16[3] * b.s16[3];
  return t;
}

SIMD_INLINE c_v64 c_v64_madd_us8(c_v64 a, c_v64 b) {
  c_v64 t;
  int32_t u;
  u = a.u8[0] * b.s8[0] + a.u8[1] * b.s8[1];
  t.s16[0] = SIMD_CLAMP(u, -32768, 32767);
  u = a.u8[2] * b.s8[2] + a.u8[3] * b.s8[3];
  t.s16[1] = SIMD_CLAMP(u, -32768, 32767);
  u = a.u8[4] * b.s8[4] + a.u8[5] * b.s8[5];
  t.s16[2] = SIMD_CLAMP(u, -32768, 32767);
  u = a.u8[6] * b.s8[6] + a.u8[7] * b.s8[7];
  t.s16[3] = SIMD_CLAMP(u, -32768, 32767);
  return t;
}

SIMD_INLINE c_v64 c_v64_avg_u8(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 8; c++) t.u8[c] = (a.u8[c] + b.u8[c] + 1) >> 1;
  return t;
}

SIMD_INLINE c_v64 c_v64_rdavg_u8(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 8; c++) t.u8[c] = (a.u8[c] + b.u8[c]) >> 1;
  return t;
}

SIMD_INLINE c_v64 c_v64_rdavg_u16(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 4; c++) t.u16[c] = (a.u16[c] + b.u16[c]) >> 1;
  return t;
}

SIMD_INLINE c_v64 c_v64_avg_u16(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 4; c++) t.u16[c] = (a.u16[c] + b.u16[c] + 1) >> 1;
  return t;
}

SIMD_INLINE c_v64 c_v64_min_u8(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 8; c++) t.u8[c] = a.u8[c] > b.u8[c] ? b.u8[c] : a.u8[c];
  return t;
}

SIMD_INLINE c_v64 c_v64_max_u8(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 8; c++) t.u8[c] = a.u8[c] > b.u8[c] ? a.u8[c] : b.u8[c];
  return t;
}

SIMD_INLINE c_v64 c_v64_min_s8(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 8; c++) t.s8[c] = a.s8[c] > b.s8[c] ? b.s8[c] : a.s8[c];
  return t;
}

SIMD_INLINE c_v64 c_v64_max_s8(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 8; c++) t.s8[c] = a.s8[c] > b.s8[c] ? a.s8[c] : b.s8[c];
  return t;
}

SIMD_INLINE c_v64 c_v64_min_s16(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 4; c++) t.s16[c] = a.s16[c] > b.s16[c] ? b.s16[c] : a.s16[c];
  return t;
}

SIMD_INLINE c_v64 c_v64_max_s16(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 4; c++) t.s16[c] = a.s16[c] > b.s16[c] ? a.s16[c] : b.s16[c];
  return t;
}

SIMD_INLINE c_v64 c_v64_cmpgt_s8(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 8; c++) t.s8[c] = -(a.s8[c] > b.s8[c]);
  return t;
}

SIMD_INLINE c_v64 c_v64_cmplt_s8(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 8; c++) t.s8[c] = -(a.s8[c] < b.s8[c]);
  return t;
}

SIMD_INLINE c_v64 c_v64_cmpeq_8(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 8; c++) t.s8[c] = -(a.u8[c] == b.u8[c]);
  return t;
}

SIMD_INLINE c_v64 c_v64_cmpgt_s16(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 4; c++) t.s16[c] = -(a.s16[c] > b.s16[c]);
  return t;
}

SIMD_INLINE c_v64 c_v64_cmplt_s16(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 4; c++) t.s16[c] = -(a.s16[c] < b.s16[c]);
  return t;
}

SIMD_INLINE c_v64 c_v64_cmpeq_16(c_v64 a, c_v64 b) {
  c_v64 t;
  int c;
  for (c = 0; c < 4; c++) t.s16[c] = -(a.u16[c] == b.u16[c]);
  return t;
}

SIMD_INLINE c_v64 c_v64_shl_8(c_v64 a, unsigned int n) {
  c_v64 t;
  int c;
  if (SIMD_CHECK && n > 7) {
    fprintf(stderr, "Error: Undefined u8 shift left %d\n", n);
    abort();
  }
  for (c = 0; c < 8; c++) t.s8[c] = (int8_t)(a.u8[c] << n);
  return t;
}

SIMD_INLINE c_v64 c_v64_shr_u8(c_v64 a, unsigned int n) {
  c_v64 t;
  int c;
  if (SIMD_CHECK && n > 7) {
    fprintf(stderr, "Error: Undefined u8 shift right %d\n", n);
    abort();
  }
  for (c = 0; c < 8; c++) t.u8[c] = a.u8[c] >> n;
  return t;
}

SIMD_INLINE c_v64 c_v64_shr_s8(c_v64 a, unsigned int n) {
  c_v64 t;
  int c;
  if (SIMD_CHECK && n > 7) {
    fprintf(stderr, "Error: Undefined s8 shift right %d\n", n);
    abort();
  }
  for (c = 0; c < 8; c++) t.s8[c] = a.s8[c] >> n;
  return t;
}

SIMD_INLINE c_v64 c_v64_shl_16(c_v64 a, unsigned int n) {
  c_v64 t;
  int c;
  if (SIMD_CHECK && n > 15) {
    fprintf(stderr, "Error: Undefined u16 shift left %d\n", n);
    abort();
  }
  for (c = 0; c < 4; c++) t.u16[c] = (uint16_t)(a.u16[c] << n);
  return t;
}

SIMD_INLINE c_v64 c_v64_shr_u16(c_v64 a, unsigned int n) {
  c_v64 t;
  int c;
  if (SIMD_CHECK && n > 15) {
    fprintf(stderr, "Error: Undefined u16 shift right %d\n", n);
    abort();
  }
  for (c = 0; c < 4; c++) t.u16[c] = a.u16[c] >> n;
  return t;
}

SIMD_INLINE c_v64 c_v64_shr_s16(c_v64 a, unsigned int n) {
  c_v64 t;
  int c;
  if (SIMD_CHECK && n > 15) {
    fprintf(stderr, "Error: undefined s16 shift right %d\n", n);
    abort();
  }
  for (c = 0; c < 4; c++) t.s16[c] = a.s16[c] >> n;
  return t;
}

SIMD_INLINE c_v64 c_v64_shl_32(c_v64 a, unsigned int n) {
  c_v64 t;
  if (SIMD_CHECK && n > 31) {
    fprintf(stderr, "Error: undefined u32 shift left %d\n", n);
    abort();
  }
  t.u32[1] = a.u32[1] << n;
  t.u32[0] = a.u32[0] << n;
  return t;
}

SIMD_INLINE c_v64 c_v64_shr_u32(c_v64 a, unsigned int n) {
  c_v64 t;
  if (SIMD_CHECK && n > 31) {
    fprintf(stderr, "Error: undefined u32 shift right %d\n", n);
    abort();
  }
  t.u32[1] = a.u32[1] >> n;
  t.u32[0] = a.u32[0] >> n;
  return t;
}

SIMD_INLINE c_v64 c_v64_shr_s32(c_v64 a, unsigned int n) {
  c_v64 t;
  if (SIMD_CHECK && n > 31) {
    fprintf(stderr, "Error: undefined s32 shift right %d\n", n);
    abort();
  }
  t.s32[1] = a.s32[1] >> n;
  t.s32[0] = a.s32[0] >> n;
  return t;
}

SIMD_INLINE c_v64 c_v64_shr_n_byte(c_v64 x, unsigned int i) {
  c_v64 t;
  t.u64 = x.u64 >> i * 8;
  return t;
}

SIMD_INLINE c_v64 c_v64_shl_n_byte(c_v64 x, unsigned int i) {
  c_v64 t;
  t.u64 = x.u64 << i * 8;
  return t;
}

SIMD_INLINE c_v64 c_v64_align(c_v64 a, c_v64 b, unsigned int c) {
  if (SIMD_CHECK && c > 7) {
    fprintf(stderr, "Error: undefined alignment %d\n", c);
    abort();
  }
  return c ? c_v64_or(c_v64_shr_n_byte(b, c), c_v64_shl_n_byte(a, 8 - c)) : b;
}

SIMD_INLINE c_v64 c_v64_shl_n_8(c_v64 a, unsigned int c) {
  return c_v64_shl_8(a, c);
}

SIMD_INLINE c_v64 c_v64_shr_n_u8(c_v64 a, unsigned int c) {
  return c_v64_shr_u8(a, c);
}

SIMD_INLINE c_v64 c_v64_shr_n_s8(c_v64 a, unsigned int c) {
  return c_v64_shr_s8(a, c);
}

SIMD_INLINE c_v64 c_v64_shl_n_16(c_v64 a, unsigned int c) {
  return c_v64_shl_16(a, c);
}

SIMD_INLINE c_v64 c_v64_shr_n_u16(c_v64 a, unsigned int c) {
  return c_v64_shr_u16(a, c);
}

SIMD_INLINE c_v64 c_v64_shr_n_s16(c_v64 a, unsigned int c) {
  return c_v64_shr_s16(a, c);
}

SIMD_INLINE c_v64 c_v64_shl_n_32(c_v64 a, unsigned int c) {
  return c_v64_shl_32(a, c);
}

SIMD_INLINE c_v64 c_v64_shr_n_u32(c_v64 a, unsigned int c) {
  return c_v64_shr_u32(a, c);
}

SIMD_INLINE c_v64 c_v64_shr_n_s32(c_v64 a, unsigned int c) {
  return c_v64_shr_s32(a, c);
}

#endif  // AOM_AOM_DSP_SIMD_V64_INTRINSICS_C_H_
