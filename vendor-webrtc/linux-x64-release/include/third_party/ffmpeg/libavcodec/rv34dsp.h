/*
 * RV30/40 decoder motion compensation functions
 * Copyright (c) 2008 Konstantin Shishkov
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
 * RV30/40 decoder motion compensation functions
 */

#ifndef AVCODEC_RV34DSP_H
#define AVCODEC_RV34DSP_H

#include "h264chroma.h"
#include "qpeldsp.h"

typedef void (*rv40_weight_func)(uint8_t *dst/*align width (8 or 16)*/,
                                 uint8_t *src1/*align width (8 or 16)*/,
                                 uint8_t *src2/*align width (8 or 16)*/,
                                 int w1, int w2, ptrdiff_t stride);

typedef void (*rv34_inv_transform_func)(int16_t *block);

typedef void (*rv34_idct_add_func)(uint8_t *dst, ptrdiff_t stride, int16_t *block);
typedef void (*rv34_idct_dc_add_func)(uint8_t *dst, ptrdiff_t stride,
                                      int   dc);

typedef void (*rv40_weak_loop_filter_func)(uint8_t *src, ptrdiff_t stride,
                                           int filter_p1, int filter_q1,
                                           int alpha, int beta,
                                           int lims, int lim_q1, int lim_p1);

typedef void (*rv40_strong_loop_filter_func)(uint8_t *src, ptrdiff_t stride,
                                             int alpha, int lims,
                                             int dmode, int chroma);

typedef int (*rv40_loop_filter_strength_func)(uint8_t *src, ptrdiff_t stride,
                                              int beta, int beta2, int edge,
                                              int *p1, int *q1);

typedef struct RV34DSPContext {
    qpel_mc_func put_pixels_tab[4][16];
    qpel_mc_func avg_pixels_tab[4][16];
    h264_chroma_mc_func put_chroma_pixels_tab[3];
    h264_chroma_mc_func avg_chroma_pixels_tab[3];
    /**
     * Biweight functions, first dimension is transform size (16/8),
     * second is whether the weight is prescaled by 1/512 to skip
     * the intermediate shifting.
     */
    rv40_weight_func rv40_weight_pixels_tab[2][2];
    rv34_inv_transform_func rv34_inv_transform;
    rv34_inv_transform_func rv34_inv_transform_dc;
    rv34_idct_add_func rv34_idct_add;
    rv34_idct_dc_add_func rv34_idct_dc_add;
    rv40_weak_loop_filter_func rv40_weak_loop_filter[2];
    rv40_strong_loop_filter_func rv40_strong_loop_filter[2];
    rv40_loop_filter_strength_func rv40_loop_filter_strength[2];
} RV34DSPContext;

void ff_rv30dsp_init(RV34DSPContext *c);
void ff_rv34dsp_init(RV34DSPContext *c);
void ff_rv40dsp_init(RV34DSPContext *c);

void ff_rv34dsp_init_arm(RV34DSPContext *c);
void ff_rv34dsp_init_riscv(RV34DSPContext *c);
void ff_rv34dsp_init_x86(RV34DSPContext *c);

void ff_rv40dsp_init_aarch64(RV34DSPContext *c);
void ff_rv40dsp_init_riscv(RV34DSPContext *c);
void ff_rv40dsp_init_x86(RV34DSPContext *c);
void ff_rv40dsp_init_arm(RV34DSPContext *c);

#endif /* AVCODEC_RV34DSP_H */
