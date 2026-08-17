/*
 * Copyright (c) 2024, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AOM_DSP_FLOW_ESTIMATION_ARM_DISFLOW_NEON_H_
#define AOM_AOM_DSP_FLOW_ESTIMATION_ARM_DISFLOW_NEON_H_

#include "aom_dsp/flow_estimation/disflow.h"

#include <arm_neon.h>
#include <math.h>

#include "aom_dsp/arm/mem_neon.h"
#include "config/aom_config.h"
#include "config/aom_dsp_rtcd.h"

static inline void get_cubic_kernel_dbl(double x, double kernel[4]) {
  // Check that the fractional position is in range.
  //
  // Note: x is calculated from, e.g., `u_frac = u - floor(u)`.
  // Mathematically, this implies that 0 <= x < 1. However, in practice it is
  // possible to have x == 1 due to floating point rounding. This is fine,
  // and we still interpolate correctly if we allow x = 1.
  assert(0 <= x && x <= 1);

  double x2 = x * x;
  double x3 = x2 * x;
  kernel[0] = -0.5 * x + x2 - 0.5 * x3;
  kernel[1] = 1.0 - 2.5 * x2 + 1.5 * x3;
  kernel[2] = 0.5 * x + 2.0 * x2 - 1.5 * x3;
  kernel[3] = -0.5 * x2 + 0.5 * x3;
}

static inline void get_cubic_kernel_int(double x, int kernel[4]) {
  double kernel_dbl[4];
  get_cubic_kernel_dbl(x, kernel_dbl);

  kernel[0] = (int)rint(kernel_dbl[0] * (1 << DISFLOW_INTERP_BITS));
  kernel[1] = (int)rint(kernel_dbl[1] * (1 << DISFLOW_INTERP_BITS));
  kernel[2] = (int)rint(kernel_dbl[2] * (1 << DISFLOW_INTERP_BITS));
  kernel[3] = (int)rint(kernel_dbl[3] * (1 << DISFLOW_INTERP_BITS));
}

static inline void sobel_filter_x(const uint8_t *src, int src_stride,
                                  int16_t *dst, int dst_stride) {
  int16_t tmp[DISFLOW_PATCH_SIZE * (DISFLOW_PATCH_SIZE + 2)];

  // Horizontal filter, using kernel {1, 0, -1}.
  const uint8_t *src_start = src - 1 * src_stride - 1;

  for (int i = 0; i < DISFLOW_PATCH_SIZE + 2; i++) {
    uint8x16_t s = vld1q_u8(src_start + i * src_stride);
    uint8x8_t s0 = vget_low_u8(s);
    uint8x8_t s2 = vget_low_u8(vextq_u8(s, s, 2));

    // Given that the kernel is {1, 0, -1} the convolution is a simple
    // subtraction.
    int16x8_t diff = vreinterpretq_s16_u16(vsubl_u8(s0, s2));

    vst1q_s16(tmp + i * DISFLOW_PATCH_SIZE, diff);
  }

  // Vertical filter, using kernel {1, 2, 1}.
  // This kernel can be split into two 2-taps kernels of value {1, 1}.
  // That way we need only 3 add operations to perform the convolution, one of
  // which can be reused for the next line.
  int16x8_t s0 = vld1q_s16(tmp);
  int16x8_t s1 = vld1q_s16(tmp + DISFLOW_PATCH_SIZE);
  int16x8_t sum01 = vaddq_s16(s0, s1);
  for (int i = 0; i < DISFLOW_PATCH_SIZE; i++) {
    int16x8_t s2 = vld1q_s16(tmp + (i + 2) * DISFLOW_PATCH_SIZE);

    int16x8_t sum12 = vaddq_s16(s1, s2);
    int16x8_t sum = vaddq_s16(sum01, sum12);

    vst1q_s16(dst + i * dst_stride, sum);

    sum01 = sum12;
    s1 = s2;
  }
}

static inline void sobel_filter_y(const uint8_t *src, int src_stride,
                                  int16_t *dst, int dst_stride) {
  int16_t tmp[DISFLOW_PATCH_SIZE * (DISFLOW_PATCH_SIZE + 2)];

  // Horizontal filter, using kernel {1, 2, 1}.
  // This kernel can be split into two 2-taps kernels of value {1, 1}.
  // That way we need only 3 add operations to perform the convolution.
  const uint8_t *src_start = src - 1 * src_stride - 1;

  for (int i = 0; i < DISFLOW_PATCH_SIZE + 2; i++) {
    uint8x16_t s = vld1q_u8(src_start + i * src_stride);
    uint8x8_t s0 = vget_low_u8(s);
    uint8x8_t s1 = vget_low_u8(vextq_u8(s, s, 1));
    uint8x8_t s2 = vget_low_u8(vextq_u8(s, s, 2));

    uint16x8_t sum01 = vaddl_u8(s0, s1);
    uint16x8_t sum12 = vaddl_u8(s1, s2);
    uint16x8_t sum = vaddq_u16(sum01, sum12);

    vst1q_s16(tmp + i * DISFLOW_PATCH_SIZE, vreinterpretq_s16_u16(sum));
  }

  // Vertical filter, using kernel {1, 0, -1}.
  // Load the whole block at once to avoid redundant loads during convolution.
  int16x8_t t[10];
  load_s16_8x10(tmp, DISFLOW_PATCH_SIZE, &t[0], &t[1], &t[2], &t[3], &t[4],
                &t[5], &t[6], &t[7], &t[8], &t[9]);

  for (int i = 0; i < DISFLOW_PATCH_SIZE; i++) {
    // Given that the kernel is {1, 0, -1} the convolution is a simple
    // subtraction.
    int16x8_t diff = vsubq_s16(t[i], t[i + 2]);

    vst1q_s16(dst + i * dst_stride, diff);
  }
}

#endif  // AOM_AOM_DSP_FLOW_ESTIMATION_ARM_DISFLOW_NEON_H_
