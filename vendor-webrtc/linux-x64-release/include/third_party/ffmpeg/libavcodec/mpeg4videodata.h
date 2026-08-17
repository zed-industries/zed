/*
 * MPEG-4 encoder/decoder data.
 * Copyright (c) 2000,2001 Fabrice Bellard
 * Copyright (c) 2002-2010 Michael Niedermayer <michaelni@gmx.at>
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

#ifndef AVCODEC_MPEG4VIDEODATA_H
#define AVCODEC_MPEG4VIDEODATA_H

#include <stdint.h>
#include "rl.h"

/* dc encoding for MPEG-4 */
extern const uint8_t ff_mpeg4_DCtab_lum[13][2];
extern const uint8_t ff_mpeg4_DCtab_chrom[13][2];

extern const uint16_t ff_mpeg4_intra_vlc[103][2];
extern const int8_t ff_mpeg4_intra_level[102];
extern const int8_t ff_mpeg4_intra_run[102];

extern RLTable ff_mpeg4_rl_intra;
void ff_mpeg4_init_rl_intra(void);

/* Note this is identical to the intra rvlc except that it is reordered. */
extern RLTable ff_rvlc_rl_inter;
extern RLTable ff_rvlc_rl_intra;

extern const uint8_t ff_sprite_trajectory_lens[15];
extern const uint8_t ff_mb_type_b_tab[4][2];

/* these matrixes will be permuted for the idct */
extern const int16_t ff_mpeg4_default_intra_matrix[64];
extern const int16_t ff_mpeg4_default_non_intra_matrix[64];

extern const uint8_t ff_mpeg4_y_dc_scale_table[32];
extern const uint8_t ff_mpeg4_c_dc_scale_table[32];

extern const uint8_t ff_mpeg4_dc_threshold[8];

extern const uint8_t ff_mpeg4_studio_dc_luma[19][2];
extern const uint8_t ff_mpeg4_studio_dc_chroma[19][2];
extern const uint8_t ff_mpeg4_studio_intra[12][24][2];

#endif /* AVCODEC_MPEG4VIDEO_H */
