/*
 * HEVC video decoder
 *
 * Copyright (C) 2012 - 2013 Guillaume Martres
 * Copyright (C) 2013 - 2014 Pierre-Edouard Lepere
 *
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

#ifndef AVCODEC_X86_HEVCDSP_H
#define AVCODEC_X86_HEVCDSP_H

#include <stddef.h>
#include <stdint.h>


#define PEL_LINK(dst, idx1, idx2, idx3, name, D, opt) \
dst[idx1][idx2][idx3] = ff_hevc_put_ ## name ## _ ## D ## _##opt;      \
dst ## _bi[idx1][idx2][idx3] = ff_hevc_put_bi_ ## name ## _ ## D ## _##opt;      \
dst ## _uni[idx1][idx2][idx3] = ff_hevc_put_uni_ ## name ## _ ## D ## _##opt;      \
dst ## _uni_w[idx1][idx2][idx3] = ff_hevc_put_uni_w_ ## name ## _ ## D ## _##opt;      \
dst ## _bi_w[idx1][idx2][idx3] = ff_hevc_put_bi_w_ ## name ## _ ## D ## _##opt


#define PEL_PROTOTYPE(name, D, opt) \
void ff_hevc_put_ ## name ## _ ## D ## _##opt(int16_t *dst, const uint8_t *_src, ptrdiff_t _srcstride, int height, intptr_t mx, intptr_t my,int width);      \
void ff_hevc_put_bi_ ## name ## _ ## D ## _##opt(uint8_t *_dst, ptrdiff_t _dststride, const uint8_t *_src, ptrdiff_t _srcstride, const int16_t *src2, int height, intptr_t mx, intptr_t my, int width);      \
void ff_hevc_put_uni_ ## name ## _ ## D ## _##opt(uint8_t *_dst, ptrdiff_t _dststride, const uint8_t *_src, ptrdiff_t _srcstride, int height, intptr_t mx, intptr_t my, int width);      \
void ff_hevc_put_uni_w_ ## name ## _ ## D ## _##opt(uint8_t *_dst, ptrdiff_t _dststride, const uint8_t *_src, ptrdiff_t _srcstride, int height, int denom, int wx, int ox, intptr_t mx, intptr_t my, int width);      \
void ff_hevc_put_bi_w_ ## name ## _ ## D ## _##opt(uint8_t *_dst, ptrdiff_t _dststride, const uint8_t *_src, ptrdiff_t _srcstride, const int16_t *src2, int height, int denom, int wx0, int wx1, int ox0, int ox1, intptr_t mx, intptr_t my, int width)


///////////////////////////////////////////////////////////////////////////////
// MC functions
///////////////////////////////////////////////////////////////////////////////

#define EPEL_PROTOTYPES(fname, bitd, opt) \
        PEL_PROTOTYPE(fname##4,  bitd, opt); \
        PEL_PROTOTYPE(fname##6,  bitd, opt); \
        PEL_PROTOTYPE(fname##8,  bitd, opt); \
        PEL_PROTOTYPE(fname##12, bitd, opt); \
        PEL_PROTOTYPE(fname##16, bitd, opt); \
        PEL_PROTOTYPE(fname##24, bitd, opt); \
        PEL_PROTOTYPE(fname##32, bitd, opt); \
        PEL_PROTOTYPE(fname##48, bitd, opt); \
        PEL_PROTOTYPE(fname##64, bitd, opt)

#define QPEL_PROTOTYPES(fname, bitd, opt) \
        PEL_PROTOTYPE(fname##4,  bitd, opt); \
        PEL_PROTOTYPE(fname##8,  bitd, opt); \
        PEL_PROTOTYPE(fname##12, bitd, opt); \
        PEL_PROTOTYPE(fname##16, bitd, opt); \
        PEL_PROTOTYPE(fname##24, bitd, opt); \
        PEL_PROTOTYPE(fname##32, bitd, opt); \
        PEL_PROTOTYPE(fname##48, bitd, opt); \
        PEL_PROTOTYPE(fname##64, bitd, opt)

#define WEIGHTING_PROTOTYPE(width, bitd, opt) \
void ff_hevc_put_uni_w##width##_##bitd##_##opt(uint8_t *dst, ptrdiff_t dststride, const int16_t *_src, int height, int denom,  int _wx, int _ox);      \
void ff_hevc_put_bi_w##width##_##bitd##_##opt(uint8_t *dst, ptrdiff_t dststride, const int16_t *_src, const int16_t *_src2, int height, int denom,  int _wx0,  int _wx1, int _ox0, int _ox1)

#define WEIGHTING_PROTOTYPES(bitd, opt) \
        WEIGHTING_PROTOTYPE(2, bitd, opt); \
        WEIGHTING_PROTOTYPE(4, bitd, opt); \
        WEIGHTING_PROTOTYPE(6, bitd, opt); \
        WEIGHTING_PROTOTYPE(8, bitd, opt); \
        WEIGHTING_PROTOTYPE(12, bitd, opt); \
        WEIGHTING_PROTOTYPE(16, bitd, opt); \
        WEIGHTING_PROTOTYPE(24, bitd, opt); \
        WEIGHTING_PROTOTYPE(32, bitd, opt); \
        WEIGHTING_PROTOTYPE(48, bitd, opt); \
        WEIGHTING_PROTOTYPE(64, bitd, opt)


///////////////////////////////////////////////////////////////////////////////
// QPEL_PIXELS EPEL_PIXELS
///////////////////////////////////////////////////////////////////////////////
EPEL_PROTOTYPES(pel_pixels ,  8, sse4);
EPEL_PROTOTYPES(pel_pixels , 10, sse4);
EPEL_PROTOTYPES(pel_pixels , 12, sse4);

void ff_hevc_put_pel_pixels16_8_avx2(int16_t *dst, const uint8_t *_src, ptrdiff_t _srcstride, int height, intptr_t mx, intptr_t my,int width);
void ff_hevc_put_pel_pixels24_8_avx2(int16_t *dst, const uint8_t *_src, ptrdiff_t _srcstride, int height, intptr_t mx, intptr_t my,int width);
void ff_hevc_put_pel_pixels32_8_avx2(int16_t *dst, const uint8_t *_src, ptrdiff_t _srcstride, int height, intptr_t mx, intptr_t my,int width);
void ff_hevc_put_pel_pixels48_8_avx2(int16_t *dst, const uint8_t *_src, ptrdiff_t _srcstride, int height, intptr_t mx, intptr_t my,int width);
void ff_hevc_put_pel_pixels64_8_avx2(int16_t *dst, const uint8_t *_src, ptrdiff_t _srcstride, int height, intptr_t mx, intptr_t my,int width);

void ff_hevc_put_pel_pixels16_10_avx2(int16_t *dst, const uint8_t *_src, ptrdiff_t _srcstride, int height, intptr_t mx, intptr_t my,int width);
void ff_hevc_put_pel_pixels24_10_avx2(int16_t *dst, const uint8_t *_src, ptrdiff_t _srcstride, int height, intptr_t mx, intptr_t my,int width);
void ff_hevc_put_pel_pixels32_10_avx2(int16_t *dst, const uint8_t *_src, ptrdiff_t _srcstride, int height, intptr_t mx, intptr_t my,int width);
void ff_hevc_put_pel_pixels48_10_avx2(int16_t *dst, const uint8_t *_src, ptrdiff_t _srcstride, int height, intptr_t mx, intptr_t my,int width);
void ff_hevc_put_pel_pixels64_10_avx2(int16_t *dst, const uint8_t *_src, ptrdiff_t _srcstride, int height, intptr_t mx, intptr_t my,int width);



void ff_hevc_put_uni_pel_pixels32_8_avx2(uint8_t *dst, ptrdiff_t dststride,const uint8_t *_src, ptrdiff_t _srcstride, int height, intptr_t mx, intptr_t my,int width);
void ff_hevc_put_uni_pel_pixels48_8_avx2(uint8_t *dst, ptrdiff_t dststride,const uint8_t *_src, ptrdiff_t _srcstride, int height, intptr_t mx, intptr_t my,int width);
void ff_hevc_put_uni_pel_pixels64_8_avx2(uint8_t *dst, ptrdiff_t dststride,const uint8_t *_src, ptrdiff_t _srcstride, int height, intptr_t mx, intptr_t my,int width);
void ff_hevc_put_uni_pel_pixels96_8_avx2(uint8_t *dst, ptrdiff_t dststride,const uint8_t *_src, ptrdiff_t _srcstride, int height, intptr_t mx, intptr_t my,int width); //used for 10bit
void ff_hevc_put_uni_pel_pixels128_8_avx2(uint8_t *dst, ptrdiff_t dststride,const uint8_t *_src, ptrdiff_t _srcstride, int height, intptr_t mx, intptr_t my,int width);//used for 10bit


void ff_hevc_put_bi_pel_pixels16_8_avx2(uint8_t *_dst, ptrdiff_t _dststride, const uint8_t *_src, ptrdiff_t _srcstride, const int16_t *src2, int height, intptr_t mx, intptr_t my, int width);
void ff_hevc_put_bi_pel_pixels24_8_avx2(uint8_t *_dst, ptrdiff_t _dststride, const uint8_t *_src, ptrdiff_t _srcstride, const int16_t *src2, int height, intptr_t mx, intptr_t my, int width);
void ff_hevc_put_bi_pel_pixels32_8_avx2(uint8_t *_dst, ptrdiff_t _dststride, const uint8_t *_src, ptrdiff_t _srcstride, const int16_t *src2, int height, intptr_t mx, intptr_t my, int width);
void ff_hevc_put_bi_pel_pixels48_8_avx2(uint8_t *_dst, ptrdiff_t _dststride, const uint8_t *_src, ptrdiff_t _srcstride, const int16_t *src2, int height, intptr_t mx, intptr_t my, int width);
void ff_hevc_put_bi_pel_pixels64_8_avx2(uint8_t *_dst, ptrdiff_t _dststride, const uint8_t *_src, ptrdiff_t _srcstride, const int16_t *src2, int height, intptr_t mx, intptr_t my, int width);

void ff_hevc_put_bi_pel_pixels16_10_avx2(uint8_t *_dst, ptrdiff_t _dststride, const uint8_t *_src, ptrdiff_t _srcstride, const int16_t *src2, int height, intptr_t mx, intptr_t my, int width);
void ff_hevc_put_bi_pel_pixels24_10_avx2(uint8_t *_dst, ptrdiff_t _dststride, const uint8_t *_src, ptrdiff_t _srcstride, const int16_t *src2, int height, intptr_t mx, intptr_t my, int width);
void ff_hevc_put_bi_pel_pixels32_10_avx2(uint8_t *_dst, ptrdiff_t _dststride, const uint8_t *_src, ptrdiff_t _srcstride, const int16_t *src2, int height, intptr_t mx, intptr_t my, int width);
void ff_hevc_put_bi_pel_pixels48_10_avx2(uint8_t *_dst, ptrdiff_t _dststride, const uint8_t *_src, ptrdiff_t _srcstride, const int16_t *src2, int height, intptr_t mx, intptr_t my, int width);
void ff_hevc_put_bi_pel_pixels64_10_avx2(uint8_t *_dst, ptrdiff_t _dststride, const uint8_t *_src, ptrdiff_t _srcstride, const int16_t *src2, int height, intptr_t mx, intptr_t my, int width);

///////////////////////////////////////////////////////////////////////////////
// EPEL
///////////////////////////////////////////////////////////////////////////////
EPEL_PROTOTYPES(epel_h ,  8, sse4);
EPEL_PROTOTYPES(epel_h , 10, sse4);
EPEL_PROTOTYPES(epel_h , 12, sse4);

EPEL_PROTOTYPES(epel_v ,  8, sse4);
EPEL_PROTOTYPES(epel_v , 10, sse4);
EPEL_PROTOTYPES(epel_v , 12, sse4);

EPEL_PROTOTYPES(epel_hv ,  8, sse4);
EPEL_PROTOTYPES(epel_hv , 10, sse4);
EPEL_PROTOTYPES(epel_hv , 12, sse4);

PEL_PROTOTYPE(epel_h16, 8, avx2);
PEL_PROTOTYPE(epel_h24, 8, avx2);
PEL_PROTOTYPE(epel_h32, 8, avx2);
PEL_PROTOTYPE(epel_h48, 8, avx2);
PEL_PROTOTYPE(epel_h64, 8, avx2);

PEL_PROTOTYPE(epel_h16,10, avx2);
PEL_PROTOTYPE(epel_h24,10, avx2);
PEL_PROTOTYPE(epel_h32,10, avx2);
PEL_PROTOTYPE(epel_h48,10, avx2);
PEL_PROTOTYPE(epel_h64,10, avx2);

PEL_PROTOTYPE(epel_v16, 8, avx2);
PEL_PROTOTYPE(epel_v24, 8, avx2);
PEL_PROTOTYPE(epel_v32, 8, avx2);
PEL_PROTOTYPE(epel_v48, 8, avx2);
PEL_PROTOTYPE(epel_v64, 8, avx2);

PEL_PROTOTYPE(epel_v16,10, avx2);
PEL_PROTOTYPE(epel_v24,10, avx2);
PEL_PROTOTYPE(epel_v32,10, avx2);
PEL_PROTOTYPE(epel_v48,10, avx2);
PEL_PROTOTYPE(epel_v64,10, avx2);

PEL_PROTOTYPE(epel_hv16, 8, avx2);
PEL_PROTOTYPE(epel_hv24, 8, avx2);
PEL_PROTOTYPE(epel_hv32, 8, avx2);
PEL_PROTOTYPE(epel_hv48, 8, avx2);
PEL_PROTOTYPE(epel_hv64, 8, avx2);

PEL_PROTOTYPE(epel_hv16,10, avx2);
PEL_PROTOTYPE(epel_hv24,10, avx2);
PEL_PROTOTYPE(epel_hv32,10, avx2);
PEL_PROTOTYPE(epel_hv48,10, avx2);
PEL_PROTOTYPE(epel_hv64,10, avx2);

///////////////////////////////////////////////////////////////////////////////
// QPEL
///////////////////////////////////////////////////////////////////////////////
QPEL_PROTOTYPES(qpel_h ,  8, sse4);
QPEL_PROTOTYPES(qpel_h , 10, sse4);
QPEL_PROTOTYPES(qpel_h , 12, sse4);

QPEL_PROTOTYPES(qpel_v,  8, sse4);
QPEL_PROTOTYPES(qpel_v, 10, sse4);
QPEL_PROTOTYPES(qpel_v, 12, sse4);

QPEL_PROTOTYPES(qpel_hv,  8, sse4);
QPEL_PROTOTYPES(qpel_hv, 10, sse4);
QPEL_PROTOTYPES(qpel_hv, 12, sse4);

PEL_PROTOTYPE(qpel_h16, 8, avx2);
PEL_PROTOTYPE(qpel_h24, 8, avx2);
PEL_PROTOTYPE(qpel_h32, 8, avx2);
PEL_PROTOTYPE(qpel_h48, 8, avx2);
PEL_PROTOTYPE(qpel_h64, 8, avx2);

PEL_PROTOTYPE(qpel_h16,10, avx2);
PEL_PROTOTYPE(qpel_h24,10, avx2);
PEL_PROTOTYPE(qpel_h32,10, avx2);
PEL_PROTOTYPE(qpel_h48,10, avx2);
PEL_PROTOTYPE(qpel_h64,10, avx2);

PEL_PROTOTYPE(qpel_v16, 8, avx2);
PEL_PROTOTYPE(qpel_v24, 8, avx2);
PEL_PROTOTYPE(qpel_v32, 8, avx2);
PEL_PROTOTYPE(qpel_v48, 8, avx2);
PEL_PROTOTYPE(qpel_v64, 8, avx2);

PEL_PROTOTYPE(qpel_v16,10, avx2);
PEL_PROTOTYPE(qpel_v24,10, avx2);
PEL_PROTOTYPE(qpel_v32,10, avx2);
PEL_PROTOTYPE(qpel_v48,10, avx2);
PEL_PROTOTYPE(qpel_v64,10, avx2);

PEL_PROTOTYPE(qpel_hv16, 8, avx2);
PEL_PROTOTYPE(qpel_hv24, 8, avx2);
PEL_PROTOTYPE(qpel_hv32, 8, avx2);
PEL_PROTOTYPE(qpel_hv48, 8, avx2);
PEL_PROTOTYPE(qpel_hv64, 8, avx2);

PEL_PROTOTYPE(qpel_hv16,10, avx2);
PEL_PROTOTYPE(qpel_hv24,10, avx2);
PEL_PROTOTYPE(qpel_hv32,10, avx2);
PEL_PROTOTYPE(qpel_hv48,10, avx2);
PEL_PROTOTYPE(qpel_hv64,10, avx2);

WEIGHTING_PROTOTYPES(8, sse4);
WEIGHTING_PROTOTYPES(10, sse4);
WEIGHTING_PROTOTYPES(12, sse4);

void ff_hevc_put_qpel_h4_8_avx512icl(int16_t *dst, const uint8_t *_src, ptrdiff_t _srcstride, int height, intptr_t mx, intptr_t my, int width);
void ff_hevc_put_qpel_h8_8_avx512icl(int16_t *dst, const uint8_t *_src, ptrdiff_t _srcstride, int height, intptr_t mx, intptr_t my, int width);
void ff_hevc_put_qpel_h16_8_avx512icl(int16_t *dst, const uint8_t *_src, ptrdiff_t _srcstride, int height, intptr_t mx, intptr_t my, int width);
void ff_hevc_put_qpel_h32_8_avx512icl(int16_t *dst, const uint8_t *_src, ptrdiff_t _srcstride, int height, intptr_t mx, intptr_t my, int width);
void ff_hevc_put_qpel_h64_8_avx512icl(int16_t *dst, const uint8_t *_src, ptrdiff_t _srcstride, int height, intptr_t mx, intptr_t my, int width);
void ff_hevc_put_qpel_hv8_8_avx512icl(int16_t *dst, const uint8_t *_src, ptrdiff_t _srcstride, int height, intptr_t mx, intptr_t my, int width);

///////////////////////////////////////////////////////////////////////////////
// TRANSFORM_ADD
///////////////////////////////////////////////////////////////////////////////

void ff_hevc_add_residual_4_8_mmxext(uint8_t *dst, const int16_t *res, ptrdiff_t stride);
void ff_hevc_add_residual_8_8_sse2(uint8_t *dst, const int16_t *res, ptrdiff_t stride);
void ff_hevc_add_residual_16_8_sse2(uint8_t *dst, const int16_t *res, ptrdiff_t stride);
void ff_hevc_add_residual_32_8_sse2(uint8_t *dst, const int16_t *res, ptrdiff_t stride);

void ff_hevc_add_residual_8_8_avx(uint8_t *dst, const int16_t *res, ptrdiff_t stride);
void ff_hevc_add_residual_16_8_avx(uint8_t *dst, const int16_t *res, ptrdiff_t stride);
void ff_hevc_add_residual_32_8_avx(uint8_t *dst, const int16_t *res, ptrdiff_t stride);

void ff_hevc_add_residual_32_8_avx2(uint8_t *dst, const int16_t *res, ptrdiff_t stride);

void ff_hevc_add_residual_4_10_mmxext(uint8_t *dst, const int16_t *res, ptrdiff_t stride);
void ff_hevc_add_residual_8_10_sse2(uint8_t *dst, const int16_t *res, ptrdiff_t stride);
void ff_hevc_add_residual_16_10_sse2(uint8_t *dst, const int16_t *res, ptrdiff_t stride);
void ff_hevc_add_residual_32_10_sse2(uint8_t *dst, const int16_t *res, ptrdiff_t stride);

void ff_hevc_add_residual_16_10_avx2(uint8_t *dst, const int16_t *res, ptrdiff_t stride);
void ff_hevc_add_residual_32_10_avx2(uint8_t *dst, const int16_t *res, ptrdiff_t stride);

#endif // AVCODEC_X86_HEVCDSP_H
