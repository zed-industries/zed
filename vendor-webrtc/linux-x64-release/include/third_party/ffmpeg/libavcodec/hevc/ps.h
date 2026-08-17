/*
 * HEVC parameter set parsing
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

#ifndef AVCODEC_HEVC_PS_H
#define AVCODEC_HEVC_PS_H

#include <stdint.h>

#include "libavutil/pixfmt.h"
#include "libavutil/rational.h"

#include "libavcodec/avcodec.h"
#include "libavcodec/get_bits.h"
#include "libavcodec/h2645_vui.h"

#include "hevc.h"

#define HEVC_VPS_MAX_LAYERS 2

typedef struct HEVCSublayerHdrParams {
    uint32_t bit_rate_value_minus1[HEVC_MAX_CPB_CNT];
    uint32_t cpb_size_value_minus1[HEVC_MAX_CPB_CNT];
    uint32_t cpb_size_du_value_minus1[HEVC_MAX_CPB_CNT];
    uint32_t bit_rate_du_value_minus1[HEVC_MAX_CPB_CNT];
    uint32_t cbr_flag;
} HEVCSublayerHdrParams;

// flags in bitmask form
typedef struct HEVCHdrFlagParams {
    uint8_t fixed_pic_rate_general_flag;
    uint8_t fixed_pic_rate_within_cvs_flag;
    uint8_t low_delay_hrd_flag;
} HEVCHdrFlagParams;

typedef struct HEVCHdrParams {
    HEVCHdrFlagParams flags;
    uint8_t nal_hrd_parameters_present_flag;
    uint8_t vcl_hrd_parameters_present_flag;
    uint8_t sub_pic_hrd_params_present_flag;
    uint8_t sub_pic_cpb_params_in_pic_timing_sei_flag;

    uint8_t tick_divisor_minus2;
    uint8_t du_cpb_removal_delay_increment_length_minus1;
    uint8_t dpb_output_delay_du_length_minus1;
    uint8_t bit_rate_scale;
    uint8_t cpb_size_scale;
    uint8_t cpb_size_du_scale;
    uint8_t initial_cpb_removal_delay_length_minus1;
    uint8_t au_cpb_removal_delay_length_minus1;
    uint8_t dpb_output_delay_length_minus1;
    uint8_t cpb_cnt_minus1[HEVC_MAX_SUB_LAYERS];
    uint16_t elemental_duration_in_tc_minus1[HEVC_MAX_SUB_LAYERS];

    HEVCSublayerHdrParams nal_params[HEVC_MAX_SUB_LAYERS];
    HEVCSublayerHdrParams vcl_params[HEVC_MAX_SUB_LAYERS];
} HEVCHdrParams;

typedef struct ShortTermRPS {
    int32_t delta_poc[32];
    uint32_t used;

    uint8_t delta_idx;
    uint8_t num_negative_pics;
    uint8_t num_delta_pocs;
    uint8_t rps_idx_num_delta_pocs;

    uint16_t abs_delta_rps;
    unsigned delta_rps_sign:1;

    unsigned rps_predict:1;
    unsigned use_delta:1;
} ShortTermRPS;

typedef struct HEVCWindow {
    unsigned int left_offset;
    unsigned int right_offset;
    unsigned int top_offset;
    unsigned int bottom_offset;
} HEVCWindow;

typedef struct VUI {
    H2645VUI common;

    int neutra_chroma_indication_flag;

    int field_seq_flag;
    int frame_field_info_present_flag;

    int default_display_window_flag;
    HEVCWindow def_disp_win;

    int vui_timing_info_present_flag;
    uint32_t vui_num_units_in_tick;
    uint32_t vui_time_scale;
    int vui_poc_proportional_to_timing_flag;
    int vui_num_ticks_poc_diff_one_minus1;
    int vui_hrd_parameters_present_flag;

    int bitstream_restriction_flag;
    int tiles_fixed_structure_flag;
    int motion_vectors_over_pic_boundaries_flag;
    int restricted_ref_pic_lists_flag;
    int min_spatial_segmentation_idc;
    int max_bytes_per_pic_denom;
    int max_bits_per_min_cu_denom;
    int log2_max_mv_length_horizontal;
    int log2_max_mv_length_vertical;
} VUI;

typedef struct PTLCommon {
    uint8_t profile_space;
    uint8_t tier_flag;
    uint8_t profile_idc;
    uint8_t profile_compatibility_flag[32];
    uint8_t progressive_source_flag;
    uint8_t interlaced_source_flag;
    uint8_t non_packed_constraint_flag;
    uint8_t frame_only_constraint_flag;
    uint8_t max_12bit_constraint_flag;
    uint8_t max_10bit_constraint_flag;
    uint8_t max_8bit_constraint_flag;
    uint8_t max_422chroma_constraint_flag;
    uint8_t max_420chroma_constraint_flag;
    uint8_t max_monochrome_constraint_flag;
    uint8_t intra_constraint_flag;
    uint8_t one_picture_only_constraint_flag;
    uint8_t lower_bit_rate_constraint_flag;
    uint8_t max_14bit_constraint_flag;
    uint8_t inbld_flag;
    uint8_t level_idc;
} PTLCommon;

typedef struct PTL {
    PTLCommon general_ptl;
    PTLCommon sub_layer_ptl[HEVC_MAX_SUB_LAYERS];

    uint8_t sub_layer_profile_present_flag[HEVC_MAX_SUB_LAYERS];
    uint8_t sub_layer_level_present_flag[HEVC_MAX_SUB_LAYERS];
} PTL;

typedef struct RepFormat {
    uint16_t pic_width_in_luma_samples;
    uint16_t pic_height_in_luma_samples;
    uint8_t  chroma_format_idc;
    uint8_t  separate_colour_plane_flag;
    uint8_t  bit_depth_luma;    ///< bit_depth_vps_luma_minus8 + 8
    uint8_t  bit_depth_chroma;  ///< bit_depth_vps_chroma_minus8 + 8
    uint16_t conf_win_left_offset;
    uint16_t conf_win_right_offset;
    uint16_t conf_win_top_offset;
    uint16_t conf_win_bottom_offset;
} RepFormat;

typedef struct HEVCVPS {
    unsigned int vps_id;

    uint8_t vps_temporal_id_nesting_flag;
    int vps_max_layers;
    int vps_max_sub_layers; ///< vps_max_temporal_layers_minus1 + 1

    PTL ptl;
    int vps_sub_layer_ordering_info_present_flag;
    unsigned int vps_max_dec_pic_buffering[HEVC_MAX_SUB_LAYERS];
    unsigned int vps_num_reorder_pics[HEVC_MAX_SUB_LAYERS];
    unsigned int vps_max_latency_increase[HEVC_MAX_SUB_LAYERS];
    int vps_max_layer_id;
    int vps_num_layer_sets; ///< vps_num_layer_sets_minus1 + 1
    uint8_t vps_timing_info_present_flag;
    uint32_t vps_num_units_in_tick;
    uint32_t vps_time_scale;
    uint8_t vps_poc_proportional_to_timing_flag;
    int vps_num_ticks_poc_diff_one; ///< vps_num_ticks_poc_diff_one_minus1 + 1
    int vps_num_hrd_parameters;

    HEVCHdrParams *hdr;

    /* VPS extension */

    /* Number of layers this VPS was parsed for, between 1 and
     * min(HEVC_VPS_MAX_LAYERS, vps_max_layers).
     *
     * Note that vps_max_layers contains the layer count declared in the
     * bitstream, while nb_layers contains the number of layers exported to
     * users of this API (which may be smaller as we only support a subset of
     * multilayer extensions).
     *
     * Arrays below documented as [layer_idx] have nb_layers valid entries.
     */
    int nb_layers;

    uint16_t scalability_mask_flag;

    // LayerIdxInVps[nuh_layer_id], i.e. a mapping of nuh_layer_id to VPS layer
    // indices. Valid values are between 0 and HEVC_VPS_MAX_LAYERS. Entries for
    // unmapped values of nuh_layer_id are set to -1.
    int8_t layer_idx[HEVC_MAX_NUH_LAYER_ID + 1];

    uint8_t layer_id_in_nuh[HEVC_VPS_MAX_LAYERS];

    uint8_t default_ref_layers_active;
    uint8_t max_one_active_ref_layer;
    uint8_t poc_lsb_aligned;
    // bitmask of poc_lsb_not_present[layer_idx]
    uint8_t poc_lsb_not_present;

    struct {
        unsigned max_dec_pic_buffering; // max_vps_dec_pic_buffering_minus1 + 1
        unsigned max_num_reorder_pics;  // max_vps_num_reorder_pics
        unsigned max_latency_increase;  // max_vps_latency_increase_plus1 - 1
    } dpb_size;

    // ViewId[layer_idx]
    uint16_t view_id[HEVC_VPS_MAX_LAYERS];

    // NumOutputLayerSets
    uint8_t num_output_layer_sets;
    // Bitmasks specifying output layer sets. i-th bit set means layer with VPS
    // index i is present in the layer set.
    uint64_t ols[HEVC_VPS_MAX_LAYERS];

    // NumDirectRefLayers[layer_idx]
    uint8_t num_direct_ref_layers[HEVC_VPS_MAX_LAYERS];
    uint8_t num_add_layer_sets;

    RepFormat rep_format;

    uint8_t *data;
    int data_size;
} HEVCVPS;

typedef struct ScalingList {
    /* This is a little wasteful, since sizeID 0 only needs 8 coeffs,
     * and size ID 3 only has 2 arrays, not 6. */
    uint8_t sl[4][6][64];
    uint8_t sl_dc[2][6];
} ScalingList;

typedef struct HEVCSPS {
    unsigned vps_id;
    int chroma_format_idc;

    HEVCWindow output_window;

    HEVCWindow pic_conf_win;

    HEVCHdrParams hdr;

    int bit_depth;
    int bit_depth_chroma;
    int pixel_shift;
    enum AVPixelFormat pix_fmt;

    unsigned int log2_max_poc_lsb;

    int max_sub_layers;
    struct {
        int max_dec_pic_buffering;
        int num_reorder_pics;
        int max_latency_increase;
    } temporal_layer[HEVC_MAX_SUB_LAYERS];

    int vui_present;
    VUI vui;
    PTL ptl;

    ScalingList scaling_list;

    unsigned int nb_st_rps;
    ShortTermRPS st_rps[HEVC_MAX_SHORT_TERM_REF_PIC_SETS];

    uint16_t lt_ref_pic_poc_lsb_sps[HEVC_MAX_LONG_TERM_REF_PICS];
    uint32_t used_by_curr_pic_lt;
    uint8_t num_long_term_ref_pics_sps;

    struct {
        uint8_t bit_depth;
        uint8_t bit_depth_chroma;
        unsigned int log2_min_pcm_cb_size;
        unsigned int log2_max_pcm_cb_size;
    } pcm;

    unsigned int log2_min_cb_size;
    unsigned int log2_diff_max_min_coding_block_size;
    unsigned int log2_min_tb_size;
    unsigned int log2_max_trafo_size;
    unsigned int log2_ctb_size;
    unsigned int log2_min_pu_size;
    unsigned int log2_diff_max_min_transform_block_size;

    int max_transform_hierarchy_depth_inter;
    int max_transform_hierarchy_depth_intra;

    uint8_t separate_colour_plane;
    uint8_t conformance_window;
    uint8_t pcm_enabled;
    uint8_t pcm_loop_filter_disabled;
    uint8_t sublayer_ordering_info;
    uint8_t temporal_id_nesting;
    uint8_t extension_present;
    uint8_t scaling_list_enabled;
    uint8_t amp_enabled;
    uint8_t sao_enabled;
    uint8_t long_term_ref_pics_present;
    uint8_t temporal_mvp_enabled;
    uint8_t strong_intra_smoothing_enabled;
    uint8_t range_extension;
    uint8_t transform_skip_rotation_enabled;
    uint8_t transform_skip_context_enabled;
    uint8_t implicit_rdpcm_enabled;
    uint8_t explicit_rdpcm_enabled;
    uint8_t extended_precision_processing;
    uint8_t intra_smoothing_disabled;
    uint8_t high_precision_offsets_enabled;
    uint8_t persistent_rice_adaptation_enabled;
    uint8_t cabac_bypass_alignment_enabled;

    uint8_t multilayer_extension;
    uint8_t sps_3d_extension;

    uint8_t scc_extension;
    uint8_t curr_pic_ref_enabled;
    uint8_t palette_mode_enabled;
    uint8_t palette_predictor_initializers_present;
    uint8_t intra_boundary_filtering_disabled;

    int palette_max_size;
    int delta_palette_max_predictor_size;
    int sps_num_palette_predictor_initializers;
    int sps_palette_predictor_initializer[3][HEVC_MAX_PALETTE_PREDICTOR_SIZE];
    int motion_vector_resolution_control_idc;

    /// coded frame dimension in various units
    int width;
    int height;
    int ctb_width;
    int ctb_height;
    int ctb_size;
    int min_cb_width;
    int min_cb_height;
    int min_tb_width;
    int min_tb_height;
    int min_pu_width;
    int min_pu_height;
    int tb_mask;

    int hshift[3];
    int vshift[3];

    int qp_bd_offset;

    uint8_t *data;
    int data_size;

    const HEVCVPS *vps; ///< RefStruct reference
} HEVCSPS;

typedef struct HEVCPPS {
    unsigned int pps_id;
    unsigned int sps_id; ///< seq_parameter_set_id

    uint8_t sign_data_hiding_flag;

    uint8_t cabac_init_present_flag;

    int num_ref_idx_l0_default_active; ///< num_ref_idx_l0_default_active_minus1 + 1
    int num_ref_idx_l1_default_active; ///< num_ref_idx_l1_default_active_minus1 + 1
    int pic_init_qp_minus26;

    uint8_t constrained_intra_pred_flag;
    uint8_t transform_skip_enabled_flag;

    uint8_t cu_qp_delta_enabled_flag;
    int diff_cu_qp_delta_depth;

    int cb_qp_offset;
    int cr_qp_offset;
    uint8_t pic_slice_level_chroma_qp_offsets_present_flag;
    uint8_t weighted_pred_flag;
    uint8_t weighted_bipred_flag;
    uint8_t output_flag_present_flag;
    uint8_t transquant_bypass_enable_flag;

    uint8_t dependent_slice_segments_enabled_flag;
    uint8_t tiles_enabled_flag;
    uint8_t entropy_coding_sync_enabled_flag;

    uint16_t num_tile_columns;   ///< num_tile_columns_minus1 + 1
    uint16_t num_tile_rows;      ///< num_tile_rows_minus1 + 1
    uint8_t uniform_spacing_flag;
    uint8_t loop_filter_across_tiles_enabled_flag;

    uint8_t seq_loop_filter_across_slices_enabled_flag;

    uint8_t deblocking_filter_control_present_flag;
    uint8_t deblocking_filter_override_enabled_flag;
    uint8_t disable_dbf;
    int beta_offset;    ///< beta_offset_div2 * 2
    int tc_offset;      ///< tc_offset_div2 * 2

    uint8_t scaling_list_data_present_flag;
    ScalingList scaling_list;

    uint8_t lists_modification_present_flag;
    int log2_parallel_merge_level; ///< log2_parallel_merge_level_minus2 + 2
    int num_extra_slice_header_bits;
    uint8_t slice_header_extension_present_flag;
    uint8_t log2_max_transform_skip_block_size;
    uint8_t pps_extension_present_flag;
    uint8_t pps_range_extensions_flag;
    uint8_t pps_multilayer_extension_flag;
    uint8_t pps_3d_extension_flag;
    uint8_t pps_scc_extension_flag;
    uint8_t cross_component_prediction_enabled_flag;
    uint8_t chroma_qp_offset_list_enabled_flag;
    uint8_t diff_cu_chroma_qp_offset_depth;
    uint8_t chroma_qp_offset_list_len_minus1;
    int8_t  cb_qp_offset_list[6];
    int8_t  cr_qp_offset_list[6];
    uint8_t log2_sao_offset_scale_luma;
    uint8_t log2_sao_offset_scale_chroma;

    // Multilayer extension parameters
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
    int8_t phase_hor_chroma[64];
    int8_t phase_ver_chroma[64];
    uint8_t colour_mapping_enabled_flag;
    uint8_t num_cm_ref_layers;
    uint8_t cm_ref_layer_id[62];
    uint8_t cm_octant_depth;
    uint8_t cm_y_part_num_log2;
    uint8_t luma_bit_depth_cm_input;
    uint8_t chroma_bit_depth_cm_input;
    uint8_t luma_bit_depth_cm_output;
    uint8_t chroma_bit_depth_cm_output;
    uint8_t cm_res_quant_bits;
    uint8_t cm_delta_flc_bits;
    int8_t cm_adapt_threshold_u_delta;
    int8_t cm_adapt_threshold_v_delta;

    // 3D extension parameters
    uint8_t pps_bit_depth_for_depth_layers_minus8;

    // SCC extension parameters
    uint8_t pps_curr_pic_ref_enabled_flag;
    uint8_t residual_adaptive_colour_transform_enabled_flag;
    uint8_t pps_slice_act_qp_offsets_present_flag;
    int8_t  pps_act_y_qp_offset;  // _plus5
    int8_t  pps_act_cb_qp_offset; // _plus5
    int8_t  pps_act_cr_qp_offset; // _plus3
    uint8_t pps_palette_predictor_initializers_present_flag;
    uint8_t pps_num_palette_predictor_initializers;
    uint8_t monochrome_palette_flag;
    uint8_t luma_bit_depth_entry;
    uint8_t chroma_bit_depth_entry;
    uint16_t pps_palette_predictor_initializer[3][HEVC_MAX_PALETTE_PREDICTOR_SIZE];

    // Inferred parameters
    unsigned int *column_width;  ///< ColumnWidth
    unsigned int *row_height;    ///< RowHeight
    unsigned int *col_bd;        ///< ColBd
    unsigned int *row_bd;        ///< RowBd
    int *col_idxX;

    int *ctb_addr_rs_to_ts; ///< CtbAddrRSToTS
    int *ctb_addr_ts_to_rs; ///< CtbAddrTSToRS
    int *tile_id;           ///< TileId
    int *tile_pos_rs;       ///< TilePosRS
    int *min_tb_addr_zs;    ///< MinTbAddrZS
    int *min_tb_addr_zs_tab;///< MinTbAddrZS

    uint8_t *data;
    int data_size;

    const HEVCSPS *sps;     ///< RefStruct reference
} HEVCPPS;

typedef struct HEVCParamSets {
    const HEVCVPS *vps_list[HEVC_MAX_VPS_COUNT]; ///< RefStruct references
    const HEVCSPS *sps_list[HEVC_MAX_SPS_COUNT]; ///< RefStruct references
    const HEVCPPS *pps_list[HEVC_MAX_PPS_COUNT]; ///< RefStruct references
} HEVCParamSets;

/**
 * Parse the SPS from the bitstream into the provided HEVCSPS struct.
 *
 * @param sps_id the SPS id will be written here
 * @param apply_defdispwin if set 1, the default display window from the VUI
 *                         will be applied to the video dimensions
 * @param vps_list if non-NULL, this function will validate that the SPS refers
 *                 to an existing VPS
 */
int ff_hevc_parse_sps(HEVCSPS *sps, GetBitContext *gb, unsigned int *sps_id,
                      unsigned nuh_layer_id, int apply_defdispwin,
                      const HEVCVPS * const *vps_list, AVCodecContext *avctx);

int ff_hevc_decode_nal_vps(GetBitContext *gb, AVCodecContext *avctx,
                           HEVCParamSets *ps);
int ff_hevc_decode_nal_sps(GetBitContext *gb, AVCodecContext *avctx,
                           HEVCParamSets *ps, unsigned nuh_layer_id,
                           int apply_defdispwin);
int ff_hevc_decode_nal_pps(GetBitContext *gb, AVCodecContext *avctx,
                           HEVCParamSets *ps);

void ff_hevc_ps_uninit(HEVCParamSets *ps);

int ff_hevc_decode_short_term_rps(GetBitContext *gb, AVCodecContext *avctx,
                                  ShortTermRPS *rps, const HEVCSPS *sps, int is_slice_header);

int ff_hevc_encode_nal_vps(HEVCVPS *vps, unsigned int id,
                           uint8_t *buf, int buf_size);

/**
 * Compute POC of the current frame and return it.
 */
int ff_hevc_compute_poc(const HEVCSPS *sps, int pocTid0, int poc_lsb, int nal_unit_type);

#endif /* AVCODEC_HEVC_PS_H */
