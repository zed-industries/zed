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

#ifndef SWSCALE_LUT3D_H
#define SWSCALE_LUT3D_H

#include <stdint.h>

#include "libavutil/csp.h"
#include "libavutil/pixfmt.h"

#include "cms.h"
#include "csputils.h"
#include "format.h"

enum {
    /* Input LUT size. This is only calculated once. */
    INPUT_LUT_BITS = 6,
    INPUT_LUT_SIZE = (1 << INPUT_LUT_BITS) + 1, /* +1 to simplify interpolation */

    /* Tone mapping LUT size. This is regenerated possibly per frame. */
    TONE_LUT_BITS = 8,
    TONE_LUT_SIZE = (1 << TONE_LUT_BITS) + 1,

    /* Output LUT size (for dynamic tone mapping). This is only calculated once. */
    OUTPUT_LUT_BITS_I  = 6,
    OUTPUT_LUT_BITS_PT = 7,

    OUTPUT_LUT_SIZE_I  = (1 << OUTPUT_LUT_BITS_I)  + 1,
    OUTPUT_LUT_SIZE_PT = (1 << OUTPUT_LUT_BITS_PT) + 1,
};

typedef struct SwsLut3D {
    SwsColorMap map;
    bool dynamic;

    /* Gamut mapping 3DLUT(s) */
    v3u16_t  input[INPUT_LUT_SIZE][INPUT_LUT_SIZE][INPUT_LUT_SIZE];
    v3u16_t output[OUTPUT_LUT_SIZE_PT][OUTPUT_LUT_SIZE_PT][OUTPUT_LUT_SIZE_I];

    /* Split tone mapping LUT (for dynamic tone mapping) */
    v2u16_t tone_map[TONE_LUT_SIZE]; /* new luma, desaturation */
} SwsLut3D;

SwsLut3D *ff_sws_lut3d_alloc(void);
void ff_sws_lut3d_free(SwsLut3D **lut3d);

/**
 * Test to see if a given format is supported by the 3DLUT input/output code.
 */
bool ff_sws_lut3d_test_fmt(enum AVPixelFormat fmt, int output);

/**
 * Pick the best compatible pixfmt for a given SwsFormat.
 */
enum AVPixelFormat ff_sws_lut3d_pick_pixfmt(SwsFormat fmt, int output);

/**
 * Recalculate the (static) 3DLUT state with new settings. This will recompute
 * everything. To only update per-frame tone mapping state, instead call
 * ff_sws_lut3d_update().
 *
 * Returns 0 or a negative error code.
 */
int ff_sws_lut3d_generate(SwsLut3D *lut3d, enum AVPixelFormat fmt_in,
                          enum AVPixelFormat fmt_out, const SwsColorMap *map);

/**
 * Update the tone mapping state. This will only use per-frame metadata. The
 * static metadata is ignored.
 */
void ff_sws_lut3d_update(SwsLut3D *lut3d, const SwsColor *new_src);

/**
 * Applies a color transformation to a plane. The format must match the format
 * provided during ff_sws_lut3d_update().
 */
void ff_sws_lut3d_apply(const SwsLut3D *lut3d, const uint8_t *in, int in_stride,
                        uint8_t *out, int out_stride, int w, int h);

#endif /* SWSCALE_LUT3D_H */
