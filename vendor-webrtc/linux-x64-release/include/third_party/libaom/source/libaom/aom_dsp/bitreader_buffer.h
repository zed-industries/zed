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

#ifndef AOM_AOM_DSP_BITREADER_BUFFER_H_
#define AOM_AOM_DSP_BITREADER_BUFFER_H_

#include <limits.h>

#include "aom/aom_integer.h"
#include "config/aom_config.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef void (*aom_rb_error_handler)(void *data);

struct aom_read_bit_buffer {
  const uint8_t *bit_buffer;
  const uint8_t *bit_buffer_end;
  uint32_t bit_offset;

  void *error_handler_data;
  aom_rb_error_handler error_handler;
};

size_t aom_rb_bytes_read(const struct aom_read_bit_buffer *rb);

int aom_rb_read_bit(struct aom_read_bit_buffer *rb);

int aom_rb_read_literal(struct aom_read_bit_buffer *rb, int bits);

uint32_t aom_rb_read_uvlc(struct aom_read_bit_buffer *rb);

#if CONFIG_AV1_DECODER
uint32_t aom_rb_read_unsigned_literal(struct aom_read_bit_buffer *rb, int bits);

int aom_rb_read_inv_signed_literal(struct aom_read_bit_buffer *rb, int bits);

int16_t aom_rb_read_signed_primitive_refsubexpfin(
    struct aom_read_bit_buffer *rb, uint16_t n, uint16_t k, int16_t ref);
#endif  // CONFIG_AV1_DECODER

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AOM_DSP_BITREADER_BUFFER_H_
