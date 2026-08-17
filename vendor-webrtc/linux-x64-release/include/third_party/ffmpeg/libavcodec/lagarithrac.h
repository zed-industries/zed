/*
 * Lagarith range decoder
 * Copyright (c) 2009 Nathan Caldwell <saintdev (at) gmail.com>
 * Copyright (c) 2009 David Conrad
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

/**
 * @file
 * Lagarith range decoder
 * @author Nathan Caldwell
 * @author David Conrad
 */

#ifndef AVCODEC_LAGARITHRAC_H
#define AVCODEC_LAGARITHRAC_H

#include <stdint.h>
#include "libavutil/intreadwrite.h"
#include "get_bits.h"

typedef struct lag_rac {
    void    *logctx;
    unsigned low;
    unsigned range;
    unsigned scale;             /**< Number of bits of precision in range. */
    unsigned hash_shift;        /**< Number of bits to shift to calculate hash for radix search. */

    const uint8_t *bytestream_start;  /**< Start of input bytestream. */
    const uint8_t *bytestream;        /**< Current position in input bytestream. */
    const uint8_t *bytestream_end;    /**< End position of input bytestream. */

    int overread;
#define MAX_OVERREAD 4

    uint32_t prob[258];         /**< Table of cumulative probability for each symbol. */
    uint8_t  range_hash[1024];   /**< Hash table mapping upper byte to approximate symbol. */
} lag_rac;

void ff_lag_rac_init(lag_rac *l, GetBitContext *gb, int length);

/* TODO: Optimize */
static inline void lag_rac_refill(lag_rac *l)
{
    while (l->range <= 0x800000) {
        l->low   <<= 8;
        l->range <<= 8;
        l->low |= 0xff & (AV_RB16(l->bytestream) >> 1);
        if (l->bytestream < l->bytestream_end)
            l->bytestream++;
        else
            l->overread++;
    }
}

/**
 * Decode a single byte from the compressed plane described by *l.
 * @param l pointer to lag_rac for the current plane
 * @return next byte of decoded data
 */
static inline uint8_t lag_get_rac(lag_rac *l)
{
    unsigned range_scaled, low_scaled;
    int val;

    lag_rac_refill(l);

    range_scaled = l->range >> l->scale;

    if (l->low < range_scaled * l->prob[255]) {
        /* val = 0 is frequent enough to deserve a shortcut */
        if (l->low < range_scaled * l->prob[1]) {
            val = 0;
        } else {
            low_scaled = l->low / (range_scaled<<(l->hash_shift));

            val = l->range_hash[low_scaled];
            while (l->low >= range_scaled * l->prob[val + 1])
                val++;
        }

        l->range = range_scaled * (l->prob[val + 1] - l->prob[val]);
    } else {
        val = 255;
        l->range -= range_scaled * l->prob[255];
    }

    if (!l->range)
        l->range = 0x80;

    l->low -= range_scaled * l->prob[val];

    return val;
}


#endif /* AVCODEC_LAGARITHRAC_H */
