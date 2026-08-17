/*
 * Header file for hardcoded motion pixels RGB to YUV table
 *
 * Copyright (c) 2009 Reimar Döffinger <Reimar.Doeffinger@gmx.de>
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

#ifndef AVCODEC_MOTIONPIXELS_TABLEGEN_H
#define AVCODEC_MOTIONPIXELS_TABLEGEN_H

#include <stdint.h>
#include "libavutil/attributes.h"

typedef struct YuvPixel {
    int8_t y, v, u;
} YuvPixel;

static int mp_yuv_to_rgb(int y, int v, int u, int clip_rgb) {
    const uint8_t *cm = ff_crop_tab + MAX_NEG_CROP;
    int r, g, b;

    r = (1000 * y + 701 * v) / 1000;
    g = (1000 * y - 357 * v - 172 * u) / 1000;
    b = (1000 * y + 886 * u) / 1000;
    if (clip_rgb)
        return ((cm[r * 8] & 0xF8) << 7) | ((cm[g * 8] & 0xF8) << 2) | (cm[b * 8] >> 3);
    if ((unsigned)r < 32 && (unsigned)g < 32 && (unsigned)b < 32)
        return (r << 10) | (g << 5) | b;
    return 1 << 15;
}

#if CONFIG_HARDCODED_TABLES
#define motionpixels_tableinit()
#include "libavcodec/motionpixels_tables.h"
#else
static YuvPixel mp_rgb_yuv_table[1 << 15];

static av_cold void mp_set_zero_yuv(YuvPixel *p)
{
    int i, j;

    for (i = 0; i < 31; ++i) {
        for (j = 31; j > i; --j)
            if (!(p[j].u | p[j].v | p[j].y))
                p[j] = p[j - 1];
        for (j = 0; j < 31 - i; ++j)
            if (!(p[j].u | p[j].v | p[j].y))
                p[j] = p[j + 1];
    }
}

static av_cold void mp_build_rgb_yuv_table(YuvPixel *p)
{
    int y, v, u, i;

    for (y = 0; y <= 31; ++y)
        for (v = -31; v <= 31; ++v)
            for (u = -31; u <= 31; ++u) {
                i = mp_yuv_to_rgb(y, v, u, 0);
                if (i < (1 << 15) && !(p[i].u | p[i].v | p[i].y)) {
                    p[i].y = y;
                    p[i].v = v;
                    p[i].u = u;
                }
            }
    for (i = 0; i < 1024; ++i)
        mp_set_zero_yuv(p + i * 32);
}

static av_cold void motionpixels_tableinit(void)
{
    mp_build_rgb_yuv_table(mp_rgb_yuv_table);
}
#endif /* CONFIG_HARDCODED_TABLES */

#endif /* AVCODEC_MOTIONPIXELS_TABLEGEN_H */
