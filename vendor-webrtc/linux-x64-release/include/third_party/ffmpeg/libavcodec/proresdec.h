/*
 * Copyright (c) 2010-2011 Maxim Poliakovski
 * Copyright (c) 2010-2011 Elvis Presley
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

#ifndef AVCODEC_PRORESDEC_H
#define AVCODEC_PRORESDEC_H

#include <stdint.h>

#include "get_bits.h"
#include "blockdsp.h"
#include "proresdsp.h"

#include "libavutil/frame.h"
#include "libavutil/pixfmt.h"

typedef struct {
    const uint8_t *data;
    unsigned mb_x;
    unsigned mb_y;
    unsigned mb_count;
    unsigned data_size;
    int ret;
} SliceContext;

typedef struct {
    BlockDSPContext bdsp;
    ProresDSPContext prodsp;
    AVFrame *frame;
    int frame_type;              ///< 0 = progressive, 1 = tff, 2 = bff
    uint8_t qmat_luma[64];
    uint8_t qmat_chroma[64];
    SliceContext *slices;
    int slice_count;             ///< number of slices in the current picture
    unsigned mb_width;           ///< width of the current picture in mb
    unsigned mb_height;          ///< height of the current picture in mb
    uint8_t progressive_scan[64];
    uint8_t interlaced_scan[64];
    const uint8_t *scan;
    int first_field;
    int alpha_info;
    void (*unpack_alpha)(GetBitContext *gb, uint16_t *dst, int num_coeffs, const int num_bits);
    enum AVPixelFormat pix_fmt;
} ProresContext;

#endif /* AVCODEC_PRORESDEC_H */
