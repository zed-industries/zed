/*
 * Copyright (c) 2003 Michael Niedermayer <michaelni@gmx.at>
 * Copyright (c) 2013 Clément Bœsch
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with FFmpeg; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */

#ifndef AVFILTER_SPP_H
#define AVFILTER_SPP_H

#include "libavutil/video_enc_params.h"
#include "libavcodec/avdct.h"
#include "avfilter.h"

#define MAX_LEVEL 6 /* quality levels */

typedef struct SPPContext {
    const AVClass *av_class;

    int log2_count;
    int qp;
    int mode;
    enum AVVideoEncParamsType qscale_type;
    int temp_linesize;
    uint8_t *src;
    uint16_t *temp;
    AVDCT *dct;
    int8_t *non_b_qp_table;
    int non_b_qp_stride;
    int use_bframe_qp;
    int hsub, vsub;

    void (*store_slice)(uint8_t *dst, const int16_t *src,
                        int dst_stride, int src_stride,
                        int width, int height, int log2_scale,
                        const uint8_t dither[8][8]);

    void (*requantize)(int16_t dst[64], const int16_t src[64],
                       int qp, const uint8_t *permutation);
} SPPContext;

void ff_spp_init_x86(SPPContext *s);

#endif /* AVFILTER_SPP_H */
