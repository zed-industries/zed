/*
 * Half-pel DSP functions.
 * Copyright (c) 2000, 2001, 2002 Fabrice Bellard
 * Copyright (c) 2002-2004 Michael Niedermayer <michaelni@gmx.at>
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
 * Half-pel DSP functions.
 */

#ifndef AVCODEC_HPELDSP_H
#define AVCODEC_HPELDSP_H

#include <stdint.h>
#include <stddef.h>

/* add and put pixel (decoding) */
// blocksizes for hpel_pixels_func are 8x4,8x8 16x8 16x16
// h for hpel_pixels_func is limited to {width/2, width} but never larger
// than 16 and never smaller than 4
typedef void (*op_pixels_func)(uint8_t *block /*align width (8 or 16)*/,
                               const uint8_t *pixels /*align 1*/,
                               ptrdiff_t line_size, int h);

/**
 * Half-pel DSP context.
 */
typedef struct HpelDSPContext {
    /**
     * Halfpel motion compensation with rounding (a+b+1)>>1.
     * this is an array[4][4] of motion compensation functions for 4
     * horizontal blocksizes (8,16) and the 4 halfpel positions<br>
     * *pixels_tab[ 0->16xH 1->8xH ][ xhalfpel + 2*yhalfpel ]
     * @param block destination where the result is stored
     * @param pixels source
     * @param line_size number of bytes in a horizontal line of block
     * @param h height
     */
    op_pixels_func put_pixels_tab[4][4];

    /**
     * Halfpel motion compensation with rounding (a+b+1)>>1.
     * This is an array[4][4] of motion compensation functions for 4
     * horizontal blocksizes (8,16) and the 4 halfpel positions<br>
     * *pixels_tab[ 0->16xH 1->8xH ][ xhalfpel + 2*yhalfpel ]
     * @param block destination into which the result is averaged (a+b+1)>>1
     * @param pixels source
     * @param line_size number of bytes in a horizontal line of block
     * @param h height
     */
    op_pixels_func avg_pixels_tab[4][4];

    /**
     * Halfpel motion compensation with no rounding (a+b)>>1.
     * this is an array[4][4] of motion compensation functions for 2
     * horizontal blocksizes (8,16) and the 4 halfpel positions<br>
     * *pixels_tab[ 0->16xH 1->8xH ][ xhalfpel + 2*yhalfpel ]
     * @param block destination where the result is stored
     * @param pixels source
     * @param line_size number of bytes in a horizontal line of block
     * @param h height
     * @note The size is kept at [4][4] to match the above pixel_tabs and avoid
     *       out of bounds reads in the motion estimation code.
     */
    op_pixels_func put_no_rnd_pixels_tab[4][4];

    /**
     * Halfpel motion compensation with no rounding (a+b)>>1.
     * this is an array[4] of motion compensation functions for 1
     * horizontal blocksize (16) and the 4 halfpel positions<br>
     * *pixels_tab[0][ xhalfpel + 2*yhalfpel ]
     * @param block destination into which the result is averaged (a+b)>>1
     * @param pixels source
     * @param line_size number of bytes in a horizontal line of block
     * @param h height
     */
    op_pixels_func avg_no_rnd_pixels_tab[4];
} HpelDSPContext;

void ff_hpeldsp_init(HpelDSPContext *c, int flags);

void ff_hpeldsp_init_aarch64(HpelDSPContext *c, int flags);
void ff_hpeldsp_init_arm(HpelDSPContext *c, int flags);
void ff_hpeldsp_init_ppc(HpelDSPContext *c, int flags);
void ff_hpeldsp_init_x86(HpelDSPContext *c, int flags);
void ff_hpeldsp_init_mips(HpelDSPContext *c, int flags);
void ff_hpeldsp_init_loongarch(HpelDSPContext *c, int flags);

#endif /* AVCODEC_HPELDSP_H */
