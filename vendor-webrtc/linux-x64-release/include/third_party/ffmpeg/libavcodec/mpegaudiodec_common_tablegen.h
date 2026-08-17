/*
 * Header file for hardcoded shared mpegaudiodec tables
 *
 * Copyright (c) 2009 Reimar Döffinger <Reimar.Doeffinger@gmx.de>
 * Copyright (c) 2020 Andreas Rheinhardt <andreas.rheinhardt@gmail.com>
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

#ifndef AVCODEC_MPEGAUDIODEC_COMMON_TABLEGEN_H
#define AVCODEC_MPEGAUDIODEC_COMMON_TABLEGEN_H

#include <stdint.h>

#define TABLE_4_3_SIZE ((8191 + 16)*4)

#if CONFIG_HARDCODED_TABLES
#define mpegaudiodec_common_tableinit()
#include "libavcodec/mpegaudiodec_common_tables.h"
#else
#include <math.h>
#include "libavutil/attributes.h"

int8_t   ff_table_4_3_exp  [TABLE_4_3_SIZE];
uint32_t ff_table_4_3_value[TABLE_4_3_SIZE];

#define FRAC_BITS 23
#define IMDCT_SCALAR 1.759

static av_cold void mpegaudiodec_common_tableinit(void)
{
    static const double exp2_lut[4] = {
        1.00000000000000000000, /* 2 ^ (0 * 0.25) */
        1.18920711500272106672, /* 2 ^ (1 * 0.25) */
        M_SQRT2               , /* 2 ^ (2 * 0.25) */
        1.68179283050742908606, /* 2 ^ (3 * 0.25) */
    };
    double pow43_val = 0;

    for (int i = 1; i < TABLE_4_3_SIZE; i++) {
        double f, fm;
        int e, m;
        double value = i / 4;
        if ((i & 3) == 0)
            pow43_val = value / IMDCT_SCALAR * cbrt(value);
        f  = pow43_val * exp2_lut[i & 3];
        fm = frexp(f, &e);
        m  = llrint(fm * (1LL << 31));
        e += FRAC_BITS - 31 + 5 - 100;

        /* normalized to FRAC_BITS */
        ff_table_4_3_value[i] =  m;
        ff_table_4_3_exp  [i] = -e;
    }
}

#endif /* CONFIG_HARDCODED_TABLES */
#endif /* AVCODEC_MPEGAUDIODEC_COMMON_TABLEGEN_H */
