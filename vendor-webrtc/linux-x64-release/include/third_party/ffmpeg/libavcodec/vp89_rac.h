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
 * Range decoder functions common to VP8 and VP9
 */

#ifndef AVCODEC_VP89_RAC_H
#define AVCODEC_VP89_RAC_H

#include <stdint.h>

#include "libavutil/attributes.h"

#include "vpx_rac.h"

// rounding is different than vpx_rac_get, is vpx_rac_get wrong?
static av_always_inline int vp89_rac_get(VPXRangeCoder *c)
{
    return vpx_rac_get_prob(c, 128);
}

static av_unused int vp89_rac_get_uint(VPXRangeCoder *c, int bits)
{
    int value = 0;

    while (bits--) {
        value = (value << 1) | vp89_rac_get(c);
    }

    return value;
}

// how probabilities are associated with decisions is different I think
// well, the new scheme fits in the old but this way has one fewer branches per decision
static av_always_inline int vp89_rac_get_tree(VPXRangeCoder *c, const int8_t (*tree)[2],
                                              const uint8_t *probs)
{
    int i = 0;

    do {
        i = tree[i][vpx_rac_get_prob(c, probs[i])];
    } while (i > 0);

    return -i;
}

#endif /* AVCODEC_VP89_RAC_H */
