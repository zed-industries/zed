/*
 * thirdpel DSP functions
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
 * thirdpel DSP functions
 */

#ifndef AVCODEC_TPELDSP_H
#define AVCODEC_TPELDSP_H

#include <stdint.h>

/* add and put pixel (decoding) */
// blocksizes for hpel_pixels_func are 8x4,8x8 16x8 16x16
// h for hpel_pixels_func is limited to {width/2, width} but never larger
// than 16 and never smaller than 4
typedef void (*tpel_mc_func)(uint8_t *block /* align width (8 or 16) */,
                             const uint8_t *pixels /* align 1 */,
                             int line_size, int w, int h);

/**
 * thirdpel DSP context
 */
typedef struct TpelDSPContext {
    /**
     * Thirdpel motion compensation with rounding (a + b + 1) >> 1.
     * this is an array[12] of motion compensation functions for the
     * 9 thirdpel positions<br>
     * *pixels_tab[xthirdpel + 4 * ythirdpel]
     * @param block destination where the result is stored
     * @param pixels source
     * @param line_size number of bytes in a horizontal line of block
     * @param h height
     */
    tpel_mc_func put_tpel_pixels_tab[11]; // FIXME individual func ptr per width?
    tpel_mc_func avg_tpel_pixels_tab[11]; // FIXME individual func ptr per width?
} TpelDSPContext;

void ff_tpeldsp_init(TpelDSPContext *c);

#endif /* AVCODEC_TPELDSP_H */
