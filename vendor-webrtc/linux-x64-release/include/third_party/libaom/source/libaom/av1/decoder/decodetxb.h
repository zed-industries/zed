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

#ifndef AOM_AV1_DECODER_DECODETXB_H_
#define AOM_AV1_DECODER_DECODETXB_H_

#include "av1/common/enums.h"

struct aom_reader;
struct AV1Common;
struct DecoderCodingBlock;
struct txb_ctx;

void av1_read_coeffs_txb(const struct AV1Common *const cm,
                         struct DecoderCodingBlock *dcb,
                         struct aom_reader *const r, const int plane,
                         const int row, const int col, const TX_SIZE tx_size);
#endif  // AOM_AV1_DECODER_DECODETXB_H_
