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

#ifndef AOM_AV1_ENCODER_GLOBAL_MOTION_H_
#define AOM_AV1_ENCODER_GLOBAL_MOTION_H_

#include "aom/aom_integer.h"
#include "aom_dsp/flow_estimation/flow_estimation.h"
#include "aom_util/aom_pthread.h"
#include "av1/encoder/enc_enums.h"

#ifdef __cplusplus
extern "C" {
#endif

#define RANSAC_NUM_MOTIONS 1
#define GM_MAX_REFINEMENT_STEPS 5
#define MAX_DIRECTIONS 2

// The structure holds a valid reference frame type and its temporal distance
// from the source frame.
typedef struct {
  int distance;
  MV_REFERENCE_FRAME frame;
} FrameDistPair;

typedef struct {
  // Array of structure which holds the global motion parameters for a given
  // motion model. motion_models[i] holds the parameters for a given motion
  // model for the ith ransac motion.
  MotionModel motion_models[RANSAC_NUM_MOTIONS];

  // Pointer to hold inliers from motion model.
  uint8_t *segment_map;
} GlobalMotionData;

typedef struct {
  // Holds the mapping of each thread to past/future direction.
  // thread_id_to_dir[i] indicates the direction id (past - 0/future - 1)
  // assigned to the ith thread.
  int8_t thread_id_to_dir[MAX_NUM_THREADS];

  // A flag which holds the early exit status based on the speed feature
  // 'prune_ref_frame_for_gm_search'. early_exit[i] will be set if the speed
  // feature based early exit happens in the direction 'i'.
  int8_t early_exit[MAX_DIRECTIONS];

  // Counter for the next reference frame to be processed.
  // next_frame_to_process[i] will hold the count of next reference frame to be
  // processed in the direction 'i'.
  int8_t next_frame_to_process[MAX_DIRECTIONS];
} GlobalMotionJobInfo;

typedef struct {
  // Data related to assigning jobs for global motion multi-threading.
  GlobalMotionJobInfo job_info;

#if CONFIG_MULTITHREAD
  // Mutex lock used while dispatching jobs.
  pthread_mutex_t *mutex_;
#endif

  // Initialized to false, set to true by the worker thread that encounters an
  // error in order to abort the processing of other worker threads.
  bool gm_mt_exit;
} AV1GlobalMotionSync;

void av1_convert_model_to_params(const double *params,
                                 WarpedMotionParams *model);

// Criteria for accepting a global motion model
static const double erroradv_tr[2] = { 0.65, 0.2 };
static const double erroradv_prod_tr = 20000;

// Early exit threshold for global motion refinement
// This is set slightly higher than erroradv_tr, as a compromise between
// two factors:
//
// 1) By rejecting un-promising models early, we can reduce the encode time
//    spent trying to refine them
//
// 2) When we refine a model, its error may decrease to below the acceptance
//    threshold even if the model is initially above the threshold
static const double erroradv_early_tr = 0.70;

int av1_is_enough_erroradvantage(double best_erroradvantage, int params_cost,
                                 double gm_erroradv_tr);

void av1_compute_feature_segmentation_map(uint8_t *segment_map, int width,
                                          int height, int *inliers,
                                          int num_inliers);

int64_t av1_segmented_frame_error(int use_hbd, int bd, const uint8_t *ref,
                                  int ref_stride, uint8_t *dst, int dst_stride,
                                  int p_width, int p_height,
                                  uint8_t *segment_map, int segment_map_stride);

// Returns the warp error between "dst" and the result of applying the
// motion params that result from fine-tuning "wm" to "ref". Note that "wm" is
// modified in place.
int64_t av1_refine_integerized_param(
    WarpedMotionParams *wm, TransformationType wmtype, int use_hbd, int bd,
    uint8_t *ref, int r_width, int r_height, int r_stride, uint8_t *dst,
    int d_width, int d_height, int d_stride, int n_refinements,
    int64_t ref_frame_error, uint8_t *segment_map, int segment_map_stride,
    double gm_erroradv_tr);

#ifdef __cplusplus
}  // extern "C"
#endif
#endif  // AOM_AV1_ENCODER_GLOBAL_MOTION_H_
