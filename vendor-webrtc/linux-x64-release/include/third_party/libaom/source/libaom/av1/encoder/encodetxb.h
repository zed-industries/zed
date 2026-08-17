/*
 * Copyright (c) 2017, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AV1_ENCODER_ENCODETXB_H_
#define AOM_AV1_ENCODER_ENCODETXB_H_

#include "config/aom_config.h"

#include "av1/common/av1_common_int.h"
#include "av1/common/blockd.h"
#include "av1/common/txb_common.h"
#include "av1/encoder/block.h"
#include "av1/encoder/encoder.h"
#include "aom_dsp/bitwriter.h"
#ifdef __cplusplus
extern "C" {
#endif

/*!\cond */
#define TXB_SKIP_CTX_MASK 15
#define DC_SIGN_CTX_SHIFT 4
#define DC_SIGN_CTX_MASK 3

int av1_get_eob_pos_token(const int eob, int *const extra);

/*!\endcond */
/*!\brief Allocate the memory resources for all the macro blocks in the current
 * coding frame.
 * \ingroup coefficient_coding
 *
 * Each macro block will need a \ref CB_COEFF_BUFFER to store information for
 * rate-distortion optimization and entropy coding of transform coefficients.
 *
 * \param[in]    cpi            Top-level encoder structure
 */
void av1_alloc_txb_buf(AV1_COMP *cpi);
/*!\brief Free the memory resources for all the macro blocks in the current
 * coding frame.
 * \ingroup coefficient_coding
 *
 * See \ref av1_alloc_txb_buf and \ref CB_COEFF_BUFFER for more details.
 *
 * \param[in]    cpi            Top-level encoder structure
 */
void av1_free_txb_buf(AV1_COMP *cpi);

/*!\brief Write quantized coefficients in a transform block into bitstream using
 * entropy coding.
 *
 * \ingroup coefficient_coding
 *
 * This function will write the quantized coefficients in a transform block into
 * the bitstream using entropy coding.
 *
 * The coding steps are as follows.
 *
 * 1) Code the end of block position "eob", which is the scan index of the
 * last non-zero coefficient plus one.
 *
 * 2) Code the lower magnitude level (<= COEFF_BASE_RANGE + NUM_BASE_LEVELS)
 * for each coefficient in reversed scan order.
 *
 * 3) Code the sign and higher magnitude level
 * (> COEFF_BASE_RANGE + NUM_BASE_LEVELS) in forward scan order.
 *
 * \param[in]    cm             Top-level structure shared by encoder and
 * decoder
 * \param[in]    x              Pointer to structure holding the data for the
                                current encoding macroblock
 * \param[in]    w              Entropy coding write pointer
 * \param[in]    blk_row      The row index of the current transform block
 * in the macroblock. Each unit has 4 pixels in y plane
 * \param[in]    blk_col      The col index of the current transform block
 * in the macroblock. Each unit has 4 pixels in y plane
 * \param[in]    plane          The index of the current plane
 * \param[in]    block          The index of the current transform block in the
 * macroblock. It's defined by number of 4x4 units that have been coded before
 * the currernt transform block
 * \param[in]    tx_size        The given transform size
 */
void av1_write_coeffs_txb(const AV1_COMMON *const cm, MACROBLOCK *const x,
                          aom_writer *w, int blk_row, int blk_col, int plane,
                          int block, TX_SIZE tx_size);

/*!\brief Write quantized coefficients of all transform blocks in an intra
 * macroblock into the bitstream using entropy coding.
 *
 * \ingroup coefficient_coding
 *
 * All transform blocks in the intra macroblock share the same transform size.
 *
 * This function use \ref av1_write_coeffs_txb() to code each transform block in
 * raster order.
 *
 * \param[in]    cm             Top-level structure shared by encoder and
 * decoder
 * \param[in]    x              Pointer to structure holding the data for the
                                current encoding macroblock
 * \param[in]    w              Entropy coding write pointer
 * \param[in]    bsize          Block size of the current macroblock
 */
void av1_write_intra_coeffs_mb(const AV1_COMMON *const cm, MACROBLOCK *x,
                               aom_writer *w, BLOCK_SIZE bsize);

/*!\brief Pack the context info of the current transform block into an uint8_t.
 * \ingroup coefficient_coding
 *
 * This context info will be collected and consolidated by its neighbor
 * transform blocks for coding transform block skip flag (tx_skip) and
 * the sign of DC coefficient (dc_sign).
 *
 * \param[in]    qcoeff         Buffer of quantized coefficients
 * \param[in]    scan_order     Coding order of coefficients in the transform
 * block
 * \param[in]    eob            The scan index of last non-zero coefficient plus
 * one
 */
uint8_t av1_get_txb_entropy_context(const tran_low_t *qcoeff,
                                    const SCAN_ORDER *scan_order, int eob);

/*!\brief Update the probability model (cdf) and the entropy context related to
 * coefficient coding for all transform blocks in the intra macroblock.
 *
 * \ingroup coefficient_coding
 *
 * This function will go through each transform block in the intra macorblock
 * and call \ref av1_update_and_record_txb_context to update the probability
 * model and entropy context properly.
 *
 * \param[in]    cpi               Top-level encoder structure
 * \param[in]    td                Top-level multithreading structure
 * \param[in]    dry_run           Whether this is a dry run.
 * \param[in]    bsize             Block size of the current macroblock
 * \param[in]    allow_update_cdf  Allowed to update probability model (cdf) or
 * not.
 */
void av1_update_intra_mb_txb_context(const AV1_COMP *cpi, ThreadData *td,
                                     RUN_TYPE dry_run, BLOCK_SIZE bsize,
                                     uint8_t allow_update_cdf);

/*!\brief Update the probability model (cdf) and the entropy context related to
 * coefficient coding for a transform block.
 *
 * \ingroup coefficient_coding
 *
 * There are regular mode and dry run for this funtion.
 *
 * Regular mode:
 *
 * The probability model (cdf) for each coding symbol in the
 * transform block will be updated.
 *
 * The entropy context of this transform block will be updated.
 *
 * Dry run:
 *
 * The probability model update will be skipped.
 *
 * The entropy context of this transform block will be updated.
 *
 * \param[in]    plane        The index of the current plane.
 * \param[in]    block        The index of the current transform block in the
 * macroblock. It's defined by number of 4x4 units that have been coded before
 * the currernt transform block.
 * \param[in]    blk_row      The row index of the current transform block
 * in the macroblock. Each unit has 4 pixels in y plane.
 * \param[in]    blk_col      The col index of the current transform block
 * in the macroblock. Each unit has 4 pixels in y plane.
 * \param[in]    plane_bsize  Block size for this plane. When the video source
 * uses chroma subsampling, the block size of UV planes will be smaller than the
 * block size of Y plane.
 * \param[in]    tx_size      The given transform size.
 * \param[in]    arg          This parameter will be translated into
 * tokenize_b_args, in which RUN_TYPE indicates using regular mode or dry run.
 */
void av1_update_and_record_txb_context(int plane, int block, int blk_row,
                                       int blk_col, BLOCK_SIZE plane_bsize,
                                       TX_SIZE tx_size, void *arg);

/*!\brief Update the entropy context related to coefficient coding for a
 * transform block.
 *
 * \ingroup coefficient_coding
 *
 * There are regular mode and dry run for this function.
 *
 * Regular mode:
 *
 * The entropy context of this transform block will be updated.
 *
 * Dry run:
 *
 * The probability model update will be skipped.
 *
 * The entropy context of this transform block will be updated.
 *
 * \param[in]    plane        The index of the current plane.
 * \param[in]    block        The index of the current transform block in the
 * macroblock. It's defined by number of 4x4 units that have been coded before
 * the currernt transform block.
 * \param[in]    blk_row      The row index of the current transform block
 * in the macroblock. Each unit has 4 pixels in y plane.
 * \param[in]    blk_col      The col index of the current transform block
 * in the macroblock. Each unit has 4 pixels in y plane.
 * \param[in]    plane_bsize  Block size for this plane. When the video source
 * uses chroma subsampling, the block size of UV planes will be smaller than the
 * block size of Y plane.
 * \param[in]    tx_size      The given transform size.
 * \param[in]    arg          This parameter will be translated into
 * tokenize_b_args, in which RUN_TYPE indicates using regular mode or dry run.
 */
void av1_record_txb_context(int plane, int block, int blk_row, int blk_col,
                            BLOCK_SIZE plane_bsize, TX_SIZE tx_size, void *arg);

/*!\brief Get the corresponding \ref CB_COEFF_BUFFER of the current macro block.
 *
 * \ingroup coefficient_coding
 *
 * The macroblock's location is described by mi_row and mi_col, row and column
 * mi indexes in the coding frame.
 *
 * Each mi unit is a 4x4 pixel block.
 *
 * \param[in]    cpi               Top-level encoder structure.
 * \param[in]    mi_row            Row mi index of the current transform block
 * in the frame.
 * \param[in]    mi_col           Column mi index of the current transform
 * block in the frame.
 * \return       CB_COEFF_BUFFER*  Pointer of \ref CB_COEFF_BUFFER associated
 * to this macroblock.
 */
CB_COEFF_BUFFER *av1_get_cb_coeff_buffer(const struct AV1_COMP *cpi, int mi_row,
                                         int mi_col);

/*!\brief Returns the entropy cost associated with skipping the current
 * transform block.
 *
 * \ingroup coefficient_coding
 *
 * \param[in]    coeff_costs    Table of entropy cost for coefficient coding.
 * \param[in]    txb_ctx        Context info for entropy coding transform block
 * skip flag (tx_skip) and the sign of DC coefficient (dc_sign).
 * \param[in]    plane          The index of the current plane
 * \param[in]    tx_size        The transform size
 */
static inline int av1_cost_skip_txb(const CoeffCosts *coeff_costs,
                                    const TXB_CTX *const txb_ctx, int plane,
                                    TX_SIZE tx_size) {
  const TX_SIZE txs_ctx = get_txsize_entropy_ctx(tx_size);
  const PLANE_TYPE plane_type = get_plane_type(plane);
  const LV_MAP_COEFF_COST *const coeff_costs_ =
      &coeff_costs->coeff_costs[txs_ctx][plane_type];
  return coeff_costs_->txb_skip_cost[txb_ctx->txb_skip_ctx][1];
}

/*!\cond */
// These numbers are empirically obtained.
static const int plane_rd_mult[REF_TYPES][PLANE_TYPES] = {
  { 17, 13 },
  { 16, 10 },
};
/*!\endcond */

#ifdef __cplusplus
}
#endif

#endif  // AOM_AV1_ENCODER_ENCODETXB_H_
