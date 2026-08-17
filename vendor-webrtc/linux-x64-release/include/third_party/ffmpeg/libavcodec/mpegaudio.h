/*
 * copyright (c) 2001 Fabrice Bellard
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
 * mpeg audio declarations for both encoder and decoder.
 */

#ifndef AVCODEC_MPEGAUDIO_H
#define AVCODEC_MPEGAUDIO_H

#ifndef USE_FLOATS
#   define USE_FLOATS 0
#endif

#include <stdint.h>
#include "libavutil/internal.h"

/* max frame size, in samples */
#define MPA_FRAME_SIZE 1152

/* max compressed frame size */
#define MPA_MAX_CODED_FRAME_SIZE 1792

#define MPA_MAX_CHANNELS 2

#define SBLIMIT 32 /* number of subbands */

#define MPA_STEREO  0
#define MPA_JSTEREO 1
#define MPA_DUAL    2
#define MPA_MONO    3

#ifndef FRAC_BITS
#define FRAC_BITS   23   /* fractional bits for sb_samples and dct */
#define WFRAC_BITS  16   /* fractional bits for window */
#endif

#define IMDCT_SCALAR 1.759

#define FRAC_ONE    (1 << FRAC_BITS)

#define FIX(a)   ((int)((a) * FRAC_ONE))

#if USE_FLOATS
#   define INTFLOAT float
#   define SUINTFLOAT float
typedef float MPA_INT;
typedef float OUT_INT;
#elif FRAC_BITS <= 15
#   define INTFLOAT int
#   define SUINTFLOAT SUINT
typedef int16_t MPA_INT;
typedef int16_t OUT_INT;
#else
#   define INTFLOAT int
#   define SUINTFLOAT SUINT
typedef int32_t MPA_INT;
typedef int16_t OUT_INT;
#endif

int ff_mpa_l2_select_table(int bitrate, int nb_channels, int freq, int lsf);

#endif /* AVCODEC_MPEGAUDIO_H */
