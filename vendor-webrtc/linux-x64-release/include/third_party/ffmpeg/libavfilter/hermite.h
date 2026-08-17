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

#ifndef AVFILTER_HERMITE_H
#define AVFILTER_HERMITE_H

static inline double hermite_interpolation(double x, double x0, double x1,
                                    double p0, double p1,
                                    double m0, double m1)
{
    double width = x1 - x0;
    double t = (x - x0) / width;
    double t2, t3;
    double ct0, ct1, ct2, ct3;

    m0 *= width;
    m1 *= width;

    t2 = t*t;
    t3 = t2*t;
    ct0 = p0;
    ct1 = m0;

    ct2 = -3 * p0 - 2 * m0 + 3 * p1 - m1;
    ct3 = 2 * p0 + m0  - 2 * p1 + m1;

    return ct3 * t3 + ct2 * t2 + ct1 * t + ct0;
}

#endif /* AVFILTER_HERMITE_H */
