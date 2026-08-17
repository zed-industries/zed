/*
 * ATRAC3 compatible decoder data
 * Copyright (c) 2006-2007 Maxim Poliakovski
 * Copyright (c) 2006-2007 Benjamin Larsson
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
 * ATRAC3 AKA RealAudio 8 compatible decoder data
 */

#ifndef AVCODEC_ATRAC3DATA_H
#define AVCODEC_ATRAC3DATA_H

#include <stdint.h>

/* VLC tables */

static const uint8_t atrac3_hufftabs[][2] = {
    /* Spectral coefficient 1 - 9 entries */
    { 31, 1 }, { 32, 3 }, { 33, 3 }, { 34, 4 }, { 35, 4 },
    { 36, 5 }, { 37, 5 }, { 38, 5 }, { 39, 5 },
    /* Spectral coefficient 2 - 5 entries */
    { 31, 1 }, { 32, 3 }, { 30, 3 }, { 33, 3 }, { 29, 3 },
    /* Spectral coefficient 3 - 7 entries */
    { 31, 1 }, { 32, 3 }, { 30, 3 }, { 33, 4 },
    { 29, 4 }, { 34, 4 }, { 28, 4 },
    /* Spectral coefficient 4 - 9 entries */
    { 31, 1 }, { 32, 3 }, { 30, 3 }, { 33, 4 }, { 29, 4 },
    { 34, 5 }, { 28, 5 }, { 35, 5 }, { 27, 5 },
    /* Spectral coefficient 5 - 15 entries */
    { 31, 2 }, { 32, 3 }, { 30, 3 }, { 33, 4 }, { 29, 4 },
    { 34, 4 }, { 28, 4 }, { 38, 4 }, { 24, 4 }, { 35, 5 },
    { 27, 5 }, { 36, 6 }, { 26, 6 }, { 37, 6 }, { 25, 6 },
    /* Spectral coefficient 6 - 31 entries */
    { 31, 3 }, { 32, 4 }, { 30, 4 }, { 33, 4 }, { 29, 4 }, { 34, 4 },
    { 28, 4 }, { 46, 4 }, { 16, 4 }, { 35, 5 }, { 27, 5 }, { 36, 5 },
    { 26, 5 }, { 37, 5 }, { 25, 5 }, { 38, 6 }, { 24, 6 }, { 39, 6 },
    { 23, 6 }, { 40, 6 }, { 22, 6 }, { 41, 6 }, { 21, 6 }, { 42, 7 },
    { 20, 7 }, { 43, 7 }, { 19, 7 }, { 44, 7 }, { 18, 7 }, { 45, 7 },
    { 17, 7 },
    /* Spectral coefficient 7 - 63 entries */
    { 31, 3 }, { 62, 4 }, {  0, 4 }, { 32, 5 }, { 30, 5 }, { 33, 5 },
    { 29, 5 }, { 34, 5 }, { 28, 5 }, { 35, 5 }, { 27, 5 }, { 36, 5 },
    { 26, 5 }, { 37, 6 }, { 25, 6 }, { 38, 6 }, { 24, 6 }, { 39, 6 },
    { 23, 6 }, { 40, 6 }, { 22, 6 }, { 41, 6 }, { 21, 6 }, { 42, 6 },
    { 20, 6 }, { 43, 6 }, { 19, 6 }, { 44, 6 }, { 18, 6 }, { 45, 7 },
    { 17, 7 }, { 46, 7 }, { 16, 7 }, { 47, 7 }, { 15, 7 }, { 48, 7 },
    { 14, 7 }, { 49, 7 }, { 13, 7 }, { 50, 7 }, { 12, 7 }, { 51, 7 },
    { 11, 7 }, { 52, 8 }, { 10, 8 }, { 53, 8 }, {  9, 8 }, { 54, 8 },
    {  8, 8 }, { 55, 8 }, {  7, 8 }, { 56, 8 }, {  6, 8 }, { 57, 8 },
    {  5, 8 }, { 58, 8 }, {  4, 8 }, { 59, 8 }, {  3, 8 }, { 60, 8 },
    {  2, 8 }, { 61, 8 }, {  1, 8 },
};

static const uint8_t huff_tab_sizes[7] = {
    9, 5, 7, 9, 15, 31, 63,
};

/* selector tables */

static const uint8_t clc_length_tab[8] = { 0, 4, 3, 3, 4, 4, 5, 6 };

static const int8_t mantissa_clc_tab[4] = { 0, 1, -2, -1 };

static const int8_t mantissa_vlc_tab[18] = {
    0, 0,  0, 1,  0, -1,  1, 0,  -1, 0,  1, 1,  1, -1,  -1, 1,  -1, -1
};


/* tables for the scalefactor decoding */

static const float inv_max_quant[8] = {
      0.0,       1.0 / 1.5, 1.0 /  2.5, 1.0 /  3.5,
      1.0 / 4.5, 1.0 / 7.5, 1.0 / 15.5, 1.0 / 31.5
};

static const uint16_t subband_tab[33] = {
      0,   8,  16,  24,  32,  40,  48,  56,
     64,  80,  96, 112, 128, 144, 160, 176,
    192, 224, 256, 288, 320, 352, 384, 416,
    448, 480, 512, 576, 640, 704, 768, 896,
    1024
};

/* joint stereo related tables */
static const float matrix_coeffs[8] = {
    0.0, 2.0, 2.0, 2.0, 0.0, 0.0, 1.0, 1.0
};

#endif /* AVCODEC_ATRAC3DATA_H */
