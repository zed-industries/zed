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

#ifndef AOM_AV1_ENCODER_TRANSFORM_SEARCH_H_
#define AOM_AV1_ENCODER_TRANSFORM_SEARCH_H_

#include "av1/common/pred_common.h"
#include "av1/encoder/encoder.h"

#ifdef __cplusplus
extern "C" {
#endif

// Set this macro as 1 to collect data about tx size selection.
#define COLLECT_TX_SIZE_DATA 0

#if COLLECT_TX_SIZE_DATA
static const char av1_tx_size_data_output_file[] = "tx_size_data.txt";
#endif

enum {
  FTXS_NONE = 0,
  FTXS_DCT_AND_1D_DCT_ONLY = 1 << 0,
  FTXS_DISABLE_TRELLIS_OPT = 1 << 1,
  FTXS_USE_TRANSFORM_DOMAIN = 1 << 2
} UENUM1BYTE(FAST_TX_SEARCH_MODE);

static inline int tx_size_cost(const MACROBLOCK *const x, BLOCK_SIZE bsize,
                               TX_SIZE tx_size) {
  assert(bsize == x->e_mbd.mi[0]->bsize);
  if (x->txfm_search_params.tx_mode_search_type != TX_MODE_SELECT ||
      !block_signals_txsize(bsize))
    return 0;

  const int32_t tx_size_cat = bsize_to_tx_size_cat(bsize);
  const int depth = tx_size_to_depth(tx_size, bsize);
  const MACROBLOCKD *const xd = &x->e_mbd;
  const int tx_size_ctx = get_tx_size_context(xd);
  return x->mode_costs.tx_size_cost[tx_size_cat][tx_size_ctx][depth];
}

/*!\brief Compute the pixel domain distortion.
 *
 * \ingroup transform_search
 * Compute the pixel domain distortion from diff on all visible 4x4s in the
 * transform block.
 *
 * \param[in]    x              Pointer to structure holding the data for the
                                current encoding macroblock
 * \param[in]    plane          Plane index
 * \param[in]    blk_row        Block row index
 * \param[in]    blk_col        Block col index
 * \param[in]    plane_bsize    Current plane block size
 * \param[in]    tx_bsize       Transform size
 * \param[in]    block_mse_q8   Block mse
 * \return       An int64_t value that is the block sse.
 */
int64_t av1_pixel_diff_dist(const MACROBLOCK *x, int plane, int blk_row,
                            int blk_col, const BLOCK_SIZE plane_bsize,
                            const BLOCK_SIZE tx_bsize,
                            unsigned int *block_mse_q8);

int64_t av1_estimate_txfm_yrd(const AV1_COMP *const cpi, MACROBLOCK *x,
                              RD_STATS *rd_stats, int64_t ref_best_rd,
                              BLOCK_SIZE bs, TX_SIZE tx_size);

/*!\brief Recursive transform size and type search.
 *
 * \ingroup transform_search
 * Search for best transform size and type for luma inter blocks. The transform
 * block partitioning can be recursive resulting in non-uniform transform sizes.
 * The best transform size and type, if found, will be saved in the MB_MODE_INFO
 * structure, and the corresponding RD stats will be saved in rd_stats.
 *
 * \param[in]    cpi            Top-level encoder structure
 * \param[in]    x              Pointer to structure holding the data for the
                                current encoding macroblock
 * \param[in]    rd_stats       Pointer to struct to keep track of the RD stats
 * \param[in]    bsize          Current macroblock size
 * \param[in]    ref_best_rd    Best RD cost seen for this block so far
 * \remark       Nothing is returned. The selected transform size and type will
                 be saved in the MB_MODE_INFO structure
 */
void av1_pick_recursive_tx_size_type_yrd(const AV1_COMP *cpi, MACROBLOCK *x,
                                         RD_STATS *rd_stats, BLOCK_SIZE bsize,
                                         int64_t ref_best_rd);

/*!\brief Uniform transform size and type search.
 *
 * \ingroup transform_search
 * Search for the best transform size and type for current macroblock block,
 * with the assumption that all the transform blocks have a uniform size
 * (VP9 style). The selected transform size and type will be saved in the
 * MB_MODE_INFO structure; the corresponding RD stats will be saved in rd_stats.
 * This function may be used for both intra and inter predicted blocks.
 *
 * \param[in]    cpi            Top-level encoder structure
 * \param[in]    x              Pointer to structure holding the data for the
                                current encoding macroblock
 * \param[in]    rd_stats       Pointer to struct to keep track of the RD stats
 * \param[in]    bs             Current macroblock size
 * \param[in]    ref_best_rd    Best RD cost seen for this block so far
 * \remark       Nothing is returned. The selected transform size and type will
                 be saved in the MB_MODE_INFO structure
 */
void av1_pick_uniform_tx_size_type_yrd(const AV1_COMP *const cpi, MACROBLOCK *x,
                                       RD_STATS *rd_stats, BLOCK_SIZE bs,
                                       int64_t ref_best_rd);

/*!\brief Chroma block transform search.
 *
 * \ingroup transform_search
 * Calculate the transform coefficient RD cost for the given chroma macroblock
 * If the current mode is intra, then this function will compute the predictor.
 *
 * \param[in]    cpi            Top-level encoder structure
 * \param[in]    x              Pointer to structure holding the data for the
                                current encoding macroblock
 * \param[in]    rd_stats       Pointer to struct to keep track of the RD stats
 * \param[in]    bsize          Current macroblock size
 * \param[in]    ref_best_rd    Best RD cost seen for this block so far
 * \return       An integer value is returned. 0: early termination triggered,
                 no valid rd cost available; 1: rd cost values are valid.
 */
int av1_txfm_uvrd(const AV1_COMP *const cpi, MACROBLOCK *x, RD_STATS *rd_stats,
                  BLOCK_SIZE bsize, int64_t ref_best_rd);

/*!\brief Transform type search with fixed transform size.
 *
 * \ingroup transform_search
 * Search for the best transform type and calculate the transform coefficients
 * RD cost of the current transform block with the specified (uniform) transform
 * size and plane. The RD results will be saved in rd_stats.
 *
 * \param[in]    x              Pointer to structure holding the data for the
                                current encoding macroblock
 * \param[in]    cpi            Top-level encoder structure
 * \param[in]    rd_stats       Pointer to struct to keep track of the RD stats
 * \param[in]    ref_best_rd    Best RD cost seen for this block so far
 * \param[in]    current_rd     Current RD cost for this block so far
 * \param[in]    plane          Plane index
 * \param[in]    plane_bsize    Size of the current macroblock considering
                                sup-sampling
 * \param[in]    tx_size        The given transform size
 * \param[in]    ftxs_mode      Transform search mode specifying desired speed
                                and quality tradeoff
 * \param[in]    skip_trellis   Binary flag indicating if trellis optimization
                                should be skipped
 *
 * \remark       Nothing is returned. The RD results will be saved in rd_stats.
 */
void av1_txfm_rd_in_plane(MACROBLOCK *x, const AV1_COMP *cpi,
                          RD_STATS *rd_stats, int64_t ref_best_rd,
                          int64_t current_rd, int plane, BLOCK_SIZE plane_bsize,
                          TX_SIZE tx_size, FAST_TX_SEARCH_MODE ftxs_mode,
                          int skip_trellis);

/*!\brief Recursive transform size and type search.
 *
 * \ingroup transform_search
 * This function combines y and uv planes' transform search processes together
 * for inter-predicted blocks (including IntraBC), when the prediction is
 * already generated. It first does subtraction to obtain the prediction error.
 * Then it calls
 * av1_pick_recursive_tx_size_type_yrd/av1_pick_uniform_tx_size_type_yrd and
 * av1_txfm_uvrd sequentially and handles possible early terminations.
 * The RD metrics are calculated and stored in rd_stats/_y/_uv.
 *
 * \param[in]    cpi            Top-level encoder structure
 * \param[in]    x              Pointer to structure holding the data for the
                                current encoding macroblock
 * \param[in]    bsize          Current macroblock size
 * \param[in]    rd_stats       Pointer to struct to keep track of the overal RD
                                stats
 * \param[in]    rd_stats_y     Pointer to struct to keep track of the RD
                                stats for the luma plane
 * \param[in]    rd_stats_uv    Pointer to struct to keep track of the RD
                                stats for the chroma planes
 * \param[in]    mode_rate      Rate cost to encode the prediction mode info. of
                                the current macroblock
 * \param[in]    ref_best_rd    Best RD cost seen for this block so far
 *
 * \return       An integer value is returned indicating if a valid transform
                 candidate is found (1) or not (0).
 */
int av1_txfm_search(const AV1_COMP *cpi, MACROBLOCK *x, BLOCK_SIZE bsize,
                    RD_STATS *rd_stats, RD_STATS *rd_stats_y,
                    RD_STATS *rd_stats_uv, int mode_rate, int64_t ref_best_rd);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_TRANSFORM_SEARCH_H_
