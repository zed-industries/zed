/*
 * MSMPEG4 backend for encoder and decoder
 * copyright (c) 2001 Fabrice Bellard
 * copyright (c) 2002-2004 Michael Niedermayer <michaelni@gmx.at>
 *
 * msmpeg4v1 & v2 stuff by Michael Niedermayer <michaelni@gmx.at>
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
 * MSMPEG4 data tables.
 */

#ifndef AVCODEC_MSMPEG4DATA_H
#define AVCODEC_MSMPEG4DATA_H

#include <stdint.h>

#include "libavutil/attributes_internal.h"

#include "rl.h"
#include "vlc.h"

/* motion vector table */
typedef struct MVTable {
    const uint16_t *table_mv_code;
    const uint8_t *table_mv_bits;
    const uint8_t *table_mvx;
    const uint8_t *table_mvy;
    uint16_t *table_mv_index; /* encoding: convert mv to index in table_mv */
    const VLCElem *vlc;       /* decoding: vlc */
} MVTable;

FF_VISIBILITY_PUSH_HIDDEN
#define NB_RL_TABLES  6

extern RLTable ff_rl_table[NB_RL_TABLES];

extern uint32_t ff_v2_dc_lum_table[512][2];
extern uint32_t ff_v2_dc_chroma_table[512][2];

extern const uint8_t ff_wmv1_y_dc_scale_table[32];
extern const uint8_t ff_wmv1_c_dc_scale_table[32];
extern const uint8_t ff_old_ff_y_dc_scale_table[32];

#define MSMPEG4_MV_TABLES_NB_ELEMS 1099
extern MVTable ff_mv_tables[2];

extern const uint8_t ff_v2_mb_type[8][2];
extern const uint8_t ff_v2_intra_cbpc[4][2];

extern const uint32_t ff_table_mb_non_intra[128][2];
extern const uint8_t  ff_table_inter_intra[4][2];

#define WMV2_INTER_CBP_TABLE_COUNT 4
extern const uint32_t (* const ff_wmv2_inter_table[WMV2_INTER_CBP_TABLE_COUNT])[2];
FF_VISIBILITY_POP_HIDDEN

#endif /* AVCODEC_MSMPEG4DATA_H */
