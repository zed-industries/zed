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

#ifndef AOM_AV1_COMMON_RESIZE_H_
#define AOM_AV1_COMMON_RESIZE_H_

#include <stdio.h>
#include "aom/aom_integer.h"
#include "av1/common/av1_common_int.h"

#ifdef __cplusplus
extern "C" {
#endif

// Filters for factor of 2 downsampling.
static const int16_t av1_down2_symeven_half_filter[] = { 56, 12, -3, -1 };
static const int16_t av1_down2_symodd_half_filter[] = { 64, 35, 0, -3 };

bool av1_resize_plane(const uint8_t *input, int height, int width,
                      int in_stride, uint8_t *output, int height2, int width2,
                      int out_stride);

void av1_upscale_normative_rows(const AV1_COMMON *cm, const uint8_t *src,
                                int src_stride, uint8_t *dst, int dst_stride,
                                int plane, int rows);

YV12_BUFFER_CONFIG *av1_realloc_and_scale_if_required(
    AV1_COMMON *cm, YV12_BUFFER_CONFIG *unscaled, YV12_BUFFER_CONFIG *scaled,
    const InterpFilter filter, const int phase, const bool use_optimized_scaler,
    const bool for_psnr, const int border_in_pixels, const bool alloc_pyramid);

bool av1_resize_and_extend_frame_nonnormative(const YV12_BUFFER_CONFIG *src,
                                              YV12_BUFFER_CONFIG *dst, int bd,
                                              int num_planes);

// Calculates the scaled dimensions from the given original dimensions and the
// resize scale denominator.
void av1_calculate_scaled_size(int *width, int *height, int resize_denom);

// Similar to above, but calculates scaled dimensions after superres from the
// given original dimensions and superres scale denominator.
void av1_calculate_scaled_superres_size(int *width, int *height,
                                        int superres_denom);

void av1_superres_upscale(AV1_COMMON *cm, BufferPool *const pool,
                          bool alloc_pyramid);

bool av1_resize_plane_to_half(const uint8_t *const input, int height, int width,
                              int in_stride, uint8_t *output, int height2,
                              int width2, int out_stride);

void down2_symeven(const uint8_t *const input, int length, uint8_t *output,
                   int start_offset);

bool should_resize_by_half(int height, int width, int height2, int width2);

// Returns 1 if a superres upscaled frame is scaled and 0 otherwise.
static inline int av1_superres_scaled(const AV1_COMMON *cm) {
  // Note: for some corner cases (e.g. cm->width of 1), there may be no scaling
  // required even though cm->superres_scale_denominator != SCALE_NUMERATOR.
  // So, the following check is more accurate.
  return (cm->width != cm->superres_upscaled_width);
}

// The optimized scaler av1_resize_and_extend_frame() can only handle scaling
// ratios >= 1/4 and <= 16. See comment in aom_scaled_2d_c() for detail.
// Visual assessment shows that if the scaling ratio or its reciprocal is not a
// multiple of 1/16, there are some artifacts in the output of the optimized
// scaler, especially on lines, due to non-exact ratio representation. SSSE3
// and NEON have a specialized 3/4 version of av1_resize_and_extend_frame()
// that does not have this issue.
//
// Use the non-normative scaler av1_resize_and_extend_frame_nonnormative()
// for other scaling ratios.
static inline bool av1_has_optimized_scaler(const int src_width,
                                            const int src_height,
                                            const int dst_width,
                                            const int dst_height) {
  bool has_optimized_scaler =
      (dst_width * 4 >= src_width && dst_height * 4 >= src_height) &&
      (dst_width <= src_width * 16 && dst_height <= src_height * 16) &&
      (16 * dst_width % src_width == 0) && (16 * src_width % dst_width == 0) &&
      (16 * dst_height % src_height == 0) &&
      (16 * src_height % dst_height == 0);
#if HAVE_SSSE3 || HAVE_NEON
  has_optimized_scaler =
      has_optimized_scaler ||
      (4 * dst_width == 3 * src_width && 4 * dst_height == 3 * src_height);
#endif
  return has_optimized_scaler;
}

#define UPSCALE_NORMATIVE_TAPS 8
extern const int16_t av1_resize_filter_normative[1 << RS_SUBPEL_BITS]
                                                [UPSCALE_NORMATIVE_TAPS];

int32_t av1_get_upscale_convolve_step(int in_length, int out_length);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_COMMON_RESIZE_H_
