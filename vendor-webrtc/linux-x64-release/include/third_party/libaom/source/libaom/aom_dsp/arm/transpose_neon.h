/*
 * Copyright (c) 2018, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AOM_DSP_ARM_TRANSPOSE_NEON_H_
#define AOM_AOM_DSP_ARM_TRANSPOSE_NEON_H_

#include <arm_neon.h>

#include "aom_dsp/aom_dsp_common.h"  // For AOM_FORCE_INLINE.
#include "config/aom_config.h"

static inline void transpose_elems_u8_8x8(
    uint8x8_t a0, uint8x8_t a1, uint8x8_t a2, uint8x8_t a3, uint8x8_t a4,
    uint8x8_t a5, uint8x8_t a6, uint8x8_t a7, uint8x8_t *o0, uint8x8_t *o1,
    uint8x8_t *o2, uint8x8_t *o3, uint8x8_t *o4, uint8x8_t *o5, uint8x8_t *o6,
    uint8x8_t *o7) {
  // Swap 8 bit elements. Goes from:
  // a0: 00 01 02 03 04 05 06 07
  // a1: 10 11 12 13 14 15 16 17
  // a2: 20 21 22 23 24 25 26 27
  // a3: 30 31 32 33 34 35 36 37
  // a4: 40 41 42 43 44 45 46 47
  // a5: 50 51 52 53 54 55 56 57
  // a6: 60 61 62 63 64 65 66 67
  // a7: 70 71 72 73 74 75 76 77
  // to:
  // b0.val[0]: 00 10 02 12 04 14 06 16  40 50 42 52 44 54 46 56
  // b0.val[1]: 01 11 03 13 05 15 07 17  41 51 43 53 45 55 47 57
  // b1.val[0]: 20 30 22 32 24 34 26 36  60 70 62 72 64 74 66 76
  // b1.val[1]: 21 31 23 33 25 35 27 37  61 71 63 73 65 75 67 77

  const uint8x16x2_t b0 = vtrnq_u8(vcombine_u8(a0, a4), vcombine_u8(a1, a5));
  const uint8x16x2_t b1 = vtrnq_u8(vcombine_u8(a2, a6), vcombine_u8(a3, a7));

  // Swap 16 bit elements resulting in:
  // c0.val[0]: 00 10 20 30 04 14 24 34  40 50 60 70 44 54 64 74
  // c0.val[1]: 02 12 22 32 06 16 26 36  42 52 62 72 46 56 66 76
  // c1.val[0]: 01 11 21 31 05 15 25 35  41 51 61 71 45 55 65 75
  // c1.val[1]: 03 13 23 33 07 17 27 37  43 53 63 73 47 57 67 77

  const uint16x8x2_t c0 = vtrnq_u16(vreinterpretq_u16_u8(b0.val[0]),
                                    vreinterpretq_u16_u8(b1.val[0]));
  const uint16x8x2_t c1 = vtrnq_u16(vreinterpretq_u16_u8(b0.val[1]),
                                    vreinterpretq_u16_u8(b1.val[1]));

  // Unzip 32 bit elements resulting in:
  // d0.val[0]: 00 10 20 30 40 50 60 70  01 11 21 31 41 51 61 71
  // d0.val[1]: 04 14 24 34 44 54 64 74  05 15 25 35 45 55 65 75
  // d1.val[0]: 02 12 22 32 42 52 62 72  03 13 23 33 43 53 63 73
  // d1.val[1]: 06 16 26 36 46 56 66 76  07 17 27 37 47 57 67 77
  const uint32x4x2_t d0 = vuzpq_u32(vreinterpretq_u32_u16(c0.val[0]),
                                    vreinterpretq_u32_u16(c1.val[0]));
  const uint32x4x2_t d1 = vuzpq_u32(vreinterpretq_u32_u16(c0.val[1]),
                                    vreinterpretq_u32_u16(c1.val[1]));

  *o0 = vreinterpret_u8_u32(vget_low_u32(d0.val[0]));
  *o1 = vreinterpret_u8_u32(vget_high_u32(d0.val[0]));
  *o2 = vreinterpret_u8_u32(vget_low_u32(d1.val[0]));
  *o3 = vreinterpret_u8_u32(vget_high_u32(d1.val[0]));
  *o4 = vreinterpret_u8_u32(vget_low_u32(d0.val[1]));
  *o5 = vreinterpret_u8_u32(vget_high_u32(d0.val[1]));
  *o6 = vreinterpret_u8_u32(vget_low_u32(d1.val[1]));
  *o7 = vreinterpret_u8_u32(vget_high_u32(d1.val[1]));
}

static inline void transpose_elems_inplace_u8_8x8(uint8x8_t *a0, uint8x8_t *a1,
                                                  uint8x8_t *a2, uint8x8_t *a3,
                                                  uint8x8_t *a4, uint8x8_t *a5,
                                                  uint8x8_t *a6,
                                                  uint8x8_t *a7) {
  transpose_elems_u8_8x8(*a0, *a1, *a2, *a3, *a4, *a5, *a6, *a7, a0, a1, a2, a3,
                         a4, a5, a6, a7);
}

static inline void transpose_arrays_u8_8x8(const uint8x8_t *in,
                                           uint8x8_t *out) {
  transpose_elems_u8_8x8(in[0], in[1], in[2], in[3], in[4], in[5], in[6], in[7],
                         &out[0], &out[1], &out[2], &out[3], &out[4], &out[5],
                         &out[6], &out[7]);
}

static AOM_FORCE_INLINE void transpose_arrays_u8_8x16(const uint8x8_t *x,
                                                      uint8x16_t *d) {
  uint8x8x2_t w0 = vzip_u8(x[0], x[1]);
  uint8x8x2_t w1 = vzip_u8(x[2], x[3]);
  uint8x8x2_t w2 = vzip_u8(x[4], x[5]);
  uint8x8x2_t w3 = vzip_u8(x[6], x[7]);

  uint8x8x2_t w8 = vzip_u8(x[8], x[9]);
  uint8x8x2_t w9 = vzip_u8(x[10], x[11]);
  uint8x8x2_t w10 = vzip_u8(x[12], x[13]);
  uint8x8x2_t w11 = vzip_u8(x[14], x[15]);

  uint16x4x2_t w4 =
      vzip_u16(vreinterpret_u16_u8(w0.val[0]), vreinterpret_u16_u8(w1.val[0]));
  uint16x4x2_t w5 =
      vzip_u16(vreinterpret_u16_u8(w2.val[0]), vreinterpret_u16_u8(w3.val[0]));
  uint16x4x2_t w12 =
      vzip_u16(vreinterpret_u16_u8(w8.val[0]), vreinterpret_u16_u8(w9.val[0]));
  uint16x4x2_t w13 = vzip_u16(vreinterpret_u16_u8(w10.val[0]),
                              vreinterpret_u16_u8(w11.val[0]));

  uint32x2x2_t w6 = vzip_u32(vreinterpret_u32_u16(w4.val[0]),
                             vreinterpret_u32_u16(w5.val[0]));
  uint32x2x2_t w7 = vzip_u32(vreinterpret_u32_u16(w4.val[1]),
                             vreinterpret_u32_u16(w5.val[1]));
  uint32x2x2_t w14 = vzip_u32(vreinterpret_u32_u16(w12.val[0]),
                              vreinterpret_u32_u16(w13.val[0]));
  uint32x2x2_t w15 = vzip_u32(vreinterpret_u32_u16(w12.val[1]),
                              vreinterpret_u32_u16(w13.val[1]));

  // Store first 4-line result
  d[0] = vreinterpretq_u8_u32(vcombine_u32(w6.val[0], w14.val[0]));
  d[1] = vreinterpretq_u8_u32(vcombine_u32(w6.val[1], w14.val[1]));
  d[2] = vreinterpretq_u8_u32(vcombine_u32(w7.val[0], w15.val[0]));
  d[3] = vreinterpretq_u8_u32(vcombine_u32(w7.val[1], w15.val[1]));

  w4 = vzip_u16(vreinterpret_u16_u8(w0.val[1]), vreinterpret_u16_u8(w1.val[1]));
  w5 = vzip_u16(vreinterpret_u16_u8(w2.val[1]), vreinterpret_u16_u8(w3.val[1]));
  w12 =
      vzip_u16(vreinterpret_u16_u8(w8.val[1]), vreinterpret_u16_u8(w9.val[1]));
  w13 = vzip_u16(vreinterpret_u16_u8(w10.val[1]),
                 vreinterpret_u16_u8(w11.val[1]));

  w6 = vzip_u32(vreinterpret_u32_u16(w4.val[0]),
                vreinterpret_u32_u16(w5.val[0]));
  w7 = vzip_u32(vreinterpret_u32_u16(w4.val[1]),
                vreinterpret_u32_u16(w5.val[1]));
  w14 = vzip_u32(vreinterpret_u32_u16(w12.val[0]),
                 vreinterpret_u32_u16(w13.val[0]));
  w15 = vzip_u32(vreinterpret_u32_u16(w12.val[1]),
                 vreinterpret_u32_u16(w13.val[1]));

  // Store second 4-line result
  d[4] = vreinterpretq_u8_u32(vcombine_u32(w6.val[0], w14.val[0]));
  d[5] = vreinterpretq_u8_u32(vcombine_u32(w6.val[1], w14.val[1]));
  d[6] = vreinterpretq_u8_u32(vcombine_u32(w7.val[0], w15.val[0]));
  d[7] = vreinterpretq_u8_u32(vcombine_u32(w7.val[1], w15.val[1]));
}

static AOM_FORCE_INLINE void transpose_arrays_u8_16x8(const uint8x16_t *x,
                                                      uint8x8_t *d) {
  uint8x16x2_t w0 = vzipq_u8(x[0], x[1]);
  uint8x16x2_t w1 = vzipq_u8(x[2], x[3]);
  uint8x16x2_t w2 = vzipq_u8(x[4], x[5]);
  uint8x16x2_t w3 = vzipq_u8(x[6], x[7]);

  uint16x8x2_t w4 = vzipq_u16(vreinterpretq_u16_u8(w0.val[0]),
                              vreinterpretq_u16_u8(w1.val[0]));
  uint16x8x2_t w5 = vzipq_u16(vreinterpretq_u16_u8(w2.val[0]),
                              vreinterpretq_u16_u8(w3.val[0]));
  uint16x8x2_t w6 = vzipq_u16(vreinterpretq_u16_u8(w0.val[1]),
                              vreinterpretq_u16_u8(w1.val[1]));
  uint16x8x2_t w7 = vzipq_u16(vreinterpretq_u16_u8(w2.val[1]),
                              vreinterpretq_u16_u8(w3.val[1]));

  uint32x4x2_t w8 = vzipq_u32(vreinterpretq_u32_u16(w4.val[0]),
                              vreinterpretq_u32_u16(w5.val[0]));
  uint32x4x2_t w9 = vzipq_u32(vreinterpretq_u32_u16(w6.val[0]),
                              vreinterpretq_u32_u16(w7.val[0]));
  uint32x4x2_t w10 = vzipq_u32(vreinterpretq_u32_u16(w4.val[1]),
                               vreinterpretq_u32_u16(w5.val[1]));
  uint32x4x2_t w11 = vzipq_u32(vreinterpretq_u32_u16(w6.val[1]),
                               vreinterpretq_u32_u16(w7.val[1]));

  d[0] = vreinterpret_u8_u32(vget_low_u32(w8.val[0]));
  d[1] = vreinterpret_u8_u32(vget_high_u32(w8.val[0]));
  d[2] = vreinterpret_u8_u32(vget_low_u32(w8.val[1]));
  d[3] = vreinterpret_u8_u32(vget_high_u32(w8.val[1]));
  d[4] = vreinterpret_u8_u32(vget_low_u32(w10.val[0]));
  d[5] = vreinterpret_u8_u32(vget_high_u32(w10.val[0]));
  d[6] = vreinterpret_u8_u32(vget_low_u32(w10.val[1]));
  d[7] = vreinterpret_u8_u32(vget_high_u32(w10.val[1]));
  d[8] = vreinterpret_u8_u32(vget_low_u32(w9.val[0]));
  d[9] = vreinterpret_u8_u32(vget_high_u32(w9.val[0]));
  d[10] = vreinterpret_u8_u32(vget_low_u32(w9.val[1]));
  d[11] = vreinterpret_u8_u32(vget_high_u32(w9.val[1]));
  d[12] = vreinterpret_u8_u32(vget_low_u32(w11.val[0]));
  d[13] = vreinterpret_u8_u32(vget_high_u32(w11.val[0]));
  d[14] = vreinterpret_u8_u32(vget_low_u32(w11.val[1]));
  d[15] = vreinterpret_u8_u32(vget_high_u32(w11.val[1]));
}

static inline uint16x8x2_t aom_vtrnq_u64_to_u16(uint32x4_t a0, uint32x4_t a1) {
  uint16x8x2_t b0;
#if AOM_ARCH_AARCH64
  b0.val[0] = vreinterpretq_u16_u64(
      vtrn1q_u64(vreinterpretq_u64_u32(a0), vreinterpretq_u64_u32(a1)));
  b0.val[1] = vreinterpretq_u16_u64(
      vtrn2q_u64(vreinterpretq_u64_u32(a0), vreinterpretq_u64_u32(a1)));
#else
  b0.val[0] = vcombine_u16(vreinterpret_u16_u32(vget_low_u32(a0)),
                           vreinterpret_u16_u32(vget_low_u32(a1)));
  b0.val[1] = vcombine_u16(vreinterpret_u16_u32(vget_high_u32(a0)),
                           vreinterpret_u16_u32(vget_high_u32(a1)));
#endif
  return b0;
}

static inline void transpose_arrays_u8_16x16(const uint8x16_t *x,
                                             uint8x16_t *d) {
  uint8x16x2_t w0 = vzipq_u8(x[0], x[1]);
  uint8x16x2_t w1 = vzipq_u8(x[2], x[3]);
  uint8x16x2_t w2 = vzipq_u8(x[4], x[5]);
  uint8x16x2_t w3 = vzipq_u8(x[6], x[7]);

  uint8x16x2_t w4 = vzipq_u8(x[8], x[9]);
  uint8x16x2_t w5 = vzipq_u8(x[10], x[11]);
  uint8x16x2_t w6 = vzipq_u8(x[12], x[13]);
  uint8x16x2_t w7 = vzipq_u8(x[14], x[15]);

  uint16x8x2_t w8 = vzipq_u16(vreinterpretq_u16_u8(w0.val[0]),
                              vreinterpretq_u16_u8(w1.val[0]));
  uint16x8x2_t w9 = vzipq_u16(vreinterpretq_u16_u8(w2.val[0]),
                              vreinterpretq_u16_u8(w3.val[0]));
  uint16x8x2_t w10 = vzipq_u16(vreinterpretq_u16_u8(w4.val[0]),
                               vreinterpretq_u16_u8(w5.val[0]));
  uint16x8x2_t w11 = vzipq_u16(vreinterpretq_u16_u8(w6.val[0]),
                               vreinterpretq_u16_u8(w7.val[0]));

  uint32x4x2_t w12 = vzipq_u32(vreinterpretq_u32_u16(w8.val[0]),
                               vreinterpretq_u32_u16(w9.val[0]));
  uint32x4x2_t w13 = vzipq_u32(vreinterpretq_u32_u16(w10.val[0]),
                               vreinterpretq_u32_u16(w11.val[0]));
  uint32x4x2_t w14 = vzipq_u32(vreinterpretq_u32_u16(w8.val[1]),
                               vreinterpretq_u32_u16(w9.val[1]));
  uint32x4x2_t w15 = vzipq_u32(vreinterpretq_u32_u16(w10.val[1]),
                               vreinterpretq_u32_u16(w11.val[1]));

  uint16x8x2_t d01 = aom_vtrnq_u64_to_u16(w12.val[0], w13.val[0]);
  d[0] = vreinterpretq_u8_u16(d01.val[0]);
  d[1] = vreinterpretq_u8_u16(d01.val[1]);
  uint16x8x2_t d23 = aom_vtrnq_u64_to_u16(w12.val[1], w13.val[1]);
  d[2] = vreinterpretq_u8_u16(d23.val[0]);
  d[3] = vreinterpretq_u8_u16(d23.val[1]);
  uint16x8x2_t d45 = aom_vtrnq_u64_to_u16(w14.val[0], w15.val[0]);
  d[4] = vreinterpretq_u8_u16(d45.val[0]);
  d[5] = vreinterpretq_u8_u16(d45.val[1]);
  uint16x8x2_t d67 = aom_vtrnq_u64_to_u16(w14.val[1], w15.val[1]);
  d[6] = vreinterpretq_u8_u16(d67.val[0]);
  d[7] = vreinterpretq_u8_u16(d67.val[1]);

  // upper half
  w8 = vzipq_u16(vreinterpretq_u16_u8(w0.val[1]),
                 vreinterpretq_u16_u8(w1.val[1]));
  w9 = vzipq_u16(vreinterpretq_u16_u8(w2.val[1]),
                 vreinterpretq_u16_u8(w3.val[1]));
  w10 = vzipq_u16(vreinterpretq_u16_u8(w4.val[1]),
                  vreinterpretq_u16_u8(w5.val[1]));
  w11 = vzipq_u16(vreinterpretq_u16_u8(w6.val[1]),
                  vreinterpretq_u16_u8(w7.val[1]));

  w12 = vzipq_u32(vreinterpretq_u32_u16(w8.val[0]),
                  vreinterpretq_u32_u16(w9.val[0]));
  w13 = vzipq_u32(vreinterpretq_u32_u16(w10.val[0]),
                  vreinterpretq_u32_u16(w11.val[0]));
  w14 = vzipq_u32(vreinterpretq_u32_u16(w8.val[1]),
                  vreinterpretq_u32_u16(w9.val[1]));
  w15 = vzipq_u32(vreinterpretq_u32_u16(w10.val[1]),
                  vreinterpretq_u32_u16(w11.val[1]));

  d01 = aom_vtrnq_u64_to_u16(w12.val[0], w13.val[0]);
  d[8] = vreinterpretq_u8_u16(d01.val[0]);
  d[9] = vreinterpretq_u8_u16(d01.val[1]);
  d23 = aom_vtrnq_u64_to_u16(w12.val[1], w13.val[1]);
  d[10] = vreinterpretq_u8_u16(d23.val[0]);
  d[11] = vreinterpretq_u8_u16(d23.val[1]);
  d45 = aom_vtrnq_u64_to_u16(w14.val[0], w15.val[0]);
  d[12] = vreinterpretq_u8_u16(d45.val[0]);
  d[13] = vreinterpretq_u8_u16(d45.val[1]);
  d67 = aom_vtrnq_u64_to_u16(w14.val[1], w15.val[1]);
  d[14] = vreinterpretq_u8_u16(d67.val[0]);
  d[15] = vreinterpretq_u8_u16(d67.val[1]);
}

static AOM_FORCE_INLINE void transpose_arrays_u8_32x16(const uint8x16x2_t *x,
                                                       uint8x16_t *d) {
  uint8x16_t x2[32];
  for (int i = 0; i < 16; ++i) {
    x2[i] = x[i].val[0];
    x2[i + 16] = x[i].val[1];
  }
  transpose_arrays_u8_16x16(x2, d);
  transpose_arrays_u8_16x16(x2 + 16, d + 16);
}

static inline void transpose_elems_inplace_u8_8x4(uint8x8_t *a0, uint8x8_t *a1,
                                                  uint8x8_t *a2,
                                                  uint8x8_t *a3) {
  // Swap 8 bit elements. Goes from:
  // a0: 00 01 02 03 04 05 06 07
  // a1: 10 11 12 13 14 15 16 17
  // a2: 20 21 22 23 24 25 26 27
  // a3: 30 31 32 33 34 35 36 37
  // to:
  // b0.val[0]: 00 10 02 12 04 14 06 16
  // b0.val[1]: 01 11 03 13 05 15 07 17
  // b1.val[0]: 20 30 22 32 24 34 26 36
  // b1.val[1]: 21 31 23 33 25 35 27 37

  const uint8x8x2_t b0 = vtrn_u8(*a0, *a1);
  const uint8x8x2_t b1 = vtrn_u8(*a2, *a3);

  // Swap 16 bit elements resulting in:
  // c0.val[0]: 00 10 20 30 04 14 24 34
  // c0.val[1]: 02 12 22 32 06 16 26 36
  // c1.val[0]: 01 11 21 31 05 15 25 35
  // c1.val[1]: 03 13 23 33 07 17 27 37

  const uint16x4x2_t c0 =
      vtrn_u16(vreinterpret_u16_u8(b0.val[0]), vreinterpret_u16_u8(b1.val[0]));
  const uint16x4x2_t c1 =
      vtrn_u16(vreinterpret_u16_u8(b0.val[1]), vreinterpret_u16_u8(b1.val[1]));

  *a0 = vreinterpret_u8_u16(c0.val[0]);
  *a1 = vreinterpret_u8_u16(c1.val[0]);
  *a2 = vreinterpret_u8_u16(c0.val[1]);
  *a3 = vreinterpret_u8_u16(c1.val[1]);
}

static inline void transpose_elems_inplace_u8_16x4(uint8x16_t *a0,
                                                   uint8x16_t *a1,
                                                   uint8x16_t *a2,
                                                   uint8x16_t *a3) {
  // Swap 8 bit elements. Goes from:
  // a0: 00 01 02 03 04 05 06 07 08 09 010 011 012 013 014 015
  // a1: 10 11 12 13 14 15 16 17 18 19 110 111 112 113 114 115
  // a2: 20 21 22 23 24 25 26 27 28 29 210 211 212 213 214 215
  // a3: 30 31 32 33 34 35 36 37 38 39 310 311 312 313 314 315
  // to:
  // b0.val[0]: 00 10 02 12 04 14 06 16 08 18 010 110 012 112 014 114
  // b0.val[1]: 01 11 03 13 05 15 07 17 09 19 011 111 013 113 015 115
  // b1.val[0]: 20 30 22 32 24 34 26 36 28 38 210 310 212 312 214 314
  // b1.val[1]: 21 31 23 33 25 35 27 37 29 39 211 311 213 313 215 315

  const uint8x16x2_t b0 = vtrnq_u8(*a0, *a1);
  const uint8x16x2_t b1 = vtrnq_u8(*a2, *a3);

  // Swap 16 bit elements resulting in:
  // c0.val[0]: 00 10 20 30 04 14 24 34 08  18  28  38  012 112 212 312
  // c0.val[1]: 02 12 22 32 06 16 26 36 09  19  29  39  013 113 213 313
  // c1.val[0]: 01 11 21 31 05 15 25 35 010 110 210 310 014 114 214 314
  // c1.val[1]: 03 13 23 33 07 17 27 37 011 111 211 311 015 115 215 315

  const uint16x8x2_t c0 = vtrnq_u16(vreinterpretq_u16_u8(b0.val[0]),
                                    vreinterpretq_u16_u8(b1.val[0]));
  const uint16x8x2_t c1 = vtrnq_u16(vreinterpretq_u16_u8(b0.val[1]),
                                    vreinterpretq_u16_u8(b1.val[1]));

  *a0 = vreinterpretq_u8_u16(c0.val[0]);
  *a1 = vreinterpretq_u8_u16(c1.val[0]);
  *a2 = vreinterpretq_u8_u16(c0.val[1]);
  *a3 = vreinterpretq_u8_u16(c1.val[1]);
}

static inline void transpose_elems_inplace_u8_4x4(uint8x8_t *a0,
                                                  uint8x8_t *a1) {
  // Swap 16 bit elements. Goes from:
  // a0: 00 01 02 03  10 11 12 13
  // a1: 20 21 22 23  30 31 32 33
  // to:
  // b0.val[0]: 00 01 20 21  10 11 30 31
  // b0.val[1]: 02 03 22 23  12 13 32 33

  const uint16x4x2_t b0 =
      vtrn_u16(vreinterpret_u16_u8(*a0), vreinterpret_u16_u8(*a1));

  // Swap 32 bit elements resulting in:
  // c0.val[0]: 00 01 20 21  02 03 22 23
  // c0.val[1]: 10 11 30 31  12 13 32 33

  const uint32x2x2_t c0 = vtrn_u32(vreinterpret_u32_u16(b0.val[0]),
                                   vreinterpret_u32_u16(b0.val[1]));

  // Swap 8 bit elements resulting in:
  // d0.val[0]: 00 10 20 30  02 12 22 32
  // d0.val[1]: 01 11 21 31  03 13 23 33

  const uint8x8x2_t d0 =
      vtrn_u8(vreinterpret_u8_u32(c0.val[0]), vreinterpret_u8_u32(c0.val[1]));

  *a0 = d0.val[0];
  *a1 = d0.val[1];
}

static inline void transpose_elems_u8_4x8(uint8x8_t a0, uint8x8_t a1,
                                          uint8x8_t a2, uint8x8_t a3,
                                          uint8x8_t a4, uint8x8_t a5,
                                          uint8x8_t a6, uint8x8_t a7,
                                          uint8x8_t *o0, uint8x8_t *o1,
                                          uint8x8_t *o2, uint8x8_t *o3) {
  // Swap 32 bit elements. Goes from:
  // a0: 00 01 02 03 XX XX XX XX
  // a1: 10 11 12 13 XX XX XX XX
  // a2: 20 21 22 23 XX XX XX XX
  // a3; 30 31 32 33 XX XX XX XX
  // a4: 40 41 42 43 XX XX XX XX
  // a5: 50 51 52 53 XX XX XX XX
  // a6: 60 61 62 63 XX XX XX XX
  // a7: 70 71 72 73 XX XX XX XX
  // to:
  // b0.val[0]: 00 01 02 03 40 41 42 43
  // b1.val[0]: 10 11 12 13 50 51 52 53
  // b2.val[0]: 20 21 22 23 60 61 62 63
  // b3.val[0]: 30 31 32 33 70 71 72 73

  const uint32x2x2_t b0 =
      vtrn_u32(vreinterpret_u32_u8(a0), vreinterpret_u32_u8(a4));
  const uint32x2x2_t b1 =
      vtrn_u32(vreinterpret_u32_u8(a1), vreinterpret_u32_u8(a5));
  const uint32x2x2_t b2 =
      vtrn_u32(vreinterpret_u32_u8(a2), vreinterpret_u32_u8(a6));
  const uint32x2x2_t b3 =
      vtrn_u32(vreinterpret_u32_u8(a3), vreinterpret_u32_u8(a7));

  // Swap 16 bit elements resulting in:
  // c0.val[0]: 00 01 20 21 40 41 60 61
  // c0.val[1]: 02 03 22 23 42 43 62 63
  // c1.val[0]: 10 11 30 31 50 51 70 71
  // c1.val[1]: 12 13 32 33 52 53 72 73

  const uint16x4x2_t c0 = vtrn_u16(vreinterpret_u16_u32(b0.val[0]),
                                   vreinterpret_u16_u32(b2.val[0]));
  const uint16x4x2_t c1 = vtrn_u16(vreinterpret_u16_u32(b1.val[0]),
                                   vreinterpret_u16_u32(b3.val[0]));

  // Swap 8 bit elements resulting in:
  // d0.val[0]: 00 10 20 30 40 50 60 70
  // d0.val[1]: 01 11 21 31 41 51 61 71
  // d1.val[0]: 02 12 22 32 42 52 62 72
  // d1.val[1]: 03 13 23 33 43 53 63 73

  const uint8x8x2_t d0 =
      vtrn_u8(vreinterpret_u8_u16(c0.val[0]), vreinterpret_u8_u16(c1.val[0]));
  const uint8x8x2_t d1 =
      vtrn_u8(vreinterpret_u8_u16(c0.val[1]), vreinterpret_u8_u16(c1.val[1]));

  *o0 = d0.val[0];
  *o1 = d0.val[1];
  *o2 = d1.val[0];
  *o3 = d1.val[1];
}

static inline void transpose_array_inplace_u16_4x4(uint16x4_t a[4]) {
  // Input:
  // 00 01 02 03
  // 10 11 12 13
  // 20 21 22 23
  // 30 31 32 33

  // b:
  // 00 10 02 12
  // 01 11 03 13
  const uint16x4x2_t b = vtrn_u16(a[0], a[1]);
  // c:
  // 20 30 22 32
  // 21 31 23 33
  const uint16x4x2_t c = vtrn_u16(a[2], a[3]);
  // d:
  // 00 10 20 30
  // 02 12 22 32
  const uint32x2x2_t d =
      vtrn_u32(vreinterpret_u32_u16(b.val[0]), vreinterpret_u32_u16(c.val[0]));
  // e:
  // 01 11 21 31
  // 03 13 23 33
  const uint32x2x2_t e =
      vtrn_u32(vreinterpret_u32_u16(b.val[1]), vreinterpret_u32_u16(c.val[1]));

  // Output:
  // 00 10 20 30
  // 01 11 21 31
  // 02 12 22 32
  // 03 13 23 33
  a[0] = vreinterpret_u16_u32(d.val[0]);
  a[1] = vreinterpret_u16_u32(e.val[0]);
  a[2] = vreinterpret_u16_u32(d.val[1]);
  a[3] = vreinterpret_u16_u32(e.val[1]);
}

static inline void transpose_array_inplace_u16_4x8(uint16x8_t a[4]) {
  // 4x8 Input:
  // a[0]: 00 01 02 03 04 05 06 07
  // a[1]: 10 11 12 13 14 15 16 17
  // a[2]: 20 21 22 23 24 25 26 27
  // a[3]: 30 31 32 33 34 35 36 37

  // b0.val[0]: 00 10 02 12 04 14 06 16
  // b0.val[1]: 01 11 03 13 05 15 07 17
  // b1.val[0]: 20 30 22 32 24 34 26 36
  // b1.val[1]: 21 31 23 33 25 35 27 37
  const uint16x8x2_t b0 = vtrnq_u16(a[0], a[1]);
  const uint16x8x2_t b1 = vtrnq_u16(a[2], a[3]);

  // c0.val[0]: 00 10 20 30 04 14 24 34
  // c0.val[1]: 02 12 22 32 06 16 26 36
  // c1.val[0]: 01 11 21 31 05 15 25 35
  // c1.val[1]: 03 13 23 33 07 17 27 37
  const uint32x4x2_t c0 = vtrnq_u32(vreinterpretq_u32_u16(b0.val[0]),
                                    vreinterpretq_u32_u16(b1.val[0]));
  const uint32x4x2_t c1 = vtrnq_u32(vreinterpretq_u32_u16(b0.val[1]),
                                    vreinterpretq_u32_u16(b1.val[1]));

  // 8x4 Output:
  // a[0]: 00 10 20 30 04 14 24 34
  // a[1]: 01 11 21 31 05 15 25 35
  // a[2]: 02 12 22 32 06 16 26 36
  // a[3]: 03 13 23 33 07 17 27 37
  a[0] = vreinterpretq_u16_u32(c0.val[0]);
  a[1] = vreinterpretq_u16_u32(c1.val[0]);
  a[2] = vreinterpretq_u16_u32(c0.val[1]);
  a[3] = vreinterpretq_u16_u32(c1.val[1]);
}

// Special transpose for loop filter.
// 4x8 Input:
// p_q:  p3 p2 p1 p0 q0 q1 q2 q3
// a[0]: 00 01 02 03 04 05 06 07
// a[1]: 10 11 12 13 14 15 16 17
// a[2]: 20 21 22 23 24 25 26 27
// a[3]: 30 31 32 33 34 35 36 37
// 8x4 Output:
// a[0]: 03 13 23 33 04 14 24 34  p0q0
// a[1]: 02 12 22 32 05 15 25 35  p1q1
// a[2]: 01 11 21 31 06 16 26 36  p2q2
// a[3]: 00 10 20 30 07 17 27 37  p3q3
// Direct reapplication of the function will reset the high halves, but
// reverse the low halves:
// p_q:  p0 p1 p2 p3 q0 q1 q2 q3
// a[0]: 33 32 31 30 04 05 06 07
// a[1]: 23 22 21 20 14 15 16 17
// a[2]: 13 12 11 10 24 25 26 27
// a[3]: 03 02 01 00 34 35 36 37
// Simply reordering the inputs (3, 2, 1, 0) will reset the low halves, but
// reverse the high halves.
// The standard transpose_u16_4x8q will produce the same reversals, but with the
// order of the low halves also restored relative to the high halves. This is
// preferable because it puts all values from the same source row back together,
// but some post-processing is inevitable.
static inline void loop_filter_transpose_u16_4x8q(uint16x8_t a[4]) {
  // b0.val[0]: 00 10 02 12 04 14 06 16
  // b0.val[1]: 01 11 03 13 05 15 07 17
  // b1.val[0]: 20 30 22 32 24 34 26 36
  // b1.val[1]: 21 31 23 33 25 35 27 37
  const uint16x8x2_t b0 = vtrnq_u16(a[0], a[1]);
  const uint16x8x2_t b1 = vtrnq_u16(a[2], a[3]);

  // Reverse odd vectors to bring the appropriate items to the front of zips.
  // b0.val[0]: 00 10 02 12 04 14 06 16
  // r0       : 03 13 01 11 07 17 05 15
  // b1.val[0]: 20 30 22 32 24 34 26 36
  // r1       : 23 33 21 31 27 37 25 35
  const uint32x4_t r0 = vrev64q_u32(vreinterpretq_u32_u16(b0.val[1]));
  const uint32x4_t r1 = vrev64q_u32(vreinterpretq_u32_u16(b1.val[1]));

  // Zip to complete the halves.
  // c0.val[0]: 00 10 20 30 02 12 22 32  p3p1
  // c0.val[1]: 04 14 24 34 06 16 26 36  q0q2
  // c1.val[0]: 03 13 23 33 01 11 21 31  p0p2
  // c1.val[1]: 07 17 27 37 05 15 25 35  q3q1
  const uint32x4x2_t c0 = vzipq_u32(vreinterpretq_u32_u16(b0.val[0]),
                                    vreinterpretq_u32_u16(b1.val[0]));
  const uint32x4x2_t c1 = vzipq_u32(r0, r1);

  // d0.val[0]: 00 10 20 30 07 17 27 37  p3q3
  // d0.val[1]: 02 12 22 32 05 15 25 35  p1q1
  // d1.val[0]: 03 13 23 33 04 14 24 34  p0q0
  // d1.val[1]: 01 11 21 31 06 16 26 36  p2q2
  const uint16x8x2_t d0 = aom_vtrnq_u64_to_u16(c0.val[0], c1.val[1]);
  // The third row of c comes first here to swap p2 with q0.
  const uint16x8x2_t d1 = aom_vtrnq_u64_to_u16(c1.val[0], c0.val[1]);

  // 8x4 Output:
  // a[0]: 03 13 23 33 04 14 24 34  p0q0
  // a[1]: 02 12 22 32 05 15 25 35  p1q1
  // a[2]: 01 11 21 31 06 16 26 36  p2q2
  // a[3]: 00 10 20 30 07 17 27 37  p3q3
  a[0] = d1.val[0];  // p0q0
  a[1] = d0.val[1];  // p1q1
  a[2] = d1.val[1];  // p2q2
  a[3] = d0.val[0];  // p3q3
}

static inline void transpose_elems_u16_4x8(
    const uint16x4_t a0, const uint16x4_t a1, const uint16x4_t a2,
    const uint16x4_t a3, const uint16x4_t a4, const uint16x4_t a5,
    const uint16x4_t a6, const uint16x4_t a7, uint16x8_t *o0, uint16x8_t *o1,
    uint16x8_t *o2, uint16x8_t *o3) {
  // Combine rows. Goes from:
  // a0: 00 01 02 03
  // a1: 10 11 12 13
  // a2: 20 21 22 23
  // a3: 30 31 32 33
  // a4: 40 41 42 43
  // a5: 50 51 52 53
  // a6: 60 61 62 63
  // a7: 70 71 72 73
  // to:
  // b0: 00 01 02 03 40 41 42 43
  // b1: 10 11 12 13 50 51 52 53
  // b2: 20 21 22 23 60 61 62 63
  // b3: 30 31 32 33 70 71 72 73

  const uint16x8_t b0 = vcombine_u16(a0, a4);
  const uint16x8_t b1 = vcombine_u16(a1, a5);
  const uint16x8_t b2 = vcombine_u16(a2, a6);
  const uint16x8_t b3 = vcombine_u16(a3, a7);

  // Swap 16 bit elements resulting in:
  // c0.val[0]: 00 10 02 12 40 50 42 52
  // c0.val[1]: 01 11 03 13 41 51 43 53
  // c1.val[0]: 20 30 22 32 60 70 62 72
  // c1.val[1]: 21 31 23 33 61 71 63 73

  const uint16x8x2_t c0 = vtrnq_u16(b0, b1);
  const uint16x8x2_t c1 = vtrnq_u16(b2, b3);

  // Swap 32 bit elements resulting in:
  // d0.val[0]: 00 10 20 30 40 50 60 70
  // d0.val[1]: 02 12 22 32 42 52 62 72
  // d1.val[0]: 01 11 21 31 41 51 61 71
  // d1.val[1]: 03 13 23 33 43 53 63 73

  const uint32x4x2_t d0 = vtrnq_u32(vreinterpretq_u32_u16(c0.val[0]),
                                    vreinterpretq_u32_u16(c1.val[0]));
  const uint32x4x2_t d1 = vtrnq_u32(vreinterpretq_u32_u16(c0.val[1]),
                                    vreinterpretq_u32_u16(c1.val[1]));

  *o0 = vreinterpretq_u16_u32(d0.val[0]);
  *o1 = vreinterpretq_u16_u32(d1.val[0]);
  *o2 = vreinterpretq_u16_u32(d0.val[1]);
  *o3 = vreinterpretq_u16_u32(d1.val[1]);
}

static inline void transpose_elems_s16_4x8(
    const int16x4_t a0, const int16x4_t a1, const int16x4_t a2,
    const int16x4_t a3, const int16x4_t a4, const int16x4_t a5,
    const int16x4_t a6, const int16x4_t a7, int16x8_t *o0, int16x8_t *o1,
    int16x8_t *o2, int16x8_t *o3) {
  // Combine rows. Goes from:
  // a0: 00 01 02 03
  // a1: 10 11 12 13
  // a2: 20 21 22 23
  // a3: 30 31 32 33
  // a4: 40 41 42 43
  // a5: 50 51 52 53
  // a6: 60 61 62 63
  // a7: 70 71 72 73
  // to:
  // b0: 00 01 02 03 40 41 42 43
  // b1: 10 11 12 13 50 51 52 53
  // b2: 20 21 22 23 60 61 62 63
  // b3: 30 31 32 33 70 71 72 73

  const int16x8_t b0 = vcombine_s16(a0, a4);
  const int16x8_t b1 = vcombine_s16(a1, a5);
  const int16x8_t b2 = vcombine_s16(a2, a6);
  const int16x8_t b3 = vcombine_s16(a3, a7);

  // Swap 16 bit elements resulting in:
  // c0.val[0]: 00 10 02 12 40 50 42 52
  // c0.val[1]: 01 11 03 13 41 51 43 53
  // c1.val[0]: 20 30 22 32 60 70 62 72
  // c1.val[1]: 21 31 23 33 61 71 63 73

  const int16x8x2_t c0 = vtrnq_s16(b0, b1);
  const int16x8x2_t c1 = vtrnq_s16(b2, b3);

  // Swap 32 bit elements resulting in:
  // d0.val[0]: 00 10 20 30 40 50 60 70
  // d0.val[1]: 02 12 22 32 42 52 62 72
  // d1.val[0]: 01 11 21 31 41 51 61 71
  // d1.val[1]: 03 13 23 33 43 53 63 73

  const int32x4x2_t d0 = vtrnq_s32(vreinterpretq_s32_s16(c0.val[0]),
                                   vreinterpretq_s32_s16(c1.val[0]));
  const int32x4x2_t d1 = vtrnq_s32(vreinterpretq_s32_s16(c0.val[1]),
                                   vreinterpretq_s32_s16(c1.val[1]));

  *o0 = vreinterpretq_s16_s32(d0.val[0]);
  *o1 = vreinterpretq_s16_s32(d1.val[0]);
  *o2 = vreinterpretq_s16_s32(d0.val[1]);
  *o3 = vreinterpretq_s16_s32(d1.val[1]);
}

static inline void transpose_elems_inplace_u16_8x8(
    uint16x8_t *a0, uint16x8_t *a1, uint16x8_t *a2, uint16x8_t *a3,
    uint16x8_t *a4, uint16x8_t *a5, uint16x8_t *a6, uint16x8_t *a7) {
  // Swap 16 bit elements. Goes from:
  // a0: 00 01 02 03 04 05 06 07
  // a1: 10 11 12 13 14 15 16 17
  // a2: 20 21 22 23 24 25 26 27
  // a3: 30 31 32 33 34 35 36 37
  // a4: 40 41 42 43 44 45 46 47
  // a5: 50 51 52 53 54 55 56 57
  // a6: 60 61 62 63 64 65 66 67
  // a7: 70 71 72 73 74 75 76 77
  // to:
  // b0.val[0]: 00 10 02 12 04 14 06 16
  // b0.val[1]: 01 11 03 13 05 15 07 17
  // b1.val[0]: 20 30 22 32 24 34 26 36
  // b1.val[1]: 21 31 23 33 25 35 27 37
  // b2.val[0]: 40 50 42 52 44 54 46 56
  // b2.val[1]: 41 51 43 53 45 55 47 57
  // b3.val[0]: 60 70 62 72 64 74 66 76
  // b3.val[1]: 61 71 63 73 65 75 67 77

  const uint16x8x2_t b0 = vtrnq_u16(*a0, *a1);
  const uint16x8x2_t b1 = vtrnq_u16(*a2, *a3);
  const uint16x8x2_t b2 = vtrnq_u16(*a4, *a5);
  const uint16x8x2_t b3 = vtrnq_u16(*a6, *a7);

  // Swap 32 bit elements resulting in:
  // c0.val[0]: 00 10 20 30 04 14 24 34
  // c0.val[1]: 02 12 22 32 06 16 26 36
  // c1.val[0]: 01 11 21 31 05 15 25 35
  // c1.val[1]: 03 13 23 33 07 17 27 37
  // c2.val[0]: 40 50 60 70 44 54 64 74
  // c2.val[1]: 42 52 62 72 46 56 66 76
  // c3.val[0]: 41 51 61 71 45 55 65 75
  // c3.val[1]: 43 53 63 73 47 57 67 77

  const uint32x4x2_t c0 = vtrnq_u32(vreinterpretq_u32_u16(b0.val[0]),
                                    vreinterpretq_u32_u16(b1.val[0]));
  const uint32x4x2_t c1 = vtrnq_u32(vreinterpretq_u32_u16(b0.val[1]),
                                    vreinterpretq_u32_u16(b1.val[1]));
  const uint32x4x2_t c2 = vtrnq_u32(vreinterpretq_u32_u16(b2.val[0]),
                                    vreinterpretq_u32_u16(b3.val[0]));
  const uint32x4x2_t c3 = vtrnq_u32(vreinterpretq_u32_u16(b2.val[1]),
                                    vreinterpretq_u32_u16(b3.val[1]));

  // Swap 64 bit elements resulting in:
  // d0.val[0]: 00 10 20 30 40 50 60 70
  // d0.val[1]: 04 14 24 34 44 54 64 74
  // d1.val[0]: 01 11 21 31 41 51 61 71
  // d1.val[1]: 05 15 25 35 45 55 65 75
  // d2.val[0]: 02 12 22 32 42 52 62 72
  // d2.val[1]: 06 16 26 36 46 56 66 76
  // d3.val[0]: 03 13 23 33 43 53 63 73
  // d3.val[1]: 07 17 27 37 47 57 67 77

  const uint16x8x2_t d0 = aom_vtrnq_u64_to_u16(c0.val[0], c2.val[0]);
  const uint16x8x2_t d1 = aom_vtrnq_u64_to_u16(c1.val[0], c3.val[0]);
  const uint16x8x2_t d2 = aom_vtrnq_u64_to_u16(c0.val[1], c2.val[1]);
  const uint16x8x2_t d3 = aom_vtrnq_u64_to_u16(c1.val[1], c3.val[1]);

  *a0 = d0.val[0];
  *a1 = d1.val[0];
  *a2 = d2.val[0];
  *a3 = d3.val[0];
  *a4 = d0.val[1];
  *a5 = d1.val[1];
  *a6 = d2.val[1];
  *a7 = d3.val[1];
}

static inline int16x8x2_t aom_vtrnq_s64_to_s16(int32x4_t a0, int32x4_t a1) {
  int16x8x2_t b0;
#if AOM_ARCH_AARCH64
  b0.val[0] = vreinterpretq_s16_s64(
      vtrn1q_s64(vreinterpretq_s64_s32(a0), vreinterpretq_s64_s32(a1)));
  b0.val[1] = vreinterpretq_s16_s64(
      vtrn2q_s64(vreinterpretq_s64_s32(a0), vreinterpretq_s64_s32(a1)));
#else
  b0.val[0] = vcombine_s16(vreinterpret_s16_s32(vget_low_s32(a0)),
                           vreinterpret_s16_s32(vget_low_s32(a1)));
  b0.val[1] = vcombine_s16(vreinterpret_s16_s32(vget_high_s32(a0)),
                           vreinterpret_s16_s32(vget_high_s32(a1)));
#endif
  return b0;
}

static inline void transpose_elems_inplace_s16_8x8(int16x8_t *a0, int16x8_t *a1,
                                                   int16x8_t *a2, int16x8_t *a3,
                                                   int16x8_t *a4, int16x8_t *a5,
                                                   int16x8_t *a6,
                                                   int16x8_t *a7) {
  // Swap 16 bit elements. Goes from:
  // a0: 00 01 02 03 04 05 06 07
  // a1: 10 11 12 13 14 15 16 17
  // a2: 20 21 22 23 24 25 26 27
  // a3: 30 31 32 33 34 35 36 37
  // a4: 40 41 42 43 44 45 46 47
  // a5: 50 51 52 53 54 55 56 57
  // a6: 60 61 62 63 64 65 66 67
  // a7: 70 71 72 73 74 75 76 77
  // to:
  // b0.val[0]: 00 10 02 12 04 14 06 16
  // b0.val[1]: 01 11 03 13 05 15 07 17
  // b1.val[0]: 20 30 22 32 24 34 26 36
  // b1.val[1]: 21 31 23 33 25 35 27 37
  // b2.val[0]: 40 50 42 52 44 54 46 56
  // b2.val[1]: 41 51 43 53 45 55 47 57
  // b3.val[0]: 60 70 62 72 64 74 66 76
  // b3.val[1]: 61 71 63 73 65 75 67 77

  const int16x8x2_t b0 = vtrnq_s16(*a0, *a1);
  const int16x8x2_t b1 = vtrnq_s16(*a2, *a3);
  const int16x8x2_t b2 = vtrnq_s16(*a4, *a5);
  const int16x8x2_t b3 = vtrnq_s16(*a6, *a7);

  // Swap 32 bit elements resulting in:
  // c0.val[0]: 00 10 20 30 04 14 24 34
  // c0.val[1]: 02 12 22 32 06 16 26 36
  // c1.val[0]: 01 11 21 31 05 15 25 35
  // c1.val[1]: 03 13 23 33 07 17 27 37
  // c2.val[0]: 40 50 60 70 44 54 64 74
  // c2.val[1]: 42 52 62 72 46 56 66 76
  // c3.val[0]: 41 51 61 71 45 55 65 75
  // c3.val[1]: 43 53 63 73 47 57 67 77

  const int32x4x2_t c0 = vtrnq_s32(vreinterpretq_s32_s16(b0.val[0]),
                                   vreinterpretq_s32_s16(b1.val[0]));
  const int32x4x2_t c1 = vtrnq_s32(vreinterpretq_s32_s16(b0.val[1]),
                                   vreinterpretq_s32_s16(b1.val[1]));
  const int32x4x2_t c2 = vtrnq_s32(vreinterpretq_s32_s16(b2.val[0]),
                                   vreinterpretq_s32_s16(b3.val[0]));
  const int32x4x2_t c3 = vtrnq_s32(vreinterpretq_s32_s16(b2.val[1]),
                                   vreinterpretq_s32_s16(b3.val[1]));

  // Swap 64 bit elements resulting in:
  // d0.val[0]: 00 10 20 30 40 50 60 70
  // d0.val[1]: 04 14 24 34 44 54 64 74
  // d1.val[0]: 01 11 21 31 41 51 61 71
  // d1.val[1]: 05 15 25 35 45 55 65 75
  // d2.val[0]: 02 12 22 32 42 52 62 72
  // d2.val[1]: 06 16 26 36 46 56 66 76
  // d3.val[0]: 03 13 23 33 43 53 63 73
  // d3.val[1]: 07 17 27 37 47 57 67 77

  const int16x8x2_t d0 = aom_vtrnq_s64_to_s16(c0.val[0], c2.val[0]);
  const int16x8x2_t d1 = aom_vtrnq_s64_to_s16(c1.val[0], c3.val[0]);
  const int16x8x2_t d2 = aom_vtrnq_s64_to_s16(c0.val[1], c2.val[1]);
  const int16x8x2_t d3 = aom_vtrnq_s64_to_s16(c1.val[1], c3.val[1]);

  *a0 = d0.val[0];
  *a1 = d1.val[0];
  *a2 = d2.val[0];
  *a3 = d3.val[0];
  *a4 = d0.val[1];
  *a5 = d1.val[1];
  *a6 = d2.val[1];
  *a7 = d3.val[1];
}

static inline void transpose_arrays_s16_8x8(const int16x8_t *a,
                                            int16x8_t *out) {
  // Swap 16 bit elements. Goes from:
  // a0: 00 01 02 03 04 05 06 07
  // a1: 10 11 12 13 14 15 16 17
  // a2: 20 21 22 23 24 25 26 27
  // a3: 30 31 32 33 34 35 36 37
  // a4: 40 41 42 43 44 45 46 47
  // a5: 50 51 52 53 54 55 56 57
  // a6: 60 61 62 63 64 65 66 67
  // a7: 70 71 72 73 74 75 76 77
  // to:
  // b0.val[0]: 00 10 02 12 04 14 06 16
  // b0.val[1]: 01 11 03 13 05 15 07 17
  // b1.val[0]: 20 30 22 32 24 34 26 36
  // b1.val[1]: 21 31 23 33 25 35 27 37
  // b2.val[0]: 40 50 42 52 44 54 46 56
  // b2.val[1]: 41 51 43 53 45 55 47 57
  // b3.val[0]: 60 70 62 72 64 74 66 76
  // b3.val[1]: 61 71 63 73 65 75 67 77

  const int16x8x2_t b0 = vtrnq_s16(a[0], a[1]);
  const int16x8x2_t b1 = vtrnq_s16(a[2], a[3]);
  const int16x8x2_t b2 = vtrnq_s16(a[4], a[5]);
  const int16x8x2_t b3 = vtrnq_s16(a[6], a[7]);

  // Swap 32 bit elements resulting in:
  // c0.val[0]: 00 10 20 30 04 14 24 34
  // c0.val[1]: 02 12 22 32 06 16 26 36
  // c1.val[0]: 01 11 21 31 05 15 25 35
  // c1.val[1]: 03 13 23 33 07 17 27 37
  // c2.val[0]: 40 50 60 70 44 54 64 74
  // c2.val[1]: 42 52 62 72 46 56 66 76
  // c3.val[0]: 41 51 61 71 45 55 65 75
  // c3.val[1]: 43 53 63 73 47 57 67 77

  const int32x4x2_t c0 = vtrnq_s32(vreinterpretq_s32_s16(b0.val[0]),
                                   vreinterpretq_s32_s16(b1.val[0]));
  const int32x4x2_t c1 = vtrnq_s32(vreinterpretq_s32_s16(b0.val[1]),
                                   vreinterpretq_s32_s16(b1.val[1]));
  const int32x4x2_t c2 = vtrnq_s32(vreinterpretq_s32_s16(b2.val[0]),
                                   vreinterpretq_s32_s16(b3.val[0]));
  const int32x4x2_t c3 = vtrnq_s32(vreinterpretq_s32_s16(b2.val[1]),
                                   vreinterpretq_s32_s16(b3.val[1]));

  // Swap 64 bit elements resulting in:
  // d0.val[0]: 00 10 20 30 40 50 60 70
  // d0.val[1]: 04 14 24 34 44 54 64 74
  // d1.val[0]: 01 11 21 31 41 51 61 71
  // d1.val[1]: 05 15 25 35 45 55 65 75
  // d2.val[0]: 02 12 22 32 42 52 62 72
  // d2.val[1]: 06 16 26 36 46 56 66 76
  // d3.val[0]: 03 13 23 33 43 53 63 73
  // d3.val[1]: 07 17 27 37 47 57 67 77

  const int16x8x2_t d0 = aom_vtrnq_s64_to_s16(c0.val[0], c2.val[0]);
  const int16x8x2_t d1 = aom_vtrnq_s64_to_s16(c1.val[0], c3.val[0]);
  const int16x8x2_t d2 = aom_vtrnq_s64_to_s16(c0.val[1], c2.val[1]);
  const int16x8x2_t d3 = aom_vtrnq_s64_to_s16(c1.val[1], c3.val[1]);

  out[0] = d0.val[0];
  out[1] = d1.val[0];
  out[2] = d2.val[0];
  out[3] = d3.val[0];
  out[4] = d0.val[1];
  out[5] = d1.val[1];
  out[6] = d2.val[1];
  out[7] = d3.val[1];
}

static inline void transpose_elems_inplace_s16_8x4(int16x8_t *a0, int16x8_t *a1,
                                                   int16x8_t *a2,
                                                   int16x8_t *a3) {
  // Swap 16 bit elements. Goes from:
  // a0: 00 01 02 03 04 05 06 07
  // a1: 10 11 12 13 14 15 16 17
  // a2: 20 21 22 23 24 25 26 27
  // a3: 30 31 32 33 34 35 36 37
  // to:
  // b0.val[0]: 00 10 02 12 04 14 06 16
  // b0.val[1]: 01 11 03 13 05 15 07 17
  // b1.val[0]: 20 30 22 32 24 34 26 36
  // b1.val[1]: 21 31 23 33 25 35 27 37

  const int16x8x2_t b0 = vtrnq_s16(*a0, *a1);
  const int16x8x2_t b1 = vtrnq_s16(*a2, *a3);

  // Swap 32 bit elements resulting in:
  // c0.val[0]: 00 10 20 30 04 14 24 34
  // c0.val[1]: 01 11 21 31 05 15 25 35
  // c1.val[0]: 02 12 22 32 06 16 26 36
  // c1.val[1]: 03 13 23 33 07 17 27 37

  const int32x4x2_t c0 = vtrnq_s32(vreinterpretq_s32_s16(b0.val[0]),
                                   vreinterpretq_s32_s16(b1.val[0]));
  const int32x4x2_t c1 = vtrnq_s32(vreinterpretq_s32_s16(b0.val[1]),
                                   vreinterpretq_s32_s16(b1.val[1]));

  *a0 = vreinterpretq_s16_s32(c0.val[0]);
  *a1 = vreinterpretq_s16_s32(c1.val[0]);
  *a2 = vreinterpretq_s16_s32(c0.val[1]);
  *a3 = vreinterpretq_s16_s32(c1.val[1]);
}

static inline void transpose_elems_inplace_u16_4x4(uint16x4_t *a0,
                                                   uint16x4_t *a1,
                                                   uint16x4_t *a2,
                                                   uint16x4_t *a3) {
  // Swap 16 bit elements. Goes from:
  // a0: 00 01 02 03
  // a1: 10 11 12 13
  // a2: 20 21 22 23
  // a3: 30 31 32 33
  // to:
  // b0.val[0]: 00 10 02 12
  // b0.val[1]: 01 11 03 13
  // b1.val[0]: 20 30 22 32
  // b1.val[1]: 21 31 23 33

  const uint16x4x2_t b0 = vtrn_u16(*a0, *a1);
  const uint16x4x2_t b1 = vtrn_u16(*a2, *a3);

  // Swap 32 bit elements resulting in:
  // c0.val[0]: 00 10 20 30
  // c0.val[1]: 02 12 22 32
  // c1.val[0]: 01 11 21 31
  // c1.val[1]: 03 13 23 33

  const uint32x2x2_t c0 = vtrn_u32(vreinterpret_u32_u16(b0.val[0]),
                                   vreinterpret_u32_u16(b1.val[0]));
  const uint32x2x2_t c1 = vtrn_u32(vreinterpret_u32_u16(b0.val[1]),
                                   vreinterpret_u32_u16(b1.val[1]));

  *a0 = vreinterpret_u16_u32(c0.val[0]);
  *a1 = vreinterpret_u16_u32(c1.val[0]);
  *a2 = vreinterpret_u16_u32(c0.val[1]);
  *a3 = vreinterpret_u16_u32(c1.val[1]);
}

static inline void transpose_elems_inplace_s16_4x4(int16x4_t *a0, int16x4_t *a1,
                                                   int16x4_t *a2,
                                                   int16x4_t *a3) {
  // Swap 16 bit elements. Goes from:
  // a0: 00 01 02 03
  // a1: 10 11 12 13
  // a2: 20 21 22 23
  // a3: 30 31 32 33
  // to:
  // b0.val[0]: 00 10 02 12
  // b0.val[1]: 01 11 03 13
  // b1.val[0]: 20 30 22 32
  // b1.val[1]: 21 31 23 33

  const int16x4x2_t b0 = vtrn_s16(*a0, *a1);
  const int16x4x2_t b1 = vtrn_s16(*a2, *a3);

  // Swap 32 bit elements resulting in:
  // c0.val[0]: 00 10 20 30
  // c0.val[1]: 02 12 22 32
  // c1.val[0]: 01 11 21 31
  // c1.val[1]: 03 13 23 33

  const int32x2x2_t c0 = vtrn_s32(vreinterpret_s32_s16(b0.val[0]),
                                  vreinterpret_s32_s16(b1.val[0]));
  const int32x2x2_t c1 = vtrn_s32(vreinterpret_s32_s16(b0.val[1]),
                                  vreinterpret_s32_s16(b1.val[1]));

  *a0 = vreinterpret_s16_s32(c0.val[0]);
  *a1 = vreinterpret_s16_s32(c1.val[0]);
  *a2 = vreinterpret_s16_s32(c0.val[1]);
  *a3 = vreinterpret_s16_s32(c1.val[1]);
}

static inline int32x4x2_t aom_vtrnq_s64_to_s32(int32x4_t a0, int32x4_t a1) {
  int32x4x2_t b0;
#if AOM_ARCH_AARCH64
  b0.val[0] = vreinterpretq_s32_s64(
      vtrn1q_s64(vreinterpretq_s64_s32(a0), vreinterpretq_s64_s32(a1)));
  b0.val[1] = vreinterpretq_s32_s64(
      vtrn2q_s64(vreinterpretq_s64_s32(a0), vreinterpretq_s64_s32(a1)));
#else
  b0.val[0] = vcombine_s32(vget_low_s32(a0), vget_low_s32(a1));
  b0.val[1] = vcombine_s32(vget_high_s32(a0), vget_high_s32(a1));
#endif
  return b0;
}

static inline void transpose_elems_s32_4x4(const int32x4_t a0,
                                           const int32x4_t a1,
                                           const int32x4_t a2,
                                           const int32x4_t a3, int32x4_t *o0,
                                           int32x4_t *o1, int32x4_t *o2,
                                           int32x4_t *o3) {
  // Swap 32 bit elements. Goes from:
  // a0: 00 01 02 03
  // a1: 10 11 12 13
  // a2: 20 21 22 23
  // a3: 30 31 32 33
  // to:
  // b0.val[0]: 00 10 02 12
  // b0.val[1]: 01 11 03 13
  // b1.val[0]: 20 30 22 32
  // b1.val[1]: 21 31 23 33

  const int32x4x2_t b0 = vtrnq_s32(a0, a1);
  const int32x4x2_t b1 = vtrnq_s32(a2, a3);

  // Swap 64 bit elements resulting in:
  // c0.val[0]: 00 10 20 30
  // c0.val[1]: 02 12 22 32
  // c1.val[0]: 01 11 21 31
  // c1.val[1]: 03 13 23 33

  const int32x4x2_t c0 = aom_vtrnq_s64_to_s32(b0.val[0], b1.val[0]);
  const int32x4x2_t c1 = aom_vtrnq_s64_to_s32(b0.val[1], b1.val[1]);

  *o0 = c0.val[0];
  *o1 = c1.val[0];
  *o2 = c0.val[1];
  *o3 = c1.val[1];
}

static inline void transpose_elems_inplace_s32_4x4(int32x4_t *a0, int32x4_t *a1,
                                                   int32x4_t *a2,
                                                   int32x4_t *a3) {
  transpose_elems_s32_4x4(*a0, *a1, *a2, *a3, a0, a1, a2, a3);
}

static inline void transpose_arrays_s32_4x4(const int32x4_t *in,
                                            int32x4_t *out) {
  transpose_elems_s32_4x4(in[0], in[1], in[2], in[3], &out[0], &out[1], &out[2],
                          &out[3]);
}

static AOM_FORCE_INLINE void transpose_arrays_s32_4nx4n(const int32x4_t *in,
                                                        int32x4_t *out,
                                                        const int width,
                                                        const int height) {
  const int h = height >> 2;
  const int w = width >> 2;
  for (int j = 0; j < w; j++) {
    for (int i = 0; i < h; i++) {
      transpose_arrays_s32_4x4(in + j * height + i * 4,
                               out + i * width + j * 4);
    }
  }
}

#define TRANSPOSE_ARRAYS_S32_WXH_NEON(w, h)                    \
  static AOM_FORCE_INLINE void transpose_arrays_s32_##w##x##h( \
      const int32x4_t *in, int32x4_t *out) {                   \
    transpose_arrays_s32_4nx4n(in, out, w, h);                 \
  }

TRANSPOSE_ARRAYS_S32_WXH_NEON(4, 8)
TRANSPOSE_ARRAYS_S32_WXH_NEON(4, 16)
TRANSPOSE_ARRAYS_S32_WXH_NEON(8, 4)
TRANSPOSE_ARRAYS_S32_WXH_NEON(8, 8)
TRANSPOSE_ARRAYS_S32_WXH_NEON(8, 16)
TRANSPOSE_ARRAYS_S32_WXH_NEON(8, 32)
TRANSPOSE_ARRAYS_S32_WXH_NEON(16, 8)
TRANSPOSE_ARRAYS_S32_WXH_NEON(16, 16)
TRANSPOSE_ARRAYS_S32_WXH_NEON(16, 32)
TRANSPOSE_ARRAYS_S32_WXH_NEON(16, 64)
TRANSPOSE_ARRAYS_S32_WXH_NEON(32, 8)
TRANSPOSE_ARRAYS_S32_WXH_NEON(32, 16)
TRANSPOSE_ARRAYS_S32_WXH_NEON(32, 32)
TRANSPOSE_ARRAYS_S32_WXH_NEON(32, 64)
TRANSPOSE_ARRAYS_S32_WXH_NEON(64, 16)
TRANSPOSE_ARRAYS_S32_WXH_NEON(64, 32)

#undef TRANSPOSE_ARRAYS_S32_WXH_NEON

static inline int64x2_t aom_vtrn1q_s64(int64x2_t a, int64x2_t b) {
#if AOM_ARCH_AARCH64
  return vtrn1q_s64(a, b);
#else
  return vcombine_s64(vget_low_s64(a), vget_low_s64(b));
#endif
}

static inline int64x2_t aom_vtrn2q_s64(int64x2_t a, int64x2_t b) {
#if AOM_ARCH_AARCH64
  return vtrn2q_s64(a, b);
#else
  return vcombine_s64(vget_high_s64(a), vget_high_s64(b));
#endif
}

static inline void transpose_elems_s32_4x8(int32x4_t a0, int32x4_t a1,
                                           int32x4_t a2, int32x4_t a3,
                                           int32x4_t a4, int32x4_t a5,
                                           int32x4_t a6, int32x4_t a7,
                                           int32x4x2_t *o0, int32x4x2_t *o1,
                                           int32x4x2_t *o2, int32x4x2_t *o3) {
  // Perform a 4 x 8 matrix transpose by building on top of the existing 4 x 4
  // matrix transpose implementation:
  // [ A ]^T => [ A^T B^T ]
  // [ B ]

  transpose_elems_inplace_s32_4x4(&a0, &a1, &a2, &a3);  // A^T
  transpose_elems_inplace_s32_4x4(&a4, &a5, &a6, &a7);  // B^T

  o0->val[0] = a0;
  o1->val[0] = a1;
  o2->val[0] = a2;
  o3->val[0] = a3;

  o0->val[1] = a4;
  o1->val[1] = a5;
  o2->val[1] = a6;
  o3->val[1] = a7;
}

static inline void transpose_elems_inplace_s32_8x8(
    int32x4x2_t *a0, int32x4x2_t *a1, int32x4x2_t *a2, int32x4x2_t *a3,
    int32x4x2_t *a4, int32x4x2_t *a5, int32x4x2_t *a6, int32x4x2_t *a7) {
  // Perform an 8 x 8 matrix transpose by building on top of the existing 4 x 4
  // matrix transpose implementation:
  // [ A B ]^T => [ A^T C^T ]
  // [ C D ]      [ B^T D^T ]

  int32x4_t q0_v1 = a0->val[0];
  int32x4_t q0_v2 = a1->val[0];
  int32x4_t q0_v3 = a2->val[0];
  int32x4_t q0_v4 = a3->val[0];

  int32x4_t q1_v1 = a0->val[1];
  int32x4_t q1_v2 = a1->val[1];
  int32x4_t q1_v3 = a2->val[1];
  int32x4_t q1_v4 = a3->val[1];

  int32x4_t q2_v1 = a4->val[0];
  int32x4_t q2_v2 = a5->val[0];
  int32x4_t q2_v3 = a6->val[0];
  int32x4_t q2_v4 = a7->val[0];

  int32x4_t q3_v1 = a4->val[1];
  int32x4_t q3_v2 = a5->val[1];
  int32x4_t q3_v3 = a6->val[1];
  int32x4_t q3_v4 = a7->val[1];

  transpose_elems_inplace_s32_4x4(&q0_v1, &q0_v2, &q0_v3, &q0_v4);  // A^T
  transpose_elems_inplace_s32_4x4(&q1_v1, &q1_v2, &q1_v3, &q1_v4);  // B^T
  transpose_elems_inplace_s32_4x4(&q2_v1, &q2_v2, &q2_v3, &q2_v4);  // C^T
  transpose_elems_inplace_s32_4x4(&q3_v1, &q3_v2, &q3_v3, &q3_v4);  // D^T

  a0->val[0] = q0_v1;
  a1->val[0] = q0_v2;
  a2->val[0] = q0_v3;
  a3->val[0] = q0_v4;

  a0->val[1] = q2_v1;
  a1->val[1] = q2_v2;
  a2->val[1] = q2_v3;
  a3->val[1] = q2_v4;

  a4->val[0] = q1_v1;
  a5->val[0] = q1_v2;
  a6->val[0] = q1_v3;
  a7->val[0] = q1_v4;

  a4->val[1] = q3_v1;
  a5->val[1] = q3_v2;
  a6->val[1] = q3_v3;
  a7->val[1] = q3_v4;
}

static inline void transpose_arrays_s16_4x4(const int16x4_t *const in,
                                            int16x4_t *const out) {
  int16x4_t a0 = in[0];
  int16x4_t a1 = in[1];
  int16x4_t a2 = in[2];
  int16x4_t a3 = in[3];

  transpose_elems_inplace_s16_4x4(&a0, &a1, &a2, &a3);

  out[0] = a0;
  out[1] = a1;
  out[2] = a2;
  out[3] = a3;
}

static inline void transpose_arrays_s16_4x8(const int16x4_t *const in,
                                            int16x8_t *const out) {
#if AOM_ARCH_AARCH64
  const int16x8_t a0 = vzip1q_s16(vcombine_s16(in[0], vdup_n_s16(0)),
                                  vcombine_s16(in[1], vdup_n_s16(0)));
  const int16x8_t a1 = vzip1q_s16(vcombine_s16(in[2], vdup_n_s16(0)),
                                  vcombine_s16(in[3], vdup_n_s16(0)));
  const int16x8_t a2 = vzip1q_s16(vcombine_s16(in[4], vdup_n_s16(0)),
                                  vcombine_s16(in[5], vdup_n_s16(0)));
  const int16x8_t a3 = vzip1q_s16(vcombine_s16(in[6], vdup_n_s16(0)),
                                  vcombine_s16(in[7], vdup_n_s16(0)));
#else
  int16x4x2_t temp;
  temp = vzip_s16(in[0], in[1]);
  const int16x8_t a0 = vcombine_s16(temp.val[0], temp.val[1]);
  temp = vzip_s16(in[2], in[3]);
  const int16x8_t a1 = vcombine_s16(temp.val[0], temp.val[1]);
  temp = vzip_s16(in[4], in[5]);
  const int16x8_t a2 = vcombine_s16(temp.val[0], temp.val[1]);
  temp = vzip_s16(in[6], in[7]);
  const int16x8_t a3 = vcombine_s16(temp.val[0], temp.val[1]);
#endif

  const int32x4x2_t b02 =
      vzipq_s32(vreinterpretq_s32_s16(a0), vreinterpretq_s32_s16(a1));
  const int32x4x2_t b13 =
      vzipq_s32(vreinterpretq_s32_s16(a2), vreinterpretq_s32_s16(a3));

#if AOM_ARCH_AARCH64
  out[0] = vreinterpretq_s16_s64(vzip1q_s64(vreinterpretq_s64_s32(b02.val[0]),
                                            vreinterpretq_s64_s32(b13.val[0])));
  out[1] = vreinterpretq_s16_s64(vzip2q_s64(vreinterpretq_s64_s32(b02.val[0]),
                                            vreinterpretq_s64_s32(b13.val[0])));
  out[2] = vreinterpretq_s16_s64(vzip1q_s64(vreinterpretq_s64_s32(b02.val[1]),
                                            vreinterpretq_s64_s32(b13.val[1])));
  out[3] = vreinterpretq_s16_s64(vzip2q_s64(vreinterpretq_s64_s32(b02.val[1]),
                                            vreinterpretq_s64_s32(b13.val[1])));
#else
  out[0] = vreinterpretq_s16_s32(
      vextq_s32(vextq_s32(b02.val[0], b02.val[0], 2), b13.val[0], 2));
  out[2] = vreinterpretq_s16_s32(
      vextq_s32(vextq_s32(b02.val[1], b02.val[1], 2), b13.val[1], 2));
  out[1] = vreinterpretq_s16_s32(
      vextq_s32(b02.val[0], vextq_s32(b13.val[0], b13.val[0], 2), 2));
  out[3] = vreinterpretq_s16_s32(
      vextq_s32(b02.val[1], vextq_s32(b13.val[1], b13.val[1], 2), 2));
#endif
}

static inline void transpose_arrays_s16_8x4(const int16x8_t *const in,
                                            int16x4_t *const out) {
  // Swap 16 bit elements. Goes from:
  // in[0]: 00 01 02 03 04 05 06 07
  // in[1]: 10 11 12 13 14 15 16 17
  // in[2]: 20 21 22 23 24 25 26 27
  // in[3]: 30 31 32 33 34 35 36 37
  // to:
  // b0.val[0]: 00 10 02 12 04 14 06 16
  // b0.val[1]: 01 11 03 13 05 15 07 17
  // b1.val[0]: 20 30 22 32 24 34 26 36
  // b1.val[1]: 21 31 23 33 25 35 27 37

  const int16x8x2_t b0 = vtrnq_s16(in[0], in[1]);
  const int16x8x2_t b1 = vtrnq_s16(in[2], in[3]);

  // Swap 32 bit elements resulting in:
  // c0.val[0]: 00 10 20 30 04 14 24 34
  // c0.val[1]: 02 12 22 32 06 16 26 36
  // c1.val[0]: 01 11 21 31 05 15 25 35
  // c1.val[1]: 03 13 23 33 07 17 27 37

  const uint32x4x2_t c0 = vtrnq_u32(vreinterpretq_u32_s16(b0.val[0]),
                                    vreinterpretq_u32_s16(b1.val[0]));
  const uint32x4x2_t c1 = vtrnq_u32(vreinterpretq_u32_s16(b0.val[1]),
                                    vreinterpretq_u32_s16(b1.val[1]));

  // Unpack 64 bit elements resulting in:
  // out[0]: 00 10 20 30
  // out[1]: 01 11 21 31
  // out[2]: 02 12 22 32
  // out[3]: 03 13 23 33
  // out[4]: 04 14 24 34
  // out[5]: 05 15 25 35
  // out[6]: 06 16 26 36
  // out[7]: 07 17 27 37

  out[0] = vget_low_s16(vreinterpretq_s16_u32(c0.val[0]));
  out[1] = vget_low_s16(vreinterpretq_s16_u32(c1.val[0]));
  out[2] = vget_low_s16(vreinterpretq_s16_u32(c0.val[1]));
  out[3] = vget_low_s16(vreinterpretq_s16_u32(c1.val[1]));
  out[4] = vget_high_s16(vreinterpretq_s16_u32(c0.val[0]));
  out[5] = vget_high_s16(vreinterpretq_s16_u32(c1.val[0]));
  out[6] = vget_high_s16(vreinterpretq_s16_u32(c0.val[1]));
  out[7] = vget_high_s16(vreinterpretq_s16_u32(c1.val[1]));
}

static inline void transpose_arrays_s64_4x4(const int64x2_t *in,
                                            int64x2_t *out) {
  // Perform a 4x4 matrix transpose going from:
  // in[0] = 00 01
  // in[1] = 02 03
  // in[2] = 10 11
  // in[3] = 12 13
  // in[4] = 20 21
  // in[5] = 22 23
  // in[6] = 30 31
  // in[7] = 32 33
  //
  // to:
  // out[0] = 00 10
  // out[1] = 20 30
  // out[2] = 01 11
  // out[3] = 21 31
  // out[4] = 02 12
  // out[5] = 22 32
  // out[6] = 03 13
  // out[7] = 23 33

  out[0] = aom_vtrn1q_s64(in[0], in[2]);
  out[1] = aom_vtrn1q_s64(in[4], in[6]);
  out[2] = aom_vtrn2q_s64(in[0], in[2]);
  out[3] = aom_vtrn2q_s64(in[4], in[6]);
  out[4] = aom_vtrn1q_s64(in[1], in[3]);
  out[5] = aom_vtrn1q_s64(in[5], in[7]);
  out[6] = aom_vtrn2q_s64(in[1], in[3]);
  out[7] = aom_vtrn2q_s64(in[5], in[7]);
}

#endif  // AOM_AOM_DSP_ARM_TRANSPOSE_NEON_H_
