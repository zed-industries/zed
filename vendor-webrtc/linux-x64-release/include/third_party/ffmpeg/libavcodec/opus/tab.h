/*
 * Copyright (c) 2012 Andrew D'Addesio
 * Copyright (c) 2013-2014 Mozilla Corporation
 * Copyright (c) 2016 Rostislav Pehlivanov <atomnuker@gmail.com>
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

#ifndef AVCODEC_OPUS_TAB_H
#define AVCODEC_OPUS_TAB_H

#include <stdint.h>

#include "libavutil/attributes_internal.h"

FF_VISIBILITY_PUSH_HIDDEN
extern const uint8_t  ff_celt_band_end[];

extern const uint8_t  ff_opus_default_coupled_streams[];

extern const uint16_t ff_silk_model_lbrr_flags_40[];
extern const uint16_t ff_silk_model_lbrr_flags_60[];

extern const uint16_t ff_silk_model_stereo_s1[];
extern const uint16_t ff_silk_model_stereo_s2[];
extern const uint16_t ff_silk_model_stereo_s3[];
extern const uint16_t ff_silk_model_mid_only[];

extern const uint16_t ff_silk_model_frame_type_inactive[];
extern const uint16_t ff_silk_model_frame_type_active[];

extern const uint16_t ff_silk_model_gain_highbits[3][9];
extern const uint16_t ff_silk_model_gain_lowbits[];
extern const uint16_t ff_silk_model_gain_delta[];

extern const uint16_t ff_silk_model_lsf_s1[2][2][33];
extern const uint16_t ff_silk_model_lsf_s2[32][10];
extern const uint16_t ff_silk_model_lsf_s2_ext[];
extern const uint16_t ff_silk_model_lsf_interpolation_offset[];

extern const uint16_t ff_silk_model_pitch_highbits[];
extern const uint16_t ff_silk_model_pitch_lowbits_nb[];
extern const uint16_t ff_silk_model_pitch_lowbits_mb[];
extern const uint16_t ff_silk_model_pitch_lowbits_wb[];
extern const uint16_t ff_silk_model_pitch_delta[];
extern const uint16_t ff_silk_model_pitch_contour_nb10ms[];
extern const uint16_t ff_silk_model_pitch_contour_nb20ms[];
extern const uint16_t ff_silk_model_pitch_contour_mbwb10ms[];
extern const uint16_t ff_silk_model_pitch_contour_mbwb20ms[];

extern const uint16_t ff_silk_model_ltp_filter[];
extern const uint16_t ff_silk_model_ltp_filter0_sel[];
extern const uint16_t ff_silk_model_ltp_filter1_sel[];
extern const uint16_t ff_silk_model_ltp_filter2_sel[];
extern const uint16_t ff_silk_model_ltp_scale_index[];

extern const uint16_t ff_silk_model_lcg_seed[];

extern const uint16_t ff_silk_model_exc_rate[2][10];

extern const uint16_t ff_silk_model_pulse_count[11][19];
extern const uint16_t ff_silk_model_pulse_location[4][168];

extern const uint16_t ff_silk_model_excitation_lsb[];
extern const uint16_t ff_silk_model_excitation_sign[3][2][7][3];

extern const int16_t  ff_silk_stereo_weights[];

extern const uint8_t  ff_silk_lsf_s2_model_sel_nbmb[32][10];
extern const uint8_t  ff_silk_lsf_s2_model_sel_wb[32][16];

extern const uint8_t  ff_silk_lsf_pred_weights_nbmb[2][9];
extern const uint8_t  ff_silk_lsf_pred_weights_wb[2][15];

extern const uint8_t  ff_silk_lsf_weight_sel_nbmb[32][9];
extern const uint8_t  ff_silk_lsf_weight_sel_wb[32][15];

extern const uint8_t  ff_silk_lsf_codebook_nbmb[32][10];
extern const uint8_t  ff_silk_lsf_codebook_wb[32][16];

extern const uint16_t ff_silk_lsf_min_spacing_nbmb[];
extern const uint16_t ff_silk_lsf_min_spacing_wb[];

extern const uint8_t  ff_silk_lsf_ordering_nbmb[];
extern const uint8_t  ff_silk_lsf_ordering_wb[];

extern const int16_t  ff_silk_cosine[];

extern const uint16_t ff_silk_pitch_scale[];
extern const uint16_t ff_silk_pitch_min_lag[];
extern const uint16_t ff_silk_pitch_max_lag[];

extern const int8_t   ff_silk_pitch_offset_nb10ms[3][2];
extern const int8_t   ff_silk_pitch_offset_nb20ms[11][4];
extern const int8_t   ff_silk_pitch_offset_mbwb10ms[12][2];
extern const int8_t   ff_silk_pitch_offset_mbwb20ms[34][4];

extern const int8_t   ff_silk_ltp_filter0_taps[8][5];
extern const int8_t   ff_silk_ltp_filter1_taps[16][5];
extern const int8_t   ff_silk_ltp_filter2_taps[32][5];

extern const uint16_t ff_silk_ltp_scale_factor[];

extern const uint8_t  ff_silk_shell_blocks[3][2];

extern const uint8_t  ff_silk_quant_offset[2][2];

extern const int      ff_silk_stereo_interp_len[3];

extern const uint16_t ff_celt_model_tapset[];
extern const uint16_t ff_celt_model_spread[];
extern const uint16_t ff_celt_model_alloc_trim[];
extern const uint16_t ff_celt_model_energy_small[];

extern const uint8_t  ff_celt_freq_bands[];
extern const uint8_t  ff_celt_freq_range[];
extern const uint8_t  ff_celt_log_freq_range[];

extern const int8_t   ff_celt_tf_select[4][2][2][2];

extern const float    ff_celt_mean_energy[];

extern const float    ff_celt_alpha_coef[];
extern const float    ff_celt_beta_coef[];

extern const uint8_t  ff_celt_coarse_energy_dist[4][2][42];

extern const uint8_t  ff_celt_static_alloc[11][21];
extern const uint8_t  ff_celt_static_caps[4][2][21];

extern const uint8_t  ff_celt_cache_bits[392];
extern const int16_t  ff_celt_cache_index[105];

extern const uint8_t  ff_celt_log2_frac[];

extern const uint8_t  ff_celt_bit_interleave[];
extern const uint8_t  ff_celt_bit_deinterleave[];

extern const uint8_t  ff_celt_hadamard_order[];

extern const uint16_t ff_celt_qn_exp2[];

extern const float    ff_celt_postfilter_taps[3][3];

extern const float    ff_celt_window2[120];

extern const float    ff_celt_window_padded[];
static const float *const ff_celt_window = &ff_celt_window_padded[8];

extern const float    ff_opus_deemph_weights[];

extern const uint32_t * const ff_celt_pvq_u_row[15];
FF_VISIBILITY_POP_HIDDEN

#endif /* AVCODEC_OPUS_TAB_H */
