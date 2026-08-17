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

#ifndef AOM_AV1_COMMON_ENTROPY_H_
#define AOM_AV1_COMMON_ENTROPY_H_

#include "config/aom_config.h"

#include "aom/aom_integer.h"
#include "aom_dsp/prob.h"

#include "av1/common/common.h"
#include "av1/common/common_data.h"
#include "av1/common/enums.h"

#ifdef __cplusplus
extern "C" {
#endif

#define TOKEN_CDF_Q_CTXS 4

#define TXB_SKIP_CONTEXTS 13

#define EOB_COEF_CONTEXTS 9

#define SIG_COEF_CONTEXTS_2D 26
#define SIG_COEF_CONTEXTS_1D 16
#define SIG_COEF_CONTEXTS_EOB 4
#define SIG_COEF_CONTEXTS (SIG_COEF_CONTEXTS_2D + SIG_COEF_CONTEXTS_1D)

#define COEFF_BASE_CONTEXTS (SIG_COEF_CONTEXTS)
#define DC_SIGN_CONTEXTS 3

#define BR_TMP_OFFSET 12
#define BR_REF_CAT 4
#define LEVEL_CONTEXTS 21

#define NUM_BASE_LEVELS 2

#define BR_CDF_SIZE (4)
#define COEFF_BASE_RANGE (4 * (BR_CDF_SIZE - 1))

#define COEFF_CONTEXT_BITS 3
#define COEFF_CONTEXT_MASK ((1 << COEFF_CONTEXT_BITS) - 1)
#define MAX_BASE_BR_RANGE (COEFF_BASE_RANGE + NUM_BASE_LEVELS + 1)

#define BASE_CONTEXT_POSITION_NUM 12

enum {
  TX_CLASS_2D = 0,
  TX_CLASS_HORIZ = 1,
  TX_CLASS_VERT = 2,
  TX_CLASSES = 3,
} UENUM1BYTE(TX_CLASS);

#define DCT_MAX_VALUE 16384
#define DCT_MAX_VALUE_HIGH10 65536
#define DCT_MAX_VALUE_HIGH12 262144

/* Coefficients are predicted via a 3-dimensional probability table indexed on
 * REF_TYPES, COEF_BANDS and COEF_CONTEXTS. */
#define REF_TYPES 2  // intra=0, inter=1

struct AV1Common;
struct frame_contexts;
void av1_reset_cdf_symbol_counters(struct frame_contexts *fc);
void av1_default_coef_probs(struct AV1Common *cm);
void av1_init_mode_probs(struct frame_contexts *fc);

struct frame_contexts;

typedef char ENTROPY_CONTEXT;

static inline int combine_entropy_contexts(ENTROPY_CONTEXT a,
                                           ENTROPY_CONTEXT b) {
  return (a != 0) + (b != 0);
}

static inline int get_entropy_context(TX_SIZE tx_size, const ENTROPY_CONTEXT *a,
                                      const ENTROPY_CONTEXT *l) {
  ENTROPY_CONTEXT above_ec = 0, left_ec = 0;

  switch (tx_size) {
    case TX_4X4:
      above_ec = a[0] != 0;
      left_ec = l[0] != 0;
      break;
    case TX_4X8:
      above_ec = a[0] != 0;
      left_ec = !!*(const uint16_t *)l;
      break;
    case TX_8X4:
      above_ec = !!*(const uint16_t *)a;
      left_ec = l[0] != 0;
      break;
    case TX_8X16:
      above_ec = !!*(const uint16_t *)a;
      left_ec = !!*(const uint32_t *)l;
      break;
    case TX_16X8:
      above_ec = !!*(const uint32_t *)a;
      left_ec = !!*(const uint16_t *)l;
      break;
    case TX_16X32:
      above_ec = !!*(const uint32_t *)a;
      left_ec = !!*(const uint64_t *)l;
      break;
    case TX_32X16:
      above_ec = !!*(const uint64_t *)a;
      left_ec = !!*(const uint32_t *)l;
      break;
    case TX_8X8:
      above_ec = !!*(const uint16_t *)a;
      left_ec = !!*(const uint16_t *)l;
      break;
    case TX_16X16:
      above_ec = !!*(const uint32_t *)a;
      left_ec = !!*(const uint32_t *)l;
      break;
    case TX_32X32:
      above_ec = !!*(const uint64_t *)a;
      left_ec = !!*(const uint64_t *)l;
      break;
    case TX_64X64:
      above_ec = !!(*(const uint64_t *)a | *(const uint64_t *)(a + 8));
      left_ec = !!(*(const uint64_t *)l | *(const uint64_t *)(l + 8));
      break;
    case TX_32X64:
      above_ec = !!*(const uint64_t *)a;
      left_ec = !!(*(const uint64_t *)l | *(const uint64_t *)(l + 8));
      break;
    case TX_64X32:
      above_ec = !!(*(const uint64_t *)a | *(const uint64_t *)(a + 8));
      left_ec = !!*(const uint64_t *)l;
      break;
    case TX_4X16:
      above_ec = a[0] != 0;
      left_ec = !!*(const uint32_t *)l;
      break;
    case TX_16X4:
      above_ec = !!*(const uint32_t *)a;
      left_ec = l[0] != 0;
      break;
    case TX_8X32:
      above_ec = !!*(const uint16_t *)a;
      left_ec = !!*(const uint64_t *)l;
      break;
    case TX_32X8:
      above_ec = !!*(const uint64_t *)a;
      left_ec = !!*(const uint16_t *)l;
      break;
    case TX_16X64:
      above_ec = !!*(const uint32_t *)a;
      left_ec = !!(*(const uint64_t *)l | *(const uint64_t *)(l + 8));
      break;
    case TX_64X16:
      above_ec = !!(*(const uint64_t *)a | *(const uint64_t *)(a + 8));
      left_ec = !!*(const uint32_t *)l;
      break;
    default: assert(0 && "Invalid transform size."); break;
  }
  return combine_entropy_contexts(above_ec, left_ec);
}

static inline TX_SIZE get_txsize_entropy_ctx(TX_SIZE txsize) {
  return (TX_SIZE)((txsize_sqr_map[txsize] + txsize_sqr_up_map[txsize] + 1) >>
                   1);
}

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_COMMON_ENTROPY_H_
