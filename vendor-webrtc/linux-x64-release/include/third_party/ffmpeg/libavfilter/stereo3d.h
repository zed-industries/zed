/*
 * Copyright (c) 2015 Paul B Mahol
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

#ifndef AVFILTER_STEREO3D_H
#define AVFILTER_STEREO3D_H

#include <stddef.h>
#include <stdint.h>

typedef struct Stereo3DDSPContext {
    void (*anaglyph)(uint8_t *dst, uint8_t *lsrc, uint8_t *rsrc,
                     ptrdiff_t dst_linesize, ptrdiff_t l_linesize, ptrdiff_t r_linesize,
                     int width, int height,
                     const int *ana_matrix_r, const int *ana_matrix_g, const int *ana_matrix_b);
} Stereo3DDSPContext;

void ff_stereo3d_init_x86(Stereo3DDSPContext *dsp);

#endif /* AVFILTER_STEREO3D_H */
