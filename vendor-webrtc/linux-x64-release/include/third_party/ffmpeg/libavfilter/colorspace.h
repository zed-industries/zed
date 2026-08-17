/*
 * Copyright (c) 2016 Ronald S. Bultje <rsbultje@gmail.com>
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

#ifndef AVFILTER_COLORSPACE_H
#define AVFILTER_COLORSPACE_H

#include "libavutil/csp.h"
#include "libavutil/frame.h"
#include "libavutil/pixfmt.h"

#define REFERENCE_WHITE 100.0f

void ff_matrix_invert_3x3(const double in[3][3], double out[3][3]);
void ff_matrix_mul_3x3(double dst[3][3],
               const double src1[3][3], const double src2[3][3]);
void ff_matrix_mul_3x3_vec(double dst[3], const double vec[3], const double mat[3][3]);
void ff_fill_rgb2xyz_table(const AVPrimaryCoefficients *coeffs,
                           const AVWhitepointCoefficients *wp,
                           double rgb2xyz[3][3]);
void ff_fill_rgb2yuv_table(const AVLumaCoefficients *coeffs,
                           double rgb2yuv[3][3]);
double ff_determine_signal_peak(AVFrame *in);
void ff_update_hdr_metadata(AVFrame *in, double peak);

#endif
