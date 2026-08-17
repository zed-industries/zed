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

#ifndef AOM_AOM_DSP_BINARY_CODES_READER_H_
#define AOM_AOM_DSP_BINARY_CODES_READER_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <assert.h>

#include "config/aom_config.h"

#include "aom/aom_integer.h"
#include "aom_dsp/bitreader.h"
#include "aom_dsp/bitreader_buffer.h"

#define aom_read_primitive_refsubexpfin(r, n, k, ref, ACCT_STR_NAME) \
  aom_read_primitive_refsubexpfin_(r, n, k, ref ACCT_STR_ARG(ACCT_STR_NAME))

uint16_t aom_read_primitive_refsubexpfin_(aom_reader *r, uint16_t n, uint16_t k,
                                          uint16_t ref ACCT_STR_PARAM);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AOM_DSP_BINARY_CODES_READER_H_
