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

#ifndef AOM_AOM_DSP_FLOW_ESTIMATION_CORNER_MATCH_H_
#define AOM_AOM_DSP_FLOW_ESTIMATION_CORNER_MATCH_H_

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <memory.h>

#include "aom_dsp/flow_estimation/corner_detect.h"
#include "aom_dsp/flow_estimation/flow_estimation.h"
#include "aom_scale/yv12config.h"

#ifdef __cplusplus
extern "C" {
#endif

#define MATCH_SZ 16
#define MATCH_SZ_BY2 ((MATCH_SZ - 1) / 2)
#define MATCH_SZ_SQ (MATCH_SZ * MATCH_SZ)

// Minimum threshold for the variance of a patch, in order for it to be
// considered useful for matching.
// This is evaluated against the scaled variance MATCH_SZ_SQ * sigma^2,
// so a setting of 1 * MATCH_SZ_SQ corresponds to an unscaled variance of 1
#define MIN_FEATURE_VARIANCE (1 * MATCH_SZ_SQ)

bool av1_compute_global_motion_feature_match(
    TransformationType type, YV12_BUFFER_CONFIG *src, YV12_BUFFER_CONFIG *ref,
    int bit_depth, int downsample_level, MotionModel *motion_models,
    int num_motion_models, bool *mem_alloc_failed);

#ifdef __cplusplus
}
#endif

#endif  // AOM_AOM_DSP_FLOW_ESTIMATION_CORNER_MATCH_H_
