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

#ifndef AVFILTER_PRESERVE_COLOR_H
#define AVFILTER_PRESERVE_COLOR_H

#include <math.h>

#include "libavutil/macros.h"

enum {
    P_NONE,
    P_LUM,
    P_MAX,
    P_AVG,
    P_SUM,
    P_NRM,
    P_PWR,
    NB_PRESERVE
};

static inline float normalize(float r, float g, float b, float max)
{
    r /= max;
    g /= max;
    b /= max;
    return sqrtf(r * r + g * g + b * b);
}

static inline float power(float r, float g, float b, float max)
{
    r /= max;
    g /= max;
    b /= max;
    return cbrtf(r * r * r + g * g * g + b * b * b);
}

static inline void preserve_color(int preserve_color,
                                  float ir, float ig, float ib,
                                  float  r, float  g, float  b,
                                  float max,
                                  float *icolor, float *ocolor)
{
    switch (preserve_color) {
    case P_LUM:
        *icolor = FFMAX3(ir, ig, ib) + FFMIN3(ir, ig, ib);
        *ocolor = FFMAX3( r,  g,  b) + FFMIN3( r,  g,  b);
        break;
    case P_MAX:
        *icolor = FFMAX3(ir, ig, ib);
        *ocolor = FFMAX3( r,  g,  b);
        break;
    case P_AVG:
        *icolor = (ir + ig + ib + 1.f) / 3.f;
        *ocolor = ( r +  g +  b + 1.f) / 3.f;
        break;
    case P_SUM:
        *icolor = ir + ig + ib;
        *ocolor =  r +  g  + b;
        break;
    case P_NRM:
        *icolor = normalize(ir, ig, ib, max);
        *ocolor = normalize( r,  g,  b, max);
        break;
    case P_PWR:
        *icolor = power(ir, ig, ib, max);
        *ocolor = power( r,  g,  b, max);
        break;
    }
}

#endif /* AVFILTER_PRESERVE_COLOR_H */
