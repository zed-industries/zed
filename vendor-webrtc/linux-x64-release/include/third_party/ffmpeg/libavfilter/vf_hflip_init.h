/*
 * Copyright (c) 2007 Benoit Fouet
 * Copyright (c) 2010 Stefano Sabatini
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

#ifndef AVFILTER_HFLIP_INIT_H
#define AVFILTER_HFLIP_INIT_H

#include <stdint.h>

#include "config.h"
#include "libavutil/attributes.h"
#include "libavutil/error.h"
#include "libavutil/intreadwrite.h"
#include "hflip.h"

static void hflip_byte_c(const uint8_t *src, uint8_t *dst, int w)
{
    for (int j = 0; j < w; j++)
        dst[j] = src[-j];
}

static void hflip_short_c(const uint8_t *ssrc, uint8_t *ddst, int w)
{
    const uint16_t *src = (const uint16_t *)ssrc;
    uint16_t *dst = (uint16_t *)ddst;

    for (int j = 0; j < w; j++)
        dst[j] = src[-j];
}

static void hflip_dword_c(const uint8_t *ssrc, uint8_t *ddst, int w)
{
    const uint32_t *src = (const uint32_t *)ssrc;
    uint32_t *dst = (uint32_t *)ddst;

    for (int j = 0; j < w; j++)
        dst[j] = src[-j];
}

static void hflip_b24_c(const uint8_t *src, uint8_t *dst, int w)
{
    const uint8_t *in  = src;
    uint8_t *out = dst;

    for (int j = 0; j < w; j++, out += 3, in -= 3) {
        int32_t v = AV_RB24(in);

        AV_WB24(out, v);
    }
}

static void hflip_b48_c(const uint8_t *src, uint8_t *dst, int w)
{
    const uint8_t *in  = src;
    uint8_t *out = dst;

    for (int j = 0; j < w; j++, out += 6, in -= 6) {
        int64_t v = AV_RB48(in);

        AV_WB48(out, v);
    }
}

static void hflip_qword_c(const uint8_t *ssrc, uint8_t *ddst, int w)
{
    const uint64_t *src = (const uint64_t *)ssrc;
    uint64_t *dst = (uint64_t *)ddst;

    for (int j = 0; j < w; j++)
        dst[j] = src[-j];
}

static av_unused int ff_hflip_init(FlipContext *s, int step[4], int nb_planes)
{
    for (int i = 0; i < nb_planes; i++) {
        step[i] *= s->bayer_plus1;
        switch (step[i]) {
        case 1: s->flip_line[i] = hflip_byte_c;  break;
        case 2: s->flip_line[i] = hflip_short_c; break;
        case 3: s->flip_line[i] = hflip_b24_c;   break;
        case 4: s->flip_line[i] = hflip_dword_c; break;
        case 6: s->flip_line[i] = hflip_b48_c;   break;
        case 8: s->flip_line[i] = hflip_qword_c; break;
        default:
            return AVERROR_BUG;
        }
    }
#if ARCH_X86
    ff_hflip_init_x86(s, step, nb_planes);
#endif

    return 0;
}

#endif /* AVFILTER_HFLIP_INIT_H */
