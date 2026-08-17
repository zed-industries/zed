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

#ifndef AOM_AV1_ENCODER_AV1_QUANTIZE_H_
#define AOM_AV1_ENCODER_AV1_QUANTIZE_H_

#include <stdbool.h>

#include "config/aom_config.h"

#include "aom/aomcx.h"
#include "av1/common/quant_common.h"
#include "av1/common/scan.h"
#include "av1/encoder/block.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct QUANT_PARAM {
  int log_scale;
  TX_SIZE tx_size;
  const qm_val_t *qmatrix;
  const qm_val_t *iqmatrix;
  int use_quant_b_adapt;
  int use_optimize_b;
  int xform_quant_idx;
} QUANT_PARAM;

typedef void (*AV1_QUANT_FACADE)(const tran_low_t *coeff_ptr, intptr_t n_coeffs,
                                 const MACROBLOCK_PLANE *p,
                                 tran_low_t *qcoeff_ptr,
                                 tran_low_t *dqcoeff_ptr, uint16_t *eob_ptr,
                                 const SCAN_ORDER *sc,
                                 const QUANT_PARAM *qparam);

// The QUANTS structure is used only for internal quantizer setup in
// av1_quantize.c.
// All of its fields use the same coefficient shift/scaling at TX.
typedef struct {
  // 0: dc 1: ac 2-8: ac repeated to SIMD width
  DECLARE_ALIGNED(16, int16_t, y_quant[QINDEX_RANGE][8]);
  DECLARE_ALIGNED(16, int16_t, y_quant_shift[QINDEX_RANGE][8]);
  DECLARE_ALIGNED(16, int16_t, y_zbin[QINDEX_RANGE][8]);
  DECLARE_ALIGNED(16, int16_t, y_round[QINDEX_RANGE][8]);

  // TODO(jingning): in progress of re-working the quantization. will decide
  // if we want to deprecate the current use of y_quant.
  DECLARE_ALIGNED(16, int16_t, y_quant_fp[QINDEX_RANGE][8]);
  DECLARE_ALIGNED(16, int16_t, u_quant_fp[QINDEX_RANGE][8]);
  DECLARE_ALIGNED(16, int16_t, v_quant_fp[QINDEX_RANGE][8]);
  DECLARE_ALIGNED(16, int16_t, y_round_fp[QINDEX_RANGE][8]);
  DECLARE_ALIGNED(16, int16_t, u_round_fp[QINDEX_RANGE][8]);
  DECLARE_ALIGNED(16, int16_t, v_round_fp[QINDEX_RANGE][8]);

  DECLARE_ALIGNED(16, int16_t, u_quant[QINDEX_RANGE][8]);
  DECLARE_ALIGNED(16, int16_t, v_quant[QINDEX_RANGE][8]);
  DECLARE_ALIGNED(16, int16_t, u_quant_shift[QINDEX_RANGE][8]);
  DECLARE_ALIGNED(16, int16_t, v_quant_shift[QINDEX_RANGE][8]);
  DECLARE_ALIGNED(16, int16_t, u_zbin[QINDEX_RANGE][8]);
  DECLARE_ALIGNED(16, int16_t, v_zbin[QINDEX_RANGE][8]);
  DECLARE_ALIGNED(16, int16_t, u_round[QINDEX_RANGE][8]);
  DECLARE_ALIGNED(16, int16_t, v_round[QINDEX_RANGE][8]);
} QUANTS;

// The Dequants structure is used only for internal quantizer setup in
// av1_quantize.c.
// Fields are suffixed according to whether or not they're expressed in
// the same coefficient shift/precision as TX or a fixed Q3 format.
typedef struct {
  DECLARE_ALIGNED(16, int16_t,
                  y_dequant_QTX[QINDEX_RANGE][8]);  // 8: SIMD width
  DECLARE_ALIGNED(16, int16_t,
                  u_dequant_QTX[QINDEX_RANGE][8]);  // 8: SIMD width
  DECLARE_ALIGNED(16, int16_t,
                  v_dequant_QTX[QINDEX_RANGE][8]);  // 8: SIMD width
} Dequants;

// The DeltaQuantParams structure holds the dc/ac deltaq parameters.
typedef struct {
  int y_dc_delta_q;
  int u_dc_delta_q;
  int u_ac_delta_q;
  int v_dc_delta_q;
  int v_ac_delta_q;
  int sharpness;
} DeltaQuantParams;

typedef struct {
  // Quantization parameters for internal quantizer setup.
  QUANTS quants;
  // Dequantization parameters for internal quantizer setup.
  Dequants dequants;
  // Deltaq parameters to track the state of the dc/ac deltaq parameters in
  // cm->quant_params. It is used to decide whether the quantizer tables need
  // to be re-initialized.
  DeltaQuantParams prev_deltaq_params;
} EncQuantDequantParams;

struct AV1_COMP;
struct AV1Common;

void av1_frame_init_quantizer(struct AV1_COMP *cpi);

void av1_init_plane_quantizers(const struct AV1_COMP *cpi, MACROBLOCK *x,
                               int segment_id, const int do_update);

void av1_build_quantizer(aom_bit_depth_t bit_depth, int y_dc_delta_q,
                         int u_dc_delta_q, int u_ac_delta_q, int v_dc_delta_q,
                         int v_ac_delta_q, QUANTS *const quants,
                         Dequants *const deq, int sharpness);

void av1_init_quantizer(EncQuantDequantParams *const enc_quant_dequant_params,
                        CommonQuantParams *quant_params,
                        aom_bit_depth_t bit_depth, int sharpness);

void av1_set_quantizer(struct AV1Common *const cm, int min_qmlevel,
                       int max_qmlevel, int q, int enable_chroma_deltaq,
                       int enable_hdr_deltaq, bool is_allintra,
                       aom_tune_metric tuning);

int av1_quantizer_to_qindex(int quantizer);

int av1_qindex_to_quantizer(int qindex);

void av1_quantize_skip(intptr_t n_coeffs, tran_low_t *qcoeff_ptr,
                       tran_low_t *dqcoeff_ptr, uint16_t *eob_ptr);

/*!\brief Quantize transform coefficients without using qmatrix
 *
 * quant_ptr, dequant_ptr and round_ptr are size 2 arrays,
 * where index 0 corresponds to dc coeff and index 1 corresponds to ac coeffs.
 *
 * \param[in]  quant_ptr    16-bit fixed point representation of inverse
 *                          quantize step size, i.e. 2^16/dequant
 * \param[in]  dequant_ptr  quantize step size
 * \param[in]  round_ptr    rounding
 * \param[in]  log_scale    the relative log scale of the transform
 *                          coefficients
 * \param[in]  scan         scan[i] indicates the position of ith to-be-coded
 *                          coefficient
 * \param[in]  coeff_count  number of coefficients
 * \param[out] qcoeff_ptr   quantized coefficients
 * \param[out] dqcoeff_ptr  dequantized coefficients
 *
 * \return The last non-zero coefficient's scan index plus 1
 */
int av1_quantize_fp_no_qmatrix(const int16_t quant_ptr[2],
                               const int16_t dequant_ptr[2],
                               const int16_t round_ptr[2], int log_scale,
                               const int16_t *scan, int coeff_count,
                               const tran_low_t *coeff_ptr,
                               tran_low_t *qcoeff_ptr, tran_low_t *dqcoeff_ptr);

void av1_quantize_fp_facade(const tran_low_t *coeff_ptr, intptr_t n_coeffs,
                            const MACROBLOCK_PLANE *p, tran_low_t *qcoeff_ptr,
                            tran_low_t *dqcoeff_ptr, uint16_t *eob_ptr,
                            const SCAN_ORDER *sc, const QUANT_PARAM *qparam);

void av1_quantize_b_facade(const tran_low_t *coeff_ptr, intptr_t n_coeffs,
                           const MACROBLOCK_PLANE *p, tran_low_t *qcoeff_ptr,
                           tran_low_t *dqcoeff_ptr, uint16_t *eob_ptr,
                           const SCAN_ORDER *sc, const QUANT_PARAM *qparam);

void av1_quantize_dc_facade(const tran_low_t *coeff_ptr, intptr_t n_coeffs,
                            const MACROBLOCK_PLANE *p, tran_low_t *qcoeff_ptr,
                            tran_low_t *dqcoeff_ptr, uint16_t *eob_ptr,
                            const SCAN_ORDER *sc, const QUANT_PARAM *qparam);

#if CONFIG_AV1_HIGHBITDEPTH
void av1_highbd_quantize_fp_facade(const tran_low_t *coeff_ptr,
                                   intptr_t n_coeffs, const MACROBLOCK_PLANE *p,
                                   tran_low_t *qcoeff_ptr,
                                   tran_low_t *dqcoeff_ptr, uint16_t *eob_ptr,
                                   const SCAN_ORDER *sc,
                                   const QUANT_PARAM *qparam);

void av1_highbd_quantize_b_facade(const tran_low_t *coeff_ptr,
                                  intptr_t n_coeffs, const MACROBLOCK_PLANE *p,
                                  tran_low_t *qcoeff_ptr,
                                  tran_low_t *dqcoeff_ptr, uint16_t *eob_ptr,
                                  const SCAN_ORDER *sc,
                                  const QUANT_PARAM *qparam);

void av1_highbd_quantize_dc_facade(const tran_low_t *coeff_ptr,
                                   intptr_t n_coeffs, const MACROBLOCK_PLANE *p,
                                   tran_low_t *qcoeff_ptr,
                                   tran_low_t *dqcoeff_ptr, uint16_t *eob_ptr,
                                   const SCAN_ORDER *sc,
                                   const QUANT_PARAM *qparam);

#endif

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_AV1_QUANTIZE_H_
