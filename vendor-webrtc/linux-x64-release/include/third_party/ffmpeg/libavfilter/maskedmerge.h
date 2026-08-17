/*
 * Copyright (c) 2015 Paul B Mahol
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

#ifndef AVFILTER_MASKEDMERGE_H
#define AVFILTER_MASKEDMERGE_H

#include "avfilter.h"
#include "framesync.h"

typedef struct MaskedMergeContext {
    const AVClass *class;
    int width[4], height[4];
    int linesize[4];
    int nb_planes;
    int planes;
    int half, depth, max;
    FFFrameSync fs;

    void (*maskedmerge)(const uint8_t *bsrc, const uint8_t *osrc,
                        const uint8_t *msrc, uint8_t *dst,
                        ptrdiff_t blinesize, ptrdiff_t olinesize,
                        ptrdiff_t mlinesize, ptrdiff_t dlinesize,
                        int w, int h,
                        int half, int shift);
} MaskedMergeContext;

void ff_maskedmerge_init_x86(MaskedMergeContext *s);

#endif /* AVFILTER_MASKEDMERGE_H */
