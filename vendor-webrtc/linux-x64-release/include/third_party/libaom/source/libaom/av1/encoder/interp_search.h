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

#ifndef AOM_AV1_ENCODER_INTERP_FILTER_SEARCH_H_
#define AOM_AV1_ENCODER_INTERP_FILTER_SEARCH_H_

#include "av1/encoder/block.h"
#include "av1/encoder/encoder.h"
#include "av1/encoder/rdopt_utils.h"

#ifdef __cplusplus
extern "C" {
#endif

/*!\cond */
#define MAX_INTERP_FILTER_STATS 128
#define DUAL_FILTER_SET_SIZE (SWITCHABLE_FILTERS * SWITCHABLE_FILTERS)

typedef struct {
  int_interpfilters filters;
  int_mv mv[2];
  int8_t ref_frames[2];
  COMPOUND_TYPE comp_type;
  int compound_idx;
  int64_t rd;
  unsigned int pred_sse;
} INTERPOLATION_FILTER_STATS;
/*!\endcond */

/*!\brief Miscellaneous arguments for inter mode search.
 */
typedef struct HandleInterModeArgs {
  /*!
   * Buffer for the above predictor in OBMC
   */
  uint8_t *above_pred_buf[MAX_MB_PLANE];
  /*!
   * Stride for the above predictor in OBMC
   */
  int above_pred_stride[MAX_MB_PLANE];
  /*!
   * Buffer for the left predictor in OBMC
   */
  uint8_t *left_pred_buf[MAX_MB_PLANE];
  /*!
   * Stride for the left predictor in OBMC
   */
  int left_pred_stride[MAX_MB_PLANE];
  /*!
   * Pointer to the first member in a 2D array which holds
   * single reference mode motion vectors to be used as a starting
   * point in the mv search for compound modes. Each array is length REF_FRAMES,
   * meaning there is a slot for a single reference motion vector for
   * each possible reference frame. The 2D array consists of N of these arrays,
   * where N is the length of the reference mv stack computed for the single
   * reference case for that particular reference frame.
   */
  int_mv (*single_newmv)[REF_FRAMES];
  /*!
   * Pointer to the first array of a 2D array with the same setup as
   * single_newmv array above. This is a 2D array to hold the rate
   * corresponding to each of the single reference mode motion vectors
   * held in single_newmv.
   */
  int (*single_newmv_rate)[REF_FRAMES];
  /*!
   * Pointer to the first array of a 2D array with the same setup as
   * single_newmv array above. This is a 2D array to hold a 0 or 1
   * validity value corresponding to each of the single reference mode motion
   * vectors held in single_newmv.
   */
  int (*single_newmv_valid)[REF_FRAMES];
  /*!
   * Pointer to the first array in a 3D array of predicted rate-distortion.
   * The dimensions of this structure are:
   * (number of possible inter modes) X
   * (number of reference MVs) X
   * (number of reference frames).
   */
  int64_t (*modelled_rd)[MAX_REF_MV_SEARCH][REF_FRAMES];
  /*!
   * Holds an estimated entropy cost for picking the current reference frame.
   * This is used to compute an rd estimate.
   */
  int ref_frame_cost;
  /*!
   * Holds an estimated entropy cost for picking single or compound
   * reference. This is used to compute an rd estimate.
   */
  int single_comp_cost;
  /*!
   * Pointer to the first element in a 3D array holding rd's of
   * SIMPLE_TRANSLATION used to prune out the motion mode search in single ref
   * modes used to determine compound ref modes. The full structure is:
   * (number of inter modes) X (length of refmv list) X (number of ref frames)
   */
  int64_t (*simple_rd)[MAX_REF_MV_SEARCH][REF_FRAMES];
  /*!
   * An integer value 0 or 1 which indicates whether or not to skip the motion
   * mode search and default to SIMPLE_TRANSLATION as a speed feature.
   */
  int skip_motion_mode;
  /*!
   * Initialized to false. If true, skips interpolation filter search and uses
   * the default EIGHTTAP_REGULAR.
   */
  bool skip_ifs;
  /*!
   * A pointer to the first element in an array of INTERINTRA_MODE types. This
   * contains the best inter_intra mode for each reference frame.
   */
  INTERINTRA_MODE *inter_intra_mode;
  /*!
   * Array of saved interpolation filter stats collected to avoid repeating
   * an interpolation filter search when the mv and ref_frame are the same
   * as a previous search.
   */
  INTERPOLATION_FILTER_STATS interp_filter_stats[MAX_INTERP_FILTER_STATS];

  /*!
   * Stack to store full pixel search start mv of NEWMV mode.
   */
  FULLPEL_MV start_mv_stack[(MAX_REF_MV_SEARCH - 1) * 2];

  /*!
   * Stack to store ref_mv_idx of NEWMV mode.
   */
  uint8_t ref_mv_idx_stack[(MAX_REF_MV_SEARCH - 1) * 2];

  /*!
   * Count of mvs in start mv stack.
   */
  int start_mv_cnt;

  /*!
   * Index of the last set of saved stats in the interp_filter_stats array.
   */
  int interp_filter_stats_idx;
  /*!
   * Estimated wedge index.
   */
  int wedge_index;
  /*!
   * Estimated wedge sign.
   */
  int wedge_sign;
  /*!
   * Estimated diff wtd index.
   */
  int diffwtd_index;
  /*!
   * Estimated cmp mode.
   */
  int cmp_mode[MODE_CTX_REF_FRAMES];
  /*!
   * The best sse during single new_mv search. Note that the sse here comes from
   * single_motion_search, and not from interpolation_filter_search. This has
   * two implications:
   * 1. The mv used to calculate the sse here does not have to be the best sse
   *    found in handle_inter_mode.
   * 2. Even if the mvs agree, the sse here can differ from the sse in \ref
   *    MACROBLOCK::pred_sse due to different interpolation filter used.
   */
  unsigned int best_single_sse_in_refs[REF_FRAMES];
  /*!
   * Holds the sse of best mode so far in the mode evaluation process. This is
   * used in intermediate termination of NEWMV mode evaluation.
   */
  unsigned int best_pred_sse;
} HandleInterModeArgs;

/*!\cond */
static const int_interpfilters filter_sets[DUAL_FILTER_SET_SIZE] = {
  { 0x00000000 }, { 0x00010000 }, { 0x00020000 },  // y = 0
  { 0x00000001 }, { 0x00010001 }, { 0x00020001 },  // y = 1
  { 0x00000002 }, { 0x00010002 }, { 0x00020002 },  // y = 2
};

int64_t av1_interpolation_filter_search(
    MACROBLOCK *const x, const AV1_COMP *const cpi,
    const TileDataEnc *tile_data, BLOCK_SIZE bsize,
    const BUFFER_SET *const tmp_dst, const BUFFER_SET *const orig_dst,
    int64_t *const rd, int *const switchable_rate, int *skip_build_pred,
    HandleInterModeArgs *args, int64_t ref_best_rd);

/*!\endcond */
#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_INTERP_FILTER_SEARCH_H_
