/*
 * V210 encoder DSP init
 *
 * Copyright (C) 2009 Michael Niedermayer <michaelni@gmx.at>
 * Copyright (c) 2009 Baptiste Coudurier <baptiste dot coudurier at gmail dot com>
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

#ifndef AVCODEC_V210ENC_INIT_H
#define AVCODEC_V210ENC_INIT_H

#include <stddef.h>
#include <stdint.h>

#include "config.h"
#include "libavutil/attributes.h"
#include "libavutil/common.h"
#include "libavutil/intreadwrite.h"
#include "v210enc.h"

#define CLIP(v, depth) av_clip(v, 1<<(depth-8), ((1<<depth)-(1<<(depth-8))-1))
#define WRITE_PIXELS(a, b, c, depth)                      \
    do {                                                  \
        val  =  CLIP(*a++, depth)  << (10-depth);         \
        val |=  (CLIP(*b++, depth) << (20-depth)) |       \
                (CLIP(*c++, depth) << (30-depth));        \
        AV_WL32(dst, val);                                \
        dst += 4;                                         \
    } while (0)

static void v210_planar_pack_8_c(const uint8_t *y, const uint8_t *u,
                                 const uint8_t *v, uint8_t *dst,
                                 ptrdiff_t width)
{
    uint32_t val;

    /* unroll this to match the assembly */
    for (int i = 0; i < width - 11; i += 12) {
        WRITE_PIXELS(u, y, v, 8);
        WRITE_PIXELS(y, u, y, 8);
        WRITE_PIXELS(v, y, u, 8);
        WRITE_PIXELS(y, v, y, 8);
        WRITE_PIXELS(u, y, v, 8);
        WRITE_PIXELS(y, u, y, 8);
        WRITE_PIXELS(v, y, u, 8);
        WRITE_PIXELS(y, v, y, 8);
    }
}

static void v210_planar_pack_10_c(const uint16_t *y, const uint16_t *u,
                                  const uint16_t *v, uint8_t *dst,
                                  ptrdiff_t width)
{
    uint32_t val;

    for (int i = 0; i < width - 5; i += 6) {
        WRITE_PIXELS(u, y, v, 10);
        WRITE_PIXELS(y, u, y, 10);
        WRITE_PIXELS(v, y, u, 10);
        WRITE_PIXELS(y, v, y, 10);
    }
}

static av_cold av_unused void ff_v210enc_init(V210EncContext *s)
{
    s->pack_line_8  = v210_planar_pack_8_c;
    s->pack_line_10 = v210_planar_pack_10_c;
    s->sample_factor_8  = 2;
    s->sample_factor_10 = 1;

#if ARCH_X86
    ff_v210enc_init_x86(s);
#endif
}

#endif /* AVCODEC_V210ENC_INIT_H */
