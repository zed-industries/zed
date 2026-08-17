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

#ifndef AVCODEC_CFHDDSP_H
#define AVCODEC_CFHDDSP_H

#include <stddef.h>
#include <stdint.h>

typedef struct CFHDDSPContext {
    void (*horiz_filter)(int16_t *output, ptrdiff_t out_stride,
                         const int16_t *low, ptrdiff_t low_stride,
                         const int16_t *high, ptrdiff_t high_stride,
                         int width, int height);

    void (*vert_filter)(int16_t *output, ptrdiff_t out_stride,
                        const int16_t *low, ptrdiff_t low_stride,
                        const int16_t *high, ptrdiff_t high_stride,
                        int width, int height);

    void (*horiz_filter_clip)(int16_t *output, const int16_t *low, const int16_t *high,
                              int width, int bpc);
} CFHDDSPContext;

void ff_cfhddsp_init(CFHDDSPContext *c, int format, int bayer);

void ff_cfhddsp_init_x86(CFHDDSPContext *c, int format, int bayer);

#endif /* AVCODEC_CFHDDSP_H */
