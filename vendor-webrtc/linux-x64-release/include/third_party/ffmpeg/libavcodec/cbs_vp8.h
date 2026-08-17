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

#ifndef AVCODEC_CBS_VP8_H
#define AVCODEC_CBS_VP8_H

#include <stddef.h>
#include <stdint.h>

#include "cbs.h"

enum {
    VP8_START_CODE_0 = 0x9D,
    VP8_START_CODE_1 = 0x01,
    VP8_START_CODE_2 = 0x2A,
};

enum {
    VP8_KEY_FRAME = 0,
    VP8_NON_KEY_FRAME = 1,
};

typedef struct VP8RawFrameHeader {
    // frame tag
    uint8_t frame_type;
    uint8_t profile;
    uint8_t show_frame;
    uint32_t first_partition_length_in_bytes;

    uint16_t width;
    uint8_t horizontal_scale;
    uint16_t height;
    uint8_t vertical_scale;

    // frame header
    uint8_t color_space;
    uint8_t clamping_type;

    // segmentation
    uint8_t segmentation_enable;
    uint8_t update_segment_map;
    uint8_t update_segment_feature_data;
    uint8_t segment_feature_mode;
    uint8_t segment_qp_update[4];
    int8_t segment_qp[4];
    uint8_t segment_loop_filter_level_update[4];
    int8_t segment_loop_filter_level[4];
    uint8_t segment_probs_update[3];
    uint8_t segment_probs[3];

    // loop filter
    uint8_t loop_filter_type;
    uint8_t loop_filter_level;
    uint8_t loop_filter_sharpness;
    uint8_t mode_ref_lf_delta_enable;
    uint8_t mode_ref_lf_delta_update;
    uint8_t ref_lf_deltas_update[4];
    int8_t ref_lf_deltas[4];
    uint8_t mode_lf_deltas_update[4];
    int8_t mode_lf_deltas[4];

    uint8_t log2_token_partitions;

    // qp
    uint8_t base_qindex;
    uint8_t y1dc_delta_q_present;
    int8_t y1dc_delta_q;
    uint8_t y2dc_delta_q_present;
    int8_t y2dc_delta_q;
    uint8_t y2ac_delta_q_present;
    int8_t y2ac_delta_q;
    uint8_t uvdc_delta_q_present;
    int8_t uvdc_delta_q;
    uint8_t uvac_delta_q_present;
    int8_t uvac_delta_q;

    // ref
    uint8_t refresh_golden_frame;
    uint8_t refresh_alternate_frame;
    uint8_t copy_buffer_to_golden;
    uint8_t copy_buffer_to_alternate;
    uint8_t ref_frame_sign_bias_golden;
    uint8_t ref_frame_sign_bias_alternate;
    uint8_t refresh_last_frame;

    uint8_t refresh_entropy_probs;

    // token probs
    uint8_t coeff_prob_update[4][8][3][11];
    uint8_t coeff_prob[4][8][3][11];

    uint8_t mb_no_skip_coeff;
    uint8_t prob_skip_false;

    uint8_t prob_intra;
    uint8_t prob_last;
    uint8_t prob_golden;

    uint8_t intra_16x16_prob_update;
    uint8_t intra_16x16_prob[4];

    uint8_t intra_chrome_prob_update;
    uint8_t intra_chrome_prob[3];

    // mv probs
    uint8_t mv_prob_update[2][19];
    uint8_t mv_prob[2][19];
} VP8RawFrameHeader;

typedef struct VP8RawFrame {
    VP8RawFrameHeader header;

    uint8_t *data;
    AVBufferRef *data_ref;
    size_t data_size;
} VP8RawFrame;

#endif /* AVCODEC_CBS_VP8_H */
