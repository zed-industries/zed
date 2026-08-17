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

#ifndef AOM_AV1_ENCODER_GLOBAL_MOTION_FACADE_H_
#define AOM_AV1_ENCODER_GLOBAL_MOTION_FACADE_H_

#ifdef __cplusplus
extern "C" {
#endif
struct yv12_buffer_config;
struct AV1_COMP;

// Allocates memory for members of GlobalMotionData.
static inline void gm_alloc_data(AV1_COMP *cpi, GlobalMotionData *gm_data) {
  AV1_COMMON *cm = &cpi->common;
  GlobalMotionInfo *gm_info = &cpi->gm_info;

  CHECK_MEM_ERROR(cm, gm_data->segment_map,
                  aom_malloc(sizeof(*gm_data->segment_map) *
                             gm_info->segment_map_w * gm_info->segment_map_h));

  av1_zero_array(gm_data->motion_models, RANSAC_NUM_MOTIONS);
  for (int m = 0; m < RANSAC_NUM_MOTIONS; m++) {
    CHECK_MEM_ERROR(cm, gm_data->motion_models[m].inliers,
                    aom_malloc(sizeof(*gm_data->motion_models[m].inliers) * 2 *
                               MAX_CORNERS));
  }
}

// Deallocates the memory allocated for members of GlobalMotionData.
static inline void gm_dealloc_data(GlobalMotionData *gm_data) {
  aom_free(gm_data->segment_map);
  gm_data->segment_map = NULL;
  for (int m = 0; m < RANSAC_NUM_MOTIONS; m++) {
    aom_free(gm_data->motion_models[m].inliers);
    gm_data->motion_models[m].inliers = NULL;
  }
}

void av1_compute_gm_for_valid_ref_frames(
    AV1_COMP *cpi, struct aom_internal_error_info *error_info,
    YV12_BUFFER_CONFIG *ref_buf[REF_FRAMES], int frame,
    MotionModel *motion_models, uint8_t *segment_map, int segment_map_w,
    int segment_map_h);
void av1_compute_global_motion_facade(struct AV1_COMP *cpi);
#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_GLOBAL_MOTION_FACADE_H_
