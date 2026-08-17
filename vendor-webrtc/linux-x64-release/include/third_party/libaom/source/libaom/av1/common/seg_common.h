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

#ifndef AOM_AV1_COMMON_SEG_COMMON_H_
#define AOM_AV1_COMMON_SEG_COMMON_H_

#include "aom_dsp/prob.h"

#ifdef __cplusplus
extern "C" {
#endif

#define MAX_SEGMENTS 8
#define SEG_TREE_PROBS (MAX_SEGMENTS - 1)

#define SEG_TEMPORAL_PRED_CTXS 3
#define SPATIAL_PREDICTION_PROBS 3

enum {
  SEG_LVL_ALT_Q,       // Use alternate Quantizer ....
  SEG_LVL_ALT_LF_Y_V,  // Use alternate loop filter value on y plane vertical
  SEG_LVL_ALT_LF_Y_H,  // Use alternate loop filter value on y plane horizontal
  SEG_LVL_ALT_LF_U,    // Use alternate loop filter value on u plane
  SEG_LVL_ALT_LF_V,    // Use alternate loop filter value on v plane
  SEG_LVL_REF_FRAME,   // Optional Segment reference frame
  SEG_LVL_SKIP,        // Optional Segment (0,0) + skip mode
  SEG_LVL_GLOBALMV,
  SEG_LVL_MAX
} UENUM1BYTE(SEG_LVL_FEATURES);

struct segmentation {
  uint8_t enabled;
  uint8_t update_map;
  uint8_t update_data;
  uint8_t temporal_update;

  int16_t feature_data[MAX_SEGMENTS][SEG_LVL_MAX];
  unsigned int feature_mask[MAX_SEGMENTS];
  int last_active_segid;  // The highest numbered segment id that has some
                          // enabled feature.
  uint8_t segid_preskip;  // Whether the segment id will be read before the
                          // skip syntax element.
                          // 1: the segment id will be read first.
                          // 0: the skip syntax element will be read first.
};

struct segmentation_probs {
  aom_cdf_prob pred_cdf[SEG_TEMPORAL_PRED_CTXS][CDF_SIZE(2)];
  aom_cdf_prob spatial_pred_seg_cdf[SPATIAL_PREDICTION_PROBS]
                                   [CDF_SIZE(MAX_SEGMENTS)];
};

static inline int segfeature_active(const struct segmentation *seg,
                                    uint8_t segment_id,
                                    SEG_LVL_FEATURES feature_id) {
  return seg->enabled && (seg->feature_mask[segment_id] & (1 << feature_id));
}

static inline void segfeatures_copy(struct segmentation *dst,
                                    const struct segmentation *src) {
  int i, j;
  for (i = 0; i < MAX_SEGMENTS; i++) {
    dst->feature_mask[i] = src->feature_mask[i];
    for (j = 0; j < SEG_LVL_MAX; j++) {
      dst->feature_data[i][j] = src->feature_data[i][j];
    }
  }
  dst->segid_preskip = src->segid_preskip;
  dst->last_active_segid = src->last_active_segid;
}

void av1_clearall_segfeatures(struct segmentation *seg);

void av1_enable_segfeature(struct segmentation *seg, int segment_id,
                           SEG_LVL_FEATURES feature_id);

void av1_calculate_segdata(struct segmentation *seg);

int av1_seg_feature_data_max(SEG_LVL_FEATURES feature_id);

int av1_is_segfeature_signed(SEG_LVL_FEATURES feature_id);

void av1_set_segdata(struct segmentation *seg, int segment_id,
                     SEG_LVL_FEATURES feature_id, int seg_data);

static inline int get_segdata(const struct segmentation *seg, int segment_id,
                              SEG_LVL_FEATURES feature_id) {
  return seg->feature_data[segment_id][feature_id];
}

static inline void set_segment_id(uint8_t *segment_ids, int mi_offset,
                                  int x_mis, int y_mis, int mi_stride,
                                  uint8_t segment_id) {
  segment_ids += mi_offset;
  for (int y = 0; y < y_mis; ++y) {
    memset(&segment_ids[y * mi_stride], segment_id,
           x_mis * sizeof(segment_ids[0]));
  }
}

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_COMMON_SEG_COMMON_H_
