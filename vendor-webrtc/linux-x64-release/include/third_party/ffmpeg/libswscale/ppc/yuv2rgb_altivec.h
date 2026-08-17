/*
 * AltiVec-enhanced yuv2yuvX
 *
 * Copyright (C) 2004 Romain Dolbeau <romain@dolbeau.org>
 * based on the equivalent C code in swscale.c
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

#ifndef SWSCALE_PPC_YUV2RGB_ALTIVEC_H
#define SWSCALE_PPC_YUV2RGB_ALTIVEC_H

#include <stdint.h>

#include "libswscale/swscale_internal.h"

#define YUV2PACKEDX_HEADER(suffix)                                  \
    void ff_yuv2 ## suffix ## _X_altivec(SwsInternal *c,            \
                                         const int16_t *lumFilter,  \
                                         const int16_t **lumSrc,    \
                                         int lumFilterSize,         \
                                         const int16_t *chrFilter,  \
                                         const int16_t **chrUSrc,   \
                                         const int16_t **chrVSrc,   \
                                         int chrFilterSize,         \
                                         const int16_t **alpSrc,    \
                                         uint8_t *dest,             \
                                         int dstW, int dstY);

YUV2PACKEDX_HEADER(abgr);
YUV2PACKEDX_HEADER(bgra);
YUV2PACKEDX_HEADER(argb);
YUV2PACKEDX_HEADER(rgba);
YUV2PACKEDX_HEADER(rgb24);
YUV2PACKEDX_HEADER(bgr24);

#endif /* SWSCALE_PPC_YUV2RGB_ALTIVEC_H */
