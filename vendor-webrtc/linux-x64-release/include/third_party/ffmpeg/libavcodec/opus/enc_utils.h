/*
 * Opus encoder
 * Copyright (c) 2017 Rostislav Pehlivanov <atomnuker@gmail.com>
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

#ifndef AVCODEC_OPUS_ENC_UTILS_H
#define AVCODEC_OPUS_ENC_UTILS_H

#include <math.h>
#include <string.h>

#include "opus.h"

typedef struct FFBesselFilter {
    float a[3];
    float b[2];
    float x[3];
    float y[3];
} FFBesselFilter;

/* Fills the coefficients, returns 1 if filter will be unstable */
static inline int bessel_reinit(FFBesselFilter *s, float n, float f0, float fs,
                                int highpass)
{
    int unstable;
    float c, cfreq, w0, k1, k2;

    if (!highpass) {
        c = (1.0f/sqrtf(sqrtf(pow(2.0f, 1.0f/n) - 3.0f/4.0f) - 0.5f))/sqrtf(3.0f);
        cfreq = c*f0/fs;
        unstable = (cfreq <= 0.0f || cfreq >= 1.0f/4.0f);
    } else {
        c = sqrtf(3.0f)*sqrtf(sqrtf(pow(2.0f, 1.0f/n) - 3.0f/4.0f) - 0.5f);
        cfreq = 0.5f - c*f0/fs;
        unstable = (cfreq <= 3.0f/8.0f || cfreq >= 1.0f/2.0f);
    }

    w0 = tanf(M_PI*cfreq);
    k1 = 3.0f * w0;
    k2 = 3.0f * w0;

    s->a[0] = k2/(1.0f + k1 + k2);
    s->a[1] = 2.0f * s->a[0];
    s->a[2] = s->a[0];
    s->b[0] = 2.0f * s->a[0] * (1.0f/k2 - 1.0f);
    s->b[1] = 1.0f - (s->a[0] + s->a[1] + s->a[2] + s->b[0]);

    if (highpass) {
        s->a[1] *= -1;
        s->b[0] *= -1;
    }

    return unstable;
}

static inline int bessel_init(FFBesselFilter *s, float n, float f0, float fs,
                              int highpass)
{
    memset(s, 0, sizeof(FFBesselFilter));
    return bessel_reinit(s, n, f0, fs, highpass);
}

static inline float bessel_filter(FFBesselFilter *s, float x)
{
    s->x[2] = s->x[1];
    s->x[1] = s->x[0];
    s->x[0] = x;
    s->y[2] = s->y[1];
    s->y[1] = s->y[0];
    s->y[0] = s->a[0]*s->x[0] + s->a[1]*s->x[1] + s->a[2]*s->x[2] + s->b[0]*s->y[1] + s->b[1]*s->y[2];
    return s->y[0];
}

#endif /* AVCODEC_OPUS_ENC_UTILS_H */
