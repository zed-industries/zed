/*
 * Shared functions between AMR codecs
 *
 * Copyright (c) 2010 Marcelo Galvao Povoa
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef AVCODEC_AMR_H
#define AVCODEC_AMR_H

#include <string.h>

#include "avcodec.h"

#ifdef AMR_USE_16BIT_TABLES
typedef uint16_t R_TABLE_TYPE;
#else
typedef uint8_t R_TABLE_TYPE;
#endif

/**
 * Fill the frame structure variables from bitstream by parsing the
 * given reordering table that uses the following format:
 *
 * Each field (16 bits) in the AMR Frame is stored as:
 * - one byte for the number of bits in the field
 * - one byte for the field index
 * - then, one byte for each bit of the field (from most-significant to least)
 *         of the position of that bit in the AMR frame.
 *
 * @param out pointer to the frame struct
 * @param size the size in bytes of the frame struct
 * @param data input bitstream after the frame header
 * @param ord_table the reordering table as above
 */
static inline void ff_amr_bit_reorder(uint16_t *out, int size,
                                      const uint8_t *data,
                                      const R_TABLE_TYPE *ord_table)
{
    int field_size;

    memset(out, 0, size);
    while ((field_size = *ord_table++)) {
        int field = 0;
        int field_offset = *ord_table++;
        while (field_size--) {
           int bit = *ord_table++;
           field <<= 1;
           field |= data[bit >> 3] >> (bit & 7) & 1;
        }
        out[field_offset >> 1] = field;
    }
}

#endif /* AVCODEC_AMR_H */
