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

#ifndef AOM_AOM_DSP_FLOW_ESTIMATION_DISFLOW_H_
#define AOM_AOM_DSP_FLOW_ESTIMATION_DISFLOW_H_

#include <stdbool.h>

#include "aom_dsp/flow_estimation/flow_estimation.h"
#include "aom_scale/yv12config.h"

#ifdef __cplusplus
extern "C" {
#endif

// Number of pyramid levels in disflow computation
#define DISFLOW_PYRAMID_LEVELS 12

// Size of square patches in the disflow dense grid
// Must be a power of 2
#define DISFLOW_PATCH_SIZE_LOG2 3
#define DISFLOW_PATCH_SIZE (1 << DISFLOW_PATCH_SIZE_LOG2)
// Center point of square patch
#define DISFLOW_PATCH_CENTER ((DISFLOW_PATCH_SIZE / 2) - 1)

// Overall scale of the `dx`, `dy` and `dt` arrays in the disflow code
// In other words, the various derivatives are calculated with an internal
// precision of (8 + DISFLOW_DERIV_SCALE_LOG2) bits, from an 8-bit input.
//
// This must be carefully synchronized with the code in sobel_filter()
// (which fills the dx and dy arrays) and compute_flow_error() (which
// fills dt); see the comments in those functions for more details
#define DISFLOW_DERIV_SCALE_LOG2 3
#define DISFLOW_DERIV_SCALE (1 << DISFLOW_DERIV_SCALE_LOG2)

// Scale factor applied to each step in the main refinement loop
//
// This should be <= 1.0 to avoid overshoot. Values below 1.0
// may help in some cases, but slow convergence overall, so
// will require careful tuning.
// TODO(rachelbarker): Tune this value
#define DISFLOW_STEP_SIZE 1.0

// Step size at which we should terminate iteration
// The idea here is that, if we take a step which is much smaller than 1px in
// size, then the values won't change much from iteration to iteration, so
// many future steps will also be small, and that won't have much effect
// on the ultimate result. So we can terminate early.
//
// To look at it another way, when we take a small step, that means that
// either we're near to convergence (so can stop), or we're stuck in a
// shallow valley and will take many iterations to get unstuck.
//
// Solving the latter properly requires fancier methods, such as "gradient
// descent with momentum". For now, we terminate to avoid wasting a ton of
// time on points which are either nearly-converged or stuck.
//
// Terminating at 1/8 px seems to give good results for global motion estimation
#define DISFLOW_STEP_SIZE_THRESOLD (1. / 8.)

// Max number of iterations if warp convergence is not found
#define DISFLOW_MAX_ITR 4

// Internal precision of cubic interpolation filters
// The limiting factor here is that:
// * Before integerizing, the maximum value of any kernel tap is 1.0
// * After integerizing, each tap must fit into an int16_t.
// Thus the largest multiplier we can get away with is 2^14 = 16384,
// as 2^15 = 32768 is too large to fit in an int16_t.
#define DISFLOW_INTERP_BITS 14

typedef struct {
  // Start of allocation for u and v buffers
  double *buf0;

  // x and y directions of flow, per patch
  double *u;
  double *v;

  // Sizes of the above arrays
  int width;
  int height;
  int stride;
} FlowField;

bool av1_compute_global_motion_disflow(
    TransformationType type, YV12_BUFFER_CONFIG *src, YV12_BUFFER_CONFIG *ref,
    int bit_depth, int downsample_level, MotionModel *motion_models,
    int num_motion_models, bool *mem_alloc_failed);

#ifdef __cplusplus
}
#endif

#endif  // AOM_AOM_DSP_FLOW_ESTIMATION_DISFLOW_H_
