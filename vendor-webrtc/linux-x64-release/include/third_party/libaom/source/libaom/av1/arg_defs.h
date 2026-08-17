/*
 * Copyright (c) 2021, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */
#ifndef AOM_AV1_ARG_DEFS_H_
#define AOM_AV1_ARG_DEFS_H_

#ifdef __cplusplus
extern "C" {
#endif

#include "config/aom_config.h"
#include "common/args_helper.h"
#if CONFIG_WEBM_IO
#include "common/webmenc.h"
#endif
#include "aom/aomcx.h"

enum TestDecodeFatality {
  TEST_DECODE_OFF,
  TEST_DECODE_FATAL,
  TEST_DECODE_WARN,
};

typedef struct av1_codec_arg_definitions {
  arg_def_t help;
  arg_def_t debugmode;
  arg_def_t outputfile;
  arg_def_t use_nv12;
  arg_def_t use_yv12;
  arg_def_t use_i420;
  arg_def_t use_i422;
  arg_def_t use_i444;
  arg_def_t codecarg;
  arg_def_t passes;
  arg_def_t pass_arg;
  arg_def_t fpf_name;
  arg_def_t limit;
  arg_def_t skip;
  arg_def_t good_dl;
  arg_def_t rt_dl;
  arg_def_t ai_dl;
  arg_def_t quietarg;
  arg_def_t verbosearg;
  arg_def_t psnrarg;
  arg_def_t use_cfg;
  arg_def_t recontest;
  arg_def_t framerate;
  arg_def_t use_webm;
  arg_def_t use_ivf;
  arg_def_t use_obu;
  arg_def_t q_hist_n;
  arg_def_t rate_hist_n;
  arg_def_t disable_warnings;
  arg_def_t disable_warning_prompt;
  arg_def_t bitdeptharg;
  arg_def_t inbitdeptharg;
  arg_def_t input_chroma_subsampling_x;
  arg_def_t input_chroma_subsampling_y;
  arg_def_t usage;
  arg_def_t threads;
  arg_def_t profile;
  arg_def_t width;
  arg_def_t height;
  arg_def_t forced_max_frame_width;
  arg_def_t forced_max_frame_height;
#if CONFIG_WEBM_IO
  arg_def_t stereo_mode;
#endif
  arg_def_t timebase;
  arg_def_t global_error_resilient;
  arg_def_t lag_in_frames;
  arg_def_t large_scale_tile;
  arg_def_t monochrome;
  arg_def_t full_still_picture_hdr;
  arg_def_t use_16bit_internal;
  arg_def_t dropframe_thresh;
  arg_def_t resize_mode;
  arg_def_t resize_denominator;
  arg_def_t resize_kf_denominator;
  arg_def_t superres_mode;
  arg_def_t superres_denominator;
  arg_def_t superres_kf_denominator;
  arg_def_t superres_qthresh;
  arg_def_t superres_kf_qthresh;
  arg_def_t end_usage;
  arg_def_t target_bitrate;
  arg_def_t min_quantizer;
  arg_def_t max_quantizer;
  arg_def_t undershoot_pct;
  arg_def_t overshoot_pct;
  arg_def_t buf_sz;
  arg_def_t buf_initial_sz;
  arg_def_t buf_optimal_sz;
  arg_def_t bias_pct;
  arg_def_t minsection_pct;
  arg_def_t maxsection_pct;
  arg_def_t fwd_kf_enabled;
  arg_def_t kf_min_dist;
  arg_def_t kf_max_dist;
  arg_def_t kf_disabled;
  arg_def_t sframe_dist;
  arg_def_t sframe_mode;
  arg_def_t save_as_annexb;
  arg_def_t noise_sens;
  arg_def_t sharpness;
  arg_def_t static_thresh;
  arg_def_t auto_altref;
  arg_def_t arnr_maxframes;
  arg_def_t arnr_strength;
  arg_def_t tune_metric;
  arg_def_t dist_metric;
  arg_def_t cq_level;
  arg_def_t max_intra_rate_pct;
#if CONFIG_AV1_ENCODER
  arg_def_t cpu_used_av1;
  arg_def_t rowmtarg;
  arg_def_t fpmtarg;
  arg_def_t tile_cols;
  arg_def_t tile_rows;
  arg_def_t auto_tiles;
  arg_def_t enable_tpl_model;
  arg_def_t enable_keyframe_filtering;
  arg_def_t tile_width;
  arg_def_t tile_height;
  arg_def_t lossless;
  arg_def_t enable_cdef;
  arg_def_t enable_restoration;
  arg_def_t enable_rect_partitions;
  arg_def_t enable_ab_partitions;
  arg_def_t enable_1to4_partitions;
  arg_def_t min_partition_size;
  arg_def_t max_partition_size;
  arg_def_t enable_dual_filter;
  arg_def_t enable_chroma_deltaq;
  arg_def_t enable_intra_edge_filter;
  arg_def_t enable_order_hint;
  arg_def_t enable_tx64;
  arg_def_t enable_flip_idtx;
  arg_def_t enable_rect_tx;
  arg_def_t enable_dist_wtd_comp;
  arg_def_t enable_masked_comp;
  arg_def_t enable_onesided_comp;
  arg_def_t enable_interintra_comp;
  arg_def_t enable_smooth_interintra;
  arg_def_t enable_diff_wtd_comp;
  arg_def_t enable_interinter_wedge;
  arg_def_t enable_interintra_wedge;
  arg_def_t enable_global_motion;
  arg_def_t enable_warped_motion;
  arg_def_t enable_filter_intra;
  arg_def_t enable_smooth_intra;
  arg_def_t enable_paeth_intra;
  arg_def_t enable_cfl_intra;
  arg_def_t enable_directional_intra;
  arg_def_t enable_diagonal_intra;
  arg_def_t force_video_mode;
  arg_def_t enable_obmc;
  arg_def_t enable_overlay;
  arg_def_t enable_palette;
  arg_def_t enable_intrabc;
  arg_def_t enable_angle_delta;
  arg_def_t disable_trellis_quant;
  arg_def_t enable_qm;
  arg_def_t qm_min;
  arg_def_t qm_max;
  arg_def_t reduced_tx_type_set;
  arg_def_t use_intra_dct_only;
  arg_def_t use_inter_dct_only;
  arg_def_t use_intra_default_tx_only;
  arg_def_t quant_b_adapt;
  arg_def_t coeff_cost_upd_freq;
  arg_def_t mode_cost_upd_freq;
  arg_def_t mv_cost_upd_freq;
  arg_def_t dv_cost_upd_freq;
  arg_def_t num_tg;
  arg_def_t mtu_size;
  arg_def_t timing_info;
#if CONFIG_TUNE_VMAF
  arg_def_t vmaf_model_path;
#endif
  arg_def_t partition_info_path;
  arg_def_t enable_rate_guide_deltaq;
  arg_def_t rate_distribution_info;
  arg_def_t film_grain_test;
  arg_def_t film_grain_table;
#if CONFIG_DENOISE
  arg_def_t denoise_noise_level;
  arg_def_t denoise_block_size;
  arg_def_t enable_dnl_denoising;
#endif
  arg_def_t enable_ref_frame_mvs;
  arg_def_t frame_parallel_decoding;
  arg_def_t error_resilient_mode;
  arg_def_t aq_mode;
  arg_def_t deltaq_mode;
  arg_def_t deltaq_strength;
  arg_def_t deltalf_mode;
  arg_def_t frame_periodic_boost;
  arg_def_t gf_cbr_boost_pct;
  arg_def_t max_inter_rate_pct;
  arg_def_t min_gf_interval;
  arg_def_t max_gf_interval;
  arg_def_t gf_min_pyr_height;
  arg_def_t gf_max_pyr_height;
  arg_def_t max_reference_frames;
  arg_def_t reduced_reference_set;
  arg_def_t target_seq_level_idx;
  arg_def_t set_min_cr;
  arg_def_t input_color_primaries;
  arg_def_t input_transfer_characteristics;
  arg_def_t input_matrix_coefficients;
  arg_def_t input_chroma_sample_position;
  arg_def_t tune_content;
  arg_def_t cdf_update_mode;
  arg_def_t superblock_size;
  arg_def_t set_tier_mask;
  arg_def_t use_fixed_qp_offsets;
  arg_def_t fixed_qp_offsets;
  arg_def_t vbr_corpus_complexity_lap;
  arg_def_t fwd_kf_dist;
  arg_def_t enable_tx_size_search;
  arg_def_t loopfilter_control;
  arg_def_t two_pass_input;
  arg_def_t two_pass_output;
  arg_def_t two_pass_width;
  arg_def_t two_pass_height;
  arg_def_t second_pass_log;
  arg_def_t auto_intra_tools_off;
  arg_def_t strict_level_conformance;
  arg_def_t kf_max_pyr_height;
  arg_def_t sb_qp_sweep;
  arg_def_t enable_low_complexity_decode;
#endif  // CONFIG_AV1_ENCODER
} av1_codec_arg_definitions_t;

extern const av1_codec_arg_definitions_t g_av1_codec_arg_defs;

#ifdef __cplusplus
}
#endif
#endif  // AOM_AV1_ARG_DEFS_H_
