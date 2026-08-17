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
#ifndef AOM_AV1_COMMON_ARM_WARP_PLANE_NEON_H_
#define AOM_AV1_COMMON_ARM_WARP_PLANE_NEON_H_

#include <assert.h>
#include <arm_neon.h>
#include <memory.h>
#include <math.h>

#include "aom_dsp/aom_dsp_common.h"
#include "aom_dsp/arm/sum_neon.h"
#include "aom_dsp/arm/transpose_neon.h"
#include "aom_ports/mem.h"
#include "config/av1_rtcd.h"
#include "av1/common/warped_motion.h"
#include "av1/common/scale.h"

static AOM_FORCE_INLINE int16x8_t horizontal_filter_4x1_f4(const uint8x16_t in,
                                                           int sx, int alpha);

static AOM_FORCE_INLINE int16x8_t horizontal_filter_8x1_f8(const uint8x16_t in,
                                                           int sx, int alpha);

static AOM_FORCE_INLINE int16x8_t horizontal_filter_4x1_f1(const uint8x16_t in,
                                                           int sx);

static AOM_FORCE_INLINE int16x8_t horizontal_filter_8x1_f1(const uint8x16_t in,
                                                           int sx);

static AOM_FORCE_INLINE int16x8_t
horizontal_filter_4x1_f1_beta0(const uint8x16_t in, int16x8_t f_s16);

static AOM_FORCE_INLINE int16x8_t
horizontal_filter_8x1_f1_beta0(const uint8x16_t in, int16x8_t f_s16);

static AOM_FORCE_INLINE void vertical_filter_4x1_f1(const int16x8_t *src,
                                                    int32x4_t *res, int sy);

static AOM_FORCE_INLINE void vertical_filter_4x1_f4(const int16x8_t *src,
                                                    int32x4_t *res, int sy,
                                                    int gamma);

static AOM_FORCE_INLINE void vertical_filter_8x1_f1(const int16x8_t *src,
                                                    int32x4_t *res_low,
                                                    int32x4_t *res_high,
                                                    int sy);

static AOM_FORCE_INLINE void vertical_filter_8x1_f8(const int16x8_t *src,
                                                    int32x4_t *res_low,
                                                    int32x4_t *res_high, int sy,
                                                    int gamma);

static AOM_FORCE_INLINE void load_filters_4(int16x8_t out[], int offset,
                                            int stride) {
  out[0] = vld1q_s16(
      av1_warped_filter[(offset + 0 * stride) >> WARPEDDIFF_PREC_BITS]);
  out[1] = vld1q_s16(
      av1_warped_filter[(offset + 1 * stride) >> WARPEDDIFF_PREC_BITS]);
  out[2] = vld1q_s16(
      av1_warped_filter[(offset + 2 * stride) >> WARPEDDIFF_PREC_BITS]);
  out[3] = vld1q_s16(
      av1_warped_filter[(offset + 3 * stride) >> WARPEDDIFF_PREC_BITS]);
}

static AOM_FORCE_INLINE void load_filters_8(int16x8_t out[], int offset,
                                            int stride) {
  out[0] = vld1q_s16(
      av1_warped_filter[(offset + 0 * stride) >> WARPEDDIFF_PREC_BITS]);
  out[1] = vld1q_s16(
      av1_warped_filter[(offset + 1 * stride) >> WARPEDDIFF_PREC_BITS]);
  out[2] = vld1q_s16(
      av1_warped_filter[(offset + 2 * stride) >> WARPEDDIFF_PREC_BITS]);
  out[3] = vld1q_s16(
      av1_warped_filter[(offset + 3 * stride) >> WARPEDDIFF_PREC_BITS]);
  out[4] = vld1q_s16(
      av1_warped_filter[(offset + 4 * stride) >> WARPEDDIFF_PREC_BITS]);
  out[5] = vld1q_s16(
      av1_warped_filter[(offset + 5 * stride) >> WARPEDDIFF_PREC_BITS]);
  out[6] = vld1q_s16(
      av1_warped_filter[(offset + 6 * stride) >> WARPEDDIFF_PREC_BITS]);
  out[7] = vld1q_s16(
      av1_warped_filter[(offset + 7 * stride) >> WARPEDDIFF_PREC_BITS]);
}

static AOM_FORCE_INLINE int clamp_iy(int iy, int height) {
  return clamp(iy, 0, height - 1);
}

static AOM_FORCE_INLINE void warp_affine_horizontal(
    const uint8_t *ref, int width, int height, int stride, int p_width,
    int p_height, int16_t alpha, int16_t beta, const int64_t x4,
    const int64_t y4, const int i, int16x8_t tmp[]) {
  const int bd = 8;
  const int reduce_bits_horiz = ROUND0_BITS;
  const int height_limit = AOMMIN(8, p_height - i) + 7;

  int32_t ix4 = (int32_t)(x4 >> WARPEDMODEL_PREC_BITS);
  int32_t iy4 = (int32_t)(y4 >> WARPEDMODEL_PREC_BITS);

  int32_t sx4 = x4 & ((1 << WARPEDMODEL_PREC_BITS) - 1);
  sx4 += alpha * (-4) + beta * (-4) + (1 << (WARPEDDIFF_PREC_BITS - 1)) +
         (WARPEDPIXEL_PREC_SHIFTS << WARPEDDIFF_PREC_BITS);
  sx4 &= ~((1 << WARP_PARAM_REDUCE_BITS) - 1);

  if (ix4 <= -7) {
    for (int k = 0; k < height_limit; ++k) {
      int iy = clamp_iy(iy4 + k - 7, height);
      int16_t dup_val =
          (1 << (bd + FILTER_BITS - reduce_bits_horiz - 1)) +
          ref[iy * stride] * (1 << (FILTER_BITS - reduce_bits_horiz));
      tmp[k] = vdupq_n_s16(dup_val);
    }
    return;
  } else if (ix4 >= width + 6) {
    for (int k = 0; k < height_limit; ++k) {
      int iy = clamp_iy(iy4 + k - 7, height);
      int16_t dup_val = (1 << (bd + FILTER_BITS - reduce_bits_horiz - 1)) +
                        ref[iy * stride + (width - 1)] *
                            (1 << (FILTER_BITS - reduce_bits_horiz));
      tmp[k] = vdupq_n_s16(dup_val);
    }
    return;
  }

  static const uint8_t kIotaArr[] = { 0, 1, 2,  3,  4,  5,  6,  7,
                                      8, 9, 10, 11, 12, 13, 14, 15 };
  const uint8x16_t indx = vld1q_u8(kIotaArr);

  const int out_of_boundary_left = -(ix4 - 6);
  const int out_of_boundary_right = (ix4 + 8) - width;

#define APPLY_HORIZONTAL_SHIFT(fn, ...)                                \
  do {                                                                 \
    if (out_of_boundary_left >= 0 || out_of_boundary_right >= 0) {     \
      for (int k = 0; k < height_limit; ++k) {                         \
        const int iy = clamp_iy(iy4 + k - 7, height);                  \
        const uint8_t *src = ref + iy * stride + ix4 - 7;              \
        uint8x16_t src_1 = vld1q_u8(src);                              \
                                                                       \
        if (out_of_boundary_left >= 0) {                               \
          int limit = out_of_boundary_left + 1;                        \
          uint8x16_t cmp_vec = vdupq_n_u8(out_of_boundary_left);       \
          uint8x16_t vec_dup = vdupq_n_u8(*(src + limit));             \
          uint8x16_t mask_val = vcleq_u8(indx, cmp_vec);               \
          src_1 = vbslq_u8(mask_val, vec_dup, src_1);                  \
        }                                                              \
        if (out_of_boundary_right >= 0) {                              \
          int limit = 15 - (out_of_boundary_right + 1);                \
          uint8x16_t cmp_vec = vdupq_n_u8(15 - out_of_boundary_right); \
          uint8x16_t vec_dup = vdupq_n_u8(*(src + limit));             \
          uint8x16_t mask_val = vcgeq_u8(indx, cmp_vec);               \
          src_1 = vbslq_u8(mask_val, vec_dup, src_1);                  \
        }                                                              \
        tmp[k] = (fn)(src_1, __VA_ARGS__);                             \
      }                                                                \
    } else {                                                           \
      for (int k = 0; k < height_limit; ++k) {                         \
        const int iy = clamp_iy(iy4 + k - 7, height);                  \
        const uint8_t *src = ref + iy * stride + ix4 - 7;              \
        uint8x16_t src_1 = vld1q_u8(src);                              \
        tmp[k] = (fn)(src_1, __VA_ARGS__);                             \
      }                                                                \
    }                                                                  \
  } while (0)

  if (p_width == 4) {
    if (beta == 0) {
      if (alpha == 0) {
        int16x8_t f_s16 =
            vld1q_s16(av1_warped_filter[sx4 >> WARPEDDIFF_PREC_BITS]);
        APPLY_HORIZONTAL_SHIFT(horizontal_filter_4x1_f1_beta0, f_s16);
      } else {
        APPLY_HORIZONTAL_SHIFT(horizontal_filter_4x1_f4, sx4, alpha);
      }
    } else {
      if (alpha == 0) {
        APPLY_HORIZONTAL_SHIFT(horizontal_filter_4x1_f1,
                               (sx4 + beta * (k - 3)));
      } else {
        APPLY_HORIZONTAL_SHIFT(horizontal_filter_4x1_f4, (sx4 + beta * (k - 3)),
                               alpha);
      }
    }
  } else {
    if (beta == 0) {
      if (alpha == 0) {
        int16x8_t f_s16 =
            vld1q_s16(av1_warped_filter[sx4 >> WARPEDDIFF_PREC_BITS]);
        APPLY_HORIZONTAL_SHIFT(horizontal_filter_8x1_f1_beta0, f_s16);
      } else {
        APPLY_HORIZONTAL_SHIFT(horizontal_filter_8x1_f8, sx4, alpha);
      }
    } else {
      if (alpha == 0) {
        APPLY_HORIZONTAL_SHIFT(horizontal_filter_8x1_f1,
                               (sx4 + beta * (k - 3)));
      } else {
        APPLY_HORIZONTAL_SHIFT(horizontal_filter_8x1_f8, (sx4 + beta * (k - 3)),
                               alpha);
      }
    }
  }
}

static AOM_FORCE_INLINE void warp_affine_vertical(
    uint8_t *pred, int p_width, int p_height, int p_stride, int is_compound,
    uint16_t *dst, int dst_stride, int do_average, int use_dist_wtd_comp_avg,
    int16_t gamma, int16_t delta, const int64_t y4, const int i, const int j,
    int16x8_t tmp[], const int fwd, const int bwd) {
  const int bd = 8;
  const int reduce_bits_horiz = ROUND0_BITS;
  const int offset_bits_vert = bd + 2 * FILTER_BITS - reduce_bits_horiz;
  int add_const_vert;
  if (is_compound) {
    add_const_vert =
        (1 << offset_bits_vert) + (1 << (COMPOUND_ROUND1_BITS - 1));
  } else {
    add_const_vert =
        (1 << offset_bits_vert) + (1 << (2 * FILTER_BITS - ROUND0_BITS - 1));
  }
  const int sub_constant = (1 << (bd - 1)) + (1 << bd);

  const int offset_bits = bd + 2 * FILTER_BITS - ROUND0_BITS;
  const int res_sub_const =
      (1 << (2 * FILTER_BITS - ROUND0_BITS - COMPOUND_ROUND1_BITS - 1)) -
      (1 << (offset_bits - COMPOUND_ROUND1_BITS)) -
      (1 << (offset_bits - COMPOUND_ROUND1_BITS - 1));

  int32_t sy4 = y4 & ((1 << WARPEDMODEL_PREC_BITS) - 1);
  sy4 += gamma * (-4) + delta * (-4) + (1 << (WARPEDDIFF_PREC_BITS - 1)) +
         (WARPEDPIXEL_PREC_SHIFTS << WARPEDDIFF_PREC_BITS);
  sy4 &= ~((1 << WARP_PARAM_REDUCE_BITS) - 1);

  if (p_width > 4) {
    for (int k = -4; k < AOMMIN(4, p_height - i - 4); ++k) {
      int sy = sy4 + delta * (k + 4);
      const int16x8_t *v_src = tmp + (k + 4);

      int32x4_t res_lo, res_hi;
      if (gamma == 0) {
        vertical_filter_8x1_f1(v_src, &res_lo, &res_hi, sy);
      } else {
        vertical_filter_8x1_f8(v_src, &res_lo, &res_hi, sy, gamma);
      }

      res_lo = vaddq_s32(res_lo, vdupq_n_s32(add_const_vert));
      res_hi = vaddq_s32(res_hi, vdupq_n_s32(add_const_vert));

      if (is_compound) {
        uint16_t *const p = (uint16_t *)&dst[(i + k + 4) * dst_stride + j];
        int16x8_t res_s16 =
            vcombine_s16(vshrn_n_s32(res_lo, COMPOUND_ROUND1_BITS),
                         vshrn_n_s32(res_hi, COMPOUND_ROUND1_BITS));
        if (do_average) {
          int16x8_t tmp16 = vreinterpretq_s16_u16(vld1q_u16(p));
          if (use_dist_wtd_comp_avg) {
            int32x4_t tmp32_lo = vmull_n_s16(vget_low_s16(tmp16), fwd);
            int32x4_t tmp32_hi = vmull_n_s16(vget_high_s16(tmp16), fwd);
            tmp32_lo = vmlal_n_s16(tmp32_lo, vget_low_s16(res_s16), bwd);
            tmp32_hi = vmlal_n_s16(tmp32_hi, vget_high_s16(res_s16), bwd);
            tmp16 = vcombine_s16(vshrn_n_s32(tmp32_lo, DIST_PRECISION_BITS),
                                 vshrn_n_s32(tmp32_hi, DIST_PRECISION_BITS));
          } else {
            tmp16 = vhaddq_s16(tmp16, res_s16);
          }
          int16x8_t res = vaddq_s16(tmp16, vdupq_n_s16(res_sub_const));
          uint8x8_t res8 = vqshrun_n_s16(
              res, 2 * FILTER_BITS - ROUND0_BITS - COMPOUND_ROUND1_BITS);
          vst1_u8(&pred[(i + k + 4) * p_stride + j], res8);
        } else {
          vst1q_u16(p, vreinterpretq_u16_s16(res_s16));
        }
      } else {
        int16x8_t res16 =
            vcombine_s16(vshrn_n_s32(res_lo, 2 * FILTER_BITS - ROUND0_BITS),
                         vshrn_n_s32(res_hi, 2 * FILTER_BITS - ROUND0_BITS));
        res16 = vsubq_s16(res16, vdupq_n_s16(sub_constant));

        uint8_t *const p = (uint8_t *)&pred[(i + k + 4) * p_stride + j];
        vst1_u8(p, vqmovun_s16(res16));
      }
    }
  } else {
    // p_width == 4
    for (int k = -4; k < AOMMIN(4, p_height - i - 4); ++k) {
      int sy = sy4 + delta * (k + 4);
      const int16x8_t *v_src = tmp + (k + 4);

      int32x4_t res_lo;
      if (gamma == 0) {
        vertical_filter_4x1_f1(v_src, &res_lo, sy);
      } else {
        vertical_filter_4x1_f4(v_src, &res_lo, sy, gamma);
      }

      res_lo = vaddq_s32(res_lo, vdupq_n_s32(add_const_vert));

      if (is_compound) {
        uint16_t *const p = (uint16_t *)&dst[(i + k + 4) * dst_stride + j];

        int16x4_t res_lo_s16 = vshrn_n_s32(res_lo, COMPOUND_ROUND1_BITS);
        if (do_average) {
          uint8_t *const dst8 = &pred[(i + k + 4) * p_stride + j];
          int16x4_t tmp16_lo = vreinterpret_s16_u16(vld1_u16(p));
          if (use_dist_wtd_comp_avg) {
            int32x4_t tmp32_lo = vmull_n_s16(tmp16_lo, fwd);
            tmp32_lo = vmlal_n_s16(tmp32_lo, res_lo_s16, bwd);
            tmp16_lo = vshrn_n_s32(tmp32_lo, DIST_PRECISION_BITS);
          } else {
            tmp16_lo = vhadd_s16(tmp16_lo, res_lo_s16);
          }
          int16x4_t res = vadd_s16(tmp16_lo, vdup_n_s16(res_sub_const));
          uint8x8_t res8 = vqshrun_n_s16(
              vcombine_s16(res, vdup_n_s16(0)),
              2 * FILTER_BITS - ROUND0_BITS - COMPOUND_ROUND1_BITS);
          vst1_lane_u32((uint32_t *)dst8, vreinterpret_u32_u8(res8), 0);
        } else {
          uint16x4_t res_u16_low = vreinterpret_u16_s16(res_lo_s16);
          vst1_u16(p, res_u16_low);
        }
      } else {
        int16x4_t res16 = vshrn_n_s32(res_lo, 2 * FILTER_BITS - ROUND0_BITS);
        res16 = vsub_s16(res16, vdup_n_s16(sub_constant));

        uint8_t *const p = (uint8_t *)&pred[(i + k + 4) * p_stride + j];
        uint8x8_t val = vqmovun_s16(vcombine_s16(res16, vdup_n_s16(0)));
        vst1_lane_u32((uint32_t *)p, vreinterpret_u32_u8(val), 0);
      }
    }
  }
}

static AOM_FORCE_INLINE void av1_warp_affine_common(
    const int32_t *mat, const uint8_t *ref, int width, int height, int stride,
    uint8_t *pred, int p_col, int p_row, int p_width, int p_height,
    int p_stride, int subsampling_x, int subsampling_y,
    ConvolveParams *conv_params, int16_t alpha, int16_t beta, int16_t gamma,
    int16_t delta) {
  const int w0 = conv_params->fwd_offset;
  const int w1 = conv_params->bck_offset;
  const int is_compound = conv_params->is_compound;
  uint16_t *const dst = conv_params->dst;
  const int dst_stride = conv_params->dst_stride;
  const int do_average = conv_params->do_average;
  const int use_dist_wtd_comp_avg = conv_params->use_dist_wtd_comp_avg;

  assert(IMPLIES(is_compound, dst != NULL));
  assert(IMPLIES(do_average, is_compound));

  for (int i = 0; i < p_height; i += 8) {
    for (int j = 0; j < p_width; j += 8) {
      const int32_t src_x = (p_col + j + 4) << subsampling_x;
      const int32_t src_y = (p_row + i + 4) << subsampling_y;
      const int64_t dst_x =
          (int64_t)mat[2] * src_x + (int64_t)mat[3] * src_y + (int64_t)mat[0];
      const int64_t dst_y =
          (int64_t)mat[4] * src_x + (int64_t)mat[5] * src_y + (int64_t)mat[1];

      const int64_t x4 = dst_x >> subsampling_x;
      const int64_t y4 = dst_y >> subsampling_y;

      int16x8_t tmp[15];
      warp_affine_horizontal(ref, width, height, stride, p_width, p_height,
                             alpha, beta, x4, y4, i, tmp);
      warp_affine_vertical(pred, p_width, p_height, p_stride, is_compound, dst,
                           dst_stride, do_average, use_dist_wtd_comp_avg, gamma,
                           delta, y4, i, j, tmp, w0, w1);
    }
  }
}

#endif  // AOM_AV1_COMMON_ARM_WARP_PLANE_NEON_H_
