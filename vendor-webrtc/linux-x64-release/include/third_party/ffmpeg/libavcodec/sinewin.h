/*
 * Copyright (c) 2008 Robert Swain
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

#ifndef AVCODEC_SINEWIN_H
#define AVCODEC_SINEWIN_H

#include "config.h"
#include "libavutil/mem_internal.h"

#if CONFIG_HARDCODED_TABLES
#   define SINETABLE_CONST const
#else
#   define SINETABLE_CONST
#endif

#define SINETABLE(size) \
    SINETABLE_CONST DECLARE_ALIGNED(32, float, ff_sine_##size)[size]

/**
 * Generate a sine window.
 * @param   window  pointer to half window
 * @param   n       size of half window
 */
void ff_sine_window_init(float *window, int n);

/**
 * initialize the specified entry of ff_sine_windows
 */
void ff_init_ff_sine_windows(int index);

extern SINETABLE(  32);
extern SINETABLE(  64);
extern SINETABLE( 128);
extern SINETABLE( 256);
extern SINETABLE( 512);
extern SINETABLE(1024);
extern SINETABLE(2048);
extern SINETABLE(4096);
extern SINETABLE(8192);

extern SINETABLE_CONST float *const ff_sine_windows[];

#endif /* AVCODEC_SINEWIN_H */
