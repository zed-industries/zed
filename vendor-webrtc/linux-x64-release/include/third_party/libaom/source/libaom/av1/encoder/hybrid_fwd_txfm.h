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

#ifndef AOM_AV1_ENCODER_HYBRID_FWD_TXFM_H_
#define AOM_AV1_ENCODER_HYBRID_FWD_TXFM_H_

#include "config/aom_config.h"

#ifdef __cplusplus
extern "C" {
#endif

void av1_fwd_txfm(const int16_t *src_diff, tran_low_t *coeff, int diff_stride,
                  TxfmParam *txfm_param);

void av1_highbd_fwd_txfm(const int16_t *src_diff, tran_low_t *coeff,
                         int diff_stride, TxfmParam *txfm_param);

/*!\brief Apply Hadamard or DCT transform
 *
 * \callergraph
 * DCT and Hadamard transforms are commonly used for quick RD score estimation.
 * The coeff buffer's size should be equal to the number of pixels
 * corresponding to tx_size.
 */
void av1_quick_txfm(int use_hadamard, TX_SIZE tx_size, BitDepthInfo bd_info,
                    const int16_t *src_diff, int src_stride, tran_low_t *coeff);
#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_HYBRID_FWD_TXFM_H_
