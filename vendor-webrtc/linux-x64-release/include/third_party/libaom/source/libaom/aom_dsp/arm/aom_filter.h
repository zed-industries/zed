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

#ifndef AOM_AOM_DSP_ARM_AOM_FILTER_H_
#define AOM_AOM_DSP_ARM_AOM_FILTER_H_

#include <stdint.h>

#include "config/aom_config.h"
#include "config/aom_dsp_rtcd.h"

static inline int get_filter_taps_convolve8(const int16_t *filter) {
  if (filter[0] | filter[7]) {
    return 8;
  }
  if (filter[1] | filter[6]) {
    return 6;
  }
  if (filter[2] | filter[5]) {
    return 4;
  }
  return 2;
}

#endif  // AOM_AOM_DSP_ARM_AOM_FILTER_H_
