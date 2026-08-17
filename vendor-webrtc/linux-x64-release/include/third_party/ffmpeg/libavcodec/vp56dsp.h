/*
 * Copyright (c) 2010 Mans Rullgard <mans@mansr.com>
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

#ifndef AVCODEC_VP56DSP_H
#define AVCODEC_VP56DSP_H

#include <stddef.h>
#include <stdint.h>

typedef struct VP56DSPContext {
    void (*edge_filter_hor)(uint8_t *yuv, ptrdiff_t stride, int t);
    void (*edge_filter_ver)(uint8_t *yuv, ptrdiff_t stride, int t);

    void (*vp6_filter_diag4)(uint8_t *dst, uint8_t *src, ptrdiff_t stride,
                             const int16_t *h_weights,const int16_t *v_weights);
} VP56DSPContext;

void ff_vp6_filter_diag4_c(uint8_t *dst, uint8_t *src, ptrdiff_t stride,
                           const int16_t *h_weights, const int16_t *v_weights);

void ff_vp5dsp_init(VP56DSPContext *s);
void ff_vp6dsp_init(VP56DSPContext *s);

void ff_vp6dsp_init_arm(VP56DSPContext *s);
void ff_vp6dsp_init_x86(VP56DSPContext *s);

#endif /* AVCODEC_VP56DSP_H */
