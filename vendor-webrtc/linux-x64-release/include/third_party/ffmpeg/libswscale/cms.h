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

#ifndef SWSCALE_CMS_H
#define SWSCALE_CMS_H

#include <stdbool.h>

#include "libavutil/csp.h"

#include "csputils.h"
#include "swscale.h"
#include "format.h"

/* Minimum, maximum, and default knee point for perceptual tone mapping [0,1] */
#define PERCEPTUAL_KNEE_MIN 0.10f
#define PERCEPTUAL_KNEE_MAX 0.80f
#define PERCEPTUAL_KNEE_DEF 0.40f

/* Ratio between source average and target average. */
#define PERCEPTUAL_ADAPTATION 0.40f

/* (Relative) chromaticity protection zone for perceptual mapping [0,1] */
#define PERCEPTUAL_DEADZONE 0.30f

/* Contrast setting for perceptual tone mapping. [0,1.5] */
#define PERCEPTUAL_CONTRAST 0.50f

/* Tuning constants for overriding the contrast near extremes */
#define SLOPE_TUNING 1.50f /* [0,10] */
#define SLOPE_OFFSET 0.20f /* [0,1] */

/* Strength of the perceptual saturation mapping component [0,1] */
#define PERCEPTUAL_STRENGTH 0.80f

/* Knee point to use for perceptual soft clipping [0,1] */
#define SOFTCLIP_KNEE 0.70f

/* I vs C curve gamma to use for colorimetric clipping [0,10] */
#define COLORIMETRIC_GAMMA 1.80f

/* Struct describing a color mapping operation */
typedef struct SwsColorMap {
    SwsColor src;
    SwsColor dst;
    SwsIntent intent;
} SwsColorMap;

/**
 * Returns true if the given color map is a semantic no-op - that is,
 * the overall RGB end to end transform would an identity mapping.
 */
bool ff_sws_color_map_noop(const SwsColorMap *map);

/**
 * Generates a single end-to-end color mapping 3DLUT embedding a static tone
 * mapping curve.
 *
 * Returns 0 on success, or a negative error code on failure.
 */
int ff_sws_color_map_generate_static(v3u16_t *lut, int size, const SwsColorMap *map);

/**
 * Generates a split pair of 3DLUTS, going to IPT and back, allowing an
 * arbitrary dynamic EETF to be nestled in between these two operations.
 *
 * See ff_sws_tone_map_generate().
 *
 * Returns 0 on success, or a negative error code on failure.
 */
int ff_sws_color_map_generate_dynamic(v3u16_t *input, v3u16_t *output,
                                      int size_input, int size_I, int size_PT,
                                      const SwsColorMap *map);

/**
 * Generate a 1D LUT of size `size` adapting intensity (I) levels from the
 * source to the destination color space. The LUT is normalized to the
 * relevant intensity range directly. The second channel of each entry returns
 * the corresponding 15-bit scaling factor for the P/T channels. The scaling
 * factor k may be applied as `(1 << 15) - k + (PT * k >> 15)`.
 *
 * This is designed to be used with sws_gamut_map_generate_dynamic().
 *
 * Returns 0 on success, or a negative error code on failure.
 */
void ff_sws_tone_map_generate(v2u16_t *lut, int size, const SwsColorMap *map);

#endif // SWSCALE_GAMUT_MAPPING_H
