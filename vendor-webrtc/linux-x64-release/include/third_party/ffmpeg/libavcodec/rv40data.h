/*
 * RealVideo 4 decoder
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
 * miscellaneous RV40 tables
 */

#ifndef AVCODEC_RV40DATA_H
#define AVCODEC_RV40DATA_H

#include <stdint.h>

/**
 * standard widths and heights coded in RV40
 */
//@{
static const int rv40_standard_widths[]   = { 160, 172, 240, 320, 352, 640, 704, 0};
static const int rv40_standard_heights[]  = { 120, 132, 144, 240, 288, 480, -8, -10, 180, 360, 576, 0};
//@}

#define MODE2_PATTERNS_NUM 20
/**
 * intra types table
 *
 * These values are actually coded 3-tuples
 * used for detecting standard block configurations.
 */
static const uint16_t rv40_aic_table_index[MODE2_PATTERNS_NUM] = {
 0x000, 0x100, 0x200,
 0x011, 0x111, 0x211, 0x511, 0x611,
 0x022, 0x122, 0x222, 0x722,
 0x272, 0x227,
 0x822, 0x282, 0x228,
 0x112, 0x116, 0x221
};

/**
 * luma quantizer values
 * The second table is used for inter blocks.
 */
static const uint8_t rv40_luma_dc_quant[2][32] = {
 {  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,
   16, 17, 17, 18, 18, 18, 19, 19, 19, 20, 20, 20, 22, 22, 22, 22 },
 {  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,
   16, 17, 18, 19, 20, 20, 21, 21, 22, 23, 23, 23, 24, 24, 24, 24 }
};

/**
 * @name Coefficients used by the RV40 loop filter
 * @{
 */

/** alpha parameter for RV40 loop filter - almost the same as in JVT-A003r1 */
static const uint8_t rv40_alpha_tab[32] = {
    128, 128, 128, 128, 128, 128, 128, 128,
    128, 128, 122,  96,  75,  59,  47,  37,
     29,  23,  18,  15,  13,  11,  10,   9,
      8,   7,   6,   5,   4,   3,   2,   1
};
/** beta parameter for RV40 loop filter - almost the same as in JVT-A003r1 */
static const uint8_t rv40_beta_tab[32] = {
     0,  0,  0,  0,  0,  0,  0,  0,  3,  3,  3,  4,  4,  4,  6,  6,
     6,  7,  8,  8,  9,  9, 10, 10, 11, 11, 12, 13, 14, 15, 16, 17
};
/** clip table for RV40 loop filter - the same as in JVT-A003r1 */
static const uint8_t rv40_filter_clip_tbl[3][32] = {
    {
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
    },
    {
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 5, 5
    },
    {
        0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1,
        1, 2, 2, 2, 2, 3, 3, 3, 4, 4, 5, 5, 5, 7, 8, 9
    }
};
/** @} */ // end loopfilter group

#endif /* AVCODEC_RV40DATA_H */
