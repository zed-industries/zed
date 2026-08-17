/*
 * Copyright (C) 2024 Zhao Zhili
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

#ifndef AVCODEC_AARCH64_H26X_DSP_H
#define AVCODEC_AARCH64_H26X_DSP_H

#include <stddef.h>
#include <stdint.h>

void ff_h26x_sao_band_filter_8x8_8_neon(uint8_t *_dst, const uint8_t *_src,
                                        ptrdiff_t stride_dst, ptrdiff_t stride_src,
                                        const int16_t *sao_offset_val, int sao_left_class,
                                        int width, int height);
void ff_hevc_sao_edge_filter_16x16_8_neon(uint8_t *dst, const uint8_t *src, ptrdiff_t stride_dst,
                                          const int16_t *sao_offset_val, int eo, int width, int height);
void ff_hevc_sao_edge_filter_8x8_8_neon(uint8_t *dst, const uint8_t *src, ptrdiff_t stride_dst,
                                        const int16_t *sao_offset_val, int eo, int width, int height);

void ff_vvc_sao_edge_filter_16x16_8_neon(uint8_t *dst, const uint8_t *src, ptrdiff_t stride_dst,
                                         const int16_t *sao_offset_val, int eo, int width, int height);
void ff_vvc_sao_edge_filter_8x8_8_neon(uint8_t *dst, const uint8_t *src, ptrdiff_t stride_dst,
                                       const int16_t *sao_offset_val, int eo, int width, int height);

#define NEON8_FNPROTO_PARTIAL_6(fn, args, ext) \
    void ff_hevc_put_hevc_##fn##_h4_8_neon##ext args;  \
    void ff_hevc_put_hevc_##fn##_h6_8_neon##ext args;  \
    void ff_hevc_put_hevc_##fn##_h8_8_neon##ext args;  \
    void ff_hevc_put_hevc_##fn##_h12_8_neon##ext args; \
    void ff_hevc_put_hevc_##fn##_h16_8_neon##ext args; \
    void ff_hevc_put_hevc_##fn##_h32_8_neon##ext args;

NEON8_FNPROTO_PARTIAL_6(qpel, (int16_t *dst, const uint8_t *_src, ptrdiff_t _srcstride, int height,
        intptr_t mx, intptr_t my, int width),)

NEON8_FNPROTO_PARTIAL_6(qpel_uni, (uint8_t *_dst, ptrdiff_t _dststride, const uint8_t *_src,
        ptrdiff_t _srcstride, int height, intptr_t mx, intptr_t my, int width),)

NEON8_FNPROTO_PARTIAL_6(qpel_bi, (uint8_t *_dst, ptrdiff_t _dststride, const uint8_t *_src,
        ptrdiff_t _srcstride, const int16_t *src2, int height, intptr_t
        mx, intptr_t my, int width),)

#define NEON8_FNPROTO(fn, args, ext) \
    void ff_hevc_put_hevc_##fn##4_8_neon##ext args; \
    void ff_hevc_put_hevc_##fn##6_8_neon##ext args; \
    void ff_hevc_put_hevc_##fn##8_8_neon##ext args; \
    void ff_hevc_put_hevc_##fn##12_8_neon##ext args; \
    void ff_hevc_put_hevc_##fn##16_8_neon##ext args; \
    void ff_hevc_put_hevc_##fn##24_8_neon##ext args; \
    void ff_hevc_put_hevc_##fn##32_8_neon##ext args; \
    void ff_hevc_put_hevc_##fn##48_8_neon##ext args; \
    void ff_hevc_put_hevc_##fn##64_8_neon##ext args

#define NEON8_FNPROTO_PARTIAL_4(fn, args, ext) \
    void ff_hevc_put_hevc_##fn##4_8_neon##ext args; \
    void ff_hevc_put_hevc_##fn##8_8_neon##ext args; \
    void ff_hevc_put_hevc_##fn##16_8_neon##ext args; \
    void ff_hevc_put_hevc_##fn##64_8_neon##ext args

#define NEON8_FNPROTO_PARTIAL_5(fn, args, ext) \
    void ff_hevc_put_hevc_##fn##4_8_neon##ext args; \
    void ff_hevc_put_hevc_##fn##8_8_neon##ext args; \
    void ff_hevc_put_hevc_##fn##16_8_neon##ext args; \
    void ff_hevc_put_hevc_##fn##32_8_neon##ext args; \
    void ff_hevc_put_hevc_##fn##64_8_neon##ext args

NEON8_FNPROTO(pel_pixels, (int16_t *dst,
        const uint8_t *src, ptrdiff_t srcstride,
        int height, intptr_t mx, intptr_t my, int width),);

NEON8_FNPROTO(pel_bi_pixels, (uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *_src, ptrdiff_t _srcstride, const int16_t *src2,
        int height, intptr_t mx, intptr_t my, int width),);

NEON8_FNPROTO(epel_bi_h, (uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, const int16_t *src2,
        int height, intptr_t mx, intptr_t my, int width),);

NEON8_FNPROTO(epel_bi_v, (uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, const int16_t *src2,
        int height, intptr_t mx, intptr_t my, int width),);

NEON8_FNPROTO(epel_bi_hv, (uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, const int16_t *src2,
        int height, intptr_t mx, intptr_t my, int width),);

NEON8_FNPROTO(epel_bi_hv, (uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, const int16_t *src2,
        int height, intptr_t mx, intptr_t my, int width), _i8mm);

NEON8_FNPROTO(epel_v, (int16_t *dst,
        const uint8_t *src, ptrdiff_t srcstride,
        int height, intptr_t mx, intptr_t my, int width),);

NEON8_FNPROTO(pel_uni_pixels, (uint8_t *_dst, ptrdiff_t _dststride,
        const uint8_t *_src, ptrdiff_t _srcstride,
        int height, intptr_t mx, intptr_t my, int width),);

NEON8_FNPROTO(pel_uni_w_pixels, (uint8_t *_dst, ptrdiff_t _dststride,
        const uint8_t *_src, ptrdiff_t _srcstride,
        int height, int denom, int wx, int ox,
        intptr_t mx, intptr_t my, int width),);

NEON8_FNPROTO(epel_uni_v, (uint8_t *dst,  ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride,
        int height, intptr_t mx, intptr_t my, int width),);

NEON8_FNPROTO(epel_uni_hv, (uint8_t *dst, ptrdiff_t _dststride,
        const uint8_t *src, ptrdiff_t srcstride,
        int height, intptr_t mx, intptr_t my, int width),);

NEON8_FNPROTO(epel_uni_hv, (uint8_t *dst, ptrdiff_t _dststride,
        const uint8_t *src, ptrdiff_t srcstride,
        int height, intptr_t mx, intptr_t my, int width), _i8mm);

NEON8_FNPROTO(epel_uni_w_v, (uint8_t *_dst,  ptrdiff_t _dststride,
        const uint8_t *_src, ptrdiff_t _srcstride,
        int height, int denom, int wx, int ox,
        intptr_t mx, intptr_t my, int width),);

NEON8_FNPROTO_PARTIAL_4(qpel_uni_w_v, (uint8_t *_dst,  ptrdiff_t _dststride,
        const uint8_t *_src, ptrdiff_t _srcstride,
        int height, int denom, int wx, int ox,
        intptr_t mx, intptr_t my, int width),);

NEON8_FNPROTO(epel_h, (int16_t *dst,
        const uint8_t *_src, ptrdiff_t _srcstride,
        int height, intptr_t mx, intptr_t my, int width),);

NEON8_FNPROTO(epel_hv, (int16_t *dst,
        const uint8_t *src, ptrdiff_t srcstride,
        int height, intptr_t mx, intptr_t my, int width), );

NEON8_FNPROTO(epel_h, (int16_t *dst,
        const uint8_t *_src, ptrdiff_t _srcstride,
        int height, intptr_t mx, intptr_t my, int width), _i8mm);

NEON8_FNPROTO(epel_hv, (int16_t *dst,
        const uint8_t *src, ptrdiff_t srcstride,
        int height, intptr_t mx, intptr_t my, int width), _i8mm);

NEON8_FNPROTO(epel_uni_w_h, (uint8_t *_dst,  ptrdiff_t _dststride,
        const uint8_t *_src, ptrdiff_t _srcstride,
        int height, int denom, int wx, int ox,
        intptr_t mx, intptr_t my, int width),);

NEON8_FNPROTO(epel_uni_w_h, (uint8_t *_dst,  ptrdiff_t _dststride,
        const uint8_t *_src, ptrdiff_t _srcstride,
        int height, int denom, int wx, int ox,
        intptr_t mx, intptr_t my, int width), _i8mm);

NEON8_FNPROTO(qpel_h, (int16_t *dst,
        const uint8_t *_src, ptrdiff_t _srcstride,
        int height, intptr_t mx, intptr_t my, int width), _i8mm);

NEON8_FNPROTO(qpel_v, (int16_t *dst,
        const uint8_t *src, ptrdiff_t srcstride,
        int height, intptr_t mx, intptr_t my, int width),);

NEON8_FNPROTO(qpel_hv, (int16_t *dst,
        const uint8_t *src, ptrdiff_t srcstride,
        int height, intptr_t mx, intptr_t my, int width),);

NEON8_FNPROTO(qpel_hv, (int16_t *dst,
        const uint8_t *src, ptrdiff_t srcstride,
        int height, intptr_t mx, intptr_t my, int width), _i8mm);

NEON8_FNPROTO(qpel_uni_v, (uint8_t *dst,  ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride,
        int height, intptr_t mx, intptr_t my, int width),);

NEON8_FNPROTO(qpel_uni_hv, (uint8_t *dst,  ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride,
        int height, intptr_t mx, intptr_t my, int width),);

NEON8_FNPROTO(qpel_uni_hv, (uint8_t *dst,  ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride,
        int height, intptr_t mx, intptr_t my, int width), _i8mm);

NEON8_FNPROTO(qpel_uni_w_h, (uint8_t *_dst,  ptrdiff_t _dststride,
        const uint8_t *_src, ptrdiff_t _srcstride,
        int height, int denom, int wx, int ox,
        intptr_t mx, intptr_t my, int width),);

NEON8_FNPROTO(qpel_uni_w_h, (uint8_t *_dst,  ptrdiff_t _dststride,
        const uint8_t *_src, ptrdiff_t _srcstride,
        int height, int denom, int wx, int ox,
        intptr_t mx, intptr_t my, int width), _i8mm);

NEON8_FNPROTO(epel_uni_w_hv, (uint8_t *_dst,  ptrdiff_t _dststride,
        const uint8_t *_src, ptrdiff_t _srcstride,
        int height, int denom, int wx, int ox,
        intptr_t mx, intptr_t my, int width),);

NEON8_FNPROTO(epel_uni_w_hv, (uint8_t *_dst,  ptrdiff_t _dststride,
        const uint8_t *_src, ptrdiff_t _srcstride,
        int height, int denom, int wx, int ox,
        intptr_t mx, intptr_t my, int width), _i8mm);

NEON8_FNPROTO_PARTIAL_5(qpel_uni_w_hv, (uint8_t *_dst,  ptrdiff_t _dststride,
        const uint8_t *_src, ptrdiff_t _srcstride,
        int height, int denom, int wx, int ox,
        intptr_t mx, intptr_t my, int width),);

NEON8_FNPROTO_PARTIAL_5(qpel_uni_w_hv, (uint8_t *_dst,  ptrdiff_t _dststride,
        const uint8_t *_src, ptrdiff_t _srcstride,
        int height, int denom, int wx, int ox,
        intptr_t mx, intptr_t my, int width), _i8mm);

NEON8_FNPROTO(qpel_bi_v, (uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, const int16_t *src2,
        int height, intptr_t mx, intptr_t my, int width),);

NEON8_FNPROTO(qpel_bi_hv, (uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, const int16_t *src2,
        int height, intptr_t mx, intptr_t my, int width),);

NEON8_FNPROTO(qpel_bi_hv, (uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, const int16_t *src2,
        int height, intptr_t mx, intptr_t my, int width), _i8mm);

#undef NEON8_FNPROTO_PARTIAL_4
#define NEON8_FNPROTO_PARTIAL_4(fn, args, ext) \
    void ff_vvc_put_##fn##_h4_8_neon##ext args;  \
    void ff_vvc_put_##fn##_h8_8_neon##ext args;  \
    void ff_vvc_put_##fn##_h16_8_neon##ext args; \
    void ff_vvc_put_##fn##_h32_8_neon##ext args;

NEON8_FNPROTO_PARTIAL_4(qpel, (int16_t *dst, const uint8_t *_src, ptrdiff_t _srcstride, int height,
        const int8_t *hf, const int8_t *vf, int width),)

NEON8_FNPROTO_PARTIAL_4(qpel_uni, (uint8_t *_dst, ptrdiff_t _dststride, const uint8_t *_src,
        ptrdiff_t _srcstride, int height, const int8_t *hf, const int8_t *vf, int width),)

NEON8_FNPROTO_PARTIAL_4(epel, (int16_t *dst, const uint8_t *_src, ptrdiff_t _srcstride, int height,
        const int8_t *hf, const int8_t *vf, int width),)

#undef NEON8_FNPROTO_PARTIAL_6
#define NEON8_FNPROTO_PARTIAL_6(fn, args, ext) \
    void ff_vvc_put_##fn##4_8_neon##ext args; \
    void ff_vvc_put_##fn##8_8_neon##ext args; \
    void ff_vvc_put_##fn##16_8_neon##ext args; \
    void ff_vvc_put_##fn##32_8_neon##ext args; \
    void ff_vvc_put_##fn##64_8_neon##ext args; \
    void ff_vvc_put_##fn##128_8_neon##ext args

NEON8_FNPROTO_PARTIAL_6(pel_pixels, (int16_t *dst,
        const uint8_t *src, ptrdiff_t srcstride, int height,
        const int8_t *hf, const int8_t *vf, int width),);

NEON8_FNPROTO_PARTIAL_6(pel_uni_pixels, (uint8_t *_dst, ptrdiff_t _dststride,
        const uint8_t *_src, ptrdiff_t _srcstride, int height,
        const int8_t *hf, const int8_t *vf, int width),);

NEON8_FNPROTO_PARTIAL_6(pel_uni_w_pixels, (uint8_t *_dst, ptrdiff_t _dststride,
        const uint8_t *_src, ptrdiff_t _srcstride,
        int height, int denom, int wx, int ox,
        const int8_t *hf, const int8_t *vf, int width),);

NEON8_FNPROTO_PARTIAL_6(qpel_h, (int16_t *dst,
        const uint8_t *_src, ptrdiff_t _srcstride, int height,
        const int8_t *hf, const int8_t *vf, int width), _i8mm);

NEON8_FNPROTO_PARTIAL_6(epel_h, (int16_t *dst,
        const uint8_t *_src, ptrdiff_t _srcstride, int height,
        const int8_t *hf, const int8_t *vf, int width), _i8mm);

void ff_vvc_put_qpel_v4_8_neon(int16_t *dst, const uint8_t *_src,
                               ptrdiff_t _srcstride, int height,
                               const int8_t *hf, const int8_t *vf, int width);

void ff_vvc_put_qpel_v8_8_neon(int16_t *dst, const uint8_t *_src,
                               ptrdiff_t _srcstride, int height,
                               const int8_t *hf, const int8_t *vf, int width);

NEON8_FNPROTO_PARTIAL_6(qpel_hv, (int16_t *dst,
        const uint8_t *src, ptrdiff_t srcstride, int height,
        const int8_t *hf, const int8_t *vf, int width),);

NEON8_FNPROTO_PARTIAL_6(qpel_hv, (int16_t *dst,
        const uint8_t *src, ptrdiff_t srcstride, int height,
        const int8_t *hf, const int8_t *vf, int width), _i8mm);

NEON8_FNPROTO_PARTIAL_6(epel_hv, (int16_t *dst,
        const uint8_t *src, ptrdiff_t srcstride, int height,
        const int8_t *hf, const int8_t *vf, int width),);

NEON8_FNPROTO_PARTIAL_6(epel_hv, (int16_t *dst,
        const uint8_t *src, ptrdiff_t srcstride, int height,
        const int8_t *hf, const int8_t *vf, int width), _i8mm);

#endif
