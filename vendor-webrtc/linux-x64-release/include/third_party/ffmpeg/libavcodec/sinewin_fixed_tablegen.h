/*
 * Header file for hardcoded sine windows
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

#ifndef AVCODEC_SINEWIN_FIXED_TABLEGEN_H
#define AVCODEC_SINEWIN_FIXED_TABLEGEN_H

#ifdef BUILD_TABLES
#undef DECLARE_ALIGNED
#define DECLARE_ALIGNED(align, type, name) type name
#else
#include "libavutil/mem_internal.h"
#endif

#define SINETABLE(size) \
    static SINETABLE_CONST DECLARE_ALIGNED(32, int, sine_##size##_fixed)[size]

#if CONFIG_HARDCODED_TABLES
#define init_sine_windows_fixed()
#define SINETABLE_CONST const
#include "libavcodec/sinewin_fixed_tables.h"
#else
// do not use libavutil/libm.h since this is compiled both
// for the host and the target and config.h is only valid for the target
#include <math.h>
#include "libavutil/attributes.h"

#define SINETABLE_CONST
SINETABLE( 96);
SINETABLE( 120);
SINETABLE( 128);
SINETABLE( 480);
SINETABLE( 512);
SINETABLE( 768);
SINETABLE( 960);
SINETABLE(1024);

#define SIN_FIX(a) (int)floor((a) * 0x80000000 + 0.5)

// Generate a sine window.
static av_cold void sine_window_init_fixed(int *window, int n)
{
    for (int i = 0; i < n; i++)
        window[i] = SIN_FIX(sinf((i + 0.5) * (M_PI / (2.0 * n))));
}

static av_cold void init_sine_windows_fixed(void)
{
    sine_window_init_fixed(sine_96_fixed,   96);
    sine_window_init_fixed(sine_120_fixed,  120);
    sine_window_init_fixed(sine_128_fixed,  128);
    sine_window_init_fixed(sine_480_fixed,  480);
    sine_window_init_fixed(sine_512_fixed,  512);
    sine_window_init_fixed(sine_768_fixed,  768);
    sine_window_init_fixed(sine_960_fixed,  960);
    sine_window_init_fixed(sine_1024_fixed, 1024);
}
#endif /* CONFIG_HARDCODED_TABLES */
#endif /* AVCODEC_SINEWIN_FIXED_TABLEGEN_H */
