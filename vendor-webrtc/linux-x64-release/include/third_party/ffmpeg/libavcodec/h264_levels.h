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

#ifndef AVCODEC_H264_LEVELS_H
#define AVCODEC_H264_LEVELS_H


#include <stdint.h>

typedef struct H264LevelDescriptor {
    char        name[4];    // Large enough for all current levels like "4.1"
    uint8_t     level_idc;
    uint8_t     constraint_set3_flag;
    uint32_t    max_mbps;
    uint32_t    max_fs;
    uint32_t    max_dpb_mbs;
    uint32_t    max_br;
    uint32_t    max_cpb;
    uint16_t    max_v_mv_r;
    uint8_t     min_cr;
    uint8_t     max_mvs_per_2mb;
} H264LevelDescriptor;

/**
 * Guess the level of a stream from some parameters.
 *
 * Unknown parameters may be zero, in which case they are ignored.
 */
const H264LevelDescriptor *ff_h264_guess_level(int profile_idc,
                                               int64_t bitrate,
                                               int framerate,
                                               int width, int height,
                                               int max_dec_frame_buffering);


#endif /* AVCODEC_H264_LEVELS_H */
