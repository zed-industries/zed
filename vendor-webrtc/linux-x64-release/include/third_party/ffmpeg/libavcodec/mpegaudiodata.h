/*
 * MPEG Audio common tables
 * copyright (c) 2002 Fabrice Bellard
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
 * mpeg audio layer common tables.
 */

#ifndef AVCODEC_MPEGAUDIODATA_H
#define AVCODEC_MPEGAUDIODATA_H

#include <stdint.h>

#include "config.h"

#include "libavutil/attributes_internal.h"
#include "vlc.h"

#define MODE_EXT_MS_STEREO 2
#define MODE_EXT_I_STEREO  1

FF_VISIBILITY_PUSH_HIDDEN
extern const uint16_t ff_mpa_bitrate_tab[2][3][15];
extern const uint16_t ff_mpa_freq_tab[3];
extern const int ff_mpa_sblimit_table[5];
extern const int ff_mpa_quant_steps[17];
extern const int ff_mpa_quant_bits[17];
extern const unsigned char * const ff_mpa_alloc_tables[5];

#define TABLE_4_3_SIZE ((8191 + 16)*4)
#if CONFIG_HARDCODED_TABLES
extern const int8_t   ff_table_4_3_exp  [TABLE_4_3_SIZE];
extern const uint32_t ff_table_4_3_value[TABLE_4_3_SIZE];
#else
extern int8_t   ff_table_4_3_exp  [TABLE_4_3_SIZE];
extern uint32_t ff_table_4_3_value[TABLE_4_3_SIZE];
#endif

/* VLCs for decoding layer 3 huffman tables */
extern const VLCElem *ff_huff_vlc[16];
extern VLC ff_huff_quad_vlc[2];

/* layer3 scale factor size */
extern const uint8_t ff_slen_table[2][16];
/* number of lsf scale factors for a given size */
extern const uint8_t ff_lsf_nsf_table[6][3][4];
extern const uint8_t ff_mpa_huff_data[32][2];

/* band size tables */
extern const uint8_t ff_band_size_long[9][22];
extern const uint8_t ff_band_size_short[9][13];
/* computed from ff_band_size_long */
extern uint16_t ff_band_index_long[9][23];

extern int16_t *const ff_division_tabs[4];

/* lower 2 bits: modulo 3, higher bits: shift */
extern uint16_t ff_scale_factor_modshift[64];

extern const uint8_t ff_mpa_pretab[2][22];

/* Initialize tables shared between the fixed and
 * floating point MPEG audio decoders. */
void ff_mpegaudiodec_common_init_static(void);
FF_VISIBILITY_POP_HIDDEN

#endif /* AVCODEC_MPEGAUDIODATA_H */
