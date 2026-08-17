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

#ifndef AVCODEC_DVDATA_H
#define AVCODEC_DVDATA_H

#include <stdint.h>

extern const uint8_t ff_dv_zigzag248_direct[64];

extern const uint8_t ff_dv_quant_shifts[22][4];
extern const uint8_t ff_dv_quant_offset[4];

#define NB_DV_VLC 409
/* The number of entries with value zero in ff_dv_vlc_level. */
#define NB_DV_ZERO_LEVEL_ENTRIES 72

extern const uint8_t ff_dv_vlc_len[NB_DV_VLC];
extern const uint8_t ff_dv_vlc_run[NB_DV_VLC];
extern const uint8_t ff_dv_vlc_level[NB_DV_VLC];

#endif /* AVCODEC_DVDATA_H */
