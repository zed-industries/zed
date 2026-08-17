/*
 * Copyright (c) 2015 Paul B Mahol
 * Copyright (c) 2015 James Darnley
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

#ifndef AVFILTER_REMOVEGRAIN_H
#define AVFILTER_REMOVEGRAIN_H

#include "avfilter.h"

typedef struct RemoveGrainContext {
    const AVClass *class;

    int mode[4];

    int nb_planes;
    int planewidth[4];
    int planeheight[4];
    int skip_even;
    int skip_odd;

    int (*rg[4])(int c, int a1, int a2, int a3, int a4, int a5, int a6, int a7, int a8);

    void (*fl[4])(uint8_t *dst, uint8_t *src, ptrdiff_t stride, int pixels);
} RemoveGrainContext;

void ff_removegrain_init_x86(RemoveGrainContext *rg);

#endif /* AVFILTER_REMOVEGRAIN_H */
