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

#ifndef AOM_AV1_ENCODER_MODEL_RD_H_
#define AOM_AV1_ENCODER_MODEL_RD_H_

#include "aom/aom_integer.h"
#include "av1/encoder/block.h"
#include "av1/encoder/encoder.h"
#include "av1/encoder/pustats.h"
#include "av1/encoder/rdopt_utils.h"
#include "config/aom_dsp_rtcd.h"

#ifdef __cplusplus
extern "C" {
#endif

// 0: Legacy model
// 1: Curve fit model
// 2: Surface fit model
// 3: DNN regression model
// 4: Full rd model
#define MODELRD_TYPE_INTERP_FILTER 1
#define MODELRD_TYPE_TX_SEARCH_PRUNE 1
#define MODELRD_TYPE_MASKED_COMPOUND 1
#define MODELRD_TYPE_INTERINTRA 1
#define MODELRD_TYPE_INTRA 1
#define MODELRD_TYPE_MOTION_MODE_RD 1

typedef void (*model_rd_for_sb_type)(
    const AV1_COMP *const cpi, BLOCK_SIZE bsize, MACROBLOCK *x, MACROBLOCKD *xd,
    int plane_from, int plane_to, int *out_rate_sum, int64_t *out_dist_sum,
    uint8_t *skip_txfm_sb, int64_t *skip_sse_sb, int *plane_rate,
    int64_t *plane_sse, int64_t *plane_dist);
typedef void (*model_rd_from_sse_type)(const AV1_COMP *const cpi,
                                       const MACROBLOCK *const x,
                                       BLOCK_SIZE plane_bsize, int plane,
                                       int64_t sse, int num_samples, int *rate,
                                       int64_t *dist);

static int64_t calculate_sse(MACROBLOCKD *const xd,
                             const struct macroblock_plane *p,
                             struct macroblockd_plane *pd, const int bw,
                             const int bh) {
  int64_t sse = 0;
  const int shift = xd->bd - 8;
#if CONFIG_AV1_HIGHBITDEPTH
  if (is_cur_buf_hbd(xd)) {
    sse = aom_highbd_sse(p->src.buf, p->src.stride, pd->dst.buf, pd->dst.stride,
                         bw, bh);
  } else {
    sse =
        aom_sse(p->src.buf, p->src.stride, pd->dst.buf, pd->dst.stride, bw, bh);
  }
#else
  sse = aom_sse(p->src.buf, p->src.stride, pd->dst.buf, pd->dst.stride, bw, bh);
#endif
  sse = ROUND_POWER_OF_TWO(sse, shift * 2);
  return sse;
}

static inline int64_t compute_sse_plane(MACROBLOCK *x, MACROBLOCKD *xd,
                                        int plane, const BLOCK_SIZE bsize) {
  struct macroblockd_plane *const pd = &xd->plane[plane];
  const BLOCK_SIZE plane_bsize =
      get_plane_block_size(bsize, pd->subsampling_x, pd->subsampling_y);
  int bw, bh;
  const struct macroblock_plane *const p = &x->plane[plane];
  get_txb_dimensions(xd, plane, plane_bsize, 0, 0, plane_bsize, NULL, NULL, &bw,
                     &bh);

  int64_t sse = calculate_sse(xd, p, pd, bw, bh);

  return sse;
}

static inline void model_rd_from_sse(const AV1_COMP *const cpi,
                                     const MACROBLOCK *const x,
                                     BLOCK_SIZE plane_bsize, int plane,
                                     int64_t sse, int num_samples, int *rate,
                                     int64_t *dist) {
  (void)num_samples;
  const MACROBLOCKD *const xd = &x->e_mbd;
  const struct macroblock_plane *const p = &x->plane[plane];
  const int dequant_shift = (is_cur_buf_hbd(xd)) ? xd->bd - 5 : 3;

  // Fast approximate the modelling function.
  if (cpi->sf.rd_sf.simple_model_rd_from_var) {
    const int64_t square_error = sse;
    int quantizer = p->dequant_QTX[1] >> dequant_shift;
    if (quantizer < 120)
      *rate = (int)AOMMIN(
          (square_error * (280 - quantizer)) >> (16 - AV1_PROB_COST_SHIFT),
          INT_MAX);
    else
      *rate = 0;
    assert(*rate >= 0);
    *dist = (square_error * quantizer) >> 8;
  } else {
    av1_model_rd_from_var_lapndz(sse, num_pels_log2_lookup[plane_bsize],
                                 p->dequant_QTX[1] >> dequant_shift, rate,
                                 dist);
  }
  *dist <<= 4;
}

// Fits a curve for rate and distortion using as feature:
// log2(sse_norm/qstep^2)
static inline void model_rd_with_curvfit(const AV1_COMP *const cpi,
                                         const MACROBLOCK *const x,
                                         BLOCK_SIZE plane_bsize, int plane,
                                         int64_t sse, int num_samples,
                                         int *rate, int64_t *dist) {
  (void)cpi;
  (void)plane_bsize;
  const MACROBLOCKD *const xd = &x->e_mbd;
  const struct macroblock_plane *const p = &x->plane[plane];
  const int dequant_shift = (is_cur_buf_hbd(xd)) ? xd->bd - 5 : 3;
  const int qstep = AOMMAX(p->dequant_QTX[1] >> dequant_shift, 1);

  if (sse == 0) {
    if (rate) *rate = 0;
    if (dist) *dist = 0;
    return;
  }
  const double sse_norm = (double)sse / num_samples;
  const double qstepsqr = (double)qstep * qstep;
  const double xqr = log2(sse_norm / qstepsqr);
  double rate_f, dist_by_sse_norm_f;
  av1_model_rd_curvfit(plane_bsize, sse_norm, xqr, &rate_f,
                       &dist_by_sse_norm_f);

  const double dist_f = dist_by_sse_norm_f * sse_norm;
  int rate_i = (int)(AOMMAX(0.0, rate_f * num_samples) + 0.5);
  int64_t dist_i = (int64_t)(AOMMAX(0.0, dist_f * num_samples) + 0.5);

  // Check if skip is better
  if (rate_i == 0) {
    dist_i = sse << 4;
  } else if (RDCOST(x->rdmult, rate_i, dist_i) >=
             RDCOST(x->rdmult, 0, sse << 4)) {
    rate_i = 0;
    dist_i = sse << 4;
  }

  if (rate) *rate = rate_i;
  if (dist) *dist = dist_i;
}

static inline void model_rd_for_sb(const AV1_COMP *const cpi, BLOCK_SIZE bsize,
                                   MACROBLOCK *x, MACROBLOCKD *xd,
                                   int plane_from, int plane_to,
                                   int *out_rate_sum, int64_t *out_dist_sum,
                                   uint8_t *skip_txfm_sb, int64_t *skip_sse_sb,
                                   int *plane_rate, int64_t *plane_sse,
                                   int64_t *plane_dist) {
  // Note our transform coeffs are 8 times an orthogonal transform.
  // Hence quantizer step is also 8 times. To get effective quantizer
  // we need to divide by 8 before sending to modeling function.
  int plane;
  const int ref = xd->mi[0]->ref_frame[0];

  int64_t rate_sum = 0;
  int64_t dist_sum = 0;
  int64_t total_sse = 0;

  assert(bsize < BLOCK_SIZES_ALL);

  for (plane = plane_from; plane <= plane_to; ++plane) {
    if (plane && !xd->is_chroma_ref) break;
    struct macroblock_plane *const p = &x->plane[plane];
    struct macroblockd_plane *const pd = &xd->plane[plane];
    const BLOCK_SIZE plane_bsize =
        get_plane_block_size(bsize, pd->subsampling_x, pd->subsampling_y);
    assert(plane_bsize < BLOCK_SIZES_ALL);
    const int bw = block_size_wide[plane_bsize];
    const int bh = block_size_high[plane_bsize];
    int64_t sse;
    int rate;
    int64_t dist;

    sse = calculate_sse(xd, p, pd, bw, bh);

    model_rd_from_sse(cpi, x, plane_bsize, plane, sse, bw * bh, &rate, &dist);

    if (plane == 0) x->pred_sse[ref] = (unsigned int)AOMMIN(sse, UINT_MAX);

    total_sse += sse;
    rate_sum += rate;
    dist_sum += dist;
    if (plane_rate) plane_rate[plane] = rate;
    if (plane_sse) plane_sse[plane] = sse;
    if (plane_dist) plane_dist[plane] = dist;
    assert(rate_sum >= 0);
  }

  if (skip_txfm_sb) *skip_txfm_sb = total_sse == 0;
  if (skip_sse_sb) *skip_sse_sb = total_sse << 4;
  rate_sum = AOMMIN(rate_sum, INT_MAX);
  *out_rate_sum = (int)rate_sum;
  *out_dist_sum = dist_sum;
}

static inline void model_rd_for_sb_with_curvfit(
    const AV1_COMP *const cpi, BLOCK_SIZE bsize, MACROBLOCK *x, MACROBLOCKD *xd,
    int plane_from, int plane_to, int *out_rate_sum, int64_t *out_dist_sum,
    uint8_t *skip_txfm_sb, int64_t *skip_sse_sb, int *plane_rate,
    int64_t *plane_sse, int64_t *plane_dist) {
  // Note our transform coeffs are 8 times an orthogonal transform.
  // Hence quantizer step is also 8 times. To get effective quantizer
  // we need to divide by 8 before sending to modeling function.
  const int ref = xd->mi[0]->ref_frame[0];

  int64_t rate_sum = 0;
  int64_t dist_sum = 0;
  int64_t total_sse = 0;

  for (int plane = plane_from; plane <= plane_to; ++plane) {
    if (plane && !xd->is_chroma_ref) break;
    struct macroblockd_plane *const pd = &xd->plane[plane];
    const BLOCK_SIZE plane_bsize =
        get_plane_block_size(bsize, pd->subsampling_x, pd->subsampling_y);
    int64_t dist, sse;
    int rate;
    int bw, bh;
    const struct macroblock_plane *const p = &x->plane[plane];
    get_txb_dimensions(xd, plane, plane_bsize, 0, 0, plane_bsize, NULL, NULL,
                       &bw, &bh);

    sse = calculate_sse(xd, p, pd, bw, bh);
    model_rd_with_curvfit(cpi, x, plane_bsize, plane, sse, bw * bh, &rate,
                          &dist);

    if (plane == 0) x->pred_sse[ref] = (unsigned int)AOMMIN(sse, UINT_MAX);

    total_sse += sse;
    rate_sum += rate;
    dist_sum += dist;

    if (plane_rate) plane_rate[plane] = rate;
    if (plane_sse) plane_sse[plane] = sse;
    if (plane_dist) plane_dist[plane] = dist;
  }

  if (skip_txfm_sb) *skip_txfm_sb = rate_sum == 0;
  if (skip_sse_sb) *skip_sse_sb = total_sse << 4;
  *out_rate_sum = (int)rate_sum;
  *out_dist_sum = dist_sum;
}

enum { MODELRD_LEGACY, MODELRD_CURVFIT, MODELRD_TYPES } UENUM1BYTE(ModelRdType);

static const model_rd_for_sb_type model_rd_sb_fn[MODELRD_TYPES] = {
  model_rd_for_sb, model_rd_for_sb_with_curvfit
};

static const model_rd_from_sse_type model_rd_sse_fn[MODELRD_TYPES] = {
  model_rd_from_sse, model_rd_with_curvfit
};

#ifdef __cplusplus
}  // extern "C"
#endif
#endif  // AOM_AV1_ENCODER_MODEL_RD_H_
