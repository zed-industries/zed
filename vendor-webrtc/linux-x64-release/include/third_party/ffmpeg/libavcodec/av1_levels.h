/*
 * Copyright (c) 2023 Intel Corporation
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

#ifndef AVCODEC_AV1_LEVELS_H
#define AVCODEC_AV1_LEVELS_H

#include <stdint.h>

typedef struct AV1LevelDescriptor {
    char     name[4];
    uint8_t  level_idx;

    uint32_t max_pic_size;
    uint32_t max_h_size;
    uint32_t max_v_size;
    uint64_t max_display_rate;
    uint64_t max_decode_rate;

    uint32_t max_header_rate;
    float    main_mbps;
    float    high_mbps;
    uint32_t main_cr;
    uint32_t high_cr;
    uint32_t max_tiles;
    uint32_t max_tile_cols;
} AV1LevelDescriptor;

/**
 * Guess the level of a stream from some parameters.
 *
 * Unknown parameters may be zero, in which case they will be ignored.
 */
const AV1LevelDescriptor *ff_av1_guess_level(int64_t bitrate,
                                             int tier,
                                             int width,
                                             int height,
                                             int tile_rows,
                                             int tile_cols,
                                             float fps);

#endif /* AVCODEC_AV1_LEVELS_H */
