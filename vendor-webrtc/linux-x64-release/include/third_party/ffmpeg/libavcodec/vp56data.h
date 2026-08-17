/*
 * Copyright (C) 2006  Aurelien Jacobs <aurel@gnuage.org>
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
 * VP5 and VP6 compatible video decoder (common data)
 */

#ifndef AVCODEC_VP56DATA_H
#define AVCODEC_VP56DATA_H

#include <stdint.h>
#include "vp56.h"

extern const uint8_t ff_vp56_b2p[];
extern const uint8_t ff_vp56_b6to4[];
extern const uint8_t ff_vp56_coeff_parse_table[6][11];
extern const uint8_t ff_vp56_def_mb_types_stats[3][10][2];
extern const VP56Tree ff_vp56_pva_tree[];
extern const VP56Tree ff_vp56_pc_tree[];
extern const uint8_t ff_vp56_coeff_bias[];
extern const uint8_t ff_vp56_coeff_bit_length[];

extern const VP56Frame ff_vp56_reference_frame[];
extern const uint8_t ff_vp56_ac_dequant[64];
extern const uint8_t ff_vp56_dc_dequant[64];
extern const uint8_t ff_vp56_pre_def_mb_type_stats[16][3][10][2];
extern const uint8_t ff_vp56_filter_threshold[];
extern const uint8_t ff_vp56_mb_type_model_model[];
extern const VP56Tree ff_vp56_pmbtm_tree[];
extern const VP56Tree ff_vp56_pmbt_tree[];
extern const int8_t ff_vp56_candidate_predictor_pos[12][2];

#endif /* AVCODEC_VP56DATA_H */
