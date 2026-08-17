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

#ifndef AVCODEC_CFHDENCDSP_H
#define AVCODEC_CFHDENCDSP_H

#include <stddef.h>
#include <stdint.h>

typedef struct CFHDEncDSPContext {
    void (*horiz_filter)(const int16_t *input, int16_t *low, int16_t *high,
                         ptrdiff_t in_stride, ptrdiff_t low_stride,
                         ptrdiff_t high_stride,
                         int width, int height);

    void (*vert_filter)(const int16_t *input, int16_t *low, int16_t *high,
                        ptrdiff_t in_stride, ptrdiff_t low_stride,
                        ptrdiff_t high_stride,
                        int width, int height);
} CFHDEncDSPContext;

void ff_cfhdencdsp_init(CFHDEncDSPContext *c);

void ff_cfhdencdsp_init_x86(CFHDEncDSPContext *c);

#endif /* AVCODEC_CFHDENCDSP_H */
