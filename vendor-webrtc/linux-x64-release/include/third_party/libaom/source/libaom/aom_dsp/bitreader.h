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

#ifndef AOM_AOM_DSP_BITREADER_H_
#define AOM_AOM_DSP_BITREADER_H_

#include <assert.h>
#include <limits.h>

#include "config/aom_config.h"

#include "aom/aomdx.h"
#include "aom/aom_integer.h"
#include "aom_dsp/entdec.h"
#include "aom_dsp/odintrin.h"
#include "aom_dsp/prob.h"

#if CONFIG_BITSTREAM_DEBUG
#include "aom_util/debug_util.h"
#endif  // CONFIG_BITSTREAM_DEBUG

#if CONFIG_ACCOUNTING
#include "av1/decoder/accounting.h"
#define ACCT_STR_NAME acct_str
#define ACCT_STR_PARAM , const char *ACCT_STR_NAME
#define ACCT_STR_ARG(s) , s
#else
#define ACCT_STR_PARAM
#define ACCT_STR_ARG(s)
#endif

#define aom_read(r, prob, ACCT_STR_NAME) \
  aom_read_(r, prob ACCT_STR_ARG(ACCT_STR_NAME))
#define aom_read_bit(r, ACCT_STR_NAME) \
  aom_read_bit_(r ACCT_STR_ARG(ACCT_STR_NAME))
#define aom_read_tree(r, tree, probs, ACCT_STR_NAME) \
  aom_read_tree_(r, tree, probs ACCT_STR_ARG(ACCT_STR_NAME))
#define aom_read_literal(r, bits, ACCT_STR_NAME) \
  aom_read_literal_(r, bits ACCT_STR_ARG(ACCT_STR_NAME))
#define aom_read_cdf(r, cdf, nsymbs, ACCT_STR_NAME) \
  aom_read_cdf_(r, cdf, nsymbs ACCT_STR_ARG(ACCT_STR_NAME))
#define aom_read_symbol(r, cdf, nsymbs, ACCT_STR_NAME) \
  aom_read_symbol_(r, cdf, nsymbs ACCT_STR_ARG(ACCT_STR_NAME))

#ifdef __cplusplus
extern "C" {
#endif

struct aom_reader {
  const uint8_t *buffer;
  const uint8_t *buffer_end;
  od_ec_dec ec;
#if CONFIG_ACCOUNTING
  Accounting *accounting;
#endif
  uint8_t allow_update_cdf;
};

typedef struct aom_reader aom_reader;

int aom_reader_init(aom_reader *r, const uint8_t *buffer, size_t size);

const uint8_t *aom_reader_find_begin(aom_reader *r);

const uint8_t *aom_reader_find_end(aom_reader *r);

// Returns true if the bit reader has tried to decode more data from the buffer
// than was actually provided.
int aom_reader_has_overflowed(const aom_reader *r);

// Returns the position in the bit reader in bits.
uint32_t aom_reader_tell(const aom_reader *r);

// Returns the position in the bit reader in 1/8th bits.
uint32_t aom_reader_tell_frac(const aom_reader *r);

#if CONFIG_ACCOUNTING
static inline void aom_process_accounting(const aom_reader *r ACCT_STR_PARAM) {
  if (r->accounting != NULL) {
    uint32_t tell_frac;
    tell_frac = aom_reader_tell_frac(r);
    aom_accounting_record(r->accounting, ACCT_STR_NAME,
                          tell_frac - r->accounting->last_tell_frac);
    r->accounting->last_tell_frac = tell_frac;
  }
}

static inline void aom_update_symb_counts(const aom_reader *r, int is_binary) {
  if (r->accounting != NULL) {
    r->accounting->syms.num_multi_syms += !is_binary;
    r->accounting->syms.num_binary_syms += !!is_binary;
  }
}
#endif

static inline int aom_read_(aom_reader *r, int prob ACCT_STR_PARAM) {
  int p = (0x7FFFFF - (prob << 15) + prob) >> 8;
  int bit = od_ec_decode_bool_q15(&r->ec, p);

#if CONFIG_BITSTREAM_DEBUG
  {
    int i;
    int ref_bit, ref_nsymbs;
    aom_cdf_prob ref_cdf[16];
    const int queue_r = bitstream_queue_get_read();
    const int frame_idx = aom_bitstream_queue_get_frame_read();
    bitstream_queue_pop(&ref_bit, ref_cdf, &ref_nsymbs);
    if (ref_nsymbs != 2) {
      fprintf(stderr,
              "\n *** [bit] nsymbs error, frame_idx_r %d nsymbs %d ref_nsymbs "
              "%d queue_r %d\n",
              frame_idx, 2, ref_nsymbs, queue_r);
      assert(0);
    }
    if ((ref_nsymbs != 2) || (ref_cdf[0] != (aom_cdf_prob)p) ||
        (ref_cdf[1] != 32767)) {
      fprintf(stderr,
              "\n *** [bit] cdf error, frame_idx_r %d cdf {%d, %d} ref_cdf {%d",
              frame_idx, p, 32767, ref_cdf[0]);
      for (i = 1; i < ref_nsymbs; ++i) fprintf(stderr, ", %d", ref_cdf[i]);
      fprintf(stderr, "} queue_r %d\n", queue_r);
      assert(0);
    }
    if (bit != ref_bit) {
      fprintf(stderr,
              "\n *** [bit] symb error, frame_idx_r %d symb %d ref_symb %d "
              "queue_r %d\n",
              frame_idx, bit, ref_bit, queue_r);
      assert(0);
    }
  }
#endif

#if CONFIG_ACCOUNTING
  if (ACCT_STR_NAME) aom_process_accounting(r, ACCT_STR_NAME);
  aom_update_symb_counts(r, 1);
#endif
  return bit;
}

static inline int aom_read_bit_(aom_reader *r ACCT_STR_PARAM) {
  int ret;
  ret = aom_read(r, 128, NULL);  // aom_prob_half
#if CONFIG_ACCOUNTING
  if (ACCT_STR_NAME) aom_process_accounting(r, ACCT_STR_NAME);
#endif
  return ret;
}

static inline int aom_read_literal_(aom_reader *r, int bits ACCT_STR_PARAM) {
  int literal = 0, bit;

  for (bit = bits - 1; bit >= 0; bit--) literal |= aom_read_bit(r, NULL) << bit;
#if CONFIG_ACCOUNTING
  if (ACCT_STR_NAME) aom_process_accounting(r, ACCT_STR_NAME);
#endif
  return literal;
}

static inline int aom_read_cdf_(aom_reader *r, const aom_cdf_prob *cdf,
                                int nsymbs ACCT_STR_PARAM) {
  int symb;
  assert(cdf != NULL);
  symb = od_ec_decode_cdf_q15(&r->ec, cdf, nsymbs);

#if CONFIG_BITSTREAM_DEBUG
  {
    int i;
    int cdf_error = 0;
    int ref_symb, ref_nsymbs;
    aom_cdf_prob ref_cdf[16];
    const int queue_r = bitstream_queue_get_read();
    const int frame_idx = aom_bitstream_queue_get_frame_read();
    bitstream_queue_pop(&ref_symb, ref_cdf, &ref_nsymbs);
    if (nsymbs != ref_nsymbs) {
      fprintf(stderr,
              "\n *** nsymbs error, frame_idx_r %d nsymbs %d ref_nsymbs %d "
              "queue_r %d\n",
              frame_idx, nsymbs, ref_nsymbs, queue_r);
      cdf_error = 0;
      assert(0);
    } else {
      for (i = 0; i < nsymbs; ++i)
        if (cdf[i] != ref_cdf[i]) cdf_error = 1;
    }
    if (cdf_error) {
      fprintf(stderr, "\n *** cdf error, frame_idx_r %d cdf {%d", frame_idx,
              cdf[0]);
      for (i = 1; i < nsymbs; ++i) fprintf(stderr, ", %d", cdf[i]);
      fprintf(stderr, "} ref_cdf {%d", ref_cdf[0]);
      for (i = 1; i < ref_nsymbs; ++i) fprintf(stderr, ", %d", ref_cdf[i]);
      fprintf(stderr, "} queue_r %d\n", queue_r);
      assert(0);
    }
    if (symb != ref_symb) {
      fprintf(
          stderr,
          "\n *** symb error, frame_idx_r %d symb %d ref_symb %d queue_r %d\n",
          frame_idx, symb, ref_symb, queue_r);
      assert(0);
    }
  }
#endif

#if CONFIG_ACCOUNTING
  if (ACCT_STR_NAME) aom_process_accounting(r, ACCT_STR_NAME);
  aom_update_symb_counts(r, (nsymbs == 2));
#endif
  return symb;
}

static inline int aom_read_symbol_(aom_reader *r, aom_cdf_prob *cdf,
                                   int nsymbs ACCT_STR_PARAM) {
  int ret;
  ret = aom_read_cdf(r, cdf, nsymbs, ACCT_STR_NAME);
  if (r->allow_update_cdf) update_cdf(cdf, ret, nsymbs);
  return ret;
}

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AOM_DSP_BITREADER_H_
