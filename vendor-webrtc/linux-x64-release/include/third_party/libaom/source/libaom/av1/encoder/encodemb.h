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

#ifndef AOM_AV1_ENCODER_ENCODEMB_H_
#define AOM_AV1_ENCODER_ENCODEMB_H_

#include "config/aom_config.h"

#include "av1/common/av1_common_int.h"
#include "av1/common/txb_common.h"
#include "av1/encoder/av1_quantize.h"
#include "av1/encoder/block.h"
#include "av1/encoder/tokenize.h"
#ifdef __cplusplus
extern "C" {
#endif

enum {
  AV1_XFORM_QUANT_FP = 0,
  AV1_XFORM_QUANT_B = 1,
  AV1_XFORM_QUANT_DC = 2,
  AV1_XFORM_QUANT_SKIP_QUANT,
  AV1_XFORM_QUANT_TYPES,
} UENUM1BYTE(AV1_XFORM_QUANT);

// TODO(any): Merge OPT_TYPe and TRELLLIS_OPT_TYPE
// Available optimization types to optimize the quantized coefficients.
enum {
  NONE_OPT = 0,            // No optimization.
  TRELLIS_OPT = 1,         // Trellis optimization. See `av1_optimize_b()`.
  DROPOUT_OPT = 2,         // Dropout optimization. See `av1_dropout_qcoeff()`.
  TRELLIS_DROPOUT_OPT = 3  // Perform dropout after trellis optimization.
} UENUM1BYTE(OPT_TYPE);

enum {
  NO_TRELLIS_OPT,          // No trellis optimization
  FULL_TRELLIS_OPT,        // Trellis optimization in all stages
  FINAL_PASS_TRELLIS_OPT,  // Trellis optimization in only the final encode pass
  NO_ESTIMATE_YRD_TRELLIS_OPT  // Disable trellis in estimate_yrd_for_sb
} UENUM1BYTE(TRELLIS_OPT_TYPE);

struct optimize_ctx {
  ENTROPY_CONTEXT ta[MAX_MB_PLANE][MAX_MIB_SIZE];
  ENTROPY_CONTEXT tl[MAX_MB_PLANE][MAX_MIB_SIZE];
};

struct encode_b_args {
  const struct AV1_COMP *cpi;
  MACROBLOCK *x;
  struct optimize_ctx *ctx;
  ENTROPY_CONTEXT *ta;
  ENTROPY_CONTEXT *tl;
  RUN_TYPE dry_run;
  TRELLIS_OPT_TYPE enable_optimize_b;
};

void av1_encode_sb(const struct AV1_COMP *cpi, MACROBLOCK *x, BLOCK_SIZE bsize,
                   RUN_TYPE dry_run);

void av1_foreach_transformed_block_in_plane(
    const MACROBLOCKD *const xd, BLOCK_SIZE plane_bsize, int plane,
    foreach_transformed_block_visitor visit, void *arg);

void av1_encode_sby_pass1(struct AV1_COMP *cpi, MACROBLOCK *x,
                          BLOCK_SIZE bsize);

void av1_setup_xform(const AV1_COMMON *cm, MACROBLOCK *x, TX_SIZE tx_size,
                     TX_TYPE tx_type, TxfmParam *txfm_param);
void av1_setup_quant(TX_SIZE tx_size, int use_optimize_b, int xform_quant_idx,
                     int use_quant_b_adapt, QUANT_PARAM *qparam);
void av1_setup_qmatrix(const CommonQuantParams *quant_params,
                       const MACROBLOCKD *xd, int plane, TX_SIZE tx_size,
                       TX_TYPE tx_type, QUANT_PARAM *qparam);

void av1_xform_dc_only(MACROBLOCK *x, int plane, int block,
                       TxfmParam *txfm_param, int64_t per_px_mean);

void av1_xform_quant(MACROBLOCK *x, int plane, int block, int blk_row,
                     int blk_col, BLOCK_SIZE plane_bsize, TxfmParam *txfm_param,
                     const QUANT_PARAM *qparam);

void av1_xform(MACROBLOCK *x, int plane, int block, int blk_row, int blk_col,
               BLOCK_SIZE plane_bsize, TxfmParam *txfm_param);

void av1_quant(MACROBLOCK *x, int plane, int block, TxfmParam *txfm_param,
               const QUANT_PARAM *qparam);

int av1_optimize_b(const struct AV1_COMP *cpi, MACROBLOCK *mb, int plane,
                   int block, TX_SIZE tx_size, TX_TYPE tx_type,
                   const TXB_CTX *const txb_ctx, int *rate_cost);

// This function can be used as (i) a further optimization to reduce the
// redundancy of quantized coefficients (a.k.a., `qcoeff`) after trellis
// optimization, or (ii) an alternative to trellis optimization in high-speed
// compression mode (e.g., real-time mode under speed-6) due to its LOW time
// complexity. The rational behind is to drop out the may-be redundant quantized
// coefficient which is among a bunch of zeros. NOTE: This algorithm is not as
// accurate as trellis optimization since the hyper-parameters are hard-coded
// instead of dynamic search. More adaptive logic may improve the performance.
// This function should be applied to all or partical block cells.
// Inputs:
//   mb: Pointer to the MACROBLOCK to perform dropout on.
//   plane: Index of the plane to which the target block belongs.
//   block: Index of the target block.
//   tx_size: Transform size of the target block.
//   tx_type: Transform type of the target block. This field is particularly
//            used to find out the scan order of the block.
//   qindex: Quantization index used for target block. In general, all blocks
//           in a same plane share the same quantization index. This field is
//           particularly used to determine how many zeros should be used to
//           drop out a coefficient.
// Returns:
//   Nothing will be returned, but `qcoeff`, `dqcoeff`, `eob`, as well as
//   `txb_entropy_ctx`, which `mb` points to, may be modified by this function.
void av1_dropout_qcoeff(MACROBLOCK *mb, int plane, int block, TX_SIZE tx_size,
                        TX_TYPE tx_type, int qindex);
// Same as above, with the number of zeroes needed before/after a coeff to drop
// it explicitly passed in, instead of being derived from qindex.
void av1_dropout_qcoeff_num(MACROBLOCK *mb, int plane, int block,
                            TX_SIZE tx_size, TX_TYPE tx_type,
                            int dropout_num_before, int dropout_num_after);

void av1_subtract_block(BitDepthInfo bd_info, int rows, int cols, int16_t *diff,
                        ptrdiff_t diff_stride, const uint8_t *src8,
                        ptrdiff_t src_stride, const uint8_t *pred8,
                        ptrdiff_t pred_stride);

void av1_subtract_txb(MACROBLOCK *x, int plane, BLOCK_SIZE plane_bsize,
                      int blk_col, int blk_row, TX_SIZE tx_size);

void av1_subtract_plane(MACROBLOCK *x, BLOCK_SIZE plane_bsize, int plane);

static inline void av1_set_txb_context(MACROBLOCK *x, int plane, int block,
                                       TX_SIZE tx_size, ENTROPY_CONTEXT *a,
                                       ENTROPY_CONTEXT *l) {
  const uint8_t ctx = x->plane[plane].txb_entropy_ctx[block];
  memset(a, ctx, tx_size_wide_unit[tx_size] * sizeof(*a));
  memset(l, ctx, tx_size_high_unit[tx_size] * sizeof(*l));
}

void av1_encode_intra_block_plane(const struct AV1_COMP *cpi, MACROBLOCK *x,
                                  BLOCK_SIZE bsize, int plane, RUN_TYPE dry_run,
                                  TRELLIS_OPT_TYPE enable_optimize_b);

static inline int is_trellis_used(TRELLIS_OPT_TYPE optimize_b,
                                  RUN_TYPE dry_run) {
  if (optimize_b == NO_TRELLIS_OPT) return false;
  if (optimize_b == FINAL_PASS_TRELLIS_OPT && dry_run != OUTPUT_ENABLED)
    return false;
  return true;
}

// Scaling terms (precision of 12 bits) to perform tx-size specific
// normalization that is used in DCT_DCT forward transform.
// For transform blocks of 1:2 and 2:1       - sqrt(2) normalization is used
// For transform blocks of 1:4 and 4:1       - factor of 2 is used
// For transform blocks TX_8x8 and below     - an additional factor of 2 is used
// For transform blocks max(width,height)=64 - currently not supported

static const uint16_t dc_coeff_scale[TX_SIZES_ALL] = {
  1024, 2048, 4096, 4096, 0,    1448, 1448, 2896, 2896, 2896,
  2896, 0,    0,    2048, 2048, 4096, 4096, 0,    0
};

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_ENCODEMB_H_
