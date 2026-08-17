/*
 * Copyright (c) 2013 Paul B Mahol
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

#ifndef AVFILTER_BLEND_H
#define AVFILTER_BLEND_H

#include "libavutil/eval.h"
#include "avfilter.h"

enum BlendMode {
    BLEND_UNSET = -1,
    BLEND_NORMAL,
    BLEND_ADDITION,
    BLEND_AND,
    BLEND_AVERAGE,
    BLEND_BURN,
    BLEND_DARKEN,
    BLEND_DIFFERENCE,
    BLEND_GRAINEXTRACT,
    BLEND_DIVIDE,
    BLEND_DODGE,
    BLEND_EXCLUSION,
    BLEND_HARDLIGHT,
    BLEND_LIGHTEN,
    BLEND_MULTIPLY,
    BLEND_NEGATION,
    BLEND_OR,
    BLEND_OVERLAY,
    BLEND_PHOENIX,
    BLEND_PINLIGHT,
    BLEND_REFLECT,
    BLEND_SCREEN,
    BLEND_SOFTLIGHT,
    BLEND_SUBTRACT,
    BLEND_VIVIDLIGHT,
    BLEND_XOR,
    BLEND_HARDMIX,
    BLEND_LINEARLIGHT,
    BLEND_GLOW,
    BLEND_GRAINMERGE,
    BLEND_MULTIPLY128,
    BLEND_HEAT,
    BLEND_FREEZE,
    BLEND_EXTREMITY,
    BLEND_SOFTDIFFERENCE,
    BLEND_GEOMETRIC,
    BLEND_HARMONIC,
    BLEND_BLEACH,
    BLEND_STAIN,
    BLEND_INTERPOLATE,
    BLEND_HARDOVERLAY,
    BLEND_NB
};

typedef struct SliceParams {
    double *values;
    int starty;
    AVExpr *e;
} SliceParams;

typedef struct FilterParams {
    enum BlendMode mode;
    double opacity;
    AVExpr **e;
    char *expr_str;
    void (*blend)(const uint8_t *top, ptrdiff_t top_linesize,
                  const uint8_t *bottom, ptrdiff_t bottom_linesize,
                  uint8_t *dst, ptrdiff_t dst_linesize,
                  ptrdiff_t width, ptrdiff_t height,
                  struct FilterParams *param, SliceParams *sliceparam);
} FilterParams;

void ff_blend_init_x86(FilterParams *param, int depth);

#endif /* AVFILTER_BLEND_H */
