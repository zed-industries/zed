/*
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

/**
 * @file
 * leb128 handling implementations
 */

#ifndef AVCODEC_LEB_H
#define AVCODEC_LEB_H

#include "get_bits.h"

/**
 * Read a unsigned integer coded as a variable number of up to eight
 * little-endian bytes, where the MSB in a byte signals another byte
 * must be read.
 * All coded bits are read, but values > UINT_MAX are truncated.
 */
static inline unsigned get_leb(GetBitContext *s) {
    int more, i = 0;
    unsigned leb = 0;

    do {
        int byte = get_bits(s, 8);
        unsigned bits = byte & 0x7f;
        more = byte & 0x80;
        if (i <= 4)
            leb |= bits << (i * 7);
        if (++i == 8)
            break;
    } while (more);

    return leb;
}

/**
 * Read a unsigned integer coded as a variable number of up to eight
 * little-endian bytes, where the MSB in a byte signals another byte
 * must be read.
 */
static inline int64_t get_leb128(GetBitContext *gb) {
    int64_t ret = 0;

    for (int i = 0; i < 8; i++) {
        int byte = get_bits(gb, 8);
        ret |= (int64_t)(byte & 0x7f) << (i * 7);
        if (!(byte & 0x80))
            break;
    }

    return ret;
}

#endif /* AVCODEC_LEB_H */
