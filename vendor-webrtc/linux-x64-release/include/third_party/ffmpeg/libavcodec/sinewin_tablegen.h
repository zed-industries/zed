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

#ifndef AVCODEC_SINEWIN_TABLEGEN_H
#define AVCODEC_SINEWIN_TABLEGEN_H

#include <assert.h>
// do not use libavutil/libm.h since this is compiled both
// for the host and the target and config.h is only valid for the target
#include <math.h>
#include "libavutil/attributes.h"
#include "libavutil/common.h"

#if !CONFIG_HARDCODED_TABLES
#ifndef BUILD_TABLES
#include "libavutil/thread.h"
#endif

SINETABLE(  32);
SINETABLE(  64);
SINETABLE( 128);
SINETABLE( 256);
SINETABLE( 512);
SINETABLE(1024);
SINETABLE(2048);
SINETABLE(4096);
SINETABLE(8192);
#else
#include "libavcodec/sinewin_tables.h"
#endif

SINETABLE_CONST float *const ff_sine_windows[] = {
    NULL, NULL, NULL, NULL, NULL, // unused
    ff_sine_32,   ff_sine_64,   ff_sine_128,
    ff_sine_256,  ff_sine_512,  ff_sine_1024,
    ff_sine_2048, ff_sine_4096, ff_sine_8192,
};

// Generate a sine window.
av_cold void ff_sine_window_init(float *window, int n)
{
    int i;
    for(i = 0; i < n; i++)
        window[i] = sinf((i + 0.5) * (M_PI / (2.0 * n)));
}

#if !CONFIG_HARDCODED_TABLES && !defined(BUILD_TABLES)
#define INIT_FF_SINE_WINDOW_INIT_FUNC(index)        \
static void init_ff_sine_window_ ## index(void)     \
{                                                   \
    ff_sine_window_init(ff_sine_windows[index], 1 << index);\
}

INIT_FF_SINE_WINDOW_INIT_FUNC(5)
INIT_FF_SINE_WINDOW_INIT_FUNC(6)
INIT_FF_SINE_WINDOW_INIT_FUNC(7)
INIT_FF_SINE_WINDOW_INIT_FUNC(8)
INIT_FF_SINE_WINDOW_INIT_FUNC(9)
INIT_FF_SINE_WINDOW_INIT_FUNC(10)
INIT_FF_SINE_WINDOW_INIT_FUNC(11)
INIT_FF_SINE_WINDOW_INIT_FUNC(12)
INIT_FF_SINE_WINDOW_INIT_FUNC(13)

static void (*const sine_window_init_func_array[])(void) = {
    init_ff_sine_window_5,
    init_ff_sine_window_6,
    init_ff_sine_window_7,
    init_ff_sine_window_8,
    init_ff_sine_window_9,
    init_ff_sine_window_10,
    init_ff_sine_window_11,
    init_ff_sine_window_12,
    init_ff_sine_window_13,
};

static AVOnce init_sine_window_once[9] = {
    AV_ONCE_INIT, AV_ONCE_INIT, AV_ONCE_INIT, AV_ONCE_INIT, AV_ONCE_INIT,
    AV_ONCE_INIT, AV_ONCE_INIT, AV_ONCE_INIT, AV_ONCE_INIT
};
#endif

av_cold void ff_init_ff_sine_windows(int index)
{
    assert(index >= 5 && index < FF_ARRAY_ELEMS(ff_sine_windows));
#if !CONFIG_HARDCODED_TABLES
#ifdef BUILD_TABLES
    ff_sine_window_init(ff_sine_windows[index], 1 << index);
#else
    ff_thread_once(&init_sine_window_once[index - 5], sine_window_init_func_array[index - 5]);
#endif
#endif
}

#endif /* AVCODEC_SINEWIN_TABLEGEN_H */
