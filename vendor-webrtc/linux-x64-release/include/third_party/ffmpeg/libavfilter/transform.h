/*
 * Copyright (C) 2010 Georg Martius <georg.martius@web.de>
 * Copyright (C) 2010 Daniel G. Taylor <dan@programmer-art.org>
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

#ifndef AVFILTER_TRANSFORM_H
#define AVFILTER_TRANSFORM_H

#include <stdint.h>

/**
 * @file
 * transform input video
 *
 * All matrices are defined as a single 9-item block of contiguous memory. For
 * example, the identity matrix would be:
 *
 *     float *matrix = {1, 0, 0,
 *                      0, 1, 0,
 *                      0, 0, 1};
 */

enum InterpolateMethod {
    INTERPOLATE_NEAREST,        //< Nearest-neighbor (fast)
    INTERPOLATE_BILINEAR,       //< Bilinear
    INTERPOLATE_BIQUADRATIC,    //< Biquadratic (best)
    INTERPOLATE_COUNT,          //< Number of interpolation methods
};

// Shortcuts for the fastest and best interpolation methods
#define INTERPOLATE_DEFAULT INTERPOLATE_BILINEAR
#define INTERPOLATE_FAST    INTERPOLATE_NEAREST
#define INTERPOLATE_BEST    INTERPOLATE_BIQUADRATIC

enum FillMethod {
    FILL_BLANK,         //< Fill zeroes at blank locations
    FILL_ORIGINAL,      //< Original image at blank locations
    FILL_CLAMP,         //< Extruded edge value at blank locations
    FILL_MIRROR,        //< Mirrored edge at blank locations
    FILL_COUNT,         //< Number of edge fill methods
};

// Shortcuts for fill methods
#define FILL_DEFAULT FILL_ORIGINAL

/**
 * Get an affine transformation matrix from given translation, rotation, and
 * zoom factors. The matrix will look like:
 *
 * [ scale_x * cos(angle),           -sin(angle),     x_shift,
 *             sin(angle),  scale_y * cos(angle),     y_shift,
 *                      0,                     0,           1 ]
 *
 * @param x_shift   horizontal translation
 * @param y_shift   vertical translation
 * @param angle     rotation in radians
 * @param scale_x   x scale percent (1.0 = 100%)
 * @param scale_y   y scale percent (1.0 = 100%)
 * @param matrix    9-item affine transformation matrix
 */
void ff_get_matrix(
    float x_shift,
    float y_shift,
    float angle,
    float scale_x,
    float scale_y,
    float *matrix
);

/**
 * Do an affine transformation with the given interpolation method. This
 * multiplies each vector [x,y,1] by the matrix and then interpolates to
 * get the final value.
 *
 * @param src         source image
 * @param dst         destination image
 * @param src_stride  source image line size in bytes
 * @param dst_stride  destination image line size in bytes
 * @param width       image width in pixels
 * @param height      image height in pixels
 * @param matrix      9-item affine transformation matrix
 * @param interpolate pixel interpolation method
 * @param fill        edge fill method
 * @return negative on error
 */
int ff_affine_transform(const uint8_t *src, uint8_t *dst,
                        int src_stride, int dst_stride,
                        int width, int height, const float *matrix,
                        enum InterpolateMethod interpolate,
                        enum FillMethod fill);

#endif /* AVFILTER_TRANSFORM_H */
