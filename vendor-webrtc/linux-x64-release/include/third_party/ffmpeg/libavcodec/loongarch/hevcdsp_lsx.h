/*
 * Copyright (c) 2022 Loongson Technology Corporation Limited
 * Contributed by Lu Wang <wanglu@loongson.cn>
 *                Hao Chen <chenhao@loongson.cn>
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

#ifndef AVCODEC_LOONGARCH_HEVCDSP_LSX_H
#define AVCODEC_LOONGARCH_HEVCDSP_LSX_H

#include "libavcodec/hevc/dsp.h"

#define MC(PEL, DIR, WIDTH)                                               \
void ff_hevc_put_hevc_##PEL##_##DIR##WIDTH##_8_lsx(int16_t *dst,          \
                                                   const uint8_t *src,    \
                                                   ptrdiff_t src_stride,  \
                                                   int height,            \
                                                   intptr_t mx,           \
                                                   intptr_t my,           \
                                                   int width)

MC(pel, pixels, 4);
MC(pel, pixels, 6);
MC(pel, pixels, 8);
MC(pel, pixels, 12);
MC(pel, pixels, 16);
MC(pel, pixels, 24);
MC(pel, pixels, 32);
MC(pel, pixels, 48);
MC(pel, pixels, 64);

MC(qpel, h, 4);
MC(qpel, h, 8);
MC(qpel, h, 12);
MC(qpel, h, 16);
MC(qpel, h, 24);
MC(qpel, h, 32);
MC(qpel, h, 48);
MC(qpel, h, 64);

MC(qpel, v, 4);
MC(qpel, v, 8);
MC(qpel, v, 12);
MC(qpel, v, 16);
MC(qpel, v, 24);
MC(qpel, v, 32);
MC(qpel, v, 48);
MC(qpel, v, 64);

MC(qpel, hv, 4);
MC(qpel, hv, 8);
MC(qpel, hv, 12);
MC(qpel, hv, 16);
MC(qpel, hv, 24);
MC(qpel, hv, 32);
MC(qpel, hv, 48);
MC(qpel, hv, 64);

MC(epel, h, 32);

MC(epel, v, 16);
MC(epel, v, 24);
MC(epel, v, 32);

MC(epel, hv, 8);
MC(epel, hv, 12);
MC(epel, hv, 16);
MC(epel, hv, 24);
MC(epel, hv, 32);

#undef MC

#define BI_MC(PEL, DIR, WIDTH)                                               \
void ff_hevc_put_hevc_bi_##PEL##_##DIR##WIDTH##_8_lsx(uint8_t *dst,          \
                                                      ptrdiff_t dst_stride,  \
                                                      const uint8_t *src,    \
                                                      ptrdiff_t src_stride,  \
                                                      const int16_t *src_16bit, \
                                                      int height,            \
                                                      intptr_t mx,           \
                                                      intptr_t my,           \
                                                      int width)

BI_MC(pel, pixels, 4);
BI_MC(pel, pixels, 6);
BI_MC(pel, pixels, 8);
BI_MC(pel, pixels, 12);
BI_MC(pel, pixels, 16);
BI_MC(pel, pixels, 24);
BI_MC(pel, pixels, 32);
BI_MC(pel, pixels, 48);
BI_MC(pel, pixels, 64);

BI_MC(qpel, h, 16);
BI_MC(qpel, h, 24);
BI_MC(qpel, h, 32);
BI_MC(qpel, h, 48);
BI_MC(qpel, h, 64);

BI_MC(qpel, v, 8);
BI_MC(qpel, v, 16);
BI_MC(qpel, v, 24);
BI_MC(qpel, v, 32);
BI_MC(qpel, v, 48);
BI_MC(qpel, v, 64);

BI_MC(qpel, hv, 8);
BI_MC(qpel, hv, 16);
BI_MC(qpel, hv, 24);
BI_MC(qpel, hv, 32);
BI_MC(qpel, hv, 48);
BI_MC(qpel, hv, 64);

BI_MC(epel, h, 4);
BI_MC(epel, h, 6);
BI_MC(epel, h, 8);
BI_MC(epel, h, 12);
BI_MC(epel, h, 16);
BI_MC(epel, h, 24);
BI_MC(epel, h, 32);
BI_MC(epel, h, 48);
BI_MC(epel, h, 64);

BI_MC(epel, v, 12);
BI_MC(epel, v, 16);
BI_MC(epel, v, 24);
BI_MC(epel, v, 32);

BI_MC(epel, hv, 6);
BI_MC(epel, hv, 8);
BI_MC(epel, hv, 16);
BI_MC(epel, hv, 24);
BI_MC(epel, hv, 32);

#undef BI_MC

#define UNI_MC(PEL, DIR, WIDTH)                                              \
void ff_hevc_put_hevc_uni_##PEL##_##DIR##WIDTH##_8_lsx(uint8_t *dst,         \
                                                       ptrdiff_t dst_stride, \
                                                       const uint8_t *src,   \
                                                       ptrdiff_t src_stride, \
                                                       int height,           \
                                                       intptr_t mx,          \
                                                       intptr_t my,          \
                                                       int width)
UNI_MC(qpel, h, 4);
UNI_MC(qpel, h, 6);
UNI_MC(qpel, h, 8);
UNI_MC(qpel, h, 12);
UNI_MC(qpel, h, 16);
UNI_MC(qpel, h, 24);
UNI_MC(qpel, h, 32);
UNI_MC(qpel, h, 48);
UNI_MC(qpel, h, 64);

UNI_MC(qpel, v, 24);
UNI_MC(qpel, v, 32);
UNI_MC(qpel, v, 48);
UNI_MC(qpel, v, 64);

UNI_MC(qpel, hv, 8);
UNI_MC(qpel, hv, 16);
UNI_MC(qpel, hv, 24);
UNI_MC(qpel, hv, 32);
UNI_MC(qpel, hv, 48);
UNI_MC(qpel, hv, 64);

UNI_MC(epel, v, 24);
UNI_MC(epel, v, 32);

UNI_MC(epel, hv, 8);
UNI_MC(epel, hv, 12);
UNI_MC(epel, hv, 16);
UNI_MC(epel, hv, 24);
UNI_MC(epel, hv, 32);

#undef UNI_MC

#define UNI_W_MC(PEL, DIR, WIDTH)                                       \
void ff_hevc_put_hevc_uni_w_##PEL##_##DIR##WIDTH##_8_lsx(uint8_t *dst,  \
                                                         ptrdiff_t      \
                                                         dst_stride,    \
                                                         const uint8_t *src,  \
                                                         ptrdiff_t      \
                                                         src_stride,    \
                                                         int height,    \
                                                         int denom,     \
                                                         int weight,    \
                                                         int offset,    \
                                                         intptr_t mx,   \
                                                         intptr_t my,   \
                                                         int width)

UNI_W_MC(qpel, hv, 8);
UNI_W_MC(qpel, hv, 16);
UNI_W_MC(qpel, hv, 24);
UNI_W_MC(qpel, hv, 32);
UNI_W_MC(qpel, hv, 48);
UNI_W_MC(qpel, hv, 64);

#undef UNI_W_MC

void ff_hevc_loop_filter_luma_h_8_lsx(uint8_t *src, ptrdiff_t stride,
                                      int32_t beta, const int32_t *tc,
                                      const uint8_t *p_is_pcm, const uint8_t *q_is_pcm);

void ff_hevc_loop_filter_luma_v_8_lsx(uint8_t *src, ptrdiff_t stride,
                                      int32_t beta, const int32_t *tc,
                                      const uint8_t *p_is_pcm, const uint8_t *q_is_pcm);

void ff_hevc_loop_filter_chroma_h_8_lsx(uint8_t *src, ptrdiff_t stride,
                                        const int32_t *tc, const uint8_t *p_is_pcm,
                                        const uint8_t *q_is_pcm);

void ff_hevc_loop_filter_chroma_v_8_lsx(uint8_t *src, ptrdiff_t stride,
                                        const int32_t *tc, const uint8_t *p_is_pcm,
                                        const uint8_t *q_is_pcm);

void ff_hevc_sao_edge_filter_8_lsx(uint8_t *dst, const uint8_t *src,
                                   ptrdiff_t stride_dst,
                                   const int16_t *sao_offset_val,
                                   int eo, int width, int height);

void ff_hevc_idct_4x4_lsx(int16_t *coeffs, int col_limit);
void ff_hevc_idct_8x8_lsx(int16_t *coeffs, int col_limit);
void ff_hevc_idct_16x16_lsx(int16_t *coeffs, int col_limit);
void ff_hevc_idct_32x32_lsx(int16_t *coeffs, int col_limit);

void ff_hevc_add_residual4x4_8_lsx(uint8_t *dst, const int16_t *res, ptrdiff_t stride);
void ff_hevc_add_residual8x8_8_lsx(uint8_t *dst, const int16_t *res, ptrdiff_t stride);
void ff_hevc_add_residual16x16_8_lsx(uint8_t *dst, const int16_t *res, ptrdiff_t stride);
void ff_hevc_add_residual32x32_8_lsx(uint8_t *dst, const int16_t *res, ptrdiff_t stride);

#define PEL_UNI_W(PEL, DIR, WIDTH)                                      \
void ff_hevc_put_hevc_##PEL##_uni_w_##DIR##WIDTH##_8_lsx(uint8_t *dst,  \
                                                         ptrdiff_t      \
                                                         dst_stride,    \
                                                         const uint8_t *src,  \
                                                         ptrdiff_t      \
                                                         src_stride,    \
                                                         int height,    \
                                                         int denom,     \
                                                         int wx,        \
                                                         int ox,        \
                                                         intptr_t mx,   \
                                                         intptr_t my,   \
                                                         int width)

PEL_UNI_W(pel, pixels, 4);
PEL_UNI_W(pel, pixels, 6);
PEL_UNI_W(pel, pixels, 8);
PEL_UNI_W(pel, pixels, 12);
PEL_UNI_W(pel, pixels, 16);
PEL_UNI_W(pel, pixels, 24);
PEL_UNI_W(pel, pixels, 32);
PEL_UNI_W(pel, pixels, 48);
PEL_UNI_W(pel, pixels, 64);

PEL_UNI_W(qpel, v, 4);
PEL_UNI_W(qpel, v, 6);
PEL_UNI_W(qpel, v, 8);
PEL_UNI_W(qpel, v, 12);
PEL_UNI_W(qpel, v, 16);
PEL_UNI_W(qpel, v, 24);
PEL_UNI_W(qpel, v, 32);
PEL_UNI_W(qpel, v, 48);
PEL_UNI_W(qpel, v, 64);

PEL_UNI_W(qpel, h, 4);
PEL_UNI_W(qpel, h, 6);
PEL_UNI_W(qpel, h, 8);
PEL_UNI_W(qpel, h, 12);
PEL_UNI_W(qpel, h, 16);
PEL_UNI_W(qpel, h, 24);
PEL_UNI_W(qpel, h, 32);
PEL_UNI_W(qpel, h, 48);
PEL_UNI_W(qpel, h, 64);

PEL_UNI_W(epel, hv, 4);
PEL_UNI_W(epel, hv, 6);
PEL_UNI_W(epel, hv, 8);
PEL_UNI_W(epel, hv, 12);
PEL_UNI_W(epel, hv, 16);
PEL_UNI_W(epel, hv, 24);
PEL_UNI_W(epel, hv, 32);
PEL_UNI_W(epel, hv, 48);
PEL_UNI_W(epel, hv, 64);

PEL_UNI_W(epel, h, 4);
PEL_UNI_W(epel, h, 6);
PEL_UNI_W(epel, h, 8);
PEL_UNI_W(epel, h, 12);
PEL_UNI_W(epel, h, 16);
PEL_UNI_W(epel, h, 24);
PEL_UNI_W(epel, h, 32);
PEL_UNI_W(epel, h, 48);
PEL_UNI_W(epel, h, 64);

PEL_UNI_W(epel, v, 4);
PEL_UNI_W(epel, v, 6);
PEL_UNI_W(epel, v, 8);
PEL_UNI_W(epel, v, 12);
PEL_UNI_W(epel, v, 16);
PEL_UNI_W(epel, v, 24);
PEL_UNI_W(epel, v, 32);
PEL_UNI_W(epel, v, 48);
PEL_UNI_W(epel, v, 64);

#undef PEL_UNI_W

#endif  // #ifndef AVCODEC_LOONGARCH_HEVCDSP_LSX_H
