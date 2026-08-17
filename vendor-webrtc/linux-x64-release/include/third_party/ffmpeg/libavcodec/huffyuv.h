/*
 * Copyright (c) 2002-2014 Michael Niedermayer <michaelni@gmx.at>
 *
 * see https://multimedia.cx/huffyuv.txt for a description of
 * the algorithm used
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
 * huffyuv codec for libavcodec.
 */

#ifndef AVCODEC_HUFFYUV_H
#define AVCODEC_HUFFYUV_H

#include <stdint.h>

#include "config.h"

#if HAVE_BIGENDIAN
#define B 3
#define G 2
#define R 1
#define A 0
#else
#define B 0
#define G 1
#define R 2
#define A 3
#endif

#define MAX_BITS 16
#define MAX_N (1<<MAX_BITS)
#define MAX_VLC_N 16384

typedef enum Predictor {
    LEFT = 0,
    PLANE,
    MEDIAN,
} Predictor;

int ff_huffyuv_generate_bits_table(uint32_t *dst, const uint8_t *len_table, int n);

#endif /* AVCODEC_HUFFYUV_H */
