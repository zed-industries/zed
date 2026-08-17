/*
 * Copyright (c) 2021, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AV1_ENCODER_TXB_RDOPT_UTILS_H_
#define AOM_AV1_ENCODER_TXB_RDOPT_UTILS_H_

#include "av1/encoder/encodetxb.h"

static const int golomb_bits_cost[32] = {
  0,       512,     512 * 3, 512 * 3, 512 * 5, 512 * 5, 512 * 5, 512 * 5,
  512 * 7, 512 * 7, 512 * 7, 512 * 7, 512 * 7, 512 * 7, 512 * 7, 512 * 7,
  512 * 9, 512 * 9, 512 * 9, 512 * 9, 512 * 9, 512 * 9, 512 * 9, 512 * 9,
  512 * 9, 512 * 9, 512 * 9, 512 * 9, 512 * 9, 512 * 9, 512 * 9, 512 * 9
};

static const int golomb_cost_diff[32] = {
  0,       512, 512 * 2, 0, 512 * 2, 0, 0, 0, 512 * 2, 0, 0, 0, 0, 0, 0, 0,
  512 * 2, 0,   0,       0, 0,       0, 0, 0, 0,       0, 0, 0, 0, 0, 0, 0
};

// Look up table of individual cost of coefficient by its quantization level.
// determined based on Laplacian distribution conditioned on estimated context
static const int costLUT[15] = { -1143, 53,   545,  825,  1031,
                                 1209,  1393, 1577, 1763, 1947,
                                 2132,  2317, 2501, 2686, 2871 };

static const int const_term = (1 << AV1_PROB_COST_SHIFT);

static const int loge_par = ((14427 << AV1_PROB_COST_SHIFT) + 5000) / 10000;

static inline int get_dqv(const int16_t *dequant, int coeff_idx,
                          const qm_val_t *iqmatrix) {
  int dqv = dequant[!!coeff_idx];
  if (iqmatrix != NULL)
    dqv =
        ((iqmatrix[coeff_idx] * dqv) + (1 << (AOM_QM_BITS - 1))) >> AOM_QM_BITS;
  return dqv;
}

static inline int64_t get_coeff_dist(tran_low_t tcoeff, tran_low_t dqcoeff,
                                     int shift, const qm_val_t *qmatrix,
                                     int coeff_idx) {
  int64_t diff = (tcoeff - dqcoeff) * (1 << shift);
  if (qmatrix == NULL) {
    return diff * diff;
  }
  // When AOM_DIST_METRIC_QM_PSNR is enabled, this mirrors the rate-distortion
  // computation done in av1_block_error_qm, improving visual quality.
  // The maximum value of `shift` is 2, `tcoeff` and `dqcoeff` are at most 22
  // bits, and AOM_QM_BITS is 5, so `diff` should fit in 29-bits. The
  // multiplication `diff * diff` then does not risk overflowing.
  diff *= qmatrix[coeff_idx];
  const int64_t error =
      (diff * diff + (1 << (2 * AOM_QM_BITS - 1))) >> (2 * AOM_QM_BITS);
  return error;
}

static int get_eob_cost(int eob, const LV_MAP_EOB_COST *txb_eob_costs,
                        const LV_MAP_COEFF_COST *txb_costs, TX_CLASS tx_class) {
  int eob_extra;
  const int eob_pt = av1_get_eob_pos_token(eob, &eob_extra);
  int eob_cost = 0;
  const int eob_multi_ctx = (tx_class == TX_CLASS_2D) ? 0 : 1;
  eob_cost = txb_eob_costs->eob_cost[eob_multi_ctx][eob_pt - 1];

  if (av1_eob_offset_bits[eob_pt] > 0) {
    const int eob_ctx = eob_pt - 3;
    const int eob_shift = av1_eob_offset_bits[eob_pt] - 1;
    const int bit = (eob_extra & (1 << eob_shift)) ? 1 : 0;
    eob_cost += txb_costs->eob_extra_cost[eob_ctx][bit];
    const int offset_bits = av1_eob_offset_bits[eob_pt];
    if (offset_bits > 1) eob_cost += av1_cost_literal(offset_bits - 1);
  }
  return eob_cost;
}

static inline int get_golomb_cost(int abs_qc) {
  if (abs_qc >= 1 + NUM_BASE_LEVELS + COEFF_BASE_RANGE) {
    const int r = abs_qc - COEFF_BASE_RANGE - NUM_BASE_LEVELS;
    const int length = get_msb(r) + 1;
    return av1_cost_literal(2 * length - 1);
  }
  return 0;
}

static inline int get_br_cost(tran_low_t level, const int *coeff_lps) {
  const int base_range = AOMMIN(level - 1 - NUM_BASE_LEVELS, COEFF_BASE_RANGE);
  return coeff_lps[base_range] + get_golomb_cost(level);
}

static inline int get_br_cost_with_diff(tran_low_t level, const int *coeff_lps,
                                        int *diff) {
  const int base_range = AOMMIN(level - 1 - NUM_BASE_LEVELS, COEFF_BASE_RANGE);
  int golomb_bits = 0;
  if (level <= COEFF_BASE_RANGE + 1 + NUM_BASE_LEVELS)
    *diff += coeff_lps[base_range + COEFF_BASE_RANGE + 1];

  if (level >= COEFF_BASE_RANGE + 1 + NUM_BASE_LEVELS) {
    int r = level - COEFF_BASE_RANGE - NUM_BASE_LEVELS;
    if (r < 32) {
      golomb_bits = golomb_bits_cost[r];
      *diff += golomb_cost_diff[r];
    } else {
      golomb_bits = get_golomb_cost(level);
      *diff += (r & (r - 1)) == 0 ? 1024 : 0;
    }
  }

  return coeff_lps[base_range] + golomb_bits;
}

static AOM_FORCE_INLINE int get_two_coeff_cost_simple(
    int ci, tran_low_t abs_qc, int coeff_ctx,
    const LV_MAP_COEFF_COST *txb_costs, int bhl, TX_CLASS tx_class,
    const uint8_t *levels, int *cost_low) {
  // this simple version assumes the coeff's scan_idx is not DC (scan_idx != 0)
  // and not the last (scan_idx != eob - 1)
  assert(ci > 0);
  int cost = txb_costs->base_cost[coeff_ctx][AOMMIN(abs_qc, 3)];
  int diff = 0;
  if (abs_qc <= 3) diff = txb_costs->base_cost[coeff_ctx][abs_qc + 4];
  if (abs_qc) {
    cost += av1_cost_literal(1);
    if (abs_qc > NUM_BASE_LEVELS) {
      const int br_ctx = get_br_ctx(levels, ci, bhl, tx_class);
      int brcost_diff = 0;
      cost += get_br_cost_with_diff(abs_qc, txb_costs->lps_cost[br_ctx],
                                    &brcost_diff);
      diff += brcost_diff;
    }
  }
  *cost_low = cost - diff;

  return cost;
}

static inline int get_coeff_cost_eob(int ci, tran_low_t abs_qc, int sign,
                                     int coeff_ctx, int dc_sign_ctx,
                                     const LV_MAP_COEFF_COST *txb_costs,
                                     int bhl, TX_CLASS tx_class) {
  int cost = 0;
  cost += txb_costs->base_eob_cost[coeff_ctx][AOMMIN(abs_qc, 3) - 1];
  if (abs_qc != 0) {
    if (ci == 0) {
      cost += txb_costs->dc_sign_cost[dc_sign_ctx][sign];
    } else {
      cost += av1_cost_literal(1);
    }
    if (abs_qc > NUM_BASE_LEVELS) {
      int br_ctx;
      br_ctx = get_br_ctx_eob(ci, bhl, tx_class);
      cost += get_br_cost(abs_qc, txb_costs->lps_cost[br_ctx]);
    }
  }
  return cost;
}

static inline int get_coeff_cost_general(int is_last, int ci, tran_low_t abs_qc,
                                         int sign, int coeff_ctx,
                                         int dc_sign_ctx,
                                         const LV_MAP_COEFF_COST *txb_costs,
                                         int bhl, TX_CLASS tx_class,
                                         const uint8_t *levels) {
  int cost = 0;
  if (is_last) {
    cost += txb_costs->base_eob_cost[coeff_ctx][AOMMIN(abs_qc, 3) - 1];
  } else {
    cost += txb_costs->base_cost[coeff_ctx][AOMMIN(abs_qc, 3)];
  }
  if (abs_qc != 0) {
    if (ci == 0) {
      cost += txb_costs->dc_sign_cost[dc_sign_ctx][sign];
    } else {
      cost += av1_cost_literal(1);
    }
    if (abs_qc > NUM_BASE_LEVELS) {
      int br_ctx;
      if (is_last)
        br_ctx = get_br_ctx_eob(ci, bhl, tx_class);
      else
        br_ctx = get_br_ctx(levels, ci, bhl, tx_class);
      cost += get_br_cost(abs_qc, txb_costs->lps_cost[br_ctx]);
    }
  }
  return cost;
}

static inline void get_qc_dqc_low(tran_low_t abs_qc, int sign, int dqv,
                                  int shift, tran_low_t *qc_low,
                                  tran_low_t *dqc_low) {
  tran_low_t abs_qc_low = abs_qc - 1;
  *qc_low = (-sign ^ abs_qc_low) + sign;
  assert((sign ? -abs_qc_low : abs_qc_low) == *qc_low);
  tran_low_t abs_dqc_low = (abs_qc_low * dqv) >> shift;
  *dqc_low = (-sign ^ abs_dqc_low) + sign;
  assert((sign ? -abs_dqc_low : abs_dqc_low) == *dqc_low);
}

static inline void update_coeff_eob_fast(int *eob, int shift,
                                         const int16_t *dequant_ptr,
                                         const int16_t *scan,
                                         const tran_low_t *coeff_ptr,
                                         tran_low_t *qcoeff_ptr,
                                         tran_low_t *dqcoeff_ptr) {
  // TODO(sarahparker) make this work for aomqm
  int eob_out = *eob;
  int zbin[2] = { dequant_ptr[0] + ROUND_POWER_OF_TWO(dequant_ptr[0] * 70, 7),
                  dequant_ptr[1] + ROUND_POWER_OF_TWO(dequant_ptr[1] * 70, 7) };

  for (int i = *eob - 1; i >= 0; i--) {
    const int rc = scan[i];
    const int qcoeff = qcoeff_ptr[rc];
    const int coeff = coeff_ptr[rc];
    const int coeff_sign = AOMSIGN(coeff);
    int64_t abs_coeff = (coeff ^ coeff_sign) - coeff_sign;

    if (((abs_coeff << (1 + shift)) < zbin[rc != 0]) || (qcoeff == 0)) {
      eob_out--;
      qcoeff_ptr[rc] = 0;
      dqcoeff_ptr[rc] = 0;
    } else {
      break;
    }
  }

  *eob = eob_out;
}
#endif  // AOM_AV1_ENCODER_TXB_RDOPT_UTILS_H_
