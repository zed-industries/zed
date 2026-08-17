/*
 * VVC CABAC decoder
 *
 * Copyright (C) 2022 Nuo Mi
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

#ifndef AVCODEC_VVC_CABAC_H
#define AVCODEC_VVC_CABAC_H

#include "ctu.h"

int ff_vvc_cabac_init(VVCLocalContext *lc, int ctu_idx, int rx, int ry);

//sao
int ff_vvc_sao_merge_flag_decode(VVCLocalContext *lc);
int ff_vvc_sao_type_idx_decode(VVCLocalContext *lc);
int ff_vvc_sao_band_position_decode(VVCLocalContext *lc);
int ff_vvc_sao_offset_abs_decode(VVCLocalContext *lc);
int ff_vvc_sao_offset_sign_decode(VVCLocalContext *lc);
int ff_vvc_sao_eo_class_decode(VVCLocalContext *lc);

//alf
int ff_vvc_alf_ctb_flag(VVCLocalContext *lc, int rx, int ry, int c_idx);
int ff_vvc_alf_use_aps_flag(VVCLocalContext *lc);
int ff_vvc_alf_luma_prev_filter_idx(VVCLocalContext *lc);
int ff_vvc_alf_luma_fixed_filter_idx(VVCLocalContext *lc);
int ff_vvc_alf_ctb_filter_alt_idx(VVCLocalContext *lc, int c_idx, int num_chroma_filters);
int ff_vvc_alf_ctb_cc_idc(VVCLocalContext *lc, int rx, int ry, int idx, int cc_filters_signalled);

//coding_tree
int ff_vvc_split_cu_flag(VVCLocalContext* lc, int x0, int y0, int cb_width, int cb_height,
    int ch_type, const VVCAllowedSplit *a);
VVCSplitMode ff_vvc_split_mode(VVCLocalContext *lc,  int x0, int y0, int cb_width, int cb_height,
    int cqt_depth, int mtt_depth, int ch_type, const VVCAllowedSplit *a);
int ff_vvc_non_inter_flag(VVCLocalContext *lc, int x0, int y0, int ch_type);

//coding unit
int ff_vvc_pred_mode_flag(VVCLocalContext *lc, int is_chroma);
int ff_vvc_pred_mode_plt_flag(VVCLocalContext *lc);
int ff_vvc_intra_bdpcm_luma_flag(VVCLocalContext *lc);
int ff_vvc_intra_bdpcm_luma_dir_flag(VVCLocalContext *lc);
int ff_vvc_intra_bdpcm_chroma_flag(VVCLocalContext *lc);
int ff_vvc_intra_bdpcm_chroma_dir_flag(VVCLocalContext *lc);
int ff_vvc_cu_skip_flag(VVCLocalContext *lc, const uint8_t *cu_skip_flag);
int ff_vvc_pred_mode_ibc_flag(VVCLocalContext *lc, int ch_type);
int ff_vvc_cu_coded_flag(VVCLocalContext *lc);
int ff_vvc_cu_qp_delta_abs(VVCLocalContext *lc);
int ff_vvc_cu_qp_delta_sign_flag(VVCLocalContext *lc);
int ff_vvc_sbt_flag(VVCLocalContext *lc);
int ff_vvc_sbt_quad_flag(VVCLocalContext *lc);
int ff_vvc_sbt_horizontal_flag(VVCLocalContext *lc);
int ff_vvc_sbt_pos_flag(VVCLocalContext *lc);

//intra
int ff_vvc_intra_mip_flag(VVCLocalContext *lc, const uint8_t *intra_mip_flag);
int ff_vvc_intra_mip_transposed_flag(VVCLocalContext *lc);
int ff_vvc_intra_mip_mode(VVCLocalContext *lc);
int ff_vvc_intra_luma_ref_idx(VVCLocalContext *lc);
int ff_vvc_intra_subpartitions_mode_flag(VVCLocalContext *lc);
enum IspType ff_vvc_isp_split_type(VVCLocalContext *lc, int intra_subpartitions_mode_flag);
int ff_vvc_intra_luma_mpm_flag(VVCLocalContext *lc);
int ff_vvc_intra_luma_not_planar_flag(VVCLocalContext *lc, int intra_subpartitions_mode_flag);
int ff_vvc_intra_luma_mpm_idx(VVCLocalContext *lc);
int ff_vvc_intra_luma_mpm_remainder(VVCLocalContext *lc);
int ff_vvc_cclm_mode_flag(VVCLocalContext *lc);
int ff_vvc_cclm_mode_idx(VVCLocalContext *lc);
int ff_vvc_intra_chroma_pred_mode(VVCLocalContext *lc);

//inter
int ff_vvc_general_merge_flag(VVCLocalContext *lc);
int ff_vvc_merge_subblock_flag(VVCLocalContext *lc);
int ff_vvc_merge_subblock_idx(VVCLocalContext *lc, int max_num_subblock_merge_cand);
int ff_vvc_regular_merge_flag(VVCLocalContext *lc, int cu_skip_flag);
int ff_vvc_merge_idx(VVCLocalContext *lc);
int ff_vvc_mmvd_merge_flag(VVCLocalContext *lc);
int ff_vvc_mmvd_cand_flag(VVCLocalContext *lc);
void ff_vvc_mmvd_offset_coding(VVCLocalContext *lc, Mv *mvd_offset, int ph_mmvd_fullpel_only_flag);
int ff_vvc_ciip_flag(VVCLocalContext *lc);
int ff_vvc_merge_gpm_partition_idx(VVCLocalContext *lc);
int ff_vvc_merge_gpm_idx(VVCLocalContext *lc, int idx);
PredFlag ff_vvc_pred_flag(VVCLocalContext *lc, int is_b);
int ff_vvc_inter_affine_flag(VVCLocalContext *lc);
int ff_vvc_cu_affine_type_flag(VVCLocalContext *lc);
int ff_vvc_sym_mvd_flag(VVCLocalContext *lc);
int ff_vvc_ref_idx_lx(VVCLocalContext *lc, uint8_t nb_refs);
int ff_vvc_abs_mvd_greater0_flag(VVCLocalContext *lc);
int ff_vvc_abs_mvd_greater1_flag(VVCLocalContext *lc);
int ff_vvc_abs_mvd_minus2(VVCLocalContext *lc);
int ff_vvc_mvd_sign_flag(VVCLocalContext *lc);
int ff_vvc_mvp_lx_flag(VVCLocalContext *lc);
int ff_vvc_amvr_shift(VVCLocalContext *lc, int inter_affine_flag, PredMode pred_mode, int has_amvr_flag);
int ff_vvc_bcw_idx(VVCLocalContext *lc, int no_backward_pred_flag);

//transform
int ff_vvc_tu_cb_coded_flag(VVCLocalContext *lc);
int ff_vvc_tu_cr_coded_flag(VVCLocalContext *lc, int tu_cb_coded_flag);
int ff_vvc_tu_y_coded_flag(VVCLocalContext *lc);
int ff_vvc_cu_chroma_qp_offset_flag(VVCLocalContext *lc);
int ff_vvc_cu_chroma_qp_offset_idx(VVCLocalContext *lc);
int ff_vvc_tu_joint_cbcr_residual_flag(VVCLocalContext *lc, int tu_cb_coded_flag, int tu_cr_coded_flag);
int ff_vvc_transform_skip_flag(VVCLocalContext *lc, int ctx);
int ff_vvc_residual_coding(VVCLocalContext *lc, TransformBlock *tb);
int ff_vvc_lfnst_idx(VVCLocalContext *lc, int inc);
int ff_vvc_mts_idx(VVCLocalContext *lc);

int ff_vvc_end_of_slice_flag_decode(VVCLocalContext *lc);
int ff_vvc_end_of_tile_one_bit(VVCLocalContext *lc);
int ff_vvc_end_of_subset_one_bit(VVCLocalContext *lc);

#endif //AVCODEC_VVC_CABAC_H
