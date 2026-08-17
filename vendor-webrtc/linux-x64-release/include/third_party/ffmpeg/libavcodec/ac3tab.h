/*
 * AC-3 tables
 * Copyright (c) 2000, 2001, 2002 Fabrice Bellard
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

#ifndef AVCODEC_AC3TAB_H
#define AVCODEC_AC3TAB_H

#include <stdint.h>

#include "ac3defs.h"

extern const uint16_t ff_ac3_frame_size_tab[38][3];
extern const uint8_t  ff_ac3_channels_tab[8];
extern const uint16_t ff_ac3_channel_layout_tab[8];
extern const uint8_t  ff_ac3_dec_channel_map[8][2][6];
extern const int      ff_ac3_sample_rate_tab[];
extern const uint16_t ff_ac3_bitrate_tab[19];
extern const uint8_t  ff_ac3_rematrix_band_tab[5];
extern const uint8_t  ff_eac3_default_cpl_band_struct[18];
extern const uint8_t  ff_ac3_bap_tab[64];
extern const uint8_t  ff_ac3_slow_decay_tab[4];
extern const uint8_t  ff_ac3_fast_decay_tab[4];
extern const uint16_t ff_ac3_slow_gain_tab[4];
extern const uint16_t ff_ac3_db_per_bit_tab[4];
extern const int16_t  ff_ac3_floor_tab[8];
extern const uint16_t ff_ac3_fast_gain_tab[8];
extern const uint8_t  ff_ac3_band_start_tab[AC3_CRITICAL_BANDS+1];
extern const uint8_t  ff_ac3_bin_to_band_tab[253];
extern const uint64_t ff_eac3_custom_channel_map_locations[16][2];

#define COMMON_CHANNEL_MAP \
    { { 0, 1,          }, { 0, 1, 2,         } },\
    { { 0,             }, { 0, 1,            } },\
    { { 0, 1,          }, { 0, 1, 2,         } },\
    { { 0, 2, 1,       }, { 0, 2, 1, 3,      } },\
    { { 0, 1, 2,       }, { 0, 1, 3, 2,      } },\
    { { 0, 2, 1, 3,    }, { 0, 2, 1, 4, 3,   } },

#endif /* AVCODEC_AC3TAB_H */
