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

#ifndef AOM_AV1_ENCODER_X86_ML_SSE3_H_
#define AOM_AV1_ENCODER_X86_ML_SSE3_H_

#include <pmmintrin.h>

void av1_nn_propagate_4to1_sse3(const float *const inputs,
                                const float *const weights,
                                __m128 *const output);

void av1_nn_propagate_4to4_sse3(const float *const inputs,
                                const float *const weights,
                                __m128 *const outputs, const int num_inputs);

void av1_nn_propagate_4to8_sse3(const float *const inputs,
                                const float *const weights, __m128 *const out_h,
                                __m128 *const out_l, const int num_inputs);

#endif  // AOM_AV1_ENCODER_X86_ML_SSE3_H_
