/*
 * Copyright (c) 2001-2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AOM_DSP_ENTDEC_H_
#define AOM_AOM_DSP_ENTDEC_H_
#include <limits.h>
#include "aom_dsp/entcode.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct od_ec_dec od_ec_dec;

#if defined(OD_ACCOUNTING) && OD_ACCOUNTING
#define OD_ACC_STR , char *acc_str
#define od_ec_dec_bits(dec, ftb, str) od_ec_dec_bits_(dec, ftb, str)
#else
#define OD_ACC_STR
#define od_ec_dec_bits(dec, ftb, str) od_ec_dec_bits_(dec, ftb)
#endif

/*The entropy decoder context.*/
struct od_ec_dec {
  /*The start of the current input buffer.*/
  const unsigned char *buf;
  /*An offset used to keep track of tell after reaching the end of the stream.
    This is constant throughout most of the decoding process, but becomes
     important once we hit the end of the buffer and stop incrementing bptr
     (and instead pretend cnt has lots of bits).*/
  int32_t tell_offs;
  /*The end of the current input buffer.*/
  const unsigned char *end;
  /*The read pointer for the entropy-coded bits.*/
  const unsigned char *bptr;
  /*The difference between the high end of the current range, (low + rng), and
     the coded value, minus 1.
    This stores up to OD_EC_WINDOW_SIZE bits of that difference, but the
     decoder only uses the top 16 bits of the window to decode the next symbol.
    As we shift up during renormalization, if we don't have enough bits left in
     the window to fill the top 16, we'll read in more bits of the coded
     value.*/
  od_ec_window dif;
  /*The number of values in the current range.*/
  uint16_t rng;
  /*The number of bits of data in the current value.*/
  int16_t cnt;
};

/*See entdec.c for further documentation.*/

void od_ec_dec_init(od_ec_dec *dec, const unsigned char *buf, uint32_t storage)
    OD_ARG_NONNULL(1) OD_ARG_NONNULL(2);

OD_WARN_UNUSED_RESULT int od_ec_decode_bool_q15(od_ec_dec *dec, unsigned f)
    OD_ARG_NONNULL(1);
OD_WARN_UNUSED_RESULT int od_ec_decode_cdf_q15(od_ec_dec *dec,
                                               const uint16_t *cdf, int nsyms)
    OD_ARG_NONNULL(1) OD_ARG_NONNULL(2);

OD_WARN_UNUSED_RESULT uint32_t od_ec_dec_bits_(od_ec_dec *dec, unsigned ftb)
    OD_ARG_NONNULL(1);

OD_WARN_UNUSED_RESULT int od_ec_dec_tell(const od_ec_dec *dec)
    OD_ARG_NONNULL(1);
OD_WARN_UNUSED_RESULT uint32_t od_ec_dec_tell_frac(const od_ec_dec *dec)
    OD_ARG_NONNULL(1);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AOM_DSP_ENTDEC_H_
