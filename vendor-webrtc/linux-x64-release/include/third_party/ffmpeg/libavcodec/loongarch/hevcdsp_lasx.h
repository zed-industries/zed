/*
 * Copyright (c) 2023 Loongson Technology Corporation Limited
 * Contributed by jinbo <jinbo@loongson.cn>
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

#ifndef AVCODEC_LOONGARCH_HEVCDSP_LASX_H
#define AVCODEC_LOONGARCH_HEVCDSP_LASX_H

#include "libavcodec/hevc/dsp.h"

#define PEL_UNI_W(PEL, DIR, WIDTH)                                       \
void ff_hevc_put_hevc_##PEL##_uni_w_##DIR##WIDTH##_8_lasx(uint8_t *dst,  \
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

PEL_UNI_W(pel, pixels, 6);
PEL_UNI_W(pel, pixels, 8);
PEL_UNI_W(pel, pixels, 12);
PEL_UNI_W(pel, pixels, 16);
PEL_UNI_W(pel, pixels, 24);
PEL_UNI_W(pel, pixels, 32);
PEL_UNI_W(pel, pixels, 48);
PEL_UNI_W(pel, pixels, 64);

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

PEL_UNI_W(epel, hv, 6);
PEL_UNI_W(epel, hv, 8);
PEL_UNI_W(epel, hv, 12);
PEL_UNI_W(epel, hv, 16);
PEL_UNI_W(epel, hv, 24);
PEL_UNI_W(epel, hv, 32);
PEL_UNI_W(epel, hv, 48);
PEL_UNI_W(epel, hv, 64);

PEL_UNI_W(epel, v, 6);
PEL_UNI_W(epel, v, 8);
PEL_UNI_W(epel, v, 12);
PEL_UNI_W(epel, v, 16);
PEL_UNI_W(epel, v, 24);
PEL_UNI_W(epel, v, 32);
PEL_UNI_W(epel, v, 48);
PEL_UNI_W(epel, v, 64);

PEL_UNI_W(epel, h, 6);
PEL_UNI_W(epel, h, 8);
PEL_UNI_W(epel, h, 12);
PEL_UNI_W(epel, h, 16);
PEL_UNI_W(epel, h, 24);
PEL_UNI_W(epel, h, 32);
PEL_UNI_W(epel, h, 48);
PEL_UNI_W(epel, h, 64);

#undef PEL_UNI_W

#define UNI_MC(PEL, DIR, WIDTH)                                               \
void ff_hevc_put_hevc_uni_##PEL##_##DIR##WIDTH##_8_lasx(uint8_t *dst,         \
                                                        ptrdiff_t dst_stride, \
                                                        const uint8_t *src,   \
                                                        ptrdiff_t src_stride, \
                                                        int height,           \
                                                        intptr_t mx,          \
                                                        intptr_t my,          \
                                                        int width)
UNI_MC(qpel, h, 12);
UNI_MC(qpel, h, 16);
UNI_MC(qpel, h, 24);
UNI_MC(qpel, h, 32);
UNI_MC(qpel, h, 48);
UNI_MC(qpel, h, 64);

#undef UNI_MC

#define BI_MC(PEL, DIR, WIDTH)                                                \
void ff_hevc_put_hevc_bi_##PEL##_##DIR##WIDTH##_8_lasx(uint8_t *dst,          \
                                                       ptrdiff_t dst_stride,  \
                                                       const uint8_t *src,    \
                                                       ptrdiff_t src_stride,  \
                                                       const int16_t *src_16bit, \
                                                       int height,            \
                                                       intptr_t mx,           \
                                                       intptr_t my,           \
                                                       int width)
BI_MC(epel, h, 12);
BI_MC(epel, h, 16);
BI_MC(epel, h, 32);
BI_MC(epel, h, 48);
BI_MC(epel, h, 64);

#undef BI_MC

void ff_hevc_idct_32x32_lasx(int16_t *coeffs, int col_limit);

#endif  // #ifndef AVCODEC_LOONGARCH_HEVCDSP_LASX_H
