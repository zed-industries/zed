/*
 * Copyright (C) 2024 Niklas Haas
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

#ifndef SWSCALE_CSPUTILS_H
#define SWSCALE_CSPUTILS_H

#include <stdint.h>
#include <stdbool.h>
#include <math.h>

#include "libavutil/attributes.h"
#include "libavutil/common.h"
#include "libavutil/csp.h"
#include "libavutil/pixfmt.h"

/* Shared constants and helpers for colorspace mapping */

#define fmixf(a, b, x) ((b) * (x) + (a) * (1 - (x)))

static inline float smoothstepf(float edge0, float edge1, float x)
{
    if (edge0 == edge1)
        return x >= edge0;
    x = (x - edge0) / (edge1 - edge0);
    x = av_clipf(x, 0.0f, 1.0f);
    return x * x * (3.0f - 2.0f * x);
}

/* 3x3 matrix math */
typedef struct SwsMatrix3x3 {
    float m[3][3];
} SwsMatrix3x3;

void ff_sws_matrix3x3_mul(SwsMatrix3x3 *a, const SwsMatrix3x3 *b);
void ff_sws_matrix3x3_rmul(const SwsMatrix3x3 *a, SwsMatrix3x3 *b);
void ff_sws_matrix3x3_invert(SwsMatrix3x3 *mat);
void ff_sws_matrix3x3_apply(const SwsMatrix3x3 *mat, float vec[3]);

SwsMatrix3x3 ff_sws_ipt_rgb2lms(const AVColorPrimariesDesc *prim);
SwsMatrix3x3 ff_sws_ipt_lms2rgb(const AVColorPrimariesDesc *prim);

/* Converts to/from XYZ (relative to the given white point, no adaptation) */
SwsMatrix3x3 ff_sws_rgb2xyz(const AVColorPrimariesDesc *prim);
SwsMatrix3x3 ff_sws_xyz2rgb(const AVColorPrimariesDesc *prim);

/* Returns an RGB -> RGB adaptation matrix */
SwsMatrix3x3 ff_sws_get_adaptation(const AVPrimaryCoefficients *prim,
                                   AVWhitepointCoefficients from,
                                   AVWhitepointCoefficients to);

/* Integer math definitions / helpers */
typedef struct v3u8_t {
    uint8_t x, y, z;
} v3u8_t;

typedef struct v2u16_t {
    uint16_t x, y;
} v2u16_t;

typedef struct v3u16_t {
    uint16_t x, y, z;
} v3u16_t;

/* Fast perceptual quantizer */
static const float PQ_M1 = 2610./4096 * 1./4,
                   PQ_M2 = 2523./4096 * 128,
                   PQ_C1 = 3424./4096,
                   PQ_C2 = 2413./4096 * 32,
                   PQ_C3 = 2392./4096 * 32;

enum { PQ_LUT_SIZE = 1024 };
extern const float ff_pq_eotf_lut[PQ_LUT_SIZE+1];

static inline float pq_eotf(float x)
{
    float idxf  = av_clipf(x, 0.0f, 1.0f) * (PQ_LUT_SIZE - 1);
    int ipart   = floorf(idxf);
    float fpart = idxf - ipart;
    return fmixf(ff_pq_eotf_lut[ipart], ff_pq_eotf_lut[ipart + 1], fpart);
}

static inline float pq_oetf(float x)
{
    x = powf(fmaxf(x * 1e-4f, 0.0f), PQ_M1);
    x = (PQ_C1 + PQ_C2 * x) / (1.0f + PQ_C3 * x);
    return powf(x, PQ_M2);
}

/* Misc colorspace math / helpers */

/**
 * Returns true if 'b' is entirely contained in 'a'. Useful for figuring out if
 * colorimetric clipping will occur or not.
 */
bool ff_prim_superset(const AVPrimaryCoefficients *a, const AVPrimaryCoefficients *b);

#endif /* SWSCALE_CSPUTILS_H */
