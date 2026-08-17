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

#ifndef AOM_AOM_DSP_VARIANCE_H_
#define AOM_AOM_DSP_VARIANCE_H_

#include "config/aom_config.h"

#include "aom/aom_integer.h"

#ifdef __cplusplus
extern "C" {
#endif

#define FILTER_BITS 7
#define FILTER_WEIGHT 128

typedef unsigned int (*aom_sad_fn_t)(const uint8_t *a, int a_stride,
                                     const uint8_t *b, int b_stride);

typedef unsigned int (*aom_sad_avg_fn_t)(const uint8_t *a, int a_stride,
                                         const uint8_t *b, int b_stride,
                                         const uint8_t *second_pred);

typedef void (*aom_copy32xn_fn_t)(const uint8_t *a, int a_stride, uint8_t *b,
                                  int b_stride, int n);

typedef void (*aom_sad_multi_d_fn_t)(const uint8_t *a, int a_stride,
                                     const uint8_t *const b_array[],
                                     int b_stride, unsigned int *sad_array);

typedef unsigned int (*aom_variance_fn_t)(const uint8_t *a, int a_stride,
                                          const uint8_t *b, int b_stride,
                                          unsigned int *sse);

typedef unsigned int (*aom_subpixvariance_fn_t)(const uint8_t *a, int a_stride,
                                                int xoffset, int yoffset,
                                                const uint8_t *b, int b_stride,
                                                unsigned int *sse);

typedef unsigned int (*aom_subp_avg_variance_fn_t)(
    const uint8_t *a, int a_stride, int xoffset, int yoffset, const uint8_t *b,
    int b_stride, unsigned int *sse, const uint8_t *second_pred);

typedef unsigned int (*aom_dist_wtd_sad_avg_fn_t)(
    const uint8_t *a, int a_stride, const uint8_t *b, int b_stride,
    const uint8_t *second_pred, const DIST_WTD_COMP_PARAMS *jcp_param);

typedef unsigned int (*aom_dist_wtd_subp_avg_variance_fn_t)(
    const uint8_t *a, int a_stride, int xoffset, int yoffset, const uint8_t *b,
    int b_stride, unsigned int *sse, const uint8_t *second_pred,
    const DIST_WTD_COMP_PARAMS *jcp_param);

typedef unsigned int (*aom_masked_sad_fn_t)(const uint8_t *src, int src_stride,
                                            const uint8_t *ref, int ref_stride,
                                            const uint8_t *second_pred,
                                            const uint8_t *msk, int msk_stride,
                                            int invert_mask);
typedef unsigned int (*aom_masked_subpixvariance_fn_t)(
    const uint8_t *src, int src_stride, int xoffset, int yoffset,
    const uint8_t *ref, int ref_stride, const uint8_t *second_pred,
    const uint8_t *msk, int msk_stride, int invert_mask, unsigned int *sse);

typedef unsigned int (*aom_obmc_sad_fn_t)(const uint8_t *pred, int pred_stride,
                                          const int32_t *wsrc,
                                          const int32_t *msk);
typedef unsigned int (*aom_obmc_variance_fn_t)(const uint8_t *pred,
                                               int pred_stride,
                                               const int32_t *wsrc,
                                               const int32_t *msk,
                                               unsigned int *sse);
typedef unsigned int (*aom_obmc_subpixvariance_fn_t)(
    const uint8_t *pred, int pred_stride, int xoffset, int yoffset,
    const int32_t *wsrc, const int32_t *msk, unsigned int *sse);

typedef struct aom_variance_vtable {
  aom_sad_fn_t sdf;
  // Same as normal sad, but downsample the rows by a factor of 2.
  aom_sad_fn_t sdsf;
  aom_sad_avg_fn_t sdaf;
  aom_variance_fn_t vf;
  aom_subpixvariance_fn_t svf;
  aom_subp_avg_variance_fn_t svaf;
  aom_sad_multi_d_fn_t sdx4df;
  aom_sad_multi_d_fn_t sdx3df;
  // Same as sadx4, but downsample the rows by a factor of 2.
  aom_sad_multi_d_fn_t sdsx4df;
  aom_masked_sad_fn_t msdf;
  aom_masked_subpixvariance_fn_t msvf;
  aom_obmc_sad_fn_t osdf;
  aom_obmc_variance_fn_t ovf;
  aom_obmc_subpixvariance_fn_t osvf;
} aom_variance_fn_ptr_t;

void aom_highbd_var_filter_block2d_bil_first_pass(
    const uint8_t *src_ptr8, uint16_t *output_ptr,
    unsigned int src_pixels_per_line, int pixel_step,
    unsigned int output_height, unsigned int output_width,
    const uint8_t *filter);

void aom_highbd_var_filter_block2d_bil_second_pass(
    const uint16_t *src_ptr, uint16_t *output_ptr,
    unsigned int src_pixels_per_line, unsigned int pixel_step,
    unsigned int output_height, unsigned int output_width,
    const uint8_t *filter);

uint32_t aom_sse_odd_size(const uint8_t *a, int a_stride, const uint8_t *b,
                          int b_stride, int w, int h);

uint64_t aom_highbd_sse_odd_size(const uint8_t *a, int a_stride,
                                 const uint8_t *b, int b_stride, int w, int h);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AOM_DSP_VARIANCE_H_
