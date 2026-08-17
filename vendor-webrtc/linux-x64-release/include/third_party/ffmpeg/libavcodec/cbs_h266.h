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

#ifndef AVCODEC_CBS_H266_H
#define AVCODEC_CBS_H266_H

#include <stddef.h>
#include <stdint.h>

#include "cbs_h2645.h"
#include "cbs_sei.h"
#include "vvc.h"

typedef struct H266RawNALUnitHeader {
    uint8_t nuh_layer_id;
    uint8_t nal_unit_type;
    uint8_t nuh_temporal_id_plus1;
    uint8_t nuh_reserved_zero_bit;
} H266RawNALUnitHeader;

typedef struct H266GeneralConstraintsInfo {
    uint8_t gci_present_flag;
    /* general */
    uint8_t gci_intra_only_constraint_flag;
    uint8_t gci_all_layers_independent_constraint_flag;
    uint8_t gci_one_au_only_constraint_flag;

    /* picture format */
    uint8_t gci_sixteen_minus_max_bitdepth_constraint_idc;
    uint8_t gci_three_minus_max_chroma_format_constraint_idc;

    /* NAL unit type related */
    uint8_t gci_no_mixed_nalu_types_in_pic_constraint_flag;
    uint8_t gci_no_trail_constraint_flag;
    uint8_t gci_no_stsa_constraint_flag;
    uint8_t gci_no_rasl_constraint_flag;
    uint8_t gci_no_radl_constraint_flag;
    uint8_t gci_no_idr_constraint_flag;
    uint8_t gci_no_cra_constraint_flag;
    uint8_t gci_no_gdr_constraint_flag;
    uint8_t gci_no_aps_constraint_flag;
    uint8_t gci_no_idr_rpl_constraint_flag;

    /* tile, slice, subpicture partitioning */
    uint8_t gci_one_tile_per_pic_constraint_flag;
    uint8_t gci_pic_header_in_slice_header_constraint_flag;
    uint8_t gci_one_slice_per_pic_constraint_flag;
    uint8_t gci_no_rectangular_slice_constraint_flag;
    uint8_t gci_one_slice_per_subpic_constraint_flag;
    uint8_t gci_no_subpic_info_constraint_flag;

    /* CTU and block partitioning */
    uint8_t gci_three_minus_max_log2_ctu_size_constraint_idc;
    uint8_t gci_no_partition_constraints_override_constraint_flag;
    uint8_t gci_no_mtt_constraint_flag;
    uint8_t gci_no_qtbtt_dual_tree_intra_constraint_flag;

    /* intra */
    uint8_t gci_no_palette_constraint_flag;
    uint8_t gci_no_ibc_constraint_flag;
    uint8_t gci_no_isp_constraint_flag;
    uint8_t gci_no_mrl_constraint_flag;
    uint8_t gci_no_mip_constraint_flag;
    uint8_t gci_no_cclm_constraint_flag;

    /* inter */
    uint8_t gci_no_ref_pic_resampling_constraint_flag;
    uint8_t gci_no_res_change_in_clvs_constraint_flag;
    uint8_t gci_no_weighted_prediction_constraint_flag;
    uint8_t gci_no_ref_wraparound_constraint_flag;
    uint8_t gci_no_temporal_mvp_constraint_flag;
    uint8_t gci_no_sbtmvp_constraint_flag;
    uint8_t gci_no_amvr_constraint_flag;
    uint8_t gci_no_bdof_constraint_flag;
    uint8_t gci_no_smvd_constraint_flag;
    uint8_t gci_no_dmvr_constraint_flag;
    uint8_t gci_no_mmvd_constraint_flag;
    uint8_t gci_no_affine_motion_constraint_flag;
    uint8_t gci_no_prof_constraint_flag;
    uint8_t gci_no_bcw_constraint_flag;
    uint8_t gci_no_ciip_constraint_flag;
    uint8_t gci_no_gpm_constraint_flag;

    /* transform, quantization, residual */
    uint8_t gci_no_luma_transform_size_64_constraint_flag;
    uint8_t gci_no_transform_skip_constraint_flag;
    uint8_t gci_no_bdpcm_constraint_flag;
    uint8_t gci_no_mts_constraint_flag;
    uint8_t gci_no_lfnst_constraint_flag;
    uint8_t gci_no_joint_cbcr_constraint_flag;
    uint8_t gci_no_sbt_constraint_flag;
    uint8_t gci_no_act_constraint_flag;
    uint8_t gci_no_explicit_scaling_list_constraint_flag;
    uint8_t gci_no_dep_quant_constraint_flag;
    uint8_t gci_no_sign_data_hiding_constraint_flag;
    uint8_t gci_no_cu_qp_delta_constraint_flag;
    uint8_t gci_no_chroma_qp_offset_constraint_flag;

    /* loop filter */
    uint8_t gci_no_sao_constraint_flag;
    uint8_t gci_no_alf_constraint_flag;
    uint8_t gci_no_ccalf_constraint_flag;
    uint8_t gci_no_lmcs_constraint_flag;
    uint8_t gci_no_ladf_constraint_flag;
    uint8_t gci_no_virtual_boundaries_constraint_flag;

    uint8_t gci_num_additional_bits;
    uint8_t gci_reserved_bit[255];

    uint8_t gci_all_rap_pictures_constraint_flag;
    uint8_t gci_no_extended_precision_processing_constraint_flag;
    uint8_t gci_no_ts_residual_coding_rice_constraint_flag;
    uint8_t gci_no_rrc_rice_extension_constraint_flag;
    uint8_t gci_no_persistent_rice_adaptation_constraint_flag;
    uint8_t gci_no_reverse_last_sig_coeff_constraint_flag;
} H266GeneralConstraintsInfo;

typedef struct H266RawProfileTierLevel {
    uint8_t  general_profile_idc;
    uint8_t  general_tier_flag;
    uint8_t  general_level_idc;
    uint8_t  ptl_frame_only_constraint_flag;
    uint8_t  ptl_multilayer_enabled_flag;
    H266GeneralConstraintsInfo general_constraints_info;
    uint8_t  ptl_sublayer_level_present_flag[VVC_MAX_SUBLAYERS - 1];
    uint8_t  sublayer_level_idc[VVC_MAX_SUBLAYERS - 1];
    uint8_t  ptl_num_sub_profiles;
    uint32_t general_sub_profile_idc[VVC_MAX_SUB_PROFILES];

    uint8_t  ptl_reserved_zero_bit;
} H266RawProfileTierLevel;

typedef struct H266RawExtensionData {
    uint8_t     *data;
    AVBufferRef *data_ref;
    size_t       bit_length;
} H266RawExtensionData;

typedef struct H266DpbParameters {
    uint8_t dpb_max_dec_pic_buffering_minus1[VVC_MAX_SUBLAYERS];
    uint8_t dpb_max_num_reorder_pics[VVC_MAX_SUBLAYERS];
    uint8_t dpb_max_latency_increase_plus1[VVC_MAX_SUBLAYERS];
} H266DpbParameters;

typedef struct H266RefPicListStruct {
    uint8_t num_ref_entries;
    uint8_t ltrp_in_header_flag;
    uint8_t inter_layer_ref_pic_flag[VVC_MAX_REF_ENTRIES];
    uint8_t st_ref_pic_flag[VVC_MAX_REF_ENTRIES];
    uint8_t abs_delta_poc_st[VVC_MAX_REF_ENTRIES];
    uint8_t strp_entry_sign_flag[VVC_MAX_REF_ENTRIES];
    uint8_t rpls_poc_lsb_lt[VVC_MAX_REF_ENTRIES];
    uint8_t ilrp_idx[VVC_MAX_REF_ENTRIES];
} H266RefPicListStruct;

typedef struct H266RefPicLists {
    uint8_t  rpl_sps_flag[2];
    uint8_t  rpl_idx[2];
    H266RefPicListStruct rpl_ref_list[2];
    uint16_t poc_lsb_lt[2][VVC_MAX_REF_ENTRIES];
    uint8_t  delta_poc_msb_cycle_present_flag[2][VVC_MAX_REF_ENTRIES];
    uint16_t delta_poc_msb_cycle_lt[2][VVC_MAX_REF_ENTRIES];
} H266RefPicLists;

typedef struct H266RawGeneralTimingHrdParameters {
    uint32_t num_units_in_tick;
    uint32_t time_scale;
    uint8_t  general_nal_hrd_params_present_flag;
    uint8_t  general_vcl_hrd_params_present_flag;
    uint8_t  general_same_pic_timing_in_all_ols_flag;
    uint8_t  general_du_hrd_params_present_flag;
    uint8_t  tick_divisor_minus2;
    uint8_t  bit_rate_scale;
    uint8_t  cpb_size_scale;
    uint8_t  cpb_size_du_scale;
    uint8_t  hrd_cpb_cnt_minus1;
} H266RawGeneralTimingHrdParameters;

typedef struct H266RawSubLayerHRDParameters {
    uint32_t bit_rate_value_minus1[VVC_MAX_SUBLAYERS][VVC_MAX_CPB_CNT];
    uint32_t cpb_size_value_minus1[VVC_MAX_SUBLAYERS][VVC_MAX_CPB_CNT];
    uint32_t cpb_size_du_value_minus1[VVC_MAX_SUBLAYERS][VVC_MAX_CPB_CNT];
    uint32_t bit_rate_du_value_minus1[VVC_MAX_SUBLAYERS][VVC_MAX_CPB_CNT];
    uint8_t  cbr_flag[VVC_MAX_SUBLAYERS][VVC_MAX_CPB_CNT];
} H266RawSubLayerHRDParameters;

typedef struct H266RawOlsTimingHrdParameters {
    uint8_t  fixed_pic_rate_general_flag[VVC_MAX_SUBLAYERS];
    uint8_t  fixed_pic_rate_within_cvs_flag[VVC_MAX_SUBLAYERS];
    uint16_t elemental_duration_in_tc_minus1[VVC_MAX_SUBLAYERS];
    uint8_t  low_delay_hrd_flag[VVC_MAX_SUBLAYERS];
    H266RawSubLayerHRDParameters nal_sub_layer_hrd_parameters;
    H266RawSubLayerHRDParameters vcl_sub_layer_hrd_parameters;
} H266RawOlsTimingHrdParameters;

typedef struct H266RawVUI {
    uint8_t  vui_progressive_source_flag;
    uint8_t  vui_interlaced_source_flag;
    uint8_t  vui_non_packed_constraint_flag;
    uint8_t  vui_non_projected_constraint_flag;

    uint8_t  vui_aspect_ratio_info_present_flag;
    uint8_t  vui_aspect_ratio_constant_flag;
    uint8_t  vui_aspect_ratio_idc;

    uint16_t vui_sar_width;
    uint16_t vui_sar_height;

    uint8_t  vui_overscan_info_present_flag;
    uint8_t  vui_overscan_appropriate_flag;

    uint8_t  vui_colour_description_present_flag;
    uint8_t  vui_colour_primaries;

    uint8_t  vui_transfer_characteristics;
    uint8_t  vui_matrix_coeffs;
    uint8_t  vui_full_range_flag;

    uint8_t  vui_chroma_loc_info_present_flag;
    uint8_t  vui_chroma_sample_loc_type_frame;
    uint8_t  vui_chroma_sample_loc_type_top_field;
    uint8_t  vui_chroma_sample_loc_type_bottom_field;
    H266RawExtensionData extension_data;
} H266RawVUI;

typedef struct H266RawOPI {
    H266RawNALUnitHeader nal_unit_header;

    uint8_t opi_ols_info_present_flag;
    uint8_t opi_htid_info_present_flag;
    uint16_t opi_ols_idx;
    uint8_t opi_htid_plus1;
    uint8_t opi_extension_flag;
    H266RawExtensionData extension_data;
} H266RawOPI;

typedef struct H266RawDCI {
    H266RawNALUnitHeader nal_unit_header;

    uint8_t dci_reserved_zero_4bits;
    uint8_t dci_num_ptls_minus1;
    H266RawProfileTierLevel dci_profile_tier_level[VVC_MAX_DCI_PTLS];
    uint8_t dci_extension_flag;
    H266RawExtensionData extension_data;
} H266RawDCI;

typedef struct H266RawVPS {
    H266RawNALUnitHeader nal_unit_header;

    uint8_t  vps_video_parameter_set_id;
    uint8_t  vps_max_layers_minus1;
    uint8_t  vps_max_sublayers_minus1;
    uint8_t  vps_default_ptl_dpb_hrd_max_tid_flag;
    uint8_t  vps_all_independent_layers_flag;
    uint8_t  vps_layer_id[VVC_MAX_LAYERS];
    uint8_t  vps_independent_layer_flag[VVC_MAX_LAYERS];
    uint8_t  vps_max_tid_ref_present_flag[VVC_MAX_LAYERS];
    uint8_t  vps_direct_ref_layer_flag[VVC_MAX_LAYERS][VVC_MAX_LAYERS - 1];
    uint8_t  vps_max_tid_il_ref_pics_plus1[VVC_MAX_LAYERS][VVC_MAX_LAYERS - 1];
    uint8_t  vps_each_layer_is_an_ols_flag;
    uint8_t  vps_ols_mode_idc;
    uint8_t  vps_num_output_layer_sets_minus2;
    uint8_t  vps_ols_output_layer_flag[VVC_MAX_TOTAL_NUM_OLSS][VVC_MAX_LAYERS];

    uint8_t  vps_num_ptls_minus1;
    uint8_t  vps_pt_present_flag[VVC_MAX_PTLS];
    uint8_t  vps_ptl_max_tid[VVC_MAX_PTLS];
    H266RawProfileTierLevel vps_profile_tier_level[VVC_MAX_PTLS];
    uint8_t  vps_ols_ptl_idx[VVC_MAX_TOTAL_NUM_OLSS];

    uint16_t vps_num_dpb_params_minus1;
    uint8_t  vps_sublayer_dpb_params_present_flag;
    uint8_t  vps_dpb_max_tid[VVC_MAX_TOTAL_NUM_OLSS];
    H266DpbParameters vps_dpb_params[VVC_MAX_TOTAL_NUM_OLSS];
    uint16_t vps_ols_dpb_pic_width[VVC_MAX_TOTAL_NUM_OLSS];
    uint16_t vps_ols_dpb_pic_height[VVC_MAX_TOTAL_NUM_OLSS];
    uint8_t  vps_ols_dpb_chroma_format[VVC_MAX_TOTAL_NUM_OLSS];
    uint8_t  vps_ols_dpb_bitdepth_minus8[VVC_MAX_TOTAL_NUM_OLSS];
    uint16_t vps_ols_dpb_params_idx[VVC_MAX_TOTAL_NUM_OLSS];

    uint8_t  vps_timing_hrd_params_present_flag;
    H266RawGeneralTimingHrdParameters vps_general_timing_hrd_parameters;
    uint8_t  vps_sublayer_cpb_params_present_flag;
    uint16_t vps_num_ols_timing_hrd_params_minus1;
    uint8_t  vps_hrd_max_tid[VVC_MAX_TOTAL_NUM_OLSS];
    H266RawOlsTimingHrdParameters vps_ols_timing_hrd_parameters;
    uint8_t  vps_ols_timing_hrd_idx[VVC_MAX_TOTAL_NUM_OLSS];

    uint8_t  vps_extension_flag;
    H266RawExtensionData extension_data;
} H266RawVPS;

typedef struct H266RawSPS {
    H266RawNALUnitHeader nal_unit_header;

    uint8_t  sps_seq_parameter_set_id;
    uint8_t  sps_video_parameter_set_id;
    uint8_t  sps_max_sublayers_minus1;
    uint8_t  sps_chroma_format_idc;
    uint8_t  sps_log2_ctu_size_minus5;
    uint8_t  sps_ptl_dpb_hrd_params_present_flag;
    H266RawProfileTierLevel profile_tier_level;
    uint8_t  sps_gdr_enabled_flag;
    uint8_t  sps_ref_pic_resampling_enabled_flag;
    uint8_t  sps_res_change_in_clvs_allowed_flag;

    uint16_t sps_pic_width_max_in_luma_samples;
    uint16_t sps_pic_height_max_in_luma_samples;

    uint8_t  sps_conformance_window_flag;
    uint16_t sps_conf_win_left_offset;
    uint16_t sps_conf_win_right_offset;
    uint16_t sps_conf_win_top_offset;
    uint16_t sps_conf_win_bottom_offset;

    uint8_t  sps_subpic_info_present_flag;
    uint16_t sps_num_subpics_minus1;
    uint8_t  sps_independent_subpics_flag;
    uint8_t  sps_subpic_same_size_flag;
    uint16_t sps_subpic_ctu_top_left_x[VVC_MAX_SLICES];
    uint16_t sps_subpic_ctu_top_left_y[VVC_MAX_SLICES];
    uint16_t sps_subpic_width_minus1[VVC_MAX_SLICES];
    uint16_t sps_subpic_height_minus1[VVC_MAX_SLICES];
    uint8_t  sps_subpic_treated_as_pic_flag[VVC_MAX_SLICES];
    uint8_t  sps_loop_filter_across_subpic_enabled_flag[VVC_MAX_SLICES];
    uint8_t  sps_subpic_id_len_minus1;
    uint8_t  sps_subpic_id_mapping_explicitly_signalled_flag;
    uint8_t  sps_subpic_id_mapping_present_flag;
    uint32_t sps_subpic_id[VVC_MAX_SLICES];


    uint8_t  sps_bitdepth_minus8;
    uint8_t  sps_entropy_coding_sync_enabled_flag;
    uint8_t  sps_entry_point_offsets_present_flag;

    uint8_t  sps_log2_max_pic_order_cnt_lsb_minus4;
    uint8_t  sps_poc_msb_cycle_flag;
    uint8_t  sps_poc_msb_cycle_len_minus1;

    uint8_t  sps_num_extra_ph_bytes;
    uint8_t  sps_extra_ph_bit_present_flag[16];

    uint8_t  sps_num_extra_sh_bytes;
    uint8_t  sps_extra_sh_bit_present_flag[16];

    uint8_t  sps_sublayer_dpb_params_flag;
    H266DpbParameters sps_dpb_params;

    uint8_t  sps_log2_min_luma_coding_block_size_minus2;
    uint8_t  sps_partition_constraints_override_enabled_flag;
    uint8_t  sps_log2_diff_min_qt_min_cb_intra_slice_luma;
    uint8_t  sps_max_mtt_hierarchy_depth_intra_slice_luma;
    uint8_t  sps_log2_diff_max_bt_min_qt_intra_slice_luma;
    uint8_t  sps_log2_diff_max_tt_min_qt_intra_slice_luma;

    uint8_t  sps_qtbtt_dual_tree_intra_flag;
    uint8_t  sps_log2_diff_min_qt_min_cb_intra_slice_chroma;
    uint8_t  sps_max_mtt_hierarchy_depth_intra_slice_chroma;
    uint8_t  sps_log2_diff_max_bt_min_qt_intra_slice_chroma;
    uint8_t  sps_log2_diff_max_tt_min_qt_intra_slice_chroma;

    uint8_t  sps_log2_diff_min_qt_min_cb_inter_slice;
    uint8_t  sps_max_mtt_hierarchy_depth_inter_slice;
    uint8_t  sps_log2_diff_max_bt_min_qt_inter_slice;
    uint8_t  sps_log2_diff_max_tt_min_qt_inter_slice;

    uint8_t  sps_max_luma_transform_size_64_flag;

    uint8_t  sps_transform_skip_enabled_flag;
    uint8_t  sps_log2_transform_skip_max_size_minus2;
    uint8_t  sps_bdpcm_enabled_flag;

    uint8_t  sps_mts_enabled_flag;
    uint8_t  sps_explicit_mts_intra_enabled_flag;
    uint8_t  sps_explicit_mts_inter_enabled_flag;

    uint8_t  sps_lfnst_enabled_flag;

    uint8_t  sps_joint_cbcr_enabled_flag;
    uint8_t  sps_same_qp_table_for_chroma_flag;

    int8_t   sps_qp_table_start_minus26[VVC_MAX_SAMPLE_ARRAYS];
    uint8_t  sps_num_points_in_qp_table_minus1[VVC_MAX_SAMPLE_ARRAYS];
    uint8_t  sps_delta_qp_in_val_minus1[VVC_MAX_SAMPLE_ARRAYS][VVC_MAX_POINTS_IN_QP_TABLE];
    uint8_t  sps_delta_qp_diff_val[VVC_MAX_SAMPLE_ARRAYS][VVC_MAX_POINTS_IN_QP_TABLE];

    uint8_t  sps_sao_enabled_flag;
    uint8_t  sps_alf_enabled_flag;
    uint8_t  sps_ccalf_enabled_flag;
    uint8_t  sps_lmcs_enabled_flag;
    uint8_t  sps_weighted_pred_flag;
    uint8_t  sps_weighted_bipred_flag;
    uint8_t  sps_long_term_ref_pics_flag;
    uint8_t  sps_inter_layer_prediction_enabled_flag;
    uint8_t  sps_idr_rpl_present_flag;
    uint8_t  sps_rpl1_same_as_rpl0_flag;

    uint8_t  sps_num_ref_pic_lists[2];
    H266RefPicListStruct sps_ref_pic_list_struct[2][VVC_MAX_REF_PIC_LISTS];

    uint8_t  sps_ref_wraparound_enabled_flag;
    uint8_t  sps_temporal_mvp_enabled_flag;
    uint8_t  sps_sbtmvp_enabled_flag;
    uint8_t  sps_amvr_enabled_flag;
    uint8_t  sps_bdof_enabled_flag;
    uint8_t  sps_bdof_control_present_in_ph_flag;
    uint8_t  sps_smvd_enabled_flag;
    uint8_t  sps_dmvr_enabled_flag;
    uint8_t  sps_dmvr_control_present_in_ph_flag;
    uint8_t  sps_mmvd_enabled_flag;
    uint8_t  sps_mmvd_fullpel_only_enabled_flag;
    uint8_t  sps_six_minus_max_num_merge_cand;
    uint8_t  sps_sbt_enabled_flag;
    uint8_t  sps_affine_enabled_flag;
    uint8_t  sps_five_minus_max_num_subblock_merge_cand;
    uint8_t  sps_6param_affine_enabled_flag;
    uint8_t  sps_affine_amvr_enabled_flag;
    uint8_t  sps_affine_prof_enabled_flag;
    uint8_t  sps_prof_control_present_in_ph_flag;
    uint8_t  sps_bcw_enabled_flag;
    uint8_t  sps_ciip_enabled_flag;
    uint8_t  sps_gpm_enabled_flag;
    uint8_t  sps_max_num_merge_cand_minus_max_num_gpm_cand;
    uint8_t  sps_log2_parallel_merge_level_minus2;
    uint8_t  sps_isp_enabled_flag;
    uint8_t  sps_mrl_enabled_flag;
    uint8_t  sps_mip_enabled_flag;
    uint8_t  sps_cclm_enabled_flag;
    uint8_t  sps_chroma_horizontal_collocated_flag;
    uint8_t  sps_chroma_vertical_collocated_flag;
    uint8_t  sps_palette_enabled_flag;
    uint8_t  sps_act_enabled_flag;
    uint8_t  sps_min_qp_prime_ts;
    uint8_t  sps_ibc_enabled_flag;
    uint8_t  sps_six_minus_max_num_ibc_merge_cand;
    uint8_t  sps_ladf_enabled_flag;
    uint8_t  sps_num_ladf_intervals_minus2;
    int8_t   sps_ladf_lowest_interval_qp_offset;
    int8_t   sps_ladf_qp_offset[4];
    uint16_t sps_ladf_delta_threshold_minus1[4];

    uint8_t  sps_explicit_scaling_list_enabled_flag;
    uint8_t  sps_scaling_matrix_for_lfnst_disabled_flag;
    uint8_t  sps_scaling_matrix_for_alternative_colour_space_disabled_flag;
    uint8_t  sps_scaling_matrix_designated_colour_space_flag;
    uint8_t  sps_dep_quant_enabled_flag;
    uint8_t  sps_sign_data_hiding_enabled_flag;

    uint8_t  sps_virtual_boundaries_enabled_flag;
    uint8_t  sps_virtual_boundaries_present_flag;
    uint8_t  sps_num_ver_virtual_boundaries;
    uint16_t sps_virtual_boundary_pos_x_minus1[VVC_MAX_VBS];
    uint8_t  sps_num_hor_virtual_boundaries;
    uint16_t sps_virtual_boundary_pos_y_minus1[VVC_MAX_VBS];

    uint8_t  sps_timing_hrd_params_present_flag;
    uint8_t  sps_sublayer_cpb_params_present_flag;
    H266RawGeneralTimingHrdParameters sps_general_timing_hrd_parameters;
    H266RawOlsTimingHrdParameters sps_ols_timing_hrd_parameters;

    uint8_t  sps_field_seq_flag;
    uint8_t  sps_vui_parameters_present_flag;
    uint16_t sps_vui_payload_size_minus1;
    H266RawVUI vui;

    uint8_t  sps_extension_flag;

    uint8_t  sps_range_extension_flag;
    uint8_t  sps_extension_7bits;

    uint8_t  sps_extended_precision_flag;
    uint8_t  sps_ts_residual_coding_rice_present_in_sh_flag;
    uint8_t  sps_rrc_rice_extension_flag;
    uint8_t  sps_persistent_rice_adaptation_enabled_flag;
    uint8_t  sps_reverse_last_sig_coeff_enabled_flag;

    H266RawExtensionData extension_data;

} H266RawSPS;

typedef struct H266RawPPS {
    H266RawNALUnitHeader nal_unit_header;

    uint8_t  pps_pic_parameter_set_id;
    uint8_t  pps_seq_parameter_set_id;
    uint8_t  pps_mixed_nalu_types_in_pic_flag;
    uint16_t pps_pic_width_in_luma_samples;
    uint16_t pps_pic_height_in_luma_samples;

    uint8_t  pps_conformance_window_flag;
    uint16_t pps_conf_win_left_offset;
    uint16_t pps_conf_win_right_offset;
    uint16_t pps_conf_win_top_offset;
    uint16_t pps_conf_win_bottom_offset;

    uint8_t  pps_scaling_window_explicit_signalling_flag;
    int16_t  pps_scaling_win_left_offset;
    int16_t  pps_scaling_win_right_offset;
    int16_t  pps_scaling_win_top_offset;
    int16_t  pps_scaling_win_bottom_offset;

    uint8_t  pps_output_flag_present_flag;
    uint8_t  pps_no_pic_partition_flag;

    uint8_t  pps_subpic_id_mapping_present_flag;
    uint16_t pps_num_subpics_minus1;
    uint8_t  pps_subpic_id_len_minus1;
    uint16_t pps_subpic_id[VVC_MAX_SLICES];

    uint8_t  pps_log2_ctu_size_minus5;
    uint8_t  pps_num_exp_tile_columns_minus1;
    uint8_t  pps_num_exp_tile_rows_minus1;
    uint16_t pps_tile_column_width_minus1[VVC_MAX_TILE_COLUMNS];
    uint16_t pps_tile_row_height_minus1[VVC_MAX_TILE_ROWS];

    uint8_t  pps_loop_filter_across_tiles_enabled_flag;
    uint8_t  pps_rect_slice_flag;
    uint8_t  pps_single_slice_per_subpic_flag;

    uint16_t pps_num_slices_in_pic_minus1;
    uint8_t  pps_tile_idx_delta_present_flag;
    uint16_t pps_slice_width_in_tiles_minus1[VVC_MAX_SLICES];
    uint16_t pps_slice_height_in_tiles_minus1[VVC_MAX_SLICES];
    uint16_t pps_num_exp_slices_in_tile[VVC_MAX_SLICES];
    uint16_t pps_exp_slice_height_in_ctus_minus1[VVC_MAX_SLICES][VVC_MAX_TILE_ROWS];
    int16_t  pps_tile_idx_delta_val[VVC_MAX_SLICES];

    uint8_t  pps_loop_filter_across_slices_enabled_flag;
    uint8_t  pps_cabac_init_present_flag;
    uint8_t  pps_num_ref_idx_default_active_minus1[2];
    uint8_t  pps_rpl1_idx_present_flag;
    uint8_t  pps_weighted_pred_flag;
    uint8_t  pps_weighted_bipred_flag;
    uint8_t  pps_ref_wraparound_enabled_flag;
    uint16_t  pps_pic_width_minus_wraparound_offset;
    int8_t   pps_init_qp_minus26;
    uint8_t  pps_cu_qp_delta_enabled_flag;
    uint8_t  pps_chroma_tool_offsets_present_flag;
    int8_t   pps_cb_qp_offset;
    int8_t   pps_cr_qp_offset;
    uint8_t  pps_joint_cbcr_qp_offset_present_flag;
    int8_t   pps_joint_cbcr_qp_offset_value;
    uint8_t  pps_slice_chroma_qp_offsets_present_flag;
    uint8_t  pps_cu_chroma_qp_offset_list_enabled_flag;
    uint8_t  pps_chroma_qp_offset_list_len_minus1;
    int8_t   pps_cb_qp_offset_list[6];
    int8_t   pps_cr_qp_offset_list[6];
    int8_t   pps_joint_cbcr_qp_offset_list[6];
    uint8_t  pps_deblocking_filter_control_present_flag;
    uint8_t  pps_deblocking_filter_override_enabled_flag;
    uint8_t  pps_deblocking_filter_disabled_flag;
    uint8_t  pps_dbf_info_in_ph_flag;

    int8_t   pps_luma_beta_offset_div2;
    int8_t   pps_luma_tc_offset_div2;
    int8_t   pps_cb_beta_offset_div2;
    int8_t   pps_cb_tc_offset_div2;
    int8_t   pps_cr_beta_offset_div2;
    int8_t   pps_cr_tc_offset_div2;

    uint8_t  pps_rpl_info_in_ph_flag;
    uint8_t  pps_sao_info_in_ph_flag;
    uint8_t  pps_alf_info_in_ph_flag;
    uint8_t  pps_wp_info_in_ph_flag;
    uint8_t  pps_qp_delta_info_in_ph_flag;

    uint8_t  pps_picture_header_extension_present_flag;
    uint8_t  pps_slice_header_extension_present_flag;
    uint8_t  pps_extension_flag;
    H266RawExtensionData extension_data;

    //calculated value;
    uint16_t num_tile_columns;
    uint16_t num_tile_rows;
    uint16_t num_tiles_in_pic;
    uint16_t slice_height_in_ctus[VVC_MAX_SLICES];          ///< sliceHeightInCtus
    uint16_t num_slices_in_subpic[VVC_MAX_SLICES];          ///< NumSlicesInSubpic
    uint16_t sub_pic_id_val[VVC_MAX_SLICES];                ///< SubpicIdVal
    uint16_t col_width_val[VVC_MAX_TILE_COLUMNS];           ///< ColWidthVal
    uint16_t row_height_val[VVC_MAX_TILE_ROWS];             ///< RowHeightVal
    uint16_t slice_top_left_tile_idx[VVC_MAX_SLICES];
    uint16_t num_slices_in_tile[VVC_MAX_SLICES];
} H266RawPPS;

typedef struct H266RawAPS {
    H266RawNALUnitHeader nal_unit_header;
    uint8_t aps_params_type;
    uint8_t aps_adaptation_parameter_set_id;
    uint8_t aps_chroma_present_flag;

    uint8_t alf_luma_filter_signal_flag;
    uint8_t alf_chroma_filter_signal_flag;
    uint8_t alf_cc_cb_filter_signal_flag;
    uint8_t alf_cc_cr_filter_signal_flag;
    uint8_t alf_luma_clip_flag;
    uint8_t alf_luma_num_filters_signalled_minus1;
    uint8_t alf_luma_coeff_delta_idx[VVC_NUM_ALF_FILTERS];
    uint8_t alf_luma_coeff_abs[VVC_NUM_ALF_FILTERS][12];
    uint8_t alf_luma_coeff_sign[VVC_NUM_ALF_FILTERS][12];
    uint8_t alf_luma_clip_idx[VVC_NUM_ALF_FILTERS][12];
    uint8_t alf_chroma_clip_flag;
    uint8_t alf_chroma_num_alt_filters_minus1;
    uint8_t alf_chroma_coeff_abs[8][6];
    uint8_t alf_chroma_coeff_sign[8][6];
    uint8_t alf_chroma_clip_idx[8][6];
    uint8_t alf_cc_cb_filters_signalled_minus1;
    uint8_t alf_cc_cb_mapped_coeff_abs[4][7];
    uint8_t alf_cc_cb_coeff_sign[4][7];
    uint8_t alf_cc_cr_filters_signalled_minus1;
    uint8_t alf_cc_cr_mapped_coeff_abs[4][7];
    uint8_t alf_cc_cr_coeff_sign[4][7];

    uint8_t scaling_list_copy_mode_flag[28];
    uint8_t scaling_list_pred_mode_flag[28];
    uint8_t scaling_list_pred_id_delta[28];
    int8_t  scaling_list_dc_coef[14];
    int8_t  scaling_list_delta_coef[28][64];

    uint8_t lmcs_min_bin_idx;
    uint8_t lmcs_delta_max_bin_idx;
    uint8_t lmcs_delta_cw_prec_minus1;
    uint16_t lmcs_delta_abs_cw[16];
    uint8_t lmcs_delta_sign_cw_flag[16];
    uint8_t lmcs_delta_abs_crs;
    uint8_t lmcs_delta_sign_crs_flag;

    uint8_t aps_extension_flag;
    H266RawExtensionData extension_data;
} H266RawAPS;

typedef struct H266RawAUD {
    H266RawNALUnitHeader nal_unit_header;
    uint8_t aud_irap_or_gdr_flag;
    uint8_t aud_pic_type;
} H266RawAUD;

typedef struct H266RawPredWeightTable {
    uint8_t  luma_log2_weight_denom;
    int8_t   delta_chroma_log2_weight_denom;

    uint8_t  num_l0_weights;
    uint8_t  luma_weight_l0_flag[15];
    uint8_t  chroma_weight_l0_flag[15];
    int8_t   delta_luma_weight_l0[15];
    int8_t   luma_offset_l0[15];
    int8_t   delta_chroma_weight_l0[15][2];
    int16_t  delta_chroma_offset_l0[15][2];

    uint8_t  num_l1_weights;
    uint8_t  luma_weight_l1_flag[15];
    uint8_t  chroma_weight_l1_flag[15];
    int8_t   delta_luma_weight_l1[15];
    int8_t   luma_offset_l1[15];
    int8_t   delta_chroma_weight_l1[15][2];
    int16_t  delta_chroma_offset_l1[15][2];

    uint8_t num_weights_l0;         ///< NumWeightsL0
    uint8_t num_weights_l1;         ///< NumWeightsL1
} H266RawPredWeightTable;

typedef struct  H266RawPictureHeader {
    uint8_t  ph_gdr_or_irap_pic_flag;
    uint8_t  ph_non_ref_pic_flag;
    uint8_t  ph_gdr_pic_flag;
    uint8_t  ph_inter_slice_allowed_flag;
    uint8_t  ph_intra_slice_allowed_flag;
    uint8_t  ph_pic_parameter_set_id;
    uint16_t ph_pic_order_cnt_lsb;
    uint8_t  ph_recovery_poc_cnt;
    uint8_t  ph_extra_bit[16];
    uint8_t  ph_poc_msb_cycle_present_flag;
    uint8_t  ph_poc_msb_cycle_val;

    uint8_t  ph_alf_enabled_flag;
    uint8_t  ph_num_alf_aps_ids_luma;
    uint8_t  ph_alf_aps_id_luma[8];
    uint8_t  ph_alf_cb_enabled_flag;
    uint8_t  ph_alf_cr_enabled_flag;
    uint8_t  ph_alf_aps_id_chroma;
    uint8_t  ph_alf_cc_cb_enabled_flag;
    uint8_t  ph_alf_cc_cb_aps_id;
    uint8_t  ph_alf_cc_cr_enabled_flag;
    uint8_t  ph_alf_cc_cr_aps_id;

    uint8_t  ph_lmcs_enabled_flag;
    uint8_t  ph_lmcs_aps_id;
    uint8_t  ph_chroma_residual_scale_flag;
    uint8_t  ph_explicit_scaling_list_enabled_flag;
    uint8_t  ph_scaling_list_aps_id;

    uint8_t  ph_virtual_boundaries_present_flag;
    uint8_t  ph_num_ver_virtual_boundaries;
    uint16_t ph_virtual_boundary_pos_x_minus1[VVC_MAX_VBS];
    uint8_t  ph_num_hor_virtual_boundaries;
    uint16_t ph_virtual_boundary_pos_y_minus1[VVC_MAX_VBS];

    uint8_t  ph_pic_output_flag;
    H266RefPicLists ph_ref_pic_lists;

    uint8_t  ph_partition_constraints_override_flag;

    uint8_t  ph_log2_diff_min_qt_min_cb_intra_slice_luma;
    uint8_t  ph_max_mtt_hierarchy_depth_intra_slice_luma;
    uint8_t  ph_log2_diff_max_bt_min_qt_intra_slice_luma;
    uint8_t  ph_log2_diff_max_tt_min_qt_intra_slice_luma;
    uint8_t  ph_log2_diff_min_qt_min_cb_intra_slice_chroma;

    uint8_t  ph_max_mtt_hierarchy_depth_intra_slice_chroma;
    uint8_t  ph_log2_diff_max_bt_min_qt_intra_slice_chroma;
    uint8_t  ph_log2_diff_max_tt_min_qt_intra_slice_chroma;

    uint8_t  ph_cu_qp_delta_subdiv_intra_slice;
    uint8_t  ph_cu_chroma_qp_offset_subdiv_intra_slice;

    uint8_t  ph_log2_diff_min_qt_min_cb_inter_slice;
    uint8_t  ph_max_mtt_hierarchy_depth_inter_slice;
    uint8_t  ph_log2_diff_max_bt_min_qt_inter_slice;
    uint8_t  ph_log2_diff_max_tt_min_qt_inter_slice;
    uint8_t  ph_cu_qp_delta_subdiv_inter_slice;
    uint8_t  ph_cu_chroma_qp_offset_subdiv_inter_slice;

    uint8_t  ph_temporal_mvp_enabled_flag;
    uint8_t  ph_collocated_from_l0_flag;
    uint8_t  ph_collocated_ref_idx;
    uint8_t  ph_mmvd_fullpel_only_flag;
    uint8_t  ph_mvd_l1_zero_flag;
    uint8_t  ph_bdof_disabled_flag;
    uint8_t  ph_dmvr_disabled_flag;
    uint8_t  ph_prof_disabled_flag;

    H266RawPredWeightTable ph_pred_weight_table;

    int8_t   ph_qp_delta;
    uint8_t  ph_joint_cbcr_sign_flag;
    uint8_t  ph_sao_luma_enabled_flag;
    uint8_t  ph_sao_chroma_enabled_flag;

    uint8_t  ph_deblocking_params_present_flag;
    uint8_t  ph_deblocking_filter_disabled_flag;
    int8_t   ph_luma_beta_offset_div2;
    int8_t   ph_luma_tc_offset_div2;
    int8_t   ph_cb_beta_offset_div2;
    int8_t   ph_cb_tc_offset_div2;
    int8_t   ph_cr_beta_offset_div2;
    int8_t   ph_cr_tc_offset_div2;

    uint8_t  ph_extension_length;
    uint8_t  ph_extension_data_byte[256];
} H266RawPictureHeader;

typedef struct H266RawPH {
    H266RawNALUnitHeader nal_unit_header;
    H266RawPictureHeader ph_picture_header;
} H266RawPH;

typedef struct  H266RawSliceHeader {
    H266RawNALUnitHeader nal_unit_header;
    uint8_t  sh_picture_header_in_slice_header_flag;
    H266RawPictureHeader sh_picture_header;

    uint16_t sh_subpic_id;
    uint16_t sh_slice_address;
    uint8_t  sh_extra_bit[16];
    uint8_t  sh_num_tiles_in_slice_minus1;
    uint8_t  sh_slice_type;
    uint8_t  sh_no_output_of_prior_pics_flag;

    uint8_t  sh_alf_enabled_flag;
    uint8_t  sh_num_alf_aps_ids_luma;
    uint8_t  sh_alf_aps_id_luma[8];
    uint8_t  sh_alf_cb_enabled_flag;
    uint8_t  sh_alf_cr_enabled_flag;
    uint8_t  sh_alf_aps_id_chroma;
    uint8_t  sh_alf_cc_cb_enabled_flag;
    uint8_t  sh_alf_cc_cb_aps_id;
    uint8_t  sh_alf_cc_cr_enabled_flag;
    uint8_t  sh_alf_cc_cr_aps_id;

    uint8_t  sh_lmcs_used_flag;
    uint8_t  sh_explicit_scaling_list_used_flag;

    H266RefPicLists sh_ref_pic_lists;

    uint8_t  sh_num_ref_idx_active_override_flag;
    uint8_t  sh_num_ref_idx_active_minus1[2];
    uint8_t  sh_cabac_init_flag;
    uint8_t  sh_collocated_from_l0_flag;
    uint8_t  sh_collocated_ref_idx;

    H266RawPredWeightTable sh_pred_weight_table;

    int8_t   sh_qp_delta;
    int8_t   sh_cb_qp_offset;
    int8_t   sh_cr_qp_offset;
    int8_t   sh_joint_cbcr_qp_offset;
    uint8_t  sh_cu_chroma_qp_offset_enabled_flag;

    uint8_t  sh_sao_luma_used_flag;
    uint8_t  sh_sao_chroma_used_flag;

    uint8_t  sh_deblocking_params_present_flag;
    uint8_t  sh_deblocking_filter_disabled_flag;
    int8_t   sh_luma_beta_offset_div2;
    int8_t   sh_luma_tc_offset_div2;
    int8_t   sh_cb_beta_offset_div2;
    int8_t   sh_cb_tc_offset_div2;
    int8_t   sh_cr_beta_offset_div2;
    int8_t   sh_cr_tc_offset_div2;
    uint8_t  sh_dep_quant_used_flag;

    uint8_t  sh_sign_data_hiding_used_flag;
    uint8_t  sh_ts_residual_coding_disabled_flag;
    uint8_t  sh_ts_residual_coding_rice_idx_minus1;
    uint8_t  sh_reverse_last_sig_coeff_flag;
    uint16_t sh_slice_header_extension_length;
    uint8_t  sh_slice_header_extension_data_byte[256];

    uint8_t  sh_entry_offset_len_minus1;
    uint32_t sh_entry_point_offset_minus1[VVC_MAX_ENTRY_POINTS];

    // derived values
    uint16_t curr_subpic_idx;               ///< CurrSubpicIdx
    uint32_t num_entry_points;              ///< NumEntryPoints
    uint8_t  num_ref_idx_active[2];         ///< NumRefIdxActive[]

} H266RawSliceHeader;

typedef struct H266RawSlice {
    H266RawSliceHeader header;

    uint8_t     *data;
    AVBufferRef *data_ref;
    size_t       header_size;
    size_t       data_size;
    int          data_bit_start;
} H266RawSlice;

typedef struct H266RawSEI {
    H266RawNALUnitHeader nal_unit_header;
    SEIRawMessageList    message_list;
} H266RawSEI;

typedef struct CodedBitstreamH266Context {
    // Reader/writer context in common with the H.264 implementation.
    CodedBitstreamH2645Context common;

    // All currently available parameter sets.  These are updated when
    // any parameter set NAL unit is read/written with this context.
    H266RawVPS  *vps[VVC_MAX_VPS_COUNT]; ///< RefStruct references
    H266RawSPS  *sps[VVC_MAX_SPS_COUNT]; ///< RefStruct references
    H266RawPPS  *pps[VVC_MAX_PPS_COUNT]; ///< RefStruct references
    H266RawPictureHeader *ph;
    void *ph_ref; ///< RefStruct reference backing ph above
} CodedBitstreamH266Context;

#endif /* AVCODEC_CBS_H266_H */
