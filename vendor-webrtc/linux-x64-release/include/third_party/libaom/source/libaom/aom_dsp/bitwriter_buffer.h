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

#ifndef AOM_AOM_DSP_BITWRITER_BUFFER_H_
#define AOM_AOM_DSP_BITWRITER_BUFFER_H_

#include "aom/aom_integer.h"

#ifdef __cplusplus
extern "C" {
#endif

struct aom_write_bit_buffer {
  uint8_t *bit_buffer;
  uint32_t bit_offset;
};

int aom_wb_is_byte_aligned(const struct aom_write_bit_buffer *wb);

uint32_t aom_wb_bytes_written(const struct aom_write_bit_buffer *wb);

void aom_wb_write_bit(struct aom_write_bit_buffer *wb, int bit);

void aom_wb_write_literal(struct aom_write_bit_buffer *wb, int data, int bits);

void aom_wb_write_unsigned_literal(struct aom_write_bit_buffer *wb,
                                   uint32_t data, int bits);

void aom_wb_overwrite_literal(struct aom_write_bit_buffer *wb, int data,
                              int bits);

void aom_wb_write_inv_signed_literal(struct aom_write_bit_buffer *wb, int data,
                                     int bits);

void aom_wb_write_uvlc(struct aom_write_bit_buffer *wb, uint32_t v);

void aom_wb_write_signed_primitive_refsubexpfin(struct aom_write_bit_buffer *wb,
                                                uint16_t n, uint16_t k,
                                                int16_t ref, int16_t v);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AOM_DSP_BITWRITER_BUFFER_H_
