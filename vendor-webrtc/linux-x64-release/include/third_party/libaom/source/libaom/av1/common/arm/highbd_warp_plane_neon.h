/*
 * Copyright (c) 2023, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */
#ifndef AOM_AV1_COMMON_ARM_HIGHBD_WARP_PLANE_NEON_H_
#define AOM_AV1_COMMON_ARM_HIGHBD_WARP_PLANE_NEON_H_

#include <arm_neon.h>
#include <assert.h>
#include <stdbool.h>

#include "aom_dsp/aom_dsp_common.h"
#include "aom_dsp/arm/mem_neon.h"
#include "aom_dsp/arm/sum_neon.h"
#include "aom_ports/mem.h"
#include "av1/common/scale.h"
#include "av1/common/warped_motion.h"
#include "config/av1_rtcd.h"

static AOM_FORCE_INLINE int16x8_t
highbd_horizontal_filter_4x1_f4(int16x8_t rv0, int16x8_t rv1, int16x8_t rv2,
                                int16x8_t rv3, int bd, int sx, int alpha);

static AOM_FORCE_INLINE int16x8_t highbd_horizontal_filter_8x1_f8(
    int16x8_t rv0, int16x8_t rv1, int16x8_t rv2, int16x8_t rv3, int16x8_t rv4,
    int16x8_t rv5, int16x8_t rv6, int16x8_t rv7, int bd, int sx, int alpha);

static AOM_FORCE_INLINE int16x8_t highbd_horizontal_filter_4x1_f1(
    int16x8_t rv0, int16x8_t rv1, int16x8_t rv2, int16x8_t rv3, int bd, int sx);

static AOM_FORCE_INLINE int16x8_t highbd_horizontal_filter_8x1_f1(
    int16x8_t rv0, int16x8_t rv1, int16x8_t rv2, int16x8_t rv3, int16x8_t rv4,
    int16x8_t rv5, int16x8_t rv6, int16x8_t rv7, int bd, int sx);

static AOM_FORCE_INLINE int32x4_t vertical_filter_4x1_f1(const int16x8_t *tmp,
                                                         int sy);

static AOM_FORCE_INLINE int32x4x2_t vertical_filter_8x1_f1(const int16x8_t *tmp,
                                                           int sy);

static AOM_FORCE_INLINE int32x4_t vertical_filter_4x1_f4(const int16x8_t *tmp,
                                                         int sy, int gamma);

static AOM_FORCE_INLINE int32x4x2_t vertical_filter_8x1_f8(const int16x8_t *tmp,
                                                           int sy, int gamma);

static AOM_FORCE_INLINE int16x8_t load_filters_1(int ofs) {
  const int ofs0 = ROUND_POWER_OF_TWO(ofs, WARPEDDIFF_PREC_BITS);

  const int16_t *base = av1_warped_filter[WARPEDPIXEL_PREC_SHIFTS];
  return vld1q_s16(base + ofs0 * 8);
}

static AOM_FORCE_INLINE void load_filters_4(int16x8_t out[], int ofs,
                                            int stride) {
  const int ofs0 = ROUND_POWER_OF_TWO(ofs + stride * 0, WARPEDDIFF_PREC_BITS);
  const int ofs1 = ROUND_POWER_OF_TWO(ofs + stride * 1, WARPEDDIFF_PREC_BITS);
  const int ofs2 = ROUND_POWER_OF_TWO(ofs + stride * 2, WARPEDDIFF_PREC_BITS);
  const int ofs3 = ROUND_POWER_OF_TWO(ofs + stride * 3, WARPEDDIFF_PREC_BITS);

  const int16_t *base = av1_warped_filter[WARPEDPIXEL_PREC_SHIFTS];
  out[0] = vld1q_s16(base + ofs0 * 8);
  out[1] = vld1q_s16(base + ofs1 * 8);
  out[2] = vld1q_s16(base + ofs2 * 8);
  out[3] = vld1q_s16(base + ofs3 * 8);
}

static AOM_FORCE_INLINE void load_filters_8(int16x8_t out[], int ofs,
                                            int stride) {
  const int ofs0 = ROUND_POWER_OF_TWO(ofs + stride * 0, WARPEDDIFF_PREC_BITS);
  const int ofs1 = ROUND_POWER_OF_TWO(ofs + stride * 1, WARPEDDIFF_PREC_BITS);
  const int ofs2 = ROUND_POWER_OF_TWO(ofs + stride * 2, WARPEDDIFF_PREC_BITS);
  const int ofs3 = ROUND_POWER_OF_TWO(ofs + stride * 3, WARPEDDIFF_PREC_BITS);
  const int ofs4 = ROUND_POWER_OF_TWO(ofs + stride * 4, WARPEDDIFF_PREC_BITS);
  const int ofs5 = ROUND_POWER_OF_TWO(ofs + stride * 5, WARPEDDIFF_PREC_BITS);
  const int ofs6 = ROUND_POWER_OF_TWO(ofs + stride * 6, WARPEDDIFF_PREC_BITS);
  const int ofs7 = ROUND_POWER_OF_TWO(ofs + stride * 7, WARPEDDIFF_PREC_BITS);

  const int16_t *base = av1_warped_filter[WARPEDPIXEL_PREC_SHIFTS];
  out[0] = vld1q_s16(base + ofs0 * 8);
  out[1] = vld1q_s16(base + ofs1 * 8);
  out[2] = vld1q_s16(base + ofs2 * 8);
  out[3] = vld1q_s16(base + ofs3 * 8);
  out[4] = vld1q_s16(base + ofs4 * 8);
  out[5] = vld1q_s16(base + ofs5 * 8);
  out[6] = vld1q_s16(base + ofs6 * 8);
  out[7] = vld1q_s16(base + ofs7 * 8);
}

static AOM_FORCE_INLINE uint16x4_t clip_pixel_highbd_vec(int32x4_t val,
                                                         int bd) {
  const int limit = (1 << bd) - 1;
  return vqmovun_s32(vminq_s32(val, vdupq_n_s32(limit)));
}

static AOM_FORCE_INLINE uint16x8x2_t clamp_horizontal(
    uint16x8x2_t src_1, int out_of_boundary_left, int out_of_boundary_right,
    const uint16_t *ref, int iy, int stride, int width, const uint16x8_t indx0,
    const uint16x8_t indx1) {
  if (out_of_boundary_left >= 0) {
    uint16x8_t cmp_vec = vdupq_n_u16(out_of_boundary_left);
    uint16x8_t vec_dup = vdupq_n_u16(ref[iy * stride]);
    uint16x8_t mask0 = vcleq_u16(indx0, cmp_vec);
    uint16x8_t mask1 = vcleq_u16(indx1, cmp_vec);
    src_1.val[0] = vbslq_u16(mask0, vec_dup, src_1.val[0]);
    src_1.val[1] = vbslq_u16(mask1, vec_dup, src_1.val[1]);
  }
  if (out_of_boundary_right >= 0) {
    uint16x8_t cmp_vec = vdupq_n_u16(15 - out_of_boundary_right);
    uint16x8_t vec_dup = vdupq_n_u16(ref[iy * stride + width - 1]);
    uint16x8_t mask0 = vcgeq_u16(indx0, cmp_vec);
    uint16x8_t mask1 = vcgeq_u16(indx1, cmp_vec);
    src_1.val[0] = vbslq_u16(mask0, vec_dup, src_1.val[0]);
    src_1.val[1] = vbslq_u16(mask1, vec_dup, src_1.val[1]);
  }
  return src_1;
}

static AOM_FORCE_INLINE void warp_affine_horizontal(const uint16_t *ref,
                                                    int width, int height,
                                                    int stride, int p_width,
                                                    int16_t alpha, int16_t beta,
                                                    int iy4, int sx4, int ix4,
                                                    int16x8_t tmp[], int bd) {
  const int round0 = (bd == 12) ? ROUND0_BITS + 2 : ROUND0_BITS;

  if (ix4 <= -7) {
    for (int k = 0; k < 15; ++k) {
      int iy = clamp(iy4 + k - 7, 0, height - 1);
      int32_t dup_val = (1 << (bd + FILTER_BITS - round0 - 1)) +
                        ref[iy * stride] * (1 << (FILTER_BITS - round0));
      tmp[k] = vdupq_n_s16(dup_val);
    }
    return;
  } else if (ix4 >= width + 6) {
    for (int k = 0; k < 15; ++k) {
      int iy = clamp(iy4 + k - 7, 0, height - 1);
      int32_t dup_val =
          (1 << (bd + FILTER_BITS - round0 - 1)) +
          ref[iy * stride + (width - 1)] * (1 << (FILTER_BITS - round0));
      tmp[k] = vdupq_n_s16(dup_val);
    }
    return;
  }

  static const uint16_t kIotaArr[] = { 0, 1, 2,  3,  4,  5,  6,  7,
                                       8, 9, 10, 11, 12, 13, 14, 15 };
  const uint16x8_t indx0 = vld1q_u16(kIotaArr);
  const uint16x8_t indx1 = vld1q_u16(kIotaArr + 8);

  const int out_of_boundary_left = -(ix4 - 6);
  const int out_of_boundary_right = (ix4 + 8) - width;

#define APPLY_HORIZONTAL_SHIFT_4X1(fn, ...)                                \
  do {                                                                     \
    if (out_of_boundary_left >= 0 || out_of_boundary_right >= 0) {         \
      for (int k = 0; k < 15; ++k) {                                       \
        const int iy = clamp(iy4 + k - 7, 0, height - 1);                  \
        const uint16_t *idx = ref + iy * stride + ix4 - 7;                 \
        /* We don't use vld1q_u16_x2 here as LLVM generates an incorrect   \
         * alignment hint for this intrinsic that causes a SIGBUS on Armv7 \
         * targets when alignment checks are enabled.                      \
         * (See bug: b/349455146) */                                       \
        uint16x8x2_t src_1 = { { vld1q_u16(idx), vld1q_u16(idx + 8) } };   \
        src_1 = clamp_horizontal(src_1, out_of_boundary_left,              \
                                 out_of_boundary_right, ref, iy, stride,   \
                                 width, indx0, indx1);                     \
        int16x8_t rv0 = vextq_s16(vreinterpretq_s16_u16(src_1.val[0]),     \
                                  vreinterpretq_s16_u16(src_1.val[1]), 0); \
        int16x8_t rv1 = vextq_s16(vreinterpretq_s16_u16(src_1.val[0]),     \
                                  vreinterpretq_s16_u16(src_1.val[1]), 1); \
        int16x8_t rv2 = vextq_s16(vreinterpretq_s16_u16(src_1.val[0]),     \
                                  vreinterpretq_s16_u16(src_1.val[1]), 2); \
        int16x8_t rv3 = vextq_s16(vreinterpretq_s16_u16(src_1.val[0]),     \
                                  vreinterpretq_s16_u16(src_1.val[1]), 3); \
        tmp[k] = (fn)(rv0, rv1, rv2, rv3, __VA_ARGS__);                    \
      }                                                                    \
    } else {                                                               \
      for (int k = 0; k < 15; ++k) {                                       \
        const int iy = clamp(iy4 + k - 7, 0, height - 1);                  \
        const uint16_t *src = ref + iy * stride + ix4;                     \
        int16x8_t rv0 = vreinterpretq_s16_u16(vld1q_u16(src - 7));         \
        int16x8_t rv1 = vreinterpretq_s16_u16(vld1q_u16(src - 6));         \
        int16x8_t rv2 = vreinterpretq_s16_u16(vld1q_u16(src - 5));         \
        int16x8_t rv3 = vreinterpretq_s16_u16(vld1q_u16(src - 4));         \
        tmp[k] = (fn)(rv0, rv1, rv2, rv3, __VA_ARGS__);                    \
      }                                                                    \
    }                                                                      \
  } while (0)

#define APPLY_HORIZONTAL_SHIFT_8X1(fn, ...)                                 \
  do {                                                                      \
    if (out_of_boundary_left >= 0 || out_of_boundary_right >= 0) {          \
      for (int k = 0; k < 15; ++k) {                                        \
        const int iy = clamp(iy4 + k - 7, 0, height - 1);                   \
        const uint16_t *idx = ref + iy * stride + ix4 - 7;                  \
        /* We don't use vld1q_u16_x2 here as LLVM generates an incorrect    \
         * alignment hint for this intrinsic that causes a SIGBUS on Armv7  \
         * targets when alignment checks are enabled.                       \
         * (See bug: b/349455146) */                                        \
        uint16x8x2_t src_1 = { { vld1q_u16(idx), vld1q_u16(idx + 8) } };    \
        src_1 = clamp_horizontal(src_1, out_of_boundary_left,               \
                                 out_of_boundary_right, ref, iy, stride,    \
                                 width, indx0, indx1);                      \
        int16x8_t rv0 = vextq_s16(vreinterpretq_s16_u16(src_1.val[0]),      \
                                  vreinterpretq_s16_u16(src_1.val[1]), 0);  \
        int16x8_t rv1 = vextq_s16(vreinterpretq_s16_u16(src_1.val[0]),      \
                                  vreinterpretq_s16_u16(src_1.val[1]), 1);  \
        int16x8_t rv2 = vextq_s16(vreinterpretq_s16_u16(src_1.val[0]),      \
                                  vreinterpretq_s16_u16(src_1.val[1]), 2);  \
        int16x8_t rv3 = vextq_s16(vreinterpretq_s16_u16(src_1.val[0]),      \
                                  vreinterpretq_s16_u16(src_1.val[1]), 3);  \
        int16x8_t rv4 = vextq_s16(vreinterpretq_s16_u16(src_1.val[0]),      \
                                  vreinterpretq_s16_u16(src_1.val[1]), 4);  \
        int16x8_t rv5 = vextq_s16(vreinterpretq_s16_u16(src_1.val[0]),      \
                                  vreinterpretq_s16_u16(src_1.val[1]), 5);  \
        int16x8_t rv6 = vextq_s16(vreinterpretq_s16_u16(src_1.val[0]),      \
                                  vreinterpretq_s16_u16(src_1.val[1]), 6);  \
        int16x8_t rv7 = vextq_s16(vreinterpretq_s16_u16(src_1.val[0]),      \
                                  vreinterpretq_s16_u16(src_1.val[1]), 7);  \
        tmp[k] = (fn)(rv0, rv1, rv2, rv3, rv4, rv5, rv6, rv7, __VA_ARGS__); \
      }                                                                     \
    } else {                                                                \
      for (int k = 0; k < 15; ++k) {                                        \
        const int iy = clamp(iy4 + k - 7, 0, height - 1);                   \
        const uint16_t *src = ref + iy * stride + ix4;                      \
        int16x8_t rv0 = vreinterpretq_s16_u16(vld1q_u16(src - 7));          \
        int16x8_t rv1 = vreinterpretq_s16_u16(vld1q_u16(src - 6));          \
        int16x8_t rv2 = vreinterpretq_s16_u16(vld1q_u16(src - 5));          \
        int16x8_t rv3 = vreinterpretq_s16_u16(vld1q_u16(src - 4));          \
        int16x8_t rv4 = vreinterpretq_s16_u16(vld1q_u16(src - 3));          \
        int16x8_t rv5 = vreinterpretq_s16_u16(vld1q_u16(src - 2));          \
        int16x8_t rv6 = vreinterpretq_s16_u16(vld1q_u16(src - 1));          \
        int16x8_t rv7 = vreinterpretq_s16_u16(vld1q_u16(src - 0));          \
        tmp[k] = (fn)(rv0, rv1, rv2, rv3, rv4, rv5, rv6, rv7, __VA_ARGS__); \
      }                                                                     \
    }                                                                       \
  } while (0)

  if (p_width == 4) {
    if (beta == 0) {
      if (alpha == 0) {
        APPLY_HORIZONTAL_SHIFT_4X1(highbd_horizontal_filter_4x1_f1, bd, sx4);
      } else {
        APPLY_HORIZONTAL_SHIFT_4X1(highbd_horizontal_filter_4x1_f4, bd, sx4,
                                   alpha);
      }
    } else {
      if (alpha == 0) {
        APPLY_HORIZONTAL_SHIFT_4X1(highbd_horizontal_filter_4x1_f1, bd,
                                   (sx4 + beta * (k - 3)));
      } else {
        APPLY_HORIZONTAL_SHIFT_4X1(highbd_horizontal_filter_4x1_f4, bd,
                                   (sx4 + beta * (k - 3)), alpha);
      }
    }
  } else {
    if (beta == 0) {
      if (alpha == 0) {
        APPLY_HORIZONTAL_SHIFT_8X1(highbd_horizontal_filter_8x1_f1, bd, sx4);
      } else {
        APPLY_HORIZONTAL_SHIFT_8X1(highbd_horizontal_filter_8x1_f8, bd, sx4,
                                   alpha);
      }
    } else {
      if (alpha == 0) {
        APPLY_HORIZONTAL_SHIFT_8X1(highbd_horizontal_filter_8x1_f1, bd,
                                   (sx4 + beta * (k - 3)));
      } else {
        APPLY_HORIZONTAL_SHIFT_8X1(highbd_horizontal_filter_8x1_f8, bd,
                                   (sx4 + beta * (k - 3)), alpha);
      }
    }
  }

#undef APPLY_HORIZONTAL_SHIFT_4X1
#undef APPLY_HORIZONTAL_SHIFT_8X1
}

static AOM_FORCE_INLINE void highbd_vertical_filter_4x1_f4(
    uint16_t *pred, int p_stride, int bd, uint16_t *dst, int dst_stride,
    bool is_compound, bool do_average, bool use_dist_wtd_comp_avg, int fwd,
    int bwd, int16_t gamma, const int16x8_t *tmp, int i, int sy, int j) {
  int32x4_t sum0 = gamma == 0 ? vertical_filter_4x1_f1(tmp, sy)
                              : vertical_filter_4x1_f4(tmp, sy, gamma);

  const int round0 = (bd == 12) ? ROUND0_BITS + 2 : ROUND0_BITS;
  const int offset_bits_vert = bd + 2 * FILTER_BITS - round0;

  sum0 = vaddq_s32(sum0, vdupq_n_s32(1 << offset_bits_vert));

  uint16_t *dst16 = &pred[i * p_stride + j];

  if (!is_compound) {
    const int reduce_bits_vert = 2 * FILTER_BITS - round0;
    sum0 = vrshlq_s32(sum0, vdupq_n_s32(-reduce_bits_vert));

    const int res_sub_const = (1 << (bd - 1)) + (1 << bd);
    sum0 = vsubq_s32(sum0, vdupq_n_s32(res_sub_const));
    uint16x4_t res0 = clip_pixel_highbd_vec(sum0, bd);
    vst1_u16(dst16, res0);
    return;
  }

  sum0 = vrshrq_n_s32(sum0, COMPOUND_ROUND1_BITS);

  uint16_t *p = &dst[i * dst_stride + j];

  if (!do_average) {
    vst1_u16(p, vqmovun_s32(sum0));
    return;
  }

  uint16x4_t p0 = vld1_u16(p);
  int32x4_t p_vec0 = vreinterpretq_s32_u32(vmovl_u16(p0));
  if (use_dist_wtd_comp_avg) {
    p_vec0 = vmulq_n_s32(p_vec0, fwd);
    p_vec0 = vmlaq_n_s32(p_vec0, sum0, bwd);
    p_vec0 = vshrq_n_s32(p_vec0, DIST_PRECISION_BITS);
  } else {
    p_vec0 = vhaddq_s32(p_vec0, sum0);
  }

  const int offset_bits = bd + 2 * FILTER_BITS - round0;
  const int round1 = COMPOUND_ROUND1_BITS;
  const int res_sub_const =
      (1 << (offset_bits - round1)) + (1 << (offset_bits - round1 - 1));
  const int round_bits = 2 * FILTER_BITS - round0 - round1;

  p_vec0 = vsubq_s32(p_vec0, vdupq_n_s32(res_sub_const));
  p_vec0 = vrshlq_s32(p_vec0, vdupq_n_s32(-round_bits));
  uint16x4_t res0 = clip_pixel_highbd_vec(p_vec0, bd);
  vst1_u16(dst16, res0);
}

static AOM_FORCE_INLINE void highbd_vertical_filter_8x1_f8(
    uint16_t *pred, int p_stride, int bd, uint16_t *dst, int dst_stride,
    bool is_compound, bool do_average, bool use_dist_wtd_comp_avg, int fwd,
    int bwd, int16_t gamma, const int16x8_t *tmp, int i, int sy, int j) {
  int32x4x2_t sums = gamma == 0 ? vertical_filter_8x1_f1(tmp, sy)
                                : vertical_filter_8x1_f8(tmp, sy, gamma);
  int32x4_t sum0 = sums.val[0];
  int32x4_t sum1 = sums.val[1];

  const int round0 = (bd == 12) ? ROUND0_BITS + 2 : ROUND0_BITS;
  const int offset_bits_vert = bd + 2 * FILTER_BITS - round0;

  sum0 = vaddq_s32(sum0, vdupq_n_s32(1 << offset_bits_vert));
  sum1 = vaddq_s32(sum1, vdupq_n_s32(1 << offset_bits_vert));

  uint16_t *dst16 = &pred[i * p_stride + j];

  if (!is_compound) {
    const int reduce_bits_vert = 2 * FILTER_BITS - round0;
    sum0 = vrshlq_s32(sum0, vdupq_n_s32(-reduce_bits_vert));
    sum1 = vrshlq_s32(sum1, vdupq_n_s32(-reduce_bits_vert));

    const int res_sub_const = (1 << (bd - 1)) + (1 << bd);
    sum0 = vsubq_s32(sum0, vdupq_n_s32(res_sub_const));
    sum1 = vsubq_s32(sum1, vdupq_n_s32(res_sub_const));
    uint16x4_t res0 = clip_pixel_highbd_vec(sum0, bd);
    uint16x4_t res1 = clip_pixel_highbd_vec(sum1, bd);
    vst1_u16(dst16, res0);
    vst1_u16(dst16 + 4, res1);
    return;
  }

  sum0 = vrshrq_n_s32(sum0, COMPOUND_ROUND1_BITS);
  sum1 = vrshrq_n_s32(sum1, COMPOUND_ROUND1_BITS);

  uint16_t *p = &dst[i * dst_stride + j];

  if (!do_average) {
    vst1_u16(p, vqmovun_s32(sum0));
    vst1_u16(p + 4, vqmovun_s32(sum1));
    return;
  }

  uint16x8_t p0 = vld1q_u16(p);
  int32x4_t p_vec0 = vreinterpretq_s32_u32(vmovl_u16(vget_low_u16(p0)));
  int32x4_t p_vec1 = vreinterpretq_s32_u32(vmovl_u16(vget_high_u16(p0)));
  if (use_dist_wtd_comp_avg) {
    p_vec0 = vmulq_n_s32(p_vec0, fwd);
    p_vec1 = vmulq_n_s32(p_vec1, fwd);
    p_vec0 = vmlaq_n_s32(p_vec0, sum0, bwd);
    p_vec1 = vmlaq_n_s32(p_vec1, sum1, bwd);
    p_vec0 = vshrq_n_s32(p_vec0, DIST_PRECISION_BITS);
    p_vec1 = vshrq_n_s32(p_vec1, DIST_PRECISION_BITS);
  } else {
    p_vec0 = vhaddq_s32(p_vec0, sum0);
    p_vec1 = vhaddq_s32(p_vec1, sum1);
  }

  const int offset_bits = bd + 2 * FILTER_BITS - round0;
  const int round1 = COMPOUND_ROUND1_BITS;
  const int res_sub_const =
      (1 << (offset_bits - round1)) + (1 << (offset_bits - round1 - 1));
  const int round_bits = 2 * FILTER_BITS - round0 - round1;

  p_vec0 = vsubq_s32(p_vec0, vdupq_n_s32(res_sub_const));
  p_vec1 = vsubq_s32(p_vec1, vdupq_n_s32(res_sub_const));

  p_vec0 = vrshlq_s32(p_vec0, vdupq_n_s32(-round_bits));
  p_vec1 = vrshlq_s32(p_vec1, vdupq_n_s32(-round_bits));
  uint16x4_t res0 = clip_pixel_highbd_vec(p_vec0, bd);
  uint16x4_t res1 = clip_pixel_highbd_vec(p_vec1, bd);
  vst1_u16(dst16, res0);
  vst1_u16(dst16 + 4, res1);
}

static AOM_FORCE_INLINE void warp_affine_vertical(
    uint16_t *pred, int p_width, int p_height, int p_stride, int bd,
    uint16_t *dst, int dst_stride, bool is_compound, bool do_average,
    bool use_dist_wtd_comp_avg, int fwd, int bwd, int16_t gamma, int16_t delta,
    const int16x8_t *tmp, int i, int sy4, int j) {
  int limit_height = p_height > 4 ? 8 : 4;

  if (p_width > 4) {
    // p_width == 8
    for (int k = 0; k < limit_height; ++k) {
      int sy = sy4 + delta * k;
      highbd_vertical_filter_8x1_f8(
          pred, p_stride, bd, dst, dst_stride, is_compound, do_average,
          use_dist_wtd_comp_avg, fwd, bwd, gamma, tmp + k, i + k, sy, j);
    }
  } else {
    // p_width == 4
    for (int k = 0; k < limit_height; ++k) {
      int sy = sy4 + delta * k;
      highbd_vertical_filter_4x1_f4(
          pred, p_stride, bd, dst, dst_stride, is_compound, do_average,
          use_dist_wtd_comp_avg, fwd, bwd, gamma, tmp + k, i + k, sy, j);
    }
  }
}

static AOM_FORCE_INLINE void highbd_warp_affine_common(
    const int32_t *mat, const uint16_t *ref, int width, int height, int stride,
    uint16_t *pred, int p_col, int p_row, int p_width, int p_height,
    int p_stride, int subsampling_x, int subsampling_y, int bd,
    ConvolveParams *conv_params, int16_t alpha, int16_t beta, int16_t gamma,
    int16_t delta) {
  uint16_t *const dst = conv_params->dst;
  const int dst_stride = conv_params->dst_stride;
  const bool is_compound = conv_params->is_compound;
  const bool do_average = conv_params->do_average;
  const bool use_dist_wtd_comp_avg = conv_params->use_dist_wtd_comp_avg;
  const int fwd = conv_params->fwd_offset;
  const int bwd = conv_params->bck_offset;

  assert(IMPLIES(is_compound, dst != NULL));

  for (int i = 0; i < p_height; i += 8) {
    for (int j = 0; j < p_width; j += 8) {
      // Calculate the center of this 8x8 block,
      // project to luma coordinates (if in a subsampled chroma plane),
      // apply the affine transformation,
      // then convert back to the original coordinates (if necessary)
      const int32_t src_x = (j + 4 + p_col) << subsampling_x;
      const int32_t src_y = (i + 4 + p_row) << subsampling_y;
      const int64_t dst_x =
          (int64_t)mat[2] * src_x + (int64_t)mat[3] * src_y + (int64_t)mat[0];
      const int64_t dst_y =
          (int64_t)mat[4] * src_x + (int64_t)mat[5] * src_y + (int64_t)mat[1];
      const int64_t x4 = dst_x >> subsampling_x;
      const int64_t y4 = dst_y >> subsampling_y;

      const int32_t ix4 = (int32_t)(x4 >> WARPEDMODEL_PREC_BITS);
      int32_t sx4 = x4 & ((1 << WARPEDMODEL_PREC_BITS) - 1);
      const int32_t iy4 = (int32_t)(y4 >> WARPEDMODEL_PREC_BITS);
      int32_t sy4 = y4 & ((1 << WARPEDMODEL_PREC_BITS) - 1);

      sx4 += alpha * (-4) + beta * (-4);
      sy4 += gamma * (-4) + delta * (-4);

      sx4 &= ~((1 << WARP_PARAM_REDUCE_BITS) - 1);
      sy4 &= ~((1 << WARP_PARAM_REDUCE_BITS) - 1);

      // Each horizontal filter result is formed by the sum of up to eight
      // multiplications by filter values and then a shift. Although both the
      // inputs and filters are loaded as int16, the input data is at most bd
      // bits and the filters are at most 8 bits each. Additionally since we
      // know all possible filter values we know that the sum of absolute
      // filter values will fit in at most 9 bits. With this in mind we can
      // conclude that the sum of each filter application will fit in bd + 9
      // bits. The shift following the summation is ROUND0_BITS (which is 3),
      // +2 for 12-bit, which gives us a final storage of:
      // bd ==  8: ( 8 + 9) - 3 => 14 bits
      // bd == 10: (10 + 9) - 3 => 16 bits
      // bd == 12: (12 + 9) - 5 => 16 bits
      // So it is safe to use int16x8_t as the intermediate storage type here.
      int16x8_t tmp[15];

      warp_affine_horizontal(ref, width, height, stride, p_width, alpha, beta,
                             iy4, sx4, ix4, tmp, bd);
      warp_affine_vertical(pred, p_width, p_height, p_stride, bd, dst,
                           dst_stride, is_compound, do_average,
                           use_dist_wtd_comp_avg, fwd, bwd, gamma, delta, tmp,
                           i, sy4, j);
    }
  }
}

#endif  // AOM_AV1_COMMON_ARM_HIGHBD_WARP_PLANE_NEON_H_
