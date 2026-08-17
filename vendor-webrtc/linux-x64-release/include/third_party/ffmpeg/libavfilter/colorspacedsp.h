/*
 * Copyright (c) 2016 Ronald S. Bultje <rsbultje@gmail.com>
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

#ifndef AVFILTER_COLORSPACEDSP_H
#define AVFILTER_COLORSPACEDSP_H

#include <stddef.h>
#include <stdint.h>

typedef void (*yuv2rgb_fn)(int16_t *rgb[3], ptrdiff_t rgb_stride,
                           uint8_t *yuv[3], const ptrdiff_t yuv_stride[3],
                           int w, int h, const int16_t yuv2rgb_coeffs[3][3][8],
                           const int16_t yuv_offset[8]);
typedef void (*rgb2yuv_fn)(uint8_t *yuv[3], const ptrdiff_t yuv_stride[3],
                           int16_t *rgb[3], ptrdiff_t rgb_stride,
                           int w, int h, const int16_t rgb2yuv_coeffs[3][3][8],
                           const int16_t yuv_offset[8]);
typedef void (*rgb2yuv_fsb_fn)(uint8_t *yuv[3], const ptrdiff_t yuv_stride[3],
                               int16_t *rgb[3], ptrdiff_t rgb_stride,
                               int w, int h, const int16_t rgb2yuv_coeffs[3][3][8],
                               const int16_t yuv_offset[8],
                               int *rnd[3][2]);
typedef void (*yuv2yuv_fn)(uint8_t *yuv_out[3], const ptrdiff_t yuv_out_stride[3],
                           uint8_t *yuv_in[3], const ptrdiff_t yuv_in_stride[3],
                           int w, int h, const int16_t yuv2yuv_coeffs[3][3][8],
                           const int16_t yuv_offset[2][8]);

enum BitDepthIndex {
    BPP_8,
    BPP_10,
    BPP_12,
    NB_BPP,
};

enum ChromaSubsamplingIndex {
    SS_444,
    SS_422,
    SS_420,
    NB_SS,
};

typedef struct ColorSpaceDSPContext {
    /* Convert input YUV pixel buffer from a user into an internal, 15bpp array
     * of intermediate RGB data. */
    yuv2rgb_fn yuv2rgb[NB_BPP][NB_SS];
    /* Convert intermediate RGB data (15bpp, internal format) into YUV data and
     * store into user-provided output buffer */
    rgb2yuv_fn rgb2yuv[NB_BPP][NB_SS];
    /* Same as rgb2yuv(), but use floyd-steinberg dithering */
    rgb2yuv_fsb_fn rgb2yuv_fsb[NB_BPP][NB_SS];
    /* Direct yuv-to-yuv conversion (input and output are both user-provided
     * buffers) */
    yuv2yuv_fn yuv2yuv[NB_BPP /* in */][NB_BPP /* out */][NB_SS];

    /* In-place 3x3 matrix multiplication. Input and output are both 15bpp
     * (our internal data format) */
    void (*multiply3x3)(int16_t *data[3], ptrdiff_t stride,
                        int w, int h, const int16_t m[3][3][8]);
} ColorSpaceDSPContext;

void ff_colorspacedsp_init(ColorSpaceDSPContext *dsp);

/* internal */
void ff_colorspacedsp_x86_init(ColorSpaceDSPContext *dsp);

#endif /* AVFILTER_COLORSPACEDSP_H */
