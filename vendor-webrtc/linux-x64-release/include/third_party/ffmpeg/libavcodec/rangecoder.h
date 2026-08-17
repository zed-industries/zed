/*
 * Range coder
 * Copyright (c) 2004 Michael Niedermayer <michaelni@gmx.at>
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
 * Range coder.
 */

#ifndef AVCODEC_RANGECODER_H
#define AVCODEC_RANGECODER_H

#include <stdint.h>

#include "libavutil/avassert.h"
#include "libavutil/intmath.h"

typedef struct RangeCoder {
    int low;
    int range;
    int outstanding_count;
    int outstanding_byte;
    uint8_t zero_state[256];
    uint8_t one_state[256];
    uint8_t *bytestream_start;
    uint8_t *bytestream;
    uint8_t *bytestream_end;
    int overread;
#define MAX_OVERREAD 2
} RangeCoder;

void ff_init_range_encoder(RangeCoder *c, uint8_t *buf, int buf_size);
void ff_init_range_decoder(RangeCoder *c, const uint8_t *buf, int buf_size);

/**
 * Terminates the range coder
 * @param version version 0 requires the decoder to know the data size in bytes
 *                version 1 needs about 1 bit more space but does not need to
 *                          carry the size from encoder to decoder
 */
int ff_rac_terminate(RangeCoder *c, int version);

void ff_build_rac_states(RangeCoder *c, int factor, int max_p);

static inline void renorm_encoder(RangeCoder *c)
{
        if (c->low - 0xFF01 >= 0x10000 - 0xFF01U) {
            int mask = c->low - 0xFF01 >> 31;
            *c->bytestream = c->outstanding_byte + 1 + mask;
            c->bytestream += c->outstanding_byte >= 0;
            for (; c->outstanding_count; c->outstanding_count--)
                *c->bytestream++ = mask;
            c->outstanding_byte = c->low >> 8;
        } else {
            c->outstanding_count++;
        }

        c->low     = (c->low & 0xFF) << 8;
        c->range <<= 8;
}

static inline int get_rac_count(RangeCoder *c)
{
    int x = c->bytestream - c->bytestream_start + c->outstanding_count;
    if (c->outstanding_byte >= 0)
        x++;
    return 8 * x - av_log2(c->range);
}

static inline void put_rac(RangeCoder *c, uint8_t *const state, int bit)
{
    int range1 = (c->range * (*state)) >> 8;

    av_assert2(*state);
    av_assert2(range1 < c->range);
    av_assert2(range1 > 0);
    if (!bit) {
        c->range -= range1;
        *state    = c->zero_state[*state];
    } else {
        c->low  += c->range - range1;
        c->range = range1;
        *state   = c->one_state[*state];
    }

    if (c->range < 0x100)
        renorm_encoder(c);
}

static inline void refill(RangeCoder *c)
{
        c->range <<= 8;
        c->low   <<= 8;
        if (c->bytestream < c->bytestream_end) {
            c->low += c->bytestream[0];
            c->bytestream++;
        } else
            c->overread ++;
}

static inline int get_rac(RangeCoder *c, uint8_t *const state)
{
    int range1 = (c->range * (*state)) >> 8;

    c->range -= range1;
    if (c->low < c->range) {
        *state = c->zero_state[*state];
        if (c->range < 0x100)
            refill(c);
        return 0;
    } else {
        c->low  -= c->range;
        *state   = c->one_state[*state];
        c->range = range1;
        if (c->range < 0x100)
            refill(c);
        return 1;
    }
}

#endif /* AVCODEC_RANGECODER_H */
