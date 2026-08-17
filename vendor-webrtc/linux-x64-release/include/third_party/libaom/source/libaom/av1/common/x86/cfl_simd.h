/*
 * Copyright (c) 2017, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AV1_COMMON_X86_CFL_SIMD_H_
#define AOM_AV1_COMMON_X86_CFL_SIMD_H_

#include "av1/common/blockd.h"

// SSSE3 version is optimal for with == 4, we reuse them in AVX2
void cfl_subsample_lbd_420_4x4_ssse3(const uint8_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_lbd_420_4x8_ssse3(const uint8_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_lbd_420_4x16_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);

// SSSE3 version is optimal for with == 8, we reuse it in AVX2
void cfl_subsample_lbd_420_8x4_ssse3(const uint8_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_lbd_420_8x8_ssse3(const uint8_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_lbd_420_8x16_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);
void cfl_subsample_lbd_420_8x32_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);

// SSSE3 version is optimal for with == 16, we reuse it in AVX2
void cfl_subsample_lbd_420_16x4_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);
void cfl_subsample_lbd_420_16x8_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);
void cfl_subsample_lbd_420_16x16_ssse3(const uint8_t *cfl_type,
                                       int input_stride, uint16_t *output_q3);
void cfl_subsample_lbd_420_16x32_ssse3(const uint8_t *cfl_type,
                                       int input_stride, uint16_t *output_q3);

// SSSE3 version is optimal for with == 4, we reuse them in AVX2
void cfl_subsample_lbd_422_4x4_ssse3(const uint8_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_lbd_422_4x8_ssse3(const uint8_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_lbd_422_4x16_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);

// SSSE3 version is optimal for with == 8, we reuse it in AVX2
void cfl_subsample_lbd_422_8x4_ssse3(const uint8_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_lbd_422_8x8_ssse3(const uint8_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_lbd_422_8x16_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);
void cfl_subsample_lbd_422_8x32_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);

// SSSE3 version is optimal for with == 16, we reuse it in AVX2
void cfl_subsample_lbd_422_16x4_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);
void cfl_subsample_lbd_422_16x8_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);
void cfl_subsample_lbd_422_16x16_ssse3(const uint8_t *cfl_type,
                                       int input_stride, uint16_t *output_q3);
void cfl_subsample_lbd_422_16x32_ssse3(const uint8_t *cfl_type,
                                       int input_stride, uint16_t *output_q3);

// SSSE3 version is optimal for with == 4, we reuse them in AVX2
void cfl_subsample_lbd_444_4x4_ssse3(const uint8_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_lbd_444_4x8_ssse3(const uint8_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_lbd_444_4x16_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);

// SSSE3 version is optimal for with == 8, we reuse it in AVX2
void cfl_subsample_lbd_444_8x4_ssse3(const uint8_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_lbd_444_8x8_ssse3(const uint8_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_lbd_444_8x16_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);
void cfl_subsample_lbd_444_8x32_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);

// SSSE3 version is optimal for with == 16, we reuse it in AVX2
void cfl_subsample_lbd_444_16x4_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);
void cfl_subsample_lbd_444_16x8_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);
void cfl_subsample_lbd_444_16x16_ssse3(const uint8_t *cfl_type,
                                       int input_stride, uint16_t *output_q3);
void cfl_subsample_lbd_444_16x32_ssse3(const uint8_t *cfl_type,
                                       int input_stride, uint16_t *output_q3);

#if CONFIG_AV1_HIGHBITDEPTH
void cfl_subsample_hbd_420_4x4_ssse3(const uint16_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_hbd_420_4x8_ssse3(const uint16_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_hbd_420_4x16_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);

// SSSE3 version is optimal for with == 8, we reuse it in AVX2
void cfl_subsample_hbd_420_8x4_ssse3(const uint16_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_hbd_420_8x8_ssse3(const uint16_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_hbd_420_8x16_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_420_8x32_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);

// SSSE3 version is faster for with == 16, we reuse it in AVX2
void cfl_subsample_hbd_420_16x4_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_420_16x8_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_420_16x16_ssse3(const uint16_t *cfl_type,
                                       int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_420_16x32_ssse3(const uint16_t *cfl_type,
                                       int input_stride, uint16_t *output_q3);

void cfl_subsample_hbd_422_4x4_ssse3(const uint16_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_hbd_422_4x8_ssse3(const uint16_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_hbd_422_4x16_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);

// SSSE3 version is optimal for with == 8, we reuse it in AVX2
void cfl_subsample_hbd_422_8x4_ssse3(const uint16_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_hbd_422_8x8_ssse3(const uint16_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_hbd_422_8x16_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_422_8x32_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);

// SSSE3 version is faster for with == 16, we reuse it in AVX2
void cfl_subsample_hbd_422_16x4_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_422_16x8_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_422_16x16_ssse3(const uint16_t *cfl_type,
                                       int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_422_16x32_ssse3(const uint16_t *cfl_type,
                                       int input_stride, uint16_t *output_q3);

void cfl_subsample_hbd_444_4x4_ssse3(const uint16_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_hbd_444_4x8_ssse3(const uint16_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_hbd_444_4x16_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);

// SSSE3 version is optimal for with == 8, we reuse it in AVX2
void cfl_subsample_hbd_444_8x4_ssse3(const uint16_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_hbd_444_8x8_ssse3(const uint16_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_hbd_444_8x16_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_444_8x32_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);

// SSSE3 version is faster for with == 16, we reuse it in AVX2
void cfl_subsample_hbd_444_16x4_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_444_16x8_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_444_16x16_ssse3(const uint16_t *cfl_type,
                                       int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_444_16x32_ssse3(const uint16_t *cfl_type,
                                       int input_stride, uint16_t *output_q3);
#endif  // CONFIG_AV1_HIGHBITDEPTH

// SSE2 version is optimal for with == 4, we reuse them in AVX2
void cfl_subtract_average_4x4_sse2(const uint16_t *src, int16_t *dst);
void cfl_subtract_average_4x8_sse2(const uint16_t *src, int16_t *dst);
void cfl_subtract_average_4x16_sse2(const uint16_t *src, int16_t *dst);

// SSE2 version is optimal for with == 8, we reuse them in AVX2
void cfl_subtract_average_8x4_sse2(const uint16_t *src, int16_t *dst);
void cfl_subtract_average_8x8_sse2(const uint16_t *src, int16_t *dst);
void cfl_subtract_average_8x16_sse2(const uint16_t *src, int16_t *dst);
void cfl_subtract_average_8x32_sse2(const uint16_t *src, int16_t *dst);

void cfl_predict_lbd_4x4_ssse3(const int16_t *pred_buf_q3, uint8_t *dst,
                               int dst_stride, int alpha_q3);
void cfl_predict_lbd_4x8_ssse3(const int16_t *pred_buf_q3, uint8_t *dst,
                               int dst_stride, int alpha_q3);
void cfl_predict_lbd_4x16_ssse3(const int16_t *pred_buf_q3, uint8_t *dst,
                                int dst_stride, int alpha_q3);

void cfl_predict_lbd_8x4_ssse3(const int16_t *pred_buf_q3, uint8_t *dst,
                               int dst_stride, int alpha_q3);
void cfl_predict_lbd_8x8_ssse3(const int16_t *pred_buf_q3, uint8_t *dst,
                               int dst_stride, int alpha_q3);
void cfl_predict_lbd_8x16_ssse3(const int16_t *pred_buf_q3, uint8_t *dst,
                                int dst_stride, int alpha_q3);
void cfl_predict_lbd_8x32_ssse3(const int16_t *pred_buf_q3, uint8_t *dst,
                                int dst_stride, int alpha_q3);

void cfl_predict_lbd_16x4_ssse3(const int16_t *pred_buf_q3, uint8_t *dst,
                                int dst_stride, int alpha_q3);
void cfl_predict_lbd_16x8_ssse3(const int16_t *pred_buf_q3, uint8_t *dst,
                                int dst_stride, int alpha_q3);
void cfl_predict_lbd_16x16_ssse3(const int16_t *pred_buf_q3, uint8_t *dst,
                                 int dst_stride, int alpha_q3);
void cfl_predict_lbd_16x32_ssse3(const int16_t *pred_buf_q3, uint8_t *dst,
                                 int dst_stride, int alpha_q3);

#if CONFIG_AV1_HIGHBITDEPTH
void cfl_predict_hbd_4x4_ssse3(const int16_t *pred_buf_q3, uint16_t *dst,
                               int dst_stride, int alpha_q3, int bd);
void cfl_predict_hbd_4x8_ssse3(const int16_t *pred_buf_q3, uint16_t *dst,
                               int dst_stride, int alpha_q3, int bd);
void cfl_predict_hbd_4x16_ssse3(const int16_t *pred_buf_q3, uint16_t *dst,
                                int dst_stride, int alpha_q3, int bd);

void cfl_predict_hbd_8x4_ssse3(const int16_t *pred_buf_q3, uint16_t *dst,
                               int dst_stride, int alpha_q3, int bd);
void cfl_predict_hbd_8x8_ssse3(const int16_t *pred_buf_q3, uint16_t *dst,
                               int dst_stride, int alpha_q3, int bd);
void cfl_predict_hbd_8x16_ssse3(const int16_t *pred_buf_q3, uint16_t *dst,
                                int dst_stride, int alpha_q3, int bd);
void cfl_predict_hbd_8x32_ssse3(const int16_t *pred_buf_q3, uint16_t *dst,
                                int dst_stride, int alpha_q3, int bd);

void cfl_predict_hbd_16x4_ssse3(const int16_t *pred_buf_q3, uint16_t *dst,
                                int dst_stride, int alpha_q3, int bd);
void cfl_predict_hbd_16x8_ssse3(const int16_t *pred_buf_q3, uint16_t *dst,
                                int dst_stride, int alpha_q3, int bd);
void cfl_predict_hbd_16x16_ssse3(const int16_t *pred_buf_q3, uint16_t *dst,
                                 int dst_stride, int alpha_q3, int bd);
void cfl_predict_hbd_16x32_ssse3(const int16_t *pred_buf_q3, uint16_t *dst,
                                 int dst_stride, int alpha_q3, int bd);
#endif  // CONFIG_AV1_HIGHBITDEPTH
#endif  // AOM_AV1_COMMON_X86_CFL_SIMD_H_
