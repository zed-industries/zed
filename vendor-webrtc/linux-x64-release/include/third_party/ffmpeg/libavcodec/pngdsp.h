/*
 * PNG image format
 * Copyright (c) 2008 Loren Merrit <lorenm@u.washington.edu>
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

#ifndef AVCODEC_PNGDSP_H
#define AVCODEC_PNGDSP_H

#include <stdint.h>

typedef struct PNGDSPContext {
    void (*add_bytes_l2)(uint8_t *dst,
                         uint8_t *src1 /* align 16 */,
                         uint8_t *src2, int w);

    /* this might write to dst[w] */
    void (*add_paeth_prediction)(uint8_t *dst, uint8_t *src,
                                 uint8_t *top, int w, int bpp);
} PNGDSPContext;

void ff_pngdsp_init(PNGDSPContext *dsp);
void ff_pngdsp_init_x86(PNGDSPContext *dsp);

#endif /* AVCODEC_PNGDSP_H */
