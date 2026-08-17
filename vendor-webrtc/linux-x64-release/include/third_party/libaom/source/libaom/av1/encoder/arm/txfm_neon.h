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

#ifndef AOM_AV1_ENCODER_ARM_TXFM_NEON_H_
#define AOM_AV1_ENCODER_ARM_TXFM_NEON_H_

#include <stdint.h>

static inline void ud_adjust_input_and_stride(int ud_flip,
                                              const int16_t **input,
                                              int *stride, int out_size) {
  if (ud_flip) {
    *input = *input + (out_size - 1) * *stride;
    *stride = -*stride;
  }
}

#endif  // AOM_AV1_ENCODER_ARM_TXFM_NEON_H_
