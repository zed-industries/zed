/*
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

#ifndef AVCODEC_COPY_BLOCK_H
#define AVCODEC_COPY_BLOCK_H

#include <stddef.h>
#include <stdint.h>

#include "libavutil/intreadwrite.h"

static inline void copy_block2(uint8_t *dst, const uint8_t *src, ptrdiff_t dstStride, ptrdiff_t srcStride, int h)
{
    int i;
    for (i = 0; i < h; i++) {
        AV_COPY16U(dst, src);
        dst += dstStride;
        src += srcStride;
    }
}

static inline void copy_block4(uint8_t *dst, const uint8_t *src, ptrdiff_t dstStride, ptrdiff_t srcStride, int h)
{
    int i;
    for (i = 0; i < h; i++) {
        AV_COPY32U(dst, src);
        dst += dstStride;
        src += srcStride;
    }
}

static inline void copy_block8(uint8_t *dst, const uint8_t *src, ptrdiff_t dstStride, ptrdiff_t srcStride, int h)
{
    int i;
    for (i = 0; i < h; i++) {
        AV_COPY64U(dst, src);
        dst += dstStride;
        src += srcStride;
    }
}

static inline void copy_block9(uint8_t *dst, const uint8_t *src, ptrdiff_t dstStride, ptrdiff_t srcStride, int h)
{
    int i;
    for (i = 0; i < h; i++) {
        AV_COPY64U(dst, src);
        dst[8] = src[8];
        dst   += dstStride;
        src   += srcStride;
    }
}

static inline void copy_block16(uint8_t *dst, const uint8_t *src, ptrdiff_t dstStride, ptrdiff_t srcStride, int h)
{
    int i;
    for (i = 0; i < h; i++) {
        AV_COPY128U(dst, src);
        dst += dstStride;
        src += srcStride;
    }
}

static inline void copy_block17(uint8_t *dst, const uint8_t *src, ptrdiff_t dstStride, ptrdiff_t srcStride, int h)
{
    int i;
    for (i = 0; i < h; i++) {
        AV_COPY128U(dst, src);
        dst[16] = src[16];
        dst    += dstStride;
        src    += srcStride;
    }
}

#endif /* AVCODEC_COPY_BLOCK_H */
