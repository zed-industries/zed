/*
 * RealVideo 3 decoder
 * copyright (c) 2007 Konstantin Shishkov
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
 * miscellaneous RV30 tables
 */

#ifndef AVCODEC_RV30DATA_H
#define AVCODEC_RV30DATA_H

#include <stdint.h>

/** DC quantizer mapping for RV30 */
static const uint8_t rv30_luma_dc_quant[32] = {
     0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 22, 22, 23, 23, 23, 24, 24, 25, 25
};

/**
 * This table is used for storing the differences
 * between the predicted and the real intra type.
 */
static const uint8_t rv30_itype_code[9*9*2] = {
    0, 0, 0, 1, 1, 0, 1, 1, 0, 2, 2, 0, 0, 3, 3, 0, 1, 2,
    2, 1, 0, 4, 4, 0, 3, 1, 1, 3, 0, 5, 5, 0, 2, 2, 1, 4,
    4, 1, 0, 6, 3, 2, 1, 5, 2, 3, 5, 1, 6, 0, 0, 7, 4, 2,
    2, 4, 3, 3, 6, 1, 1, 6, 7, 0, 0, 8, 5, 2, 4, 3, 2, 5,
    3, 4, 1, 7, 4, 4, 7, 1, 8, 0, 6, 2, 3, 5, 5, 3, 2, 6,
    1, 8, 2, 7, 7, 2, 8, 1, 5, 4, 4, 5, 3, 6, 6, 3, 8, 2,
    4, 6, 5, 5, 6, 4, 2, 8, 7, 3, 3, 7, 6, 5, 5, 6, 7, 4,
    4, 7, 8, 3, 3, 8, 7, 5, 8, 4, 5, 7, 4, 8, 6, 6, 7, 6,
    5, 8, 8, 5, 6, 7, 8, 6, 7, 7, 6, 8, 8, 7, 7, 8, 8, 8,
};

/**
 * This table is used for retrieving the current intra type
 * based on its neighbors and adjustment provided by
 * code read and decoded before.
 *
 * This is really a three-dimensional matrix with dimensions
 * [-1..9][-1..9][0..9]. The first and second coordinates are
 * determined by the top and left neighbors (-1 if unavailable).
 */
static const uint8_t rv30_itype_from_context[900] = {
    0, 9, 9, 9, 9, 9, 9, 9, 9,
    0, 2, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9,
    2, 0, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9,

    0, 1, 9, 9, 9, 9, 9, 9, 9,
    0, 2, 1, 6, 4, 8, 5, 7, 3,
    1, 0, 2, 6, 5, 4, 3, 8, 7,
    2, 8, 0, 1, 7, 4, 3, 6, 5,
    2, 0, 1, 3, 8, 5, 4, 7, 6,
    2, 0, 1, 4, 6, 7, 8, 3, 5,
    0, 1, 5, 2, 6, 3, 8, 4, 7,
    0, 1, 6, 2, 4, 7, 5, 8, 3,
    2, 7, 0, 1, 4, 8, 6, 3, 5,
    2, 8, 0, 1, 7, 3, 4, 5, 6,

    1, 0, 9, 9, 9, 9, 9, 9, 9,
    1, 2, 5, 6, 3, 0, 4, 8, 7,
    1, 6, 2, 5, 3, 0, 4, 8, 7,
    2, 1, 7, 6, 8, 3, 5, 0, 4,
    1, 2, 5, 3, 6, 8, 4, 7, 0,
    1, 6, 2, 0, 4, 5, 8, 7, 3,
    1, 5, 2, 6, 3, 8, 4, 0, 7,
    1, 6, 0, 2, 4, 5, 7, 3, 8,
    2, 1, 7, 6, 0, 8, 5, 4, 3,
    1, 2, 7, 8, 3, 4, 5, 6, 0,

    9, 9, 9, 9, 9, 9, 9, 9, 9,
    0, 2, 1, 8, 7, 6, 5, 4, 3,
    1, 2, 0, 6, 5, 7, 4, 8, 3,
    2, 8, 7, 1, 0, 6, 4, 3, 5,
    2, 0, 8, 1, 3, 7, 5, 4, 6,
    2, 0, 4, 1, 7, 8, 6, 3, 5,
    2, 0, 1, 5, 8, 4, 6, 7, 3,
    2, 0, 6, 1, 4, 7, 8, 5, 3,
    2, 7, 8, 1, 0, 5, 4, 6, 3,
    2, 8, 7, 1, 0, 4, 3, 6, 5,

    9, 9, 9, 9, 9, 9, 9, 9, 9,
    0, 2, 1, 3, 5, 8, 6, 4, 7,
    1, 0, 2, 5, 3, 6, 4, 8, 7,
    2, 8, 1, 0, 3, 5, 7, 6, 4,
    3, 2, 5, 8, 1, 4, 6, 7, 0,
    4, 2, 0, 6, 1, 5, 8, 3, 7,
    5, 3, 1, 2, 8, 6, 4, 0, 7,
    1, 6, 0, 2, 4, 5, 8, 3, 7,
    2, 7, 0, 1, 5, 4, 8, 6, 3,
    2, 8, 3, 5, 1, 0, 7, 6, 4,

    9, 9, 9, 9, 9, 9, 9, 9, 9,
    2, 0, 6, 1, 4, 7, 5, 8, 3,
    1, 6, 2, 0, 4, 5, 3, 7, 8,
    2, 8, 7, 6, 4, 0, 1, 5, 3,
    4, 2, 1, 0, 6, 8, 3, 5, 7,
    4, 2, 6, 0, 1, 5, 7, 8, 3,
    1, 2, 5, 0, 6, 3, 4, 7, 8,
    6, 4, 0, 1, 2, 7, 5, 3, 8,
    2, 7, 4, 6, 0, 1, 8, 5, 3,
    2, 8, 7, 4, 6, 1, 3, 5, 0,

    9, 9, 9, 9, 9, 9, 9, 9, 9,
    5, 1, 2, 3, 6, 8, 0, 4, 7,
    1, 5, 6, 3, 2, 0, 4, 8, 7,
    2, 1, 5, 3, 6, 8, 7, 4, 0,
    5, 3, 1, 2, 6, 8, 4, 7, 0,
    1, 6, 2, 4, 5, 8, 0, 3, 7,
    5, 1, 3, 6, 2, 0, 8, 4, 7,
    1, 6, 5, 2, 0, 4, 3, 7, 8,
    2, 7, 1, 6, 5, 0, 8, 3, 4,
    2, 5, 1, 3, 6, 8, 4, 0, 7,

    9, 9, 9, 9, 9, 9, 9, 9, 9,
    1, 6, 2, 0, 5, 4, 3, 7, 8,
    1, 6, 5, 4, 2, 3, 0, 7, 8,
    2, 1, 6, 7, 4, 8, 5, 3, 0,
    2, 1, 6, 5, 8, 4, 3, 0, 7,
    6, 4, 1, 2, 0, 5, 7, 8, 3,
    1, 6, 5, 2, 3, 0, 4, 8, 7,
    6, 1, 4, 0, 2, 7, 5, 3, 8,
    2, 7, 4, 6, 1, 5, 0, 8, 3,
    2, 1, 6, 8, 4, 7, 3, 5, 0,

    9, 9, 9, 9, 9, 9, 9, 9, 9,
    2, 0, 4, 7, 6, 1, 8, 5, 3,
    6, 1, 2, 0, 4, 7, 5, 8, 3,
    2, 7, 8, 0, 1, 6, 4, 3, 5,
    2, 4, 0, 8, 3, 1, 7, 6, 5,
    4, 2, 7, 0, 6, 1, 8, 5, 3,
    2, 1, 0, 8, 5, 6, 7, 4, 3,
    2, 6, 4, 1, 7, 0, 5, 8, 3,
    2, 7, 4, 0, 8, 6, 1, 5, 3,
    2, 8, 7, 4, 1, 0, 3, 6, 5,

    9, 9, 9, 9, 9, 9, 9, 9, 9,
    2, 0, 8, 1, 3, 4, 6, 5, 7,
    1, 2, 0, 6, 8, 5, 7, 3, 4,
    2, 8, 7, 1, 0, 3, 6, 5, 4,
    8, 3, 2, 5, 1, 0, 4, 7, 6,
    2, 0, 4, 8, 5, 1, 7, 6, 3,
    2, 1, 0, 8, 5, 3, 6, 4, 7,
    2, 1, 6, 0, 8, 4, 5, 7, 3,
    2, 7, 8, 4, 0, 6, 1, 5, 3,
    2, 8, 3, 0, 7, 4, 1, 6, 5,
};

/**
 * Loop filter limits are taken from this table.
 */
static const uint8_t rv30_loop_filt_lim[32] = {
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5
};
#endif /* AVCODEC_RV30DATA_H */
