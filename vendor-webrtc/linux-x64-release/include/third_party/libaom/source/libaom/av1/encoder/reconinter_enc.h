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

#ifndef AOM_AV1_ENCODER_RECONINTER_ENC_H_
#define AOM_AV1_ENCODER_RECONINTER_ENC_H_

#include "aom/aom_integer.h"
#include "av1/common/av1_common_int.h"
#include "av1/common/blockd.h"
#include "av1/common/convolve.h"
#include "av1/common/filter.h"
#include "av1/common/reconinter.h"
#include "av1/common/warped_motion.h"

#ifdef __cplusplus
extern "C" {
#endif

void aom_comp_mask_upsampled_pred(MACROBLOCKD *xd, const AV1_COMMON *const cm,
                                  int mi_row, int mi_col, const MV *const mv,
                                  uint8_t *comp_pred, const uint8_t *pred,
                                  int width, int height, int subpel_x_q3,
                                  int subpel_y_q3, const uint8_t *ref,
                                  int ref_stride, const uint8_t *mask,
                                  int mask_stride, int invert_mask,
                                  int subpel_search);

void aom_highbd_comp_mask_upsampled_pred(
    MACROBLOCKD *xd, const struct AV1Common *const cm, int mi_row, int mi_col,
    const MV *const mv, uint8_t *comp_pred8, const uint8_t *pred8, int width,
    int height, int subpel_x_q3, int subpel_y_q3, const uint8_t *ref8,
    int ref_stride, const uint8_t *mask, int mask_stride, int invert_mask,
    int bd, int subpel_search);

// Build single or compound reference inter predictors for all planes.
// Can build inter-intra predictors, masked predictors etc as well.
void av1_enc_build_inter_predictor(const AV1_COMMON *cm, MACROBLOCKD *xd,
                                   int mi_row, int mi_col,
                                   const BUFFER_SET *ctx, BLOCK_SIZE bsize,
                                   int plane_from, int plane_to);

void av1_enc_build_inter_predictor_y(MACROBLOCKD *xd, int mi_row, int mi_col);

void av1_enc_build_inter_predictor_y_nonrd(MACROBLOCKD *xd,
                                           InterPredParams *inter_pred_params,
                                           const SubpelParams *subpel_params);

// Build one inter predictor. It is called for building predictor for single
// reference case, or just the 1st or 2nd reference in compound reference case.
// Can build both regular and masked predictors.
void av1_enc_build_one_inter_predictor(uint8_t *dst, int dst_stride,
                                       const MV *src_mv,
                                       InterPredParams *inter_pred_params);

void av1_build_prediction_by_above_preds(const AV1_COMMON *cm, MACROBLOCKD *xd,
                                         uint8_t *tmp_buf[MAX_MB_PLANE],
                                         int tmp_width[MAX_MB_PLANE],
                                         int tmp_height[MAX_MB_PLANE],
                                         int tmp_stride[MAX_MB_PLANE]);

void av1_build_prediction_by_left_preds(const AV1_COMMON *cm, MACROBLOCKD *xd,
                                        uint8_t *tmp_buf[MAX_MB_PLANE],
                                        int tmp_width[MAX_MB_PLANE],
                                        int tmp_height[MAX_MB_PLANE],
                                        int tmp_stride[MAX_MB_PLANE]);

void av1_build_obmc_inter_predictors_sb(const AV1_COMMON *cm, MACROBLOCKD *xd);

// |ext_dst*| are indexed from |plane_from| to |plane_to| inclusive.
void av1_build_inter_predictors_for_planes_single_buf(
    MACROBLOCKD *xd, BLOCK_SIZE bsize, int plane_from, int plane_to, int ref,
    uint8_t *ext_dst[], int ext_dst_stride[]);

// |ext_dst*| are indexed from |plane_from| to |plane_to| inclusive.
void av1_build_wedge_inter_predictor_from_buf(MACROBLOCKD *xd, BLOCK_SIZE bsize,
                                              int plane_from, int plane_to,
                                              uint8_t *ext_dst0[],
                                              int ext_dst_stride0[],
                                              uint8_t *ext_dst1[],
                                              int ext_dst_stride1[]);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_RECONINTER_ENC_H_
