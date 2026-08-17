/*
 * VC-1 and WMV3 decoder - DSP functions
 * Copyright (c) 2006 Konstantin Shishkov
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
 * VC-1 and WMV3 decoder
 */

#ifndef AVCODEC_VC1DSP_H
#define AVCODEC_VC1DSP_H

#include "hpeldsp.h"
#include "h264chroma.h"

typedef void (*vc1op_pixels_func)(uint8_t *block/*align width (8 or 16)*/, const uint8_t *pixels/*align 1*/, ptrdiff_t line_size, int h);

typedef struct VC1DSPContext {
    /* vc1 functions */
    void (*vc1_inv_trans_8x8)(int16_t *b);
    void (*vc1_inv_trans_8x4)(uint8_t *dest, ptrdiff_t stride, int16_t *block);
    void (*vc1_inv_trans_4x8)(uint8_t *dest, ptrdiff_t stride, int16_t *block);
    void (*vc1_inv_trans_4x4)(uint8_t *dest, ptrdiff_t stride, int16_t *block);
    void (*vc1_inv_trans_8x8_dc)(uint8_t *dest, ptrdiff_t stride, int16_t *block);
    void (*vc1_inv_trans_8x4_dc)(uint8_t *dest, ptrdiff_t stride, int16_t *block);
    void (*vc1_inv_trans_4x8_dc)(uint8_t *dest, ptrdiff_t stride, int16_t *block);
    void (*vc1_inv_trans_4x4_dc)(uint8_t *dest, ptrdiff_t stride, int16_t *block);
    void (*vc1_v_overlap)(uint8_t *src, ptrdiff_t stride);
    void (*vc1_h_overlap)(uint8_t *src, ptrdiff_t stride);
    void (*vc1_v_s_overlap)(int16_t *top,  int16_t *bottom);
    void (*vc1_h_s_overlap)(int16_t *left, int16_t *right, ptrdiff_t left_stride, ptrdiff_t right_stride, int flags);
    void (*vc1_v_loop_filter4)(uint8_t *src, ptrdiff_t stride, int pq);
    void (*vc1_h_loop_filter4)(uint8_t *src, ptrdiff_t stride, int pq);
    void (*vc1_v_loop_filter8)(uint8_t *src, ptrdiff_t stride, int pq);
    void (*vc1_h_loop_filter8)(uint8_t *src, ptrdiff_t stride, int pq);
    void (*vc1_v_loop_filter16)(uint8_t *src, ptrdiff_t stride, int pq);
    void (*vc1_h_loop_filter16)(uint8_t *src, ptrdiff_t stride, int pq);

    /* put 8x8 block with bicubic interpolation and quarterpel precision
     * last argument is actually round value instead of height
     */
    vc1op_pixels_func put_vc1_mspel_pixels_tab[2][16];
    vc1op_pixels_func avg_vc1_mspel_pixels_tab[2][16];

    /* This is really one func used in VC-1 decoding */
    h264_chroma_mc_func put_no_rnd_vc1_chroma_pixels_tab[3];
    h264_chroma_mc_func avg_no_rnd_vc1_chroma_pixels_tab[3];

    /* Windows Media Image functions */
    void (*sprite_h)(uint8_t *dst, const uint8_t *src, int offset, int advance, int count);
    void (*sprite_v_single)(uint8_t *dst, const uint8_t *src1a, const uint8_t *src1b, int offset, int width);
    void (*sprite_v_double_noscale)(uint8_t *dst, const uint8_t *src1a, const uint8_t *src2a, int alpha, int width);
    void (*sprite_v_double_onescale)(uint8_t *dst, const uint8_t *src1a, const uint8_t *src1b, int offset1,
                                                   const uint8_t *src2a, int alpha, int width);
    void (*sprite_v_double_twoscale)(uint8_t *dst, const uint8_t *src1a, const uint8_t *src1b, int offset1,
                                                   const uint8_t *src2a, const uint8_t *src2b, int offset2,
                                     int alpha, int width);

    /**
     * Search buf from the start for up to size bytes. Return the index
     * of a zero byte, or >= size if not found. Ideally, use lookahead
     * to filter out any zero bytes that are known to not be followed by
     * one or more further zero bytes and a one byte.
     */
    int (*startcode_find_candidate)(const uint8_t *buf, int size);

    /* Copy a buffer, removing startcode emulation escape bytes as we go */
    int (*vc1_unescape_buffer)(const uint8_t *src, int size, uint8_t *dst);
} VC1DSPContext;

void ff_vc1dsp_init(VC1DSPContext* c);
void ff_vc1dsp_init_aarch64(VC1DSPContext* dsp);
void ff_vc1dsp_init_arm(VC1DSPContext* dsp);
void ff_vc1dsp_init_ppc(VC1DSPContext *c);
void ff_vc1dsp_init_riscv(VC1DSPContext *c);
void ff_vc1dsp_init_x86(VC1DSPContext* dsp);
void ff_vc1dsp_init_mips(VC1DSPContext* dsp);
void ff_vc1dsp_init_loongarch(VC1DSPContext* dsp);

#endif /* AVCODEC_VC1DSP_H */
