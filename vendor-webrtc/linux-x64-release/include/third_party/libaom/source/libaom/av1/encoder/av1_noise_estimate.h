/*
 * Copyright (c) 2020, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AV1_ENCODER_AV1_NOISE_ESTIMATE_H_
#define AOM_AV1_ENCODER_AV1_NOISE_ESTIMATE_H_

#include "av1/encoder/block.h"
#include "aom_scale/yv12config.h"

#ifdef __cplusplus
extern "C" {
#endif

#define MAX_VAR_HIST_BINS 20

typedef enum noise_level { kLowLow, kLow, kMedium, kHigh } NOISE_LEVEL;

typedef struct noise_estimate {
  int enabled;
  NOISE_LEVEL level;
  int value;
  int thresh;
  int adapt_thresh;
  int count;
  int last_w;
  int last_h;
  int num_frames_estimate;
} NOISE_ESTIMATE;

struct AV1_COMP;

void av1_noise_estimate_init(NOISE_ESTIMATE *const ne, int width, int height);

NOISE_LEVEL av1_noise_estimate_extract_level(NOISE_ESTIMATE *const ne);

void av1_update_noise_estimate(struct AV1_COMP *const cpi);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_AV1_NOISE_ESTIMATE_H_
