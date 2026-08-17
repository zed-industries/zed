/*
 * Copyright (C) 2013 Ronald S. Bultje <rsbultje gmail com>
 * Copyright (C) 2013 Clément Bœsch <u pkh me>
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

#ifndef AVCODEC_VP9DATA_H
#define AVCODEC_VP9DATA_H

#include <stdint.h>

#include "vp9dec.h"

extern const uint8_t ff_vp9_bwh_tab[2][N_BS_SIZES][2];
extern const int8_t ff_vp9_partition_tree[3][2];
extern const uint8_t ff_vp9_default_kf_partition_probs[4][4][3];
extern const int8_t ff_vp9_segmentation_tree[7][2];
extern const int8_t ff_vp9_intramode_tree[9][2];
extern const uint8_t ff_vp9_default_kf_ymode_probs[10][10][9];
extern const uint8_t ff_vp9_default_kf_uvmode_probs[10][9];
extern const int8_t ff_vp9_inter_mode_tree[3][2];
extern const int8_t ff_vp9_filter_tree[2][2];
extern const enum FilterMode ff_vp9_filter_lut[3];
extern const int16_t ff_vp9_dc_qlookup[3][256];
extern const int16_t ff_vp9_ac_qlookup[3][256];
extern const enum TxfmType ff_vp9_intra_txfm_type[14];
extern const int16_t ff_vp9_default_scan_4x4[16];
extern const int16_t ff_vp9_col_scan_4x4[16];
extern const int16_t ff_vp9_row_scan_4x4[16];
extern const int16_t ff_vp9_default_scan_8x8[64];
extern const int16_t ff_vp9_col_scan_8x8[64];
extern const int16_t ff_vp9_row_scan_8x8[64];
extern const int16_t ff_vp9_default_scan_16x16[256];
extern const int16_t ff_vp9_col_scan_16x16[256];
extern const int16_t ff_vp9_row_scan_16x16[256];
extern const int16_t ff_vp9_default_scan_32x32[1024];
extern const int16_t * const ff_vp9_scans[5][4];
extern const int16_t ff_vp9_default_scan_4x4_nb[16][2];
extern const int16_t ff_vp9_col_scan_4x4_nb[16][2];
extern const int16_t ff_vp9_row_scan_4x4_nb[16][2];
extern const int16_t ff_vp9_default_scan_8x8_nb[64][2];
extern const int16_t ff_vp9_col_scan_8x8_nb[64][2];
extern const int16_t ff_vp9_row_scan_8x8_nb[64][2];
extern const int16_t ff_vp9_default_scan_16x16_nb[256][2];
extern const int16_t ff_vp9_col_scan_16x16_nb[256][2];
extern const int16_t ff_vp9_row_scan_16x16_nb[256][2];
extern const int16_t ff_vp9_default_scan_32x32_nb[1024][2];
extern const int16_t (* const ff_vp9_scans_nb[5][4])[2];
extern const uint8_t ff_vp9_model_pareto8[256][8];
extern const ProbContext ff_vp9_default_probs;
extern const uint8_t ff_vp9_default_coef_probs[4][2][2][6][6][3];
extern const int8_t ff_vp9_mv_joint_tree[3][2];
extern const int8_t ff_vp9_mv_class_tree[10][2];
extern const int8_t ff_vp9_mv_fp_tree[3][2];

#endif /* AVCODEC_VP9DATA_H */
