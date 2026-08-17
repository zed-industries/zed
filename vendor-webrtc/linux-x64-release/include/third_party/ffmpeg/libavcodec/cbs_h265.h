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

#ifndef AVCODEC_CBS_H265_H
#define AVCODEC_CBS_H265_H

#include <stddef.h>
#include <stdint.h>

#include "cbs_h2645.h"
#include "cbs_sei.h"

#include "hevc/hevc.h"

typedef struct H265RawNALUnitHeader {
    uint8_t nal_unit_type;
    uint8_t nuh_layer_id;
    uint8_t nuh_temporal_id_plus1;
} H265RawNALUnitHeader;

typedef struct H265RawProfileTierLevel {
    uint8_t general_profile_space;
    uint8_t general_tier_flag;
    uint8_t general_profile_idc;

    uint8_t general_profile_compatibility_flag[32];

    uint8_t general_progressive_source_flag;
    uint8_t general_interlaced_source_flag;
    uint8_t general_non_packed_constraint_flag;
    uint8_t general_frame_only_constraint_flag;

    uint8_t general_max_12bit_constraint_flag;
    uint8_t general_max_10bit_constraint_flag;
    uint8_t general_max_8bit_constraint_flag;
    uint8_t general_max_422chroma_constraint_flag;
    uint8_t general_max_420chroma_constraint_flag;
    uint8_t general_max_monochrome_constraint_flag;
    uint8_t general_intra_constraint_flag;
    uint8_t general_one_picture_only_constraint_flag;
    uint8_t general_lower_bit_rate_constraint_flag;
    uint8_t general_max_14bit_constraint_flag;

    uint8_t general_inbld_flag;

    uint8_t general_level_idc;

    uint8_t sub_layer_profile_present_flag[HEVC_MAX_SUB_LAYERS];
    uint8_t sub_layer_level_present_flag[HEVC_MAX_SUB_LAYERS];

    uint8_t sub_layer_profile_space[HEVC_MAX_SUB_LAYERS];
    uint8_t sub_layer_tier_flag[HEVC_MAX_SUB_LAYERS];
    uint8_t sub_layer_profile_idc[HEVC_MAX_SUB_LAYERS];

    uint8_t sub_layer_profile_compatibility_flag[HEVC_MAX_SUB_LAYERS][32];

    uint8_t sub_layer_progressive_source_flag[HEVC_MAX_SUB_LAYERS];
    uint8_t sub_layer_interlaced_source_flag[HEVC_MAX_SUB_LAYERS];
    uint8_t sub_layer_non_packed_constraint_flag[HEVC_MAX_SUB_LAYERS];
    uint8_t sub_layer_frame_only_constraint_flag[HEVC_MAX_SUB_LAYERS];

    uint8_t sub_layer_max_12bit_constraint_flag[HEVC_MAX_SUB_LAYERS];
    uint8_t sub_layer_max_10bit_constraint_flag[HEVC_MAX_SUB_LAYERS];
    uint8_t sub_layer_max_8bit_constraint_flag[HEVC_MAX_SUB_LAYERS];
    uint8_t sub_layer_max_422chroma_constraint_flag[HEVC_MAX_SUB_LAYERS];
    uint8_t sub_layer_max_420chroma_constraint_flag[HEVC_MAX_SUB_LAYERS];
    uint8_t sub_layer_max_monochrome_constraint_flag[HEVC_MAX_SUB_LAYERS];
    uint8_t sub_layer_intra_constraint_flag[HEVC_MAX_SUB_LAYERS];
    uint8_t sub_layer_one_picture_only_constraint_flag[HEVC_MAX_SUB_LAYERS];
    uint8_t sub_layer_lower_bit_rate_constraint_flag[HEVC_MAX_SUB_LAYERS];
    uint8_t sub_layer_max_14bit_constraint_flag[HEVC_MAX_SUB_LAYERS];

    uint8_t sub_layer_inbld_flag[HEVC_MAX_SUB_LAYERS];

    uint8_t sub_layer_level_idc[HEVC_MAX_SUB_LAYERS];
} H265RawProfileTierLevel;

typedef struct H265RawSubLayerHRDParameters {
    uint32_t bit_rate_value_minus1[HEVC_MAX_CPB_CNT];
    uint32_t cpb_size_value_minus1[HEVC_MAX_CPB_CNT];
    uint32_t cpb_size_du_value_minus1[HEVC_MAX_CPB_CNT];
    uint32_t bit_rate_du_value_minus1[HEVC_MAX_CPB_CNT];
    uint8_t cbr_flag[HEVC_MAX_CPB_CNT];
} H265RawSubLayerHRDParameters;

typedef struct H265RawHRDParameters {
    uint8_t nal_hrd_parameters_present_flag;
    uint8_t vcl_hrd_parameters_present_flag;

    uint8_t sub_pic_hrd_params_present_flag;
    uint8_t tick_divisor_minus2;
    uint8_t du_cpb_removal_delay_increment_length_minus1;
    uint8_t sub_pic_cpb_params_in_pic_timing_sei_flag;
    uint8_t dpb_output_delay_du_length_minus1;

    uint8_t bit_rate_scale;
    uint8_t cpb_size_scale;
    uint8_t cpb_size_du_scale;

    uint8_t initial_cpb_removal_delay_length_minus1;
    uint8_t au_cpb_removal_delay_length_minus1;
    uint8_t dpb_output_delay_length_minus1;

    uint8_t fixed_pic_rate_general_flag[HEVC_MAX_SUB_LAYERS];
    uint8_t fixed_pic_rate_within_cvs_flag[HEVC_MAX_SUB_LAYERS];
    uint16_t elemental_duration_in_tc_minus1[HEVC_MAX_SUB_LAYERS];
    uint8_t low_delay_hrd_flag[HEVC_MAX_SUB_LAYERS];
    uint8_t cpb_cnt_minus1[HEVC_MAX_SUB_LAYERS];
    H265RawSubLayerHRDParameters nal_sub_layer_hrd_parameters[HEVC_MAX_SUB_LAYERS];
    H265RawSubLayerHRDParameters vcl_sub_layer_hrd_parameters[HEVC_MAX_SUB_LAYERS];
} H265RawHRDParameters;

typedef struct H265RawVUI {
    uint8_t aspect_ratio_info_present_flag;
    uint8_t aspect_ratio_idc;
    uint16_t sar_width;
    uint16_t sar_height;

    uint8_t overscan_info_present_flag;
    uint8_t overscan_appropriate_flag;

    uint8_t video_signal_type_present_flag;
    uint8_t video_format;
    uint8_t video_full_range_flag;
    uint8_t colour_description_present_flag;
    uint8_t colour_primaries;
    uint8_t transfer_characteristics;
    uint8_t matrix_coefficients;

    uint8_t chroma_loc_info_present_flag;
    uint8_t chroma_sample_loc_type_top_field;
    uint8_t chroma_sample_loc_type_bottom_field;

    uint8_t neutral_chroma_indication_flag;
    uint8_t field_seq_flag;
    uint8_t frame_field_info_present_flag;

    uint8_t default_display_window_flag;
    uint16_t def_disp_win_left_offset;
    uint16_t def_disp_win_right_offset;
    uint16_t def_disp_win_top_offset;
    uint16_t def_disp_win_bottom_offset;

    uint8_t vui_timing_info_present_flag;
    uint32_t vui_num_units_in_tick;
    uint32_t vui_time_scale;
    uint8_t vui_poc_proportional_to_timing_flag;
    uint32_t vui_num_ticks_poc_diff_one_minus1;
    uint8_t vui_hrd_parameters_present_flag;
    H265RawHRDParameters hrd_parameters;

    uint8_t bitstream_restriction_flag;
    uint8_t tiles_fixed_structure_flag;
    uint8_t motion_vectors_over_pic_boundaries_flag;
    uint8_t restricted_ref_pic_lists_flag;
    uint16_t min_spatial_segmentation_idc;
    uint8_t max_bytes_per_pic_denom;
    uint8_t max_bits_per_min_cu_denom;
    uint8_t log2_max_mv_length_horizontal;
    uint8_t log2_max_mv_length_vertical;
} H265RawVUI;

typedef struct H265RawExtensionData {
    uint8_t     *data;
    AVBufferRef *data_ref;
    size_t       bit_length;
} H265RawExtensionData;

typedef struct H265RawVPS {
    H265RawNALUnitHeader nal_unit_header;

    uint8_t vps_video_parameter_set_id;

    uint8_t vps_base_layer_internal_flag;
    uint8_t vps_base_layer_available_flag;
    uint8_t vps_max_layers_minus1;
    uint8_t vps_max_sub_layers_minus1;
    uint8_t vps_temporal_id_nesting_flag;

    H265RawProfileTierLevel profile_tier_level;

    uint8_t vps_sub_layer_ordering_info_present_flag;
    uint8_t vps_max_dec_pic_buffering_minus1[HEVC_MAX_SUB_LAYERS];
    uint8_t vps_max_num_reorder_pics[HEVC_MAX_SUB_LAYERS];
    uint32_t vps_max_latency_increase_plus1[HEVC_MAX_SUB_LAYERS];

    uint8_t vps_max_layer_id;
    uint16_t vps_num_layer_sets_minus1;
    uint8_t layer_id_included_flag[HEVC_MAX_LAYER_SETS][HEVC_MAX_LAYERS];

    uint8_t vps_timing_info_present_flag;
    uint32_t vps_num_units_in_tick;
    uint32_t vps_time_scale;
    uint8_t vps_poc_proportional_to_timing_flag;
    uint32_t vps_num_ticks_poc_diff_one_minus1;
    uint16_t vps_num_hrd_parameters;
    uint16_t hrd_layer_set_idx[HEVC_MAX_LAYER_SETS];
    uint8_t cprms_present_flag[HEVC_MAX_LAYER_SETS];
    H265RawHRDParameters hrd_parameters[HEVC_MAX_LAYER_SETS];

    uint8_t vps_extension_flag;
    H265RawExtensionData extension_data;
} H265RawVPS;

typedef struct H265RawSTRefPicSet {
    uint8_t inter_ref_pic_set_prediction_flag;

    uint8_t delta_idx_minus1;
    uint8_t delta_rps_sign;
    uint16_t abs_delta_rps_minus1;

    uint8_t used_by_curr_pic_flag[HEVC_MAX_REFS];
    uint8_t use_delta_flag[HEVC_MAX_REFS];

    uint8_t num_negative_pics;
    uint8_t num_positive_pics;
    uint16_t delta_poc_s0_minus1[HEVC_MAX_REFS];
    uint8_t used_by_curr_pic_s0_flag[HEVC_MAX_REFS];
    uint16_t delta_poc_s1_minus1[HEVC_MAX_REFS];
    uint8_t used_by_curr_pic_s1_flag[HEVC_MAX_REFS];
} H265RawSTRefPicSet;

typedef struct H265RawScalingList {
    uint8_t scaling_list_pred_mode_flag[4][6];
    uint8_t scaling_list_pred_matrix_id_delta[4][6];
    int16_t scaling_list_dc_coef_minus8[4][6];
    int8_t scaling_list_delta_coeff[4][6][64];
} H265RawScalingList;

typedef struct H265RawSPS {
    H265RawNALUnitHeader nal_unit_header;

    uint8_t sps_video_parameter_set_id;

    uint8_t sps_max_sub_layers_minus1;
    uint8_t sps_ext_or_max_sub_layers_minus1;
    uint8_t sps_temporal_id_nesting_flag;

    H265RawProfileTierLevel profile_tier_level;

    uint8_t sps_seq_parameter_set_id;

    uint8_t update_rep_format_flag;
    uint8_t sps_rep_format_idx;

    uint8_t chroma_format_idc;
    uint8_t separate_colour_plane_flag;

    uint16_t pic_width_in_luma_samples;
    uint16_t pic_height_in_luma_samples;

    uint8_t conformance_window_flag;
    uint16_t conf_win_left_offset;
    uint16_t conf_win_right_offset;
    uint16_t conf_win_top_offset;
    uint16_t conf_win_bottom_offset;

    uint8_t bit_depth_luma_minus8;
    uint8_t bit_depth_chroma_minus8;

    uint8_t log2_max_pic_order_cnt_lsb_minus4;

    uint8_t sps_sub_layer_ordering_info_present_flag;
    uint8_t sps_max_dec_pic_buffering_minus1[HEVC_MAX_SUB_LAYERS];
    uint8_t sps_max_num_reorder_pics[HEVC_MAX_SUB_LAYERS];
    uint32_t sps_max_latency_increase_plus1[HEVC_MAX_SUB_LAYERS];

    uint8_t log2_min_luma_coding_block_size_minus3;
    uint8_t log2_diff_max_min_luma_coding_block_size;
    uint8_t log2_min_luma_transform_block_size_minus2;
    uint8_t log2_diff_max_min_luma_transform_block_size;
    uint8_t max_transform_hierarchy_depth_inter;
    uint8_t max_transform_hierarchy_depth_intra;

    uint8_t scaling_list_enabled_flag;
    uint8_t sps_infer_scaling_list_flag;
    uint8_t sps_scaling_list_ref_layer_id;
    uint8_t sps_scaling_list_data_present_flag;
    H265RawScalingList scaling_list;

    uint8_t amp_enabled_flag;
    uint8_t sample_adaptive_offset_enabled_flag;

    uint8_t pcm_enabled_flag;
    uint8_t pcm_sample_bit_depth_luma_minus1;
    uint8_t pcm_sample_bit_depth_chroma_minus1;
    uint8_t log2_min_pcm_luma_coding_block_size_minus3;
    uint8_t log2_diff_max_min_pcm_luma_coding_block_size;
    uint8_t pcm_loop_filter_disabled_flag;

    uint8_t num_short_term_ref_pic_sets;
    H265RawSTRefPicSet st_ref_pic_set[HEVC_MAX_SHORT_TERM_REF_PIC_SETS];

    uint8_t long_term_ref_pics_present_flag;
    uint8_t num_long_term_ref_pics_sps;
    uint16_t lt_ref_pic_poc_lsb_sps[HEVC_MAX_LONG_TERM_REF_PICS];
    uint8_t used_by_curr_pic_lt_sps_flag[HEVC_MAX_LONG_TERM_REF_PICS];

    uint8_t sps_temporal_mvp_enabled_flag;
    uint8_t strong_intra_smoothing_enabled_flag;

    uint8_t vui_parameters_present_flag;
    H265RawVUI vui;

    uint8_t sps_extension_present_flag;
    uint8_t sps_range_extension_flag;
    uint8_t sps_multilayer_extension_flag;
    uint8_t sps_3d_extension_flag;
    uint8_t sps_scc_extension_flag;
    uint8_t sps_extension_4bits;

    H265RawExtensionData extension_data;

    // Range extension.
    uint8_t transform_skip_rotation_enabled_flag;
    uint8_t transform_skip_context_enabled_flag;
    uint8_t implicit_rdpcm_enabled_flag;
    uint8_t explicit_rdpcm_enabled_flag;
    uint8_t extended_precision_processing_flag;
    uint8_t intra_smoothing_disabled_flag;
    uint8_t high_precision_offsets_enabled_flag;
    uint8_t persistent_rice_adaptation_enabled_flag;
    uint8_t cabac_bypass_alignment_enabled_flag;

    // Screen content coding extension.
    uint8_t sps_curr_pic_ref_enabled_flag;
    uint8_t palette_mode_enabled_flag;
    uint8_t palette_max_size;
    uint8_t delta_palette_max_predictor_size;
    uint8_t sps_palette_predictor_initializer_present_flag;
    uint8_t sps_num_palette_predictor_initializer_minus1;
    uint16_t sps_palette_predictor_initializers[3][128];

    uint8_t motion_vector_resolution_control_idc;
    uint8_t intra_boundary_filtering_disable_flag;

    // Multilayer extension.
    uint8_t inter_view_mv_vert_constraint_flag;
} H265RawSPS;

typedef struct H265RawPPS {
    H265RawNALUnitHeader nal_unit_header;

    uint8_t pps_pic_parameter_set_id;
    uint8_t pps_seq_parameter_set_id;

    uint8_t dependent_slice_segments_enabled_flag;
    uint8_t output_flag_present_flag;
    uint8_t num_extra_slice_header_bits;
    uint8_t sign_data_hiding_enabled_flag;
    uint8_t cabac_init_present_flag;

    uint8_t num_ref_idx_l0_default_active_minus1;
    uint8_t num_ref_idx_l1_default_active_minus1;

    int8_t init_qp_minus26;

    uint8_t constrained_intra_pred_flag;
    uint8_t transform_skip_enabled_flag;
    uint8_t cu_qp_delta_enabled_flag;
    uint8_t diff_cu_qp_delta_depth;

    int8_t pps_cb_qp_offset;
    int8_t pps_cr_qp_offset;
    uint8_t pps_slice_chroma_qp_offsets_present_flag;

    uint8_t weighted_pred_flag;
    uint8_t weighted_bipred_flag;

    uint8_t transquant_bypass_enabled_flag;
    uint8_t tiles_enabled_flag;
    uint8_t entropy_coding_sync_enabled_flag;

    uint8_t num_tile_columns_minus1;
    uint8_t num_tile_rows_minus1;
    uint8_t uniform_spacing_flag;
    uint16_t column_width_minus1[HEVC_MAX_TILE_COLUMNS];
    uint16_t row_height_minus1[HEVC_MAX_TILE_ROWS];
    uint8_t loop_filter_across_tiles_enabled_flag;

    uint8_t pps_loop_filter_across_slices_enabled_flag;
    uint8_t deblocking_filter_control_present_flag;
    uint8_t deblocking_filter_override_enabled_flag;
    uint8_t pps_deblocking_filter_disabled_flag;
    int8_t pps_beta_offset_div2;
    int8_t pps_tc_offset_div2;

    uint8_t pps_scaling_list_data_present_flag;
    H265RawScalingList scaling_list;

    uint8_t lists_modification_present_flag;
    uint8_t log2_parallel_merge_level_minus2;

    uint8_t slice_segment_header_extension_present_flag;

    uint8_t pps_extension_present_flag;
    uint8_t pps_range_extension_flag;
    uint8_t pps_multilayer_extension_flag;
    uint8_t pps_3d_extension_flag;
    uint8_t pps_scc_extension_flag;
    uint8_t pps_extension_4bits;

    H265RawExtensionData extension_data;

    // Range extension.
    uint8_t log2_max_transform_skip_block_size_minus2;
    uint8_t cross_component_prediction_enabled_flag;
    uint8_t chroma_qp_offset_list_enabled_flag;
    uint8_t diff_cu_chroma_qp_offset_depth;
    uint8_t chroma_qp_offset_list_len_minus1;
    int8_t cb_qp_offset_list[6];
    int8_t cr_qp_offset_list[6];
    uint8_t log2_sao_offset_scale_luma;
    uint8_t log2_sao_offset_scale_chroma;

    // Screen content coding extension.
    uint8_t pps_curr_pic_ref_enabled_flag;
    uint8_t residual_adaptive_colour_transform_enabled_flag;
    uint8_t pps_slice_act_qp_offsets_present_flag;
    int8_t pps_act_y_qp_offset_plus5;
    int8_t pps_act_cb_qp_offset_plus5;
    int8_t pps_act_cr_qp_offset_plus3;

    uint8_t pps_palette_predictor_initializer_present_flag;
    uint8_t pps_num_palette_predictor_initializer;
    uint8_t monochrome_palette_flag;
    uint8_t luma_bit_depth_entry_minus8;
    uint8_t chroma_bit_depth_entry_minus8;
    uint16_t pps_palette_predictor_initializers[3][128];

    // Multilayer extension.
    uint8_t poc_reset_info_present_flag;
    uint8_t pps_infer_scaling_list_flag;
    uint8_t pps_scaling_list_ref_layer_id;
    uint8_t num_ref_loc_offsets;
    uint8_t ref_loc_offset_layer_id[64];
    uint8_t scaled_ref_layer_offset_present_flag[64];
    int16_t scaled_ref_layer_left_offset[64];
    int16_t scaled_ref_layer_top_offset[64];
    int16_t scaled_ref_layer_right_offset[64];
    int16_t scaled_ref_layer_bottom_offset[64];
    uint8_t ref_region_offset_present_flag[64];
    int16_t ref_region_left_offset[64];
    int16_t ref_region_top_offset[64];
    int16_t ref_region_right_offset[64];
    int16_t ref_region_bottom_offset[64];
    uint8_t resample_phase_set_present_flag[64];
    uint8_t phase_hor_luma[64];
    uint8_t phase_ver_luma[64];
    uint8_t phase_hor_chroma_plus8[64];
    uint8_t phase_ver_chroma_plus8[64];
    uint8_t colour_mapping_enabled_flag;
    uint8_t num_cm_ref_layers_minus1;
    uint8_t cm_ref_layer_id[62];
    uint8_t cm_octant_depth;
    uint8_t cm_y_part_num_log2;
    uint8_t luma_bit_depth_cm_input_minus8;
    uint8_t chroma_bit_depth_cm_input_minus8;
    uint8_t luma_bit_depth_cm_output_minus8;
    uint8_t chroma_bit_depth_cm_output_minus8;
    uint8_t cm_res_quant_bits;
    uint8_t cm_delta_flc_bits_minus1;
    int16_t cm_adapt_threshold_u_delta;
    int16_t cm_adapt_threshold_v_delta;
    uint8_t split_octant_flag[2];
    uint8_t coded_res_flag[12][2][2][4];
    uint8_t res_coeff_q[12][2][2][4][3];
    uint32_t res_coeff_s[12][2][2][4][3];
    uint8_t res_coeff_r[12][2][2][4][3];
} H265RawPPS;

typedef struct H265RawAUD {
    H265RawNALUnitHeader nal_unit_header;

    uint8_t pic_type;
} H265RawAUD;

typedef struct  H265RawSliceHeader {
    H265RawNALUnitHeader nal_unit_header;

    uint8_t first_slice_segment_in_pic_flag;
    uint8_t no_output_of_prior_pics_flag;
    uint8_t slice_pic_parameter_set_id;

    uint8_t dependent_slice_segment_flag;
    uint16_t slice_segment_address;

    uint8_t slice_reserved_flag[8];
    uint8_t slice_type;

    uint8_t pic_output_flag;
    uint8_t colour_plane_id;

    uint16_t slice_pic_order_cnt_lsb;

    uint8_t short_term_ref_pic_set_sps_flag;
    H265RawSTRefPicSet short_term_ref_pic_set;
    uint8_t short_term_ref_pic_set_idx;

    uint8_t num_long_term_sps;
    uint8_t num_long_term_pics;
    uint8_t lt_idx_sps[HEVC_MAX_REFS];
    uint8_t poc_lsb_lt[HEVC_MAX_REFS];
    uint8_t used_by_curr_pic_lt_flag[HEVC_MAX_REFS];
    uint8_t delta_poc_msb_present_flag[HEVC_MAX_REFS];
    uint32_t delta_poc_msb_cycle_lt[HEVC_MAX_REFS];

    uint8_t slice_temporal_mvp_enabled_flag;

    uint8_t slice_sao_luma_flag;
    uint8_t slice_sao_chroma_flag;

    uint8_t num_ref_idx_active_override_flag;
    uint8_t num_ref_idx_l0_active_minus1;
    uint8_t num_ref_idx_l1_active_minus1;

    uint8_t ref_pic_list_modification_flag_l0;
    uint8_t list_entry_l0[HEVC_MAX_REFS];
    uint8_t ref_pic_list_modification_flag_l1;
    uint8_t list_entry_l1[HEVC_MAX_REFS];

    uint8_t mvd_l1_zero_flag;
    uint8_t cabac_init_flag;
    uint8_t collocated_from_l0_flag;
    uint8_t collocated_ref_idx;

    uint8_t luma_log2_weight_denom;
    int8_t delta_chroma_log2_weight_denom;
    uint8_t luma_weight_l0_flag[HEVC_MAX_REFS];
    uint8_t chroma_weight_l0_flag[HEVC_MAX_REFS];
    int8_t delta_luma_weight_l0[HEVC_MAX_REFS];
    int16_t luma_offset_l0[HEVC_MAX_REFS];
    int8_t delta_chroma_weight_l0[HEVC_MAX_REFS][2];
    int16_t chroma_offset_l0[HEVC_MAX_REFS][2];
    uint8_t luma_weight_l1_flag[HEVC_MAX_REFS];
    uint8_t chroma_weight_l1_flag[HEVC_MAX_REFS];
    int8_t delta_luma_weight_l1[HEVC_MAX_REFS];
    int16_t luma_offset_l1[HEVC_MAX_REFS];
    int8_t delta_chroma_weight_l1[HEVC_MAX_REFS][2];
    int16_t chroma_offset_l1[HEVC_MAX_REFS][2];

    uint8_t five_minus_max_num_merge_cand;
    uint8_t use_integer_mv_flag;

    int8_t slice_qp_delta;
    int8_t slice_cb_qp_offset;
    int8_t slice_cr_qp_offset;
    int8_t slice_act_y_qp_offset;
    int8_t slice_act_cb_qp_offset;
    int8_t slice_act_cr_qp_offset;
    uint8_t cu_chroma_qp_offset_enabled_flag;

    uint8_t deblocking_filter_override_flag;
    uint8_t slice_deblocking_filter_disabled_flag;
    int8_t slice_beta_offset_div2;
    int8_t slice_tc_offset_div2;
    uint8_t slice_loop_filter_across_slices_enabled_flag;

    uint16_t num_entry_point_offsets;
    uint8_t offset_len_minus1;
    uint32_t entry_point_offset_minus1[HEVC_MAX_ENTRY_POINT_OFFSETS];

    uint16_t slice_segment_header_extension_length;
    uint8_t slice_segment_header_extension_data_byte[256];
} H265RawSliceHeader;


typedef struct H265RawSlice {
    H265RawSliceHeader header;

    uint8_t     *data;
    AVBufferRef *data_ref;
    size_t       data_size;
    int          data_bit_start;
} H265RawSlice;


typedef struct H265RawSEIBufferingPeriod {
    uint8_t  bp_seq_parameter_set_id;
    uint8_t  irap_cpb_params_present_flag;
    uint32_t cpb_delay_offset;
    uint32_t dpb_delay_offset;
    uint8_t  concatenation_flag;
    uint32_t au_cpb_removal_delay_delta_minus1;

    uint32_t nal_initial_cpb_removal_delay[HEVC_MAX_CPB_CNT];
    uint32_t nal_initial_cpb_removal_offset[HEVC_MAX_CPB_CNT];
    uint32_t nal_initial_alt_cpb_removal_delay[HEVC_MAX_CPB_CNT];
    uint32_t nal_initial_alt_cpb_removal_offset[HEVC_MAX_CPB_CNT];

    uint32_t vcl_initial_cpb_removal_delay[HEVC_MAX_CPB_CNT];
    uint32_t vcl_initial_cpb_removal_offset[HEVC_MAX_CPB_CNT];
    uint32_t vcl_initial_alt_cpb_removal_delay[HEVC_MAX_CPB_CNT];
    uint32_t vcl_initial_alt_cpb_removal_offset[HEVC_MAX_CPB_CNT];

    uint8_t  use_alt_cpb_params_flag;
} H265RawSEIBufferingPeriod;

typedef struct H265RawSEIPicTiming {
    uint8_t pic_struct;
    uint8_t source_scan_type;
    uint8_t duplicate_flag;

    uint32_t au_cpb_removal_delay_minus1;
    uint32_t pic_dpb_output_delay;
    uint32_t pic_dpb_output_du_delay;

    uint16_t num_decoding_units_minus1;
    uint8_t  du_common_cpb_removal_delay_flag;
    uint32_t du_common_cpb_removal_delay_increment_minus1;
    uint16_t num_nalus_in_du_minus1[HEVC_MAX_SLICE_SEGMENTS];
    uint32_t du_cpb_removal_delay_increment_minus1[HEVC_MAX_SLICE_SEGMENTS];
} H265RawSEIPicTiming;

typedef struct H265RawSEIPanScanRect {
    uint32_t pan_scan_rect_id;
    uint8_t  pan_scan_rect_cancel_flag;
    uint8_t  pan_scan_cnt_minus1;
    int32_t  pan_scan_rect_left_offset[3];
    int32_t  pan_scan_rect_right_offset[3];
    int32_t  pan_scan_rect_top_offset[3];
    int32_t  pan_scan_rect_bottom_offset[3];
    uint16_t pan_scan_rect_persistence_flag;
} H265RawSEIPanScanRect;

typedef struct H265RawSEIRecoveryPoint {
    int16_t recovery_poc_cnt;
    uint8_t exact_match_flag;
    uint8_t broken_link_flag;
} H265RawSEIRecoveryPoint;

typedef struct H265RawFilmGrainCharacteristics {
    uint8_t      film_grain_characteristics_cancel_flag;
    uint8_t      film_grain_model_id;
    uint8_t      separate_colour_description_present_flag;
    uint8_t      film_grain_bit_depth_luma_minus8;
    uint8_t      film_grain_bit_depth_chroma_minus8;
    uint8_t      film_grain_full_range_flag;
    uint8_t      film_grain_colour_primaries;
    uint8_t      film_grain_transfer_characteristics;
    uint8_t      film_grain_matrix_coeffs;
    uint8_t      blending_mode_id;
    uint8_t      log2_scale_factor;
    uint8_t      comp_model_present_flag[3];
    uint8_t      num_intensity_intervals_minus1[3];
    uint8_t      num_model_values_minus1[3];
    uint8_t      intensity_interval_lower_bound[3][256];
    uint8_t      intensity_interval_upper_bound[3][256];
    int16_t      comp_model_value[3][256][6];
    uint8_t      film_grain_characteristics_persistence_flag;
} H265RawFilmGrainCharacteristics;

typedef struct H265RawSEIDisplayOrientation {
    uint8_t display_orientation_cancel_flag;
    uint8_t hor_flip;
    uint8_t ver_flip;
    uint16_t anticlockwise_rotation;
    uint16_t display_orientation_repetition_period;
    uint8_t display_orientation_persistence_flag;
} H265RawSEIDisplayOrientation;

typedef struct H265RawSEIActiveParameterSets {
    uint8_t active_video_parameter_set_id;
    uint8_t self_contained_cvs_flag;
    uint8_t no_parameter_set_update_flag;
    uint8_t num_sps_ids_minus1;
    uint8_t active_seq_parameter_set_id[HEVC_MAX_SPS_COUNT];
    uint8_t layer_sps_idx[HEVC_MAX_LAYERS];
} H265RawSEIActiveParameterSets;

typedef struct H265RawSEIDecodedPictureHash {
    uint8_t  hash_type;
    uint8_t  picture_md5[3][16];
    uint16_t picture_crc[3];
    uint32_t picture_checksum[3];
} H265RawSEIDecodedPictureHash;

typedef struct H265RawSEITimeCode {
    uint8_t  num_clock_ts;
    uint8_t  clock_timestamp_flag[3];
    uint8_t  units_field_based_flag[3];
    uint8_t  counting_type[3];
    uint8_t  full_timestamp_flag[3];
    uint8_t  discontinuity_flag[3];
    uint8_t  cnt_dropped_flag[3];
    uint16_t n_frames[3];
    uint8_t  seconds_value[3];
    uint8_t  minutes_value[3];
    uint8_t  hours_value[3];
    uint8_t  seconds_flag[3];
    uint8_t  minutes_flag[3];
    uint8_t  hours_flag[3];
    uint8_t  time_offset_length[3];
    int32_t  time_offset_value[3];
} H265RawSEITimeCode;

typedef struct H265RawSEIAlphaChannelInfo {
    uint8_t  alpha_channel_cancel_flag;
    uint8_t  alpha_channel_use_idc;
    uint8_t  alpha_channel_bit_depth_minus8;
    uint16_t alpha_transparent_value;
    uint16_t alpha_opaque_value;
    uint8_t  alpha_channel_incr_flag;
    uint8_t  alpha_channel_clip_flag;
    uint8_t  alpha_channel_clip_type_flag;
} H265RawSEIAlphaChannelInfo;

typedef struct H265RawSEI3DReferenceDisplaysInfo {
    uint8_t prec_ref_display_width;
    uint8_t ref_viewing_distance_flag;
    uint8_t prec_ref_viewing_dist;
    uint8_t num_ref_displays_minus1;
    uint16_t left_view_id[32];
    uint16_t right_view_id[32];
    uint8_t exponent_ref_display_width[32];
    uint8_t mantissa_ref_display_width[32];
    uint8_t exponent_ref_viewing_distance[32];
    uint8_t mantissa_ref_viewing_distance[32];
    uint8_t additional_shift_present_flag[32];
    uint16_t num_sample_shift_plus512[32];
    uint8_t three_dimensional_reference_displays_extension_flag;
} H265RawSEI3DReferenceDisplaysInfo;

typedef struct H265RawSEI {
    H265RawNALUnitHeader nal_unit_header;
    SEIRawMessageList    message_list;
} H265RawSEI;

typedef struct H265RawFiller {
    H265RawNALUnitHeader nal_unit_header;

    uint32_t filler_size;
} H265RawFiller;

typedef struct CodedBitstreamH265Context {
    // Reader/writer context in common with the H.264 implementation.
    CodedBitstreamH2645Context common;

    // All currently available parameter sets.  These are updated when
    // any parameter set NAL unit is read/written with this context.
    H265RawVPS *vps[HEVC_MAX_VPS_COUNT]; ///< RefStruct references
    H265RawSPS *sps[HEVC_MAX_SPS_COUNT]; ///< RefStruct references
    H265RawPPS *pps[HEVC_MAX_PPS_COUNT]; ///< RefStruct references

    // The currently active parameter sets.  These are updated when any
    // NAL unit refers to the relevant parameter set.  These pointers
    // must also be present in the arrays above.
    const H265RawVPS *active_vps;
    const H265RawSPS *active_sps;
    const H265RawPPS *active_pps;
} CodedBitstreamH265Context;


#endif /* AVCODEC_CBS_H265_H */
