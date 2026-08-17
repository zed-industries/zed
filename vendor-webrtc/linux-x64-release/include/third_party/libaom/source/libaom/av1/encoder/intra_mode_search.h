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

/*!\file
 * \brief Declares high level functions to search through intra modes.
 */
#ifndef AOM_AV1_ENCODER_INTRA_MODE_SEARCH_H_
#define AOM_AV1_ENCODER_INTRA_MODE_SEARCH_H_

#include "av1/encoder/encoder.h"

#ifdef __cplusplus
extern "C" {
#endif

/*! \brief Variables related to intra-mode search during inter frame coding.
 *
 * \ingroup intra_mode_search
 * This is a set of variables used during intra-mode search for inter frames.
 * This includes an histogram of gradient speed features and a cache of uv
 * prediction to avoid repeated search of chroma prediction.
 */
typedef struct IntraModeSearchState {
  /*!
   * \brief The best luma intra-mode found so far
   */
  PREDICTION_MODE best_intra_mode;

  /** \name Speed feature variables
   * Variables to help with pruning some luma intra-modes during inter frame
   * coding process.
   */
  /**@{*/
  /*!
   * \brief Whether to terminate all intra mode search.
   */
  int skip_intra_modes;
  /*!
   * \brief Whether a directional mode is pruned.
   */
  uint8_t directional_mode_skip_mask[INTRA_MODES];
  /*!
   * \brief Whether \ref directional_mode_skip_mask is valid for pruning.
   */
  int dir_mode_skip_mask_ready;
  /**@}*/

  /** \name Chroma mode search cache
   * A cache of the best chroma prediction mode to avoid having to search for
   * chroma predictions repeatedly in \ref
   * av1_search_intra_uv_modes_in_interframe()
   */
  /**@{*/
  int rate_uv_intra;          /*!< \brief Total rate to transmit uv_mode */
  int rate_uv_tokenonly;      /*!< \brief Rate transmit txfm tokens */
  int64_t dist_uvs;           /*!< \brief Distortion of the uv_mode's recon */
  uint8_t skip_uvs;           /*!< \brief Whether the uv txfm is skippable */
  UV_PREDICTION_MODE mode_uv; /*!< \brief The best uv mode */
  PALETTE_MODE_INFO pmi_uv;   /*!< \brief Color map if mode_uv is palette */
  int8_t uv_angle_delta;      /*!< \brief Angle delta if mode_uv directional */
  /**@}*/
} IntraModeSearchState;

/*!\brief Evaluate a given luma intra-mode for inter frames.
 *
 * \ingroup intra_mode_search
 * \callgraph
 * \callergraph
 * This function handles an intra-mode luma prediction when the current frame
 * is an inter frame. This is the intra-mode counterpart of handle_inter_mode.
 * This function performs an intra luma prediction using the mode specified by
 * x->e_mbd.mi[0]->mode. This function does *not* support palette mode
 * prediction in the luma channel.
 *
 * \param[in,out]    intra_search_state Structure to intra search state.
 * \param[in]        cpi                Top-level encoder structure.
 * \param[in,out]    x                  Pointer to structure holding all the
 *                                      data for the current macroblock.
 * \param[in]        bsize              Current partition block size.
 * \param[in]        ref_frame_cost     The entropy cost for signaling that the
 *                                      current ref frame is an intra frame.
 * \param[in]        ctx                Structure to hold the number of 4x4 blks
 *                                      to copy tx_type and txfm_skip arrays.
 * \param[out]       rd_stats_y         Struct to keep track of the current
 *                                      intra-mode's rd_stats (luma only).
 * \param[in]        best_rd            Best RD seen for this block so far.
 * \param[out]       mode_cost_y        The cost needed to signal the current
 *                                      intra mode.
 * \param[out]       rd_y               The rdcost of the chosen mode.
 * \param[in]        best_model_rd      Best model RD seen for this block so far
 * \param[in]        top_intra_model_rd Top intra model RD seen for this
 *                                      block so far.
 *
 * \return Returns 1 if a valid intra mode is found, 0 otherwise.
 * The corresponding values in x->e_mbd.mi[0], rd_stats_y, mode_cost_y, and
 * rd_y are also updated. Moreover, in the first evaluation with directional
 * mode, a prune_mask computed with histogram of gradient is also stored in
 * intra_search_state.
 */
int av1_handle_intra_y_mode(IntraModeSearchState *intra_search_state,
                            const AV1_COMP *cpi, MACROBLOCK *x,
                            BLOCK_SIZE bsize, unsigned int ref_frame_cost,
                            const PICK_MODE_CONTEXT *ctx, RD_STATS *rd_stats_y,
                            int64_t best_rd, int *mode_cost_y, int64_t *rd_y,
                            int64_t *best_model_rd,
                            int64_t top_intra_model_rd[]);

/*!\brief Search through all chroma intra-modes for inter frames.
 *
 * \ingroup intra_mode_search
 * \callgraph
 * \callergraph
 * This function handles intra-mode chroma prediction when the current frame
 * is an inter frame. This is done by calling \ref av1_rd_pick_intra_sbuv_mode
 * with some additional book-keeping.
 *
 * \param[in,out]    intra_search_state Structure to intra search state.
 * \param[in]        cpi                Top-level encoder structure.
 * \param[in,out]    x                  Pointer to structure holding all the
 *                                      data for the current macroblock.
 * \param[in]        bsize              Current partition block size.
 * \param[out]       rd_stats           Struct to keep track of the current
 *                                      intra-mode's rd_stats (all planes).
 * \param[out]       rd_stats_y         Struct to keep track of the current
 *                                      intra-mode's rd_stats (luma only).
 * \param[out]       rd_stats_uv        Struct to keep track of the current
 *                                      intra-mode's rd_stats (chroma only).
 * \param[in]        best_rd            Best RD seen for this block so far.
 *
 * \return Returns 1 if a valid intra mode is found, 0 otherwise.
 * The corresponding values in x->e_mbd.mi[0], rd_stats(_y|_uv)  are also
 * updated. Moreover, in the first evocation of the function, the chroma intra
 * mode result is cached in intra_search_state to be used in subsequent calls.
 */
int av1_search_intra_uv_modes_in_interframe(
    IntraModeSearchState *intra_search_state, const AV1_COMP *cpi,
    MACROBLOCK *x, BLOCK_SIZE bsize, RD_STATS *rd_stats,
    const RD_STATS *rd_stats_y, RD_STATS *rd_stats_uv, int64_t best_rd);

/*!\brief Evaluate luma palette mode for inter frames.
 *
 * \ingroup intra_mode_search
 * \callergraph
 * \callgraph
 * This function handles luma palette mode when the current frame is an
 * inter frame.
 *
 * \param[in]    intra_search_state Structure to hold the best luma intra mode
 *                                  and cache chroma prediction for speed up.
 * \param[in]    cpi                Top-level encoder structure.
 * \param[in]    x                  Pointer to structure holding all the data
 *                                  for the current macroblock.
 * \param[in]    bsize              Current partition block size.
 * \param[in]    ref_frame_cost     The entropy cost for signaling that the
 *                                  current ref frame is an intra frame.
 * \param[in]    ctx                Structure to hold the number of 4x4 blks to
 *                                  copy the tx_type and txfm_skip arrays.
 * \param[in]    this_rd_cost       Struct to keep track of palette mode's
 *                                  rd_stats.
 * \param[in]    best_rd            Best RD seen for this block so far.
 *
 * \return Returns whether luma palette mode can skip the txfm. The
 * corresponding mbmi, this_rd_costs, intra_search_state, and tx_type arrays in
 * ctx are also updated.
 */
int av1_search_palette_mode(IntraModeSearchState *intra_search_state,
                            const AV1_COMP *cpi, MACROBLOCK *x,
                            BLOCK_SIZE bsize, unsigned int ref_frame_cost,
                            PICK_MODE_CONTEXT *ctx, RD_STATS *this_rd_cost,
                            int64_t best_rd);

/*!\brief Evaluate luma palette mode for inter frames.
 *
 * \ingroup intra_mode_search
 * \callergraph
 * \callgraph
 * This function handles luma palette mode when the current frame is an
 * inter frame.
 *
 * \param[in]    cpi                Top-level encoder structure.
 * \param[in]    x                  Pointer to structure holding all the data
 *                                  for the current macroblock.
 * \param[in]    bsize              Current partition block size.
 * \param[in]    ref_frame_cost     The entropy cost for signaling that the
 *                                  current ref frame is an intra frame.
 * \param[in]    ctx                Structure to hold the number of 4x4 blks to
 *                                  copy the tx_type and txfm_skip arrays.
 * \param[in]    this_rd_cost       Struct to keep track of palette mode's
 *                                  rd_stats.
 * \param[in]    best_rd            Best RD seen for this block so far.
 */
void av1_search_palette_mode_luma(const AV1_COMP *cpi, MACROBLOCK *x,
                                  BLOCK_SIZE bsize, unsigned int ref_frame_cost,
                                  PICK_MODE_CONTEXT *ctx,
                                  RD_STATS *this_rd_cost, int64_t best_rd);

/*!\brief Perform intra-mode search on luma channels for intra frames.
 *
 * \ingroup intra_mode_search
 * \callgraph
 * \callergraph
 * This function performs intra-mode search on the luma channel when the
 * current frame is intra-only. This function does not search intrabc mode,
 * but it does search palette and filter_intra.
 *
 * \param[in]    cpi                Top-level encoder structure.
 * \param[in]    x                  Pointer to structure holding all the data
 *                                  for the current macroblock.
 * \param[in]    rate               The total rate needed to predict the current
 *                                  chroma block.
 * \param[in]    rate_tokenonly     The rate without the cost of sending the
 *                                  prediction modes.
 *                                  chroma block.
 *                                  after the reconstruction.
 * \param[in]    distortion         The chroma distortion of the best prediction
 *                                  after the reconstruction.
 * \param[in]    skippable          Whether we can skip txfm process.
 * \param[in]    bsize              Current partition block size.
 * \param[in]    best_rd            Best RD seen for this block so far.
 * \param[in]    ctx                Structure to hold the number of 4x4 blks to
 *                                  copy the tx_type and txfm_skip arrays.
 *
 * \return Returns the rd_cost if this function finds a mode better than
 * best_rd, otherwise returns INT64_MAX. This also updates the mbmi, the rate
 * and distortion, and the tx_type arrays in ctx.
 */
int64_t av1_rd_pick_intra_sby_mode(const AV1_COMP *const cpi, MACROBLOCK *x,
                                   int *rate, int *rate_tokenonly,
                                   int64_t *distortion, uint8_t *skippable,
                                   BLOCK_SIZE bsize, int64_t best_rd,
                                   PICK_MODE_CONTEXT *ctx);

/*!\brief Perform intra-mode search on chroma channels.
 *
 * \ingroup intra_mode_search
 * \callergraph
 * \callgraph
 * This function performs intra-mode search on the chroma channels. Just like
 * \ref av1_rd_pick_intra_sby_mode(), this function searches over palette mode
 * (filter_intra is not available on chroma planes). Unlike \ref
 * av1_rd_pick_intra_sby_mode() this function is used by both inter and intra
 * frames.
 *
 * \param[in]    cpi                Top-level encoder structure.
 * \param[in]    x                  Pointer to structure holding all the data
 *                                  for the current macroblock.
 * \param[in]    rate               The total rate needed to predict the current
 *                                  chroma block.
 * \param[in]    rate_tokenonly     The rate without the cost of sending the
 *                                  prediction modes.
 *                                  chroma block.
 *                                  after the reconstruction.
 * \param[in]    distortion         The chroma distortion of the best prediction
 *                                  after the reconstruction.
 * \param[in]    skippable          Whether we can skip txfm process.
 * \param[in]    bsize              Current partition block size.
 * \param[in]    max_tx_size        The maximum tx_size available
 *
 * \return Returns the rd_cost of the best uv mode found. This also updates the
 * mbmi, the rate and distortion, distortion.
 */
int64_t av1_rd_pick_intra_sbuv_mode(const AV1_COMP *const cpi, MACROBLOCK *x,
                                    int *rate, int *rate_tokenonly,
                                    int64_t *distortion, uint8_t *skippable,
                                    BLOCK_SIZE bsize, TX_SIZE max_tx_size);

/*! \brief Return the number of colors in src. Used by palette mode.
 */
void av1_count_colors(const uint8_t *src, int stride, int rows, int cols,
                      int *val_count, int *num_colors);

/*! \brief See \ref av1_count_colors(), but for highbd.
 */
void av1_count_colors_highbd(const uint8_t *src8, int stride, int rows,
                             int cols, int bit_depth, int *val_count,
                             int *val_count_8bit, int *num_color_bins,
                             int *num_colors);

/*! \brief Initializes the \ref IntraModeSearchState struct.
 */
static inline void init_intra_mode_search_state(
    IntraModeSearchState *intra_search_state) {
  memset(intra_search_state, 0, sizeof(*intra_search_state));
  intra_search_state->rate_uv_intra = INT_MAX;
}

/*! \brief set the luma intra mode and delta angles for a given mode index.
 * The total number of luma intra mode is LUMA_MODE_COUNT = 61.
 * The first 13 modes are from DC_PRED to PAETH_PRED, followed by directional
 * modes. Each of the main 8 directional modes have 6 = MAX_ANGLE_DELTA * 2
 * delta angles.
 * \param[in]    mode_idx                  mode index in intra mode decision
 *                                         process.
 * \param[in]    mbmi                      Pointer to structure holding the mode
 *                                         info for the current macroblock.
 * \param[in]    reorder_delta_angle_eval  Indicates whether to reorder the
 *                                         evaluation of delta angle modes.
 */
void set_y_mode_and_delta_angle(const int mode_idx, MB_MODE_INFO *const mbmi,
                                int reorder_delta_angle_eval);
#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_INTRA_MODE_SEARCH_H_
