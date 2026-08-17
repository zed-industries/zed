/*
 * Copyright (c) 2010 Nolan Lum <nol888@gmail.com>
 * Copyright (c) 2009 Loren Merritt <lorenm@u.washington.edu>
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

#ifndef AVFILTER_GRADFUN_H
#define AVFILTER_GRADFUN_H

#include "avfilter.h"

/// Holds instance-specific information for gradfun.
typedef struct GradFunContext {
    const AVClass *class;
    float strength;
    int thresh;    ///< threshold for gradient algorithm
    int radius;    ///< blur radius
    int chroma_w;  ///< width of the chroma planes
    int chroma_h;  ///< weight of the chroma planes
    int chroma_r;  ///< blur radius for the chroma planes
    uint16_t *buf; ///< holds image data for blur algorithm passed into filter.
    /// DSP functions.
    void (*filter_line) (uint8_t *dst, const uint8_t *src, const uint16_t *dc, int width, int thresh, const uint16_t *dithers);
    void (*blur_line) (uint16_t *dc, uint16_t *buf, const uint16_t *buf1, const uint8_t *src, int src_linesize, int width);
} GradFunContext;

void ff_gradfun_init_x86(GradFunContext *gf);

void ff_gradfun_filter_line_c(uint8_t *dst, const uint8_t *src, const uint16_t *dc, int width, int thresh, const uint16_t *dithers);
void ff_gradfun_blur_line_c(uint16_t *dc, uint16_t *buf, const uint16_t *buf1, const uint8_t *src, int src_linesize, int width);

#endif /* AVFILTER_GRADFUN_H */
