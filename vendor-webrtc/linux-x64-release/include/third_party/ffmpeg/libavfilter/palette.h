/*
 * Copyright (c) 2020 Björn Ottosson
 * Copyright (c) 2022 Clément Bœsch <u pkh me>
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

#ifndef AVFILTER_PALETTE_H
#define AVFILTER_PALETTE_H

#include <math.h>
#include <stdint.h>

#include "libavutil/attributes.h"

struct Lab {
    int32_t L, a, b;
};

/**
 * Map sRGB 8-bit color component to a 16-bit linear value (gamma
 * expand from electrical to optical value).
 */
int32_t ff_srgb_u8_to_linear_int(uint8_t x);

/**
 * Map a 16-bit linear value to a sRGB 8-bit color component (gamma
 * compressed from optical to electrical value).
 */
uint8_t ff_linear_int_to_srgb_u8(int32_t x);

/**
 * sRGB (non-linear) to OkLab conversion
 * @see https://bottosson.github.io/posts/oklab/
 */
struct Lab ff_srgb_u8_to_oklab_int(uint32_t srgb);

/**
 * OkLab to sRGB (non-linear) conversion
 * @see https://bottosson.github.io/posts/oklab/
 */
uint32_t ff_oklab_int_to_srgb_u8(struct Lab c);

/*
 * lowbias32 hashing from https://nullprogram.com/blog/2018/07/31/
 */
uint32_t ff_lowbias32(uint32_t x);

#endif /* AVFILTER_PALETTE_H */
