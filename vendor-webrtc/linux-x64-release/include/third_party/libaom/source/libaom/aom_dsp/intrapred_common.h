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

#ifndef AOM_AOM_DSP_INTRAPRED_COMMON_H_
#define AOM_AOM_DSP_INTRAPRED_COMMON_H_

#include "config/aom_config.h"

// Weights are quadratic from '1' to '1 / block_size', scaled by
// 2^SMOOTH_WEIGHT_LOG2_SCALE.
#define SMOOTH_WEIGHT_LOG2_SCALE 8

// Note these arrays are aligned to ensure NEON loads using a cast to uint32_t*
// have sufficient alignment. Using 8 preserves the potential for an alignment
// hint in load_weight_w8(). For that case, this could be increased to 16 to
// allow an aligned load in x86.
DECLARE_ALIGNED(8, static const uint8_t, smooth_weights[]) = {
  // bs = 4
  255, 149, 85, 64,
  // bs = 8
  255, 197, 146, 105, 73, 50, 37, 32,
  // bs = 16
  255, 225, 196, 170, 145, 123, 102, 84, 68, 54, 43, 33, 26, 20, 17, 16,
  // bs = 32
  255, 240, 225, 210, 196, 182, 169, 157, 145, 133, 122, 111, 101, 92, 83, 74,
  66, 59, 52, 45, 39, 34, 29, 25, 21, 17, 14, 12, 10, 9, 8, 8,
  // bs = 64
  255, 248, 240, 233, 225, 218, 210, 203, 196, 189, 182, 176, 169, 163, 156,
  150, 144, 138, 133, 127, 121, 116, 111, 106, 101, 96, 91, 86, 82, 77, 73, 69,
  65, 61, 57, 54, 50, 47, 44, 41, 38, 35, 32, 29, 27, 25, 22, 20, 18, 16, 15,
  13, 12, 10, 9, 8, 7, 6, 6, 5, 5, 4, 4, 4
};

DECLARE_ALIGNED(8, static const uint16_t, smooth_weights_u16[]) = {
  // block dimension = 4
  255, 149, 85, 64,
  // block dimension = 8
  255, 197, 146, 105, 73, 50, 37, 32,
  // block dimension = 16
  255, 225, 196, 170, 145, 123, 102, 84, 68, 54, 43, 33, 26, 20, 17, 16,
  // block dimension = 32
  255, 240, 225, 210, 196, 182, 169, 157, 145, 133, 122, 111, 101, 92, 83, 74,
  66, 59, 52, 45, 39, 34, 29, 25, 21, 17, 14, 12, 10, 9, 8, 8,
  // block dimension = 64
  255, 248, 240, 233, 225, 218, 210, 203, 196, 189, 182, 176, 169, 163, 156,
  150, 144, 138, 133, 127, 121, 116, 111, 106, 101, 96, 91, 86, 82, 77, 73, 69,
  65, 61, 57, 54, 50, 47, 44, 41, 38, 35, 32, 29, 27, 25, 22, 20, 18, 16, 15,
  13, 12, 10, 9, 8, 7, 6, 6, 5, 5, 4, 4, 4
};

#endif  // AOM_AOM_DSP_INTRAPRED_COMMON_H_
