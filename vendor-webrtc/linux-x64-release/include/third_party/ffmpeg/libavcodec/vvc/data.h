/*
 * VVC shared tables
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

#ifndef AVCODEC_VVC_DATA_H
#define AVCODEC_VVC_DATA_H

#include <stdint.h>

extern const uint8_t ff_vvc_diag_scan_x[5][5][16 * 16];
extern const uint8_t ff_vvc_diag_scan_y[5][5][16 * 16];

extern const uint8_t ff_vvc_scaling_pred_8[8 * 8];
extern const uint8_t ff_vvc_scaling_pred_16[8 * 8];
extern const int ff_vvc_scaling_list0[8 * 8];

extern const int8_t ff_vvc_dct8_4x4[4][4];
extern const int8_t ff_vvc_dct8_8x8[8][8];
extern const int8_t ff_vvc_dct8_16x16[16][16];
extern const int8_t ff_vvc_dct8_32x32[32][32];
extern const int8_t ff_vvc_dst7_4x4[4][4];
extern const int8_t ff_vvc_dst7_8x8[8][8];
extern const int8_t ff_vvc_dst7_16x16[16][16];
extern const int8_t ff_vvc_dst7_32x32[32][32];
extern const int8_t ff_vvc_lfnst_4x4[4][2][16][16];
extern const int8_t ff_vvc_lfnst_8x8[4][2][16][48];
extern const uint8_t ff_vvc_lfnst_tr_set_index[95];
extern uint8_t ff_vvc_default_scale_m[64 * 64];

#define VVC_INTER_LUMA_FILTER_TYPE_AFFINE   4

#define VVC_INTER_LUMA_FILTER_TYPES         7
#define VVC_INTER_CHROMA_FILTER_TYPES       3

#define VVC_INTER_LUMA_FACTS        16
#define VVC_INTER_LUMA_TAPS          8
#define VVC_INTER_CHROMA_FACTS      32
#define VVC_INTER_CHROMA_TAPS        4
#define VVC_INTER_LUMA_DMVR_FACTS   16
#define VVC_INTER_LUMA_DMVR_TAPS     2
extern const int8_t ff_vvc_inter_luma_filters[VVC_INTER_LUMA_FILTER_TYPES][VVC_INTER_LUMA_FACTS][VVC_INTER_LUMA_TAPS];
extern const int8_t ff_vvc_inter_chroma_filters[VVC_INTER_CHROMA_FILTER_TYPES][VVC_INTER_CHROMA_FACTS][VVC_INTER_CHROMA_TAPS];
extern const int8_t ff_vvc_inter_luma_dmvr_filters[VVC_INTER_LUMA_DMVR_FACTS][VVC_INTER_LUMA_DMVR_TAPS];

#define VVC_INTRA_LUMA_TYPES         2
#define VVC_INTRA_LUMA_FACTS        32
#define VVC_INTRA_LUMA_TAPS          4
extern const int8_t ff_vvc_intra_luma_filter[VVC_INTRA_LUMA_TYPES][VVC_INTRA_LUMA_FACTS][VVC_INTRA_LUMA_TAPS];

#define VVC_GPM_NUM_PARTITION       64
#define VVC_GPM_NUM_ANGLES          32
#define VVC_GPM_WEIGHT_SIZE        112
extern const uint8_t ff_vvc_gpm_angle_idx[VVC_GPM_NUM_PARTITION];
extern const uint8_t ff_vvc_gpm_distance_idx[VVC_GPM_NUM_PARTITION];
extern const  int8_t ff_vvc_gpm_distance_lut[VVC_GPM_NUM_ANGLES];
extern const uint8_t ff_vvc_gpm_angle_to_mirror[VVC_GPM_NUM_ANGLES];
extern const uint8_t ff_vvc_gpm_angle_to_weights_idx[VVC_GPM_NUM_ANGLES];
extern const uint8_t ff_vvc_gpm_weights_offset_x[VVC_GPM_NUM_PARTITION][4][4];
extern const uint8_t ff_vvc_gpm_weights_offset_y[VVC_GPM_NUM_PARTITION][4][4];
extern const uint8_t ff_vvc_gpm_weights[6][VVC_GPM_WEIGHT_SIZE * VVC_GPM_WEIGHT_SIZE];

extern const int16_t  ff_vvc_alf_fix_filt_coeff[64][12];
extern const uint8_t ff_vvc_alf_class_to_filt_map[16][25];
extern const uint8_t ff_vvc_alf_aps_class_to_filt_map[25];

const uint8_t* ff_vvc_get_mip_matrix(const int size_id, const int mode_idx);

#endif /* AVCODEC_VVC_DATA_H */
