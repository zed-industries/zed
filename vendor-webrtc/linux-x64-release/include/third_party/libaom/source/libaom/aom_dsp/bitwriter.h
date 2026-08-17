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

#ifndef AOM_AOM_DSP_BITWRITER_H_
#define AOM_AOM_DSP_BITWRITER_H_

#include <assert.h>

#include "config/aom_config.h"

#include "aom_dsp/entenc.h"
#include "aom_dsp/prob.h"

#if CONFIG_RD_DEBUG
#include "av1/common/blockd.h"
#include "av1/encoder/cost.h"
#endif

#if CONFIG_BITSTREAM_DEBUG
#include "aom_util/debug_util.h"
#endif  // CONFIG_BITSTREAM_DEBUG

#ifdef __cplusplus
extern "C" {
#endif

struct aom_writer {
  unsigned int pos;
  uint8_t *buffer;
  od_ec_enc ec;
  uint8_t allow_update_cdf;
};

typedef struct aom_writer aom_writer;

typedef struct TOKEN_STATS {
  int cost;
#if CONFIG_RD_DEBUG
  int txb_coeff_cost_map[TXB_COEFF_COST_MAP_SIZE][TXB_COEFF_COST_MAP_SIZE];
#endif
} TOKEN_STATS;

static inline void init_token_stats(TOKEN_STATS *token_stats) {
#if CONFIG_RD_DEBUG
  int r, c;
  for (r = 0; r < TXB_COEFF_COST_MAP_SIZE; ++r) {
    for (c = 0; c < TXB_COEFF_COST_MAP_SIZE; ++c) {
      token_stats->txb_coeff_cost_map[r][c] = 0;
    }
  }
#endif
  token_stats->cost = 0;
}

void aom_start_encode(aom_writer *w, uint8_t *buffer);

// Returns a negative number on error. Caller must check the return value and
// handle error.
int aom_stop_encode(aom_writer *w);

int aom_tell_size(aom_writer *w);

static inline void aom_write(aom_writer *w, int bit, int probability) {
  int p = (0x7FFFFF - (probability << 15) + probability) >> 8;
#if CONFIG_BITSTREAM_DEBUG
  aom_cdf_prob cdf[2] = { (aom_cdf_prob)p, 32767 };
  bitstream_queue_push(bit, cdf, 2);
#endif

  od_ec_encode_bool_q15(&w->ec, bit, p);
}

static inline void aom_write_bit(aom_writer *w, int bit) {
  aom_write(w, bit, 128);  // aom_prob_half
}

static inline void aom_write_literal(aom_writer *w, int data, int bits) {
  int bit;

  for (bit = bits - 1; bit >= 0; bit--) aom_write_bit(w, 1 & (data >> bit));
}

static inline void aom_write_cdf(aom_writer *w, int symb,
                                 const aom_cdf_prob *cdf, int nsymbs) {
#if CONFIG_BITSTREAM_DEBUG
  bitstream_queue_push(symb, cdf, nsymbs);
#endif

  od_ec_encode_cdf_q15(&w->ec, symb, cdf, nsymbs);
}

static inline void aom_write_symbol(aom_writer *w, int symb, aom_cdf_prob *cdf,
                                    int nsymbs) {
  aom_write_cdf(w, symb, cdf, nsymbs);
  if (w->allow_update_cdf) update_cdf(cdf, symb, nsymbs);
}

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AOM_DSP_BITWRITER_H_
