/*
 * Copyright (C) 2006  Aurelien Jacobs <aurel@gnuage.org>
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
 * Common VP5-VP9 range decoder stuff
 */

#ifndef AVCODEC_VPX_RAC_H
#define AVCODEC_VPX_RAC_H

#include <stdint.h>

#include "config.h"
#include "libavutil/attributes.h"
#include "bytestream.h"

typedef struct VPXRangeCoder {
    int high;
    int bits; /* stored negated (i.e. negative "bits" is a positive number of
                 bits left) in order to eliminate a negate in cache refilling */
    const uint8_t *buffer;
    const uint8_t *end;
    unsigned int code_word;
    int end_reached;
} VPXRangeCoder;

extern const uint8_t ff_vpx_norm_shift[256];
int ff_vpx_init_range_decoder(VPXRangeCoder *c, const uint8_t *buf, int buf_size);

/**
 * returns 1 if the end of the stream has been reached, 0 otherwise.
 */
static av_always_inline int vpx_rac_is_end(VPXRangeCoder *c)
{
    if (c->end <= c->buffer && c->bits >= 0)
        c->end_reached ++;
    return c->end_reached > 10;
}

static av_always_inline unsigned int vpx_rac_renorm(VPXRangeCoder *c)
{
    int shift = ff_vpx_norm_shift[c->high];
    int bits = c->bits;
    unsigned int code_word = c->code_word;

    c->high   <<= shift;
    code_word <<= shift;
    bits       += shift;
    if(bits >= 0 && c->buffer < c->end) {
        code_word |= bytestream_get_be16(&c->buffer) << bits;
        bits -= 16;
    }
    c->bits = bits;
    return code_word;
}

#if   ARCH_ARM
#include "arm/vpx_arith.h"
#elif ARCH_X86
#include "x86/vpx_arith.h"
#endif

#ifndef vpx_rac_get_prob
#define vpx_rac_get_prob vpx_rac_get_prob
static av_always_inline int vpx_rac_get_prob(VPXRangeCoder *c, uint8_t prob)
{
    unsigned int code_word = vpx_rac_renorm(c);
    unsigned int low = 1 + (((c->high - 1) * prob) >> 8);
    unsigned int low_shift = low << 16;
    int bit = code_word >= low_shift;

    c->high = bit ? c->high - low : low;
    c->code_word = bit ? code_word - low_shift : code_word;

    return bit;
}
#endif

#ifndef vpx_rac_get_prob_branchy
// branchy variant, to be used where there's a branch based on the bit decoded
static av_always_inline int vpx_rac_get_prob_branchy(VPXRangeCoder *c, int prob)
{
    unsigned long code_word = vpx_rac_renorm(c);
    unsigned low = 1 + (((c->high - 1) * prob) >> 8);
    unsigned low_shift = low << 16;

    if (code_word >= low_shift) {
        c->high     -= low;
        c->code_word = code_word - low_shift;
        return 1;
    }

    c->high = low;
    c->code_word = code_word;
    return 0;
}
#endif

static av_always_inline int vpx_rac_get(VPXRangeCoder *c)
{
    unsigned int code_word = vpx_rac_renorm(c);
    /* equiprobable */
    int low = (c->high + 1) >> 1;
    unsigned int low_shift = low << 16;
    int bit = code_word >= low_shift;
    if (bit) {
        c->high   -= low;
        code_word -= low_shift;
    } else {
        c->high = low;
    }

    c->code_word = code_word;
    return bit;
}

#endif /* AVCODEC_VPX_RAC_H */
