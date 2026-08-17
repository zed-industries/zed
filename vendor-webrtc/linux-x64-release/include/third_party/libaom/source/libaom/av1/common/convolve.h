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

#ifndef AOM_AV1_COMMON_CONVOLVE_H_
#define AOM_AV1_COMMON_CONVOLVE_H_
#include "av1/common/filter.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef uint16_t CONV_BUF_TYPE;
typedef struct ConvolveParams {
  int do_average;
  CONV_BUF_TYPE *dst;
  int dst_stride;
  int round_0;
  int round_1;
  int plane;
  int is_compound;
  int use_dist_wtd_comp_avg;
  int fwd_offset;
  int bck_offset;
} ConvolveParams;

typedef struct WienerConvolveParams {
  int round_0;
  int round_1;
} WienerConvolveParams;

#define ROUND0_BITS 3
#define COMPOUND_ROUND1_BITS 7
#define WIENER_ROUND0_BITS 3

#define WIENER_CLAMP_LIMIT(r0, bd) (1 << ((bd) + 1 + FILTER_BITS - r0))

typedef void (*aom_convolve_fn_t)(const uint8_t *src, int src_stride,
                                  uint8_t *dst, int dst_stride, int w, int h,
                                  const InterpFilterParams *filter_params_x,
                                  const InterpFilterParams *filter_params_y,
                                  const int subpel_x_qn, const int subpel_y_qn,
                                  ConvolveParams *conv_params);

typedef void (*aom_highbd_convolve_fn_t)(
    const uint16_t *src, int src_stride, uint16_t *dst, int dst_stride, int w,
    int h, const InterpFilterParams *filter_params_x,
    const InterpFilterParams *filter_params_y, const int subpel_x_qn,
    const int subpel_y_qn, ConvolveParams *conv_params, int bd);

struct AV1Common;
struct scale_factors;

void av1_convolve_2d_facade(const uint8_t *src, int src_stride, uint8_t *dst,
                            int dst_stride, int w, int h,
                            const InterpFilterParams *interp_filters[2],
                            const int subpel_x_qn, int x_step_q4,
                            const int subpel_y_qn, int y_step_q4, int scaled,
                            ConvolveParams *conv_params);

static inline ConvolveParams get_conv_params_no_round(int cmp_index, int plane,
                                                      CONV_BUF_TYPE *dst,
                                                      int dst_stride,
                                                      int is_compound, int bd) {
  ConvolveParams conv_params;
  assert(IMPLIES(cmp_index, is_compound));

  conv_params.is_compound = is_compound;
  conv_params.use_dist_wtd_comp_avg = 0;
  conv_params.round_0 = ROUND0_BITS;
  conv_params.round_1 = is_compound ? COMPOUND_ROUND1_BITS
                                    : 2 * FILTER_BITS - conv_params.round_0;
#if CONFIG_AV1_HIGHBITDEPTH
  const int intbufrange = bd + FILTER_BITS - conv_params.round_0 + 2;
  assert(IMPLIES(bd < 12, intbufrange <= 16));
  if (intbufrange > 16) {
    conv_params.round_0 += intbufrange - 16;
    if (!is_compound) conv_params.round_1 -= intbufrange - 16;
  }
#else
  (void)bd;
#endif  // CONFIG_AV1_HIGHBITDEPTH
  // TODO(yunqing): The following dst should only be valid while
  // is_compound = 1;
  conv_params.dst = dst;
  conv_params.dst_stride = dst_stride;
  conv_params.plane = plane;

  // By default, set do average to 1 if this is the second single prediction
  // in a compound mode.
  conv_params.do_average = cmp_index;
  return conv_params;
}

static inline ConvolveParams get_conv_params(int do_average, int plane,
                                             int bd) {
  return get_conv_params_no_round(do_average, plane, NULL, 0, 0, bd);
}

static inline WienerConvolveParams get_conv_params_wiener(int bd) {
  WienerConvolveParams conv_params;
  conv_params.round_0 = WIENER_ROUND0_BITS;
  conv_params.round_1 = 2 * FILTER_BITS - conv_params.round_0;
  const int intbufrange = bd + FILTER_BITS - conv_params.round_0 + 2;
  assert(IMPLIES(bd < 12, intbufrange <= 16));
  if (intbufrange > 16) {
    conv_params.round_0 += intbufrange - 16;
    conv_params.round_1 -= intbufrange - 16;
  }
  return conv_params;
}

void av1_highbd_convolve_2d_facade(const uint8_t *src8, int src_stride,
                                   uint8_t *dst, int dst_stride, int w, int h,
                                   const InterpFilterParams *interp_filters[2],
                                   const int subpel_x_qn, int x_step_q4,
                                   const int subpel_y_qn, int y_step_q4,
                                   int scaled, ConvolveParams *conv_params,
                                   int bd);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_COMMON_CONVOLVE_H_
