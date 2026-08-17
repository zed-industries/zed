/*
 * copyright (c) 2000,2001 Fabrice Bellard
 * H.263+ support
 * copyright (c) 2001 Juan J. Sierralta P
 * copyright (c) 2002-2004 Michael Niedermayer <michaelni@gmx.at>
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
 * H.263 tables.
 */

#ifndef AVCODEC_H263DATA_H
#define AVCODEC_H263DATA_H

#include <stdint.h>

#include "libavutil/rational.h"

#include "rl.h"

extern const AVRational ff_h263_pixel_aspect[16];

/* intra MCBPC, mb_type = (intra), then (intraq) */
extern const uint8_t ff_h263_intra_MCBPC_code[9];
extern const uint8_t ff_h263_intra_MCBPC_bits[9];

/* inter MCBPC, mb_type = (inter), (intra), (interq), (intraq), (inter4v) */
/* Changed the tables for interq and inter4v+q, following the standard ** Juanjo ** */
extern const uint8_t ff_h263_inter_MCBPC_code[28];
extern const uint8_t ff_h263_inter_MCBPC_bits[28];

extern const uint8_t ff_h263_mbtype_b_tab[15][2];

extern const uint8_t ff_cbpc_b_tab[4][2];
extern const uint8_t ff_h263_cbpy_tab[16][2];

extern const uint8_t ff_mvtab[33][2];

/* third non intra table */
extern const uint16_t ff_inter_vlc[103][2];

extern const int8_t ff_inter_level[102];
extern const int8_t ff_inter_run[102];

extern RLTable ff_h263_rl_inter;
extern RLTable ff_rl_intra_aic;

extern const uint16_t ff_h263_format[8][2];

extern const uint8_t ff_aic_dc_scale_table[32];

extern const uint8_t ff_modified_quant_tab[2][32];

extern const uint8_t ff_h263_chroma_qscale_table[32];

extern const uint16_t ff_mba_max[6];
extern const uint8_t ff_mba_length[7];

#endif /* AVCODEC_H263DATA_H */
