/*
 * Copyright (c) 2003 Michael Niedermayer <michaelni@gmx.at>
 * Copyright (C) 2005 Nikolaj Poroshin <porosh3@psu.ru>
 * Copyright (c) 2014 Arwa Arif <arwaarif1994@gmail.com>
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

#ifndef AVFILTER_FSPP_H
#define AVFILTER_FSPP_H

#include "libavutil/video_enc_params.h"
#include "avfilter.h"

#define BLOCKSZ 12
#define MAX_LEVEL 5

#define DCTSIZE 8
#define DCTSIZE_S "8"

#define FIX(x,s)  ((x) * (1 << s) + 0.5)

#define MULTIPLY16H(x,k)   (((x) * (k)) >> 16)
#define THRESHOLD(r,x,t)                         \
    if(((unsigned)((x) + t)) > t * 2) r = (x);   \
    else r = 0;
#define DESCALE(x,n)  (((x) + (1 << ((n) - 1))) >> n)

typedef int32_t int_simd16_t;
static const int16_t FIX_0_382683433   = FIX(0.382683433, 14);
static const int16_t FIX_0_541196100   = FIX(0.541196100, 14);
static const int16_t FIX_0_707106781   = FIX(M_SQRT1_2  , 14);
static const int16_t FIX_1_306562965   = FIX(1.306562965, 14);
static const int16_t FIX_1_414213562_A = FIX(M_SQRT2    , 14);
static const int16_t FIX_1_847759065   = FIX(1.847759065, 13);
static const int16_t FIX_2_613125930   = FIX(-2.613125930, 13);
static const int16_t FIX_1_414213562   = FIX(M_SQRT2    , 13);
static const int16_t FIX_1_082392200   = FIX(1.082392200, 13);

typedef struct FSPPContext {
    AVClass *class;
    uint64_t threshold_mtx_noq[8 * 2];
    uint64_t threshold_mtx[8 * 2];        //used in both C & MMX (& later SSE2) versions

    int log2_count;
    int strength;
    int hsub;
    int vsub;
    int temp_stride;
    int qp;
    enum AVVideoEncParamsType qscale_type;
    int prev_q;
    uint8_t *src;
    int16_t *temp;
    int8_t  *non_b_qp_table;
    int non_b_qp_stride;
    int use_bframe_qp;

    void (*store_slice)(uint8_t *dst, int16_t *src,
                        ptrdiff_t dst_stride, ptrdiff_t src_stride,
                        ptrdiff_t width, ptrdiff_t height, ptrdiff_t log2_scale);

    void (*store_slice2)(uint8_t *dst, int16_t *src,
                         ptrdiff_t dst_stride, ptrdiff_t src_stride,
                         ptrdiff_t width, ptrdiff_t height, ptrdiff_t log2_scale);

    void (*mul_thrmat)(int16_t *thr_adr_noq, int16_t *thr_adr, int q);

    void (*column_fidct)(int16_t *thr_adr, int16_t *data,
                         int16_t *output, int cnt);

    void (*row_idct)(int16_t *workspace, int16_t *output_adr,
                     ptrdiff_t output_stride, int cnt);

    void (*row_fdct)(int16_t *data, const uint8_t *pixels,
                     ptrdiff_t line_size, int cnt);

} FSPPContext;

void ff_fspp_init_x86(FSPPContext *fspp);

#endif /* AVFILTER_FSPP_H */
