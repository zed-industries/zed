/*
 * V210 decoder DSP init
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

#ifndef AVCODEC_V210DEC_INIT_H
#define AVCODEC_V210DEC_INIT_H

#include <stdint.h>

#include "config.h"
#include "libavutil/attributes.h"
#include "libavutil/bswap.h"
#include "v210dec.h"

#define READ_PIXELS(a, b, c)         \
    do {                             \
        val  = av_le2ne32(*src++);   \
        *a++ =  val & 0x3FF;         \
        *b++ = (val >> 10) & 0x3FF;  \
        *c++ = (val >> 20) & 0x3FF;  \
    } while (0)

static void v210_planar_unpack_c(const uint32_t *src, uint16_t *y, uint16_t *u, uint16_t *v, int width)
{
    uint32_t val;

    for (int i = 0; i < width - 5; i += 6) {
        READ_PIXELS(u, y, v);
        READ_PIXELS(y, u, y);
        READ_PIXELS(v, y, u);
        READ_PIXELS(y, v, y);
    }
}

static av_unused av_cold void ff_v210dec_init(V210DecContext *s)
{
    s->unpack_frame = v210_planar_unpack_c;
#if ARCH_X86
    ff_v210_x86_init(s);
#endif
}

#endif /* AVCODEC_V210DEC_INIT_H */
