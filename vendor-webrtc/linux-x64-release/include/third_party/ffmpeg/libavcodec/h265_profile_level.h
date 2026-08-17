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

#ifndef AVCODEC_H265_PROFILE_LEVEL_H
#define AVCODEC_H265_PROFILE_LEVEL_H

#include <stdint.h>

#include "cbs_h265.h"


typedef struct H265LevelDescriptor {
    char        name[4];    // Large enough for all current levels like "4.1"
    uint8_t     level_idc;

    // Table A.6.
    uint32_t    max_luma_ps;
    uint32_t    max_cpb_main;
    uint32_t    max_cpb_high;
    uint16_t    max_slice_segments_per_picture;
    uint8_t     max_tile_rows;
    uint8_t     max_tile_cols;

    // Table A.7.
    uint32_t    max_luma_sr;
    uint32_t    max_br_main;
    uint32_t    max_br_high;
    uint8_t     min_cr_base_main;
    uint8_t     min_cr_base_high;
} H265LevelDescriptor;

typedef struct H265ProfileDescriptor {
    const char *name;
    uint8_t profile_idc;
    uint8_t high_throughput;

    // Tables A.2, A.3 and A.5.
    uint8_t max_14bit;
    uint8_t max_12bit;
    uint8_t max_10bit;
    uint8_t max_8bit;
    uint8_t max_422chroma;
    uint8_t max_420chroma;
    uint8_t max_monochrome;
    uint8_t intra;
    uint8_t one_picture_only;
    uint8_t lower_bit_rate;

    // Table A.8.
    uint16_t cpb_vcl_factor;
    uint16_t cpb_nal_factor;
    float format_capability_factor;
    float min_cr_scale_factor;
    uint8_t max_dpb_pic_buf;
} H265ProfileDescriptor;


const H265ProfileDescriptor *ff_h265_get_profile(const H265RawProfileTierLevel *ptl);


/**
 * Guess the level of a stream from some parameters.
 *
 * Unknown parameters may be zero, in which case they are ignored.
 */
const H265LevelDescriptor *ff_h265_guess_level(const H265RawProfileTierLevel *ptl,
                                               int64_t bitrate,
                                               int width, int height,
                                               int slice_segments,
                                               int tile_rows, int tile_cols,
                                               int max_dec_pic_buffering);

#endif /* AVCODEC_H265_PROFILE_LEVEL_H */
