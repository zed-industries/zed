/*
 * AAC encoder data
 * Copyright (c) 2015 Rostislav Pehlivanov ( atomnuker gmail com )
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

/**
 * @file
 * AAC encoder data
 * @author Rostislav Pehlivanov ( atomnuker gmail com )
 */

#ifndef AVCODEC_AACENCTAB_H
#define AVCODEC_AACENCTAB_H

#include "libavutil/channel_layout.h"
#include "aac.h"
#include "defs.h"

/** Total number of usable codebooks **/
#define CB_TOT 12

/** Total number of codebooks, including special ones **/
#define CB_TOT_ALL 15

#define AAC_MAX_CHANNELS 16

extern const uint8_t *const ff_aac_swb_size_1024[];
extern const int      ff_aac_swb_size_1024_len;
extern const uint8_t *const ff_aac_swb_size_128[];
extern const int      ff_aac_swb_size_128_len;

/* Supported layouts without using a PCE */
static const AVChannelLayout aac_normal_chan_layouts[7] = {
    AV_CHANNEL_LAYOUT_MONO,
    AV_CHANNEL_LAYOUT_STEREO,
    AV_CHANNEL_LAYOUT_SURROUND,
    AV_CHANNEL_LAYOUT_4POINT0,
    AV_CHANNEL_LAYOUT_5POINT0_BACK,
    AV_CHANNEL_LAYOUT_5POINT1_BACK,
    AV_CHANNEL_LAYOUT_7POINT1,
};

/** default channel configurations */
static const uint8_t aac_chan_configs[AAC_MAX_CHANNELS][6] = {
    {1, TYPE_SCE},                                         // 1 channel  - single channel element
    {1, TYPE_CPE},                                         // 2 channels - channel pair
    {2, TYPE_SCE, TYPE_CPE},                               // 3 channels - center + stereo
    {3, TYPE_SCE, TYPE_CPE, TYPE_SCE},                     // 4 channels - front center + stereo + back center
    {3, TYPE_SCE, TYPE_CPE, TYPE_CPE},                     // 5 channels - front center + stereo + back stereo
    {4, TYPE_SCE, TYPE_CPE, TYPE_CPE, TYPE_LFE},           // 6 channels - front center + stereo + back stereo + LFE
    {0},                                                   // 7 channels - invalid without PCE
    {5, TYPE_SCE, TYPE_CPE, TYPE_CPE, TYPE_CPE, TYPE_LFE}, // 8 channels - front center + front stereo + side stereo + back stereo + LFE
};

/**
 * Table to remap channels from libavcodec's default order to AAC order.
 */
static const uint8_t aac_chan_maps[AAC_MAX_CHANNELS][AAC_MAX_CHANNELS] = {
    { 0 },
    { 0, 1 },
    { 2, 0, 1 },
    { 2, 0, 1, 3 },
    { 2, 0, 1, 3, 4 },
    { 2, 0, 1, 4, 5, 3 },
    { 0 },
    { 2, 0, 1, 6, 7, 4, 5, 3 },
};

/** bits needed to code codebook run value for long windows */
static const uint8_t run_value_bits_long[64] = {
     5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,
     5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5, 10,
    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 15
};

/** bits needed to code codebook run value for short windows */
static const uint8_t run_value_bits_short[16] = {
    3, 3, 3, 3, 3, 3, 3, 6, 6, 6, 6, 6, 6, 6, 6, 9
};

/* TNS starting SFBs for long and short windows */
static const uint8_t tns_min_sfb_short[16] = {
    2, 2, 2, 3, 3, 4, 6, 6, 8, 10, 10, 12, 12, 12, 12, 12
};

static const uint8_t tns_min_sfb_long[16] = {
    12, 13, 15, 16, 17, 20, 25, 26, 24, 28, 30, 31, 31, 31, 31, 31
};

static const uint8_t * const tns_min_sfb[2] = {
    tns_min_sfb_long, tns_min_sfb_short
};

static const uint8_t * const run_value_bits[2] = {
    run_value_bits_long, run_value_bits_short
};

/** Map to convert values from BandCodingPath index to a codebook index **/
static const uint8_t aac_cb_out_map[CB_TOT_ALL]  = {0,1,2,3,4,5,6,7,8,9,10,11,13,14,15};
/** Inverse map to convert from codebooks to BandCodingPath indices **/
static const uint8_t aac_cb_in_map[CB_TOT_ALL+1] = {0,1,2,3,4,5,6,7,8,9,10,11,0,12,13,14};

static const uint8_t aac_cb_range [12] = {0, 3, 3, 3, 3, 9, 9, 8, 8, 13, 13, 17};
static const uint8_t aac_cb_maxval[12] = {0, 1, 1, 2, 2, 4, 4, 7, 7, 12, 12, 16};

static const unsigned char aac_maxval_cb[] = {
    0, 1, 3, 5, 5, 7, 7, 7, 9, 9, 9, 9, 9, 11
};

static const int aacenc_profiles[] = {
    AV_PROFILE_AAC_LOW,
    AV_PROFILE_MPEG2_AAC_LOW,
};

#endif /* AVCODEC_AACENCTAB_H */
