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

#ifndef AVCODEC_CBS_AV1_H
#define AVCODEC_CBS_AV1_H

#include <stddef.h>
#include <stdint.h>

#include "av1.h"
#include "cbs.h"


typedef struct AV1RawOBUHeader {
    uint8_t obu_forbidden_bit;
    uint8_t obu_type;
    uint8_t obu_extension_flag;
    uint8_t obu_has_size_field;
    uint8_t obu_reserved_1bit;

    uint8_t temporal_id;
    uint8_t spatial_id;
    uint8_t extension_header_reserved_3bits;
} AV1RawOBUHeader;

typedef struct AV1RawColorConfig {
    uint8_t high_bitdepth;
    uint8_t twelve_bit;
    uint8_t mono_chrome;

    uint8_t color_description_present_flag;
    uint8_t color_primaries;
    uint8_t transfer_characteristics;
    uint8_t matrix_coefficients;

    uint8_t color_range;
    uint8_t subsampling_x;
    uint8_t subsampling_y;
    uint8_t chroma_sample_position;
    uint8_t separate_uv_delta_q;
} AV1RawColorConfig;

typedef struct AV1RawTimingInfo {
    uint32_t num_units_in_display_tick;
    uint32_t time_scale;

    uint8_t equal_picture_interval;
    uint32_t num_ticks_per_picture_minus_1;
} AV1RawTimingInfo;

typedef struct AV1RawDecoderModelInfo {
    uint8_t  buffer_delay_length_minus_1;
    uint32_t num_units_in_decoding_tick;
    uint8_t  buffer_removal_time_length_minus_1;
    uint8_t  frame_presentation_time_length_minus_1;
} AV1RawDecoderModelInfo;

typedef struct AV1RawSequenceHeader {
    uint8_t seq_profile;
    uint8_t still_picture;
    uint8_t reduced_still_picture_header;

    uint8_t timing_info_present_flag;
    uint8_t decoder_model_info_present_flag;
    uint8_t initial_display_delay_present_flag;
    uint8_t operating_points_cnt_minus_1;

    AV1RawTimingInfo       timing_info;
    AV1RawDecoderModelInfo decoder_model_info;

    uint16_t operating_point_idc[AV1_MAX_OPERATING_POINTS];
    uint8_t  seq_level_idx[AV1_MAX_OPERATING_POINTS];
    uint8_t  seq_tier[AV1_MAX_OPERATING_POINTS];
    uint8_t  decoder_model_present_for_this_op[AV1_MAX_OPERATING_POINTS];
    uint32_t decoder_buffer_delay[AV1_MAX_OPERATING_POINTS];
    uint32_t encoder_buffer_delay[AV1_MAX_OPERATING_POINTS];
    uint8_t  low_delay_mode_flag[AV1_MAX_OPERATING_POINTS];
    uint8_t  initial_display_delay_present_for_this_op[AV1_MAX_OPERATING_POINTS];
    uint8_t  initial_display_delay_minus_1[AV1_MAX_OPERATING_POINTS];

    uint8_t  frame_width_bits_minus_1;
    uint8_t  frame_height_bits_minus_1;
    uint16_t max_frame_width_minus_1;
    uint16_t max_frame_height_minus_1;

    uint8_t frame_id_numbers_present_flag;
    uint8_t delta_frame_id_length_minus_2;
    uint8_t additional_frame_id_length_minus_1;

    uint8_t use_128x128_superblock;
    uint8_t enable_filter_intra;
    uint8_t enable_intra_edge_filter;
    uint8_t enable_interintra_compound;
    uint8_t enable_masked_compound;
    uint8_t enable_warped_motion;
    uint8_t enable_dual_filter;

    uint8_t enable_order_hint;
    uint8_t enable_jnt_comp;
    uint8_t enable_ref_frame_mvs;

    uint8_t seq_choose_screen_content_tools;
    uint8_t seq_force_screen_content_tools;
    uint8_t seq_choose_integer_mv;
    uint8_t seq_force_integer_mv;

    uint8_t order_hint_bits_minus_1;

    uint8_t enable_superres;
    uint8_t enable_cdef;
    uint8_t enable_restoration;

    AV1RawColorConfig color_config;

    uint8_t film_grain_params_present;
} AV1RawSequenceHeader;

typedef struct AV1RawFilmGrainParams {
    uint8_t  apply_grain;
    uint16_t grain_seed;
    uint8_t  update_grain;
    uint8_t  film_grain_params_ref_idx;
    uint8_t  num_y_points;
    uint8_t  point_y_value[14];
    uint8_t  point_y_scaling[14];
    uint8_t  chroma_scaling_from_luma;
    uint8_t  num_cb_points;
    uint8_t  point_cb_value[10];
    uint8_t  point_cb_scaling[10];
    uint8_t  num_cr_points;
    uint8_t  point_cr_value[10];
    uint8_t  point_cr_scaling[10];
    uint8_t  grain_scaling_minus_8;
    uint8_t  ar_coeff_lag;
    uint8_t  ar_coeffs_y_plus_128[24];
    uint8_t  ar_coeffs_cb_plus_128[25];
    uint8_t  ar_coeffs_cr_plus_128[25];
    uint8_t  ar_coeff_shift_minus_6;
    uint8_t  grain_scale_shift;
    uint8_t  cb_mult;
    uint8_t  cb_luma_mult;
    uint16_t cb_offset;
    uint8_t  cr_mult;
    uint8_t  cr_luma_mult;
    uint16_t cr_offset;
    uint8_t  overlap_flag;
    uint8_t  clip_to_restricted_range;
} AV1RawFilmGrainParams;

typedef struct AV1RawFrameHeader {
    uint8_t  show_existing_frame;
    uint8_t  frame_to_show_map_idx;
    uint32_t frame_presentation_time;
    uint32_t display_frame_id;

    uint8_t frame_type;
    uint8_t show_frame;
    uint8_t showable_frame;

    uint8_t error_resilient_mode;
    uint8_t disable_cdf_update;
    uint8_t allow_screen_content_tools;
    uint8_t force_integer_mv;

    uint32_t current_frame_id;
    uint8_t  frame_size_override_flag;
    uint8_t  order_hint;

    uint8_t  buffer_removal_time_present_flag;
    uint32_t buffer_removal_time[AV1_MAX_OPERATING_POINTS];

    uint8_t  primary_ref_frame;
    uint16_t frame_width_minus_1;
    uint16_t frame_height_minus_1;
    uint8_t  use_superres;
    uint8_t  coded_denom;
    uint8_t  render_and_frame_size_different;
    uint16_t render_width_minus_1;
    uint16_t render_height_minus_1;

    uint8_t found_ref[AV1_REFS_PER_FRAME];

    uint8_t refresh_frame_flags;
    uint8_t allow_intrabc;
    uint8_t ref_order_hint[AV1_NUM_REF_FRAMES];
    uint8_t frame_refs_short_signaling;
    uint8_t last_frame_idx;
    uint8_t golden_frame_idx;
    int8_t  ref_frame_idx[AV1_REFS_PER_FRAME];
    uint32_t delta_frame_id_minus1[AV1_REFS_PER_FRAME];

    uint8_t allow_high_precision_mv;
    uint8_t is_filter_switchable;
    uint8_t interpolation_filter;
    uint8_t is_motion_mode_switchable;
    uint8_t use_ref_frame_mvs;

    uint8_t disable_frame_end_update_cdf;

    uint8_t uniform_tile_spacing_flag;
    uint8_t tile_cols_log2;
    uint8_t tile_rows_log2;
    uint8_t tile_start_col_sb[AV1_MAX_TILE_COLS];
    uint8_t tile_start_row_sb[AV1_MAX_TILE_COLS];
    uint8_t width_in_sbs_minus_1[AV1_MAX_TILE_COLS];
    uint8_t height_in_sbs_minus_1[AV1_MAX_TILE_ROWS];
    uint16_t context_update_tile_id;
    uint8_t tile_size_bytes_minus1;

    // These are derived values, but it's very unhelpful to have to
    // recalculate them all the time so we store them here.
    uint16_t tile_cols;
    uint16_t tile_rows;

    uint8_t base_q_idx;
    int8_t  delta_q_y_dc;
    uint8_t diff_uv_delta;
    int8_t  delta_q_u_dc;
    int8_t  delta_q_u_ac;
    int8_t  delta_q_v_dc;
    int8_t  delta_q_v_ac;
    uint8_t using_qmatrix;
    uint8_t qm_y;
    uint8_t qm_u;
    uint8_t qm_v;

    uint8_t segmentation_enabled;
    uint8_t segmentation_update_map;
    uint8_t segmentation_temporal_update;
    uint8_t segmentation_update_data;
    uint8_t feature_enabled[AV1_MAX_SEGMENTS][AV1_SEG_LVL_MAX];
    int16_t feature_value[AV1_MAX_SEGMENTS][AV1_SEG_LVL_MAX];

    uint8_t delta_q_present;
    uint8_t delta_q_res;
    uint8_t delta_lf_present;
    uint8_t delta_lf_res;
    uint8_t delta_lf_multi;

    uint8_t loop_filter_level[4];
    uint8_t loop_filter_sharpness;
    uint8_t loop_filter_delta_enabled;
    uint8_t loop_filter_delta_update;
    uint8_t update_ref_delta[AV1_TOTAL_REFS_PER_FRAME];
    int8_t  loop_filter_ref_deltas[AV1_TOTAL_REFS_PER_FRAME];
    uint8_t update_mode_delta[2];
    int8_t  loop_filter_mode_deltas[2];

    uint8_t cdef_damping_minus_3;
    uint8_t cdef_bits;
    uint8_t cdef_y_pri_strength[8];
    uint8_t cdef_y_sec_strength[8];
    uint8_t cdef_uv_pri_strength[8];
    uint8_t cdef_uv_sec_strength[8];

    uint8_t lr_type[3];
    uint8_t lr_unit_shift;
    uint8_t lr_uv_shift;

    uint8_t tx_mode;
    uint8_t reference_select;
    uint8_t skip_mode_present;

    uint8_t allow_warped_motion;
    uint8_t reduced_tx_set;

    uint8_t is_global[AV1_TOTAL_REFS_PER_FRAME];
    uint8_t is_rot_zoom[AV1_TOTAL_REFS_PER_FRAME];
    uint8_t is_translation[AV1_TOTAL_REFS_PER_FRAME];
    //AV1RawSubexp gm_params[AV1_TOTAL_REFS_PER_FRAME][6];
    uint32_t gm_params[AV1_TOTAL_REFS_PER_FRAME][6];

    AV1RawFilmGrainParams film_grain;
} AV1RawFrameHeader;

typedef struct AV1RawTileData {
    uint8_t     *data;
    AVBufferRef *data_ref;
    size_t       data_size;
} AV1RawTileData;

typedef struct AV1RawTileGroup {
    uint8_t  tile_start_and_end_present_flag;
    uint16_t tg_start;
    uint16_t tg_end;

    AV1RawTileData tile_data;
} AV1RawTileGroup;

typedef struct AV1RawFrame {
    AV1RawFrameHeader header;
    AV1RawTileGroup   tile_group;
} AV1RawFrame;

typedef struct AV1RawTileList {
    uint8_t output_frame_width_in_tiles_minus_1;
    uint8_t output_frame_height_in_tiles_minus_1;
    uint16_t tile_count_minus_1;

    AV1RawTileData tile_data;
} AV1RawTileList;

typedef struct AV1RawMetadataHDRCLL {
    uint16_t max_cll;
    uint16_t max_fall;
} AV1RawMetadataHDRCLL;

typedef struct AV1RawMetadataHDRMDCV {
    uint16_t primary_chromaticity_x[3];
    uint16_t primary_chromaticity_y[3];
    uint16_t white_point_chromaticity_x;
    uint16_t white_point_chromaticity_y;
    uint32_t luminance_max;
    uint32_t luminance_min;
} AV1RawMetadataHDRMDCV;

typedef struct AV1RawMetadataScalability {
    uint8_t scalability_mode_idc;
    uint8_t spatial_layers_cnt_minus_1;
    uint8_t spatial_layer_dimensions_present_flag;
    uint8_t spatial_layer_description_present_flag;
    uint8_t temporal_group_description_present_flag;
    uint8_t scalability_structure_reserved_3bits;
    uint16_t spatial_layer_max_width[4];
    uint16_t spatial_layer_max_height[4];
    uint8_t spatial_layer_ref_id[4];
    uint8_t temporal_group_size;
    uint8_t temporal_group_temporal_id[255];
    uint8_t temporal_group_temporal_switching_up_point_flag[255];
    uint8_t temporal_group_spatial_switching_up_point_flag[255];
    uint8_t temporal_group_ref_cnt[255];
    uint8_t temporal_group_ref_pic_diff[255][7];
} AV1RawMetadataScalability;

typedef struct AV1RawMetadataITUTT35 {
    uint8_t itu_t_t35_country_code;
    uint8_t itu_t_t35_country_code_extension_byte;

    uint8_t     *payload;
    AVBufferRef *payload_ref;
    size_t       payload_size;
} AV1RawMetadataITUTT35;

typedef struct AV1RawMetadataTimecode {
    uint8_t  counting_type;
    uint8_t  full_timestamp_flag;
    uint8_t  discontinuity_flag;
    uint8_t  cnt_dropped_flag;
    uint16_t n_frames;
    uint8_t  seconds_value;
    uint8_t  minutes_value;
    uint8_t  hours_value;
    uint8_t  seconds_flag;
    uint8_t  minutes_flag;
    uint8_t  hours_flag;
    uint8_t  time_offset_length;
    uint32_t time_offset_value;
} AV1RawMetadataTimecode;

typedef struct AV1RawMetadataUnknown {
    uint8_t     *payload;
    AVBufferRef *payload_ref;
    size_t       payload_size;
} AV1RawMetadataUnknown;

typedef struct AV1RawMetadata {
    uint64_t metadata_type;
    union {
        AV1RawMetadataHDRCLL      hdr_cll;
        AV1RawMetadataHDRMDCV     hdr_mdcv;
        AV1RawMetadataScalability scalability;
        AV1RawMetadataITUTT35     itut_t35;
        AV1RawMetadataTimecode    timecode;
        AV1RawMetadataUnknown     unknown;
    } metadata;
} AV1RawMetadata;

typedef struct AV1RawPadding {
    uint8_t     *payload;
    AVBufferRef *payload_ref;
    size_t       payload_size;
} AV1RawPadding;


typedef struct AV1RawOBU {
    AV1RawOBUHeader header;

    size_t obu_size;

    union {
        AV1RawSequenceHeader sequence_header;
        AV1RawFrameHeader    frame_header;
        AV1RawFrame          frame;
        AV1RawTileGroup      tile_group;
        AV1RawTileList       tile_list;
        AV1RawMetadata       metadata;
        AV1RawPadding        padding;
    } obu;
} AV1RawOBU;

typedef struct AV1ReferenceFrameState {
    int valid;          // RefValid
    int frame_id;       // RefFrameId
    int upscaled_width; // RefUpscaledWidth
    int frame_width;    // RefFrameWidth
    int frame_height;   // RefFrameHeight
    int render_width;   // RefRenderWidth
    int render_height;  // RefRenderHeight
    int frame_type;     // RefFrameType
    int subsampling_x;  // RefSubsamplingX
    int subsampling_y;  // RefSubsamplingY
    int bit_depth;      // RefBitDepth
    int order_hint;     // RefOrderHint

    int saved_order_hints[AV1_TOTAL_REFS_PER_FRAME]; // SavedOrderHints[ref]

    int8_t  loop_filter_ref_deltas[AV1_TOTAL_REFS_PER_FRAME];
    int8_t  loop_filter_mode_deltas[2];
    uint8_t feature_enabled[AV1_MAX_SEGMENTS][AV1_SEG_LVL_MAX];
    int16_t feature_value[AV1_MAX_SEGMENTS][AV1_SEG_LVL_MAX];
} AV1ReferenceFrameState;

typedef struct CodedBitstreamAV1Context {
    const AVClass *class;

    AV1RawSequenceHeader *sequence_header;
    /** A RefStruct reference backing sequence_header. */
    AV1RawOBU            *sequence_header_ref;

    int     seen_frame_header;
    AVBufferRef *frame_header_ref;
    uint8_t     *frame_header;
    size_t       frame_header_size;

    int temporal_id;
    int spatial_id;
    int operating_point_idc;

    int bit_depth;
    int order_hint;
    int frame_width;
    int frame_height;
    int upscaled_width;
    int render_width;
    int render_height;

    int num_planes;
    int coded_lossless;
    int all_lossless;
    int tile_cols;
    int tile_rows;
    int tile_num;

    int order_hints[AV1_TOTAL_REFS_PER_FRAME];         // OrderHints
    int ref_frame_sign_bias[AV1_TOTAL_REFS_PER_FRAME]; // RefFrameSignBias

    AV1ReferenceFrameState ref[AV1_NUM_REF_FRAMES];

    // AVOptions
    int operating_point;
    // When writing, fix the length in bytes of the obu_size field.
    // Writing will fail with an error if an OBU larger than can be
    // represented by the fixed size is encountered.
    int fixed_obu_size_length;

    int8_t  loop_filter_ref_deltas[AV1_TOTAL_REFS_PER_FRAME];
    int8_t  loop_filter_mode_deltas[2];
    uint8_t feature_enabled[AV1_MAX_SEGMENTS][AV1_SEG_LVL_MAX];
    int16_t feature_value[AV1_MAX_SEGMENTS][AV1_SEG_LVL_MAX];
} CodedBitstreamAV1Context;


#endif /* AVCODEC_CBS_AV1_H */
