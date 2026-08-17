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

#ifndef AVCODEC_CBS_VP9_H
#define AVCODEC_CBS_VP9_H

#include <stddef.h>
#include <stdint.h>

#include "cbs.h"


// Miscellaneous constants (section 3).
enum {
    VP9_REFS_PER_FRAME = 3,

    VP9_MIN_TILE_WIDTH_B64 = 4,
    VP9_MAX_TILE_WIDTH_B64 = 64,

    VP9_NUM_REF_FRAMES = 8,
    VP9_MAX_REF_FRAMES = 4,

    VP9_MAX_SEGMENTS = 8,
    VP9_SEG_LVL_MAX  = 4,
};

// Frame types (section 7.2).
enum {
    VP9_KEY_FRAME     = 0,
    VP9_NON_KEY_FRAME = 1,
};

// Frame sync bytes (section 7.2.1).
enum {
    VP9_FRAME_SYNC_0 = 0x49,
    VP9_FRAME_SYNC_1 = 0x83,
    VP9_FRAME_SYNC_2 = 0x42,
};

// Color space values (section 7.2.2).
enum {
    VP9_CS_UNKNOWN   = 0,
    VP9_CS_BT_601    = 1,
    VP9_CS_BT_709    = 2,
    VP9_CS_SMPTE_170 = 3,
    VP9_CS_SMPTE_240 = 4,
    VP9_CS_BT_2020   = 5,
    VP9_CS_RESERVED  = 6,
    VP9_CS_RGB       = 7,
};

// Reference frame types (section 7.4.12).
enum {
    VP9_INTRA_FRAME  = 0,
    VP9_LAST_FRAME   = 1,
    VP9_GOLDEN_FRAME = 2,
    VP9_ALTREF_FRAME = 3,
};

// Superframe properties (section B.3).
enum {
    VP9_MAX_FRAMES_IN_SUPERFRAME = 8,

    VP9_SUPERFRAME_MARKER = 6,
};


typedef struct VP9RawFrameHeader {
    uint8_t frame_marker;
    uint8_t profile_low_bit;
    uint8_t profile_high_bit;

    uint8_t show_existing_frame;
    uint8_t frame_to_show_map_idx;

    uint8_t frame_type;
    uint8_t show_frame;
    uint8_t error_resilient_mode;

    // Color config.
    uint8_t ten_or_twelve_bit;
    uint8_t color_space;
    uint8_t color_range;
    uint8_t subsampling_x;
    uint8_t subsampling_y;

    uint8_t refresh_frame_flags;

    uint8_t intra_only;
    uint8_t reset_frame_context;

    uint8_t ref_frame_idx[VP9_REFS_PER_FRAME];
    uint8_t ref_frame_sign_bias[VP9_MAX_REF_FRAMES];

    uint8_t allow_high_precision_mv;

    uint8_t refresh_frame_context;
    uint8_t frame_parallel_decoding_mode;

    uint8_t frame_context_idx;

    // Frame/render size.
    uint8_t found_ref[VP9_REFS_PER_FRAME];
    uint16_t frame_width_minus_1;
    uint16_t frame_height_minus_1;
    uint8_t render_and_frame_size_different;
    uint16_t render_width_minus_1;
    uint16_t render_height_minus_1;

    // Interpolation filter.
    uint8_t is_filter_switchable;
    uint8_t raw_interpolation_filter_type;

    // Loop filter params.
    uint8_t loop_filter_level;
    uint8_t loop_filter_sharpness;
    uint8_t loop_filter_delta_enabled;
    uint8_t loop_filter_delta_update;
    uint8_t update_ref_delta[VP9_MAX_REF_FRAMES];
    int8_t loop_filter_ref_deltas[VP9_MAX_REF_FRAMES];
    uint8_t update_mode_delta[2];
    int8_t loop_filter_mode_deltas[2];

    // Quantization params.
    uint8_t base_q_idx;
    int8_t delta_q_y_dc;
    int8_t delta_q_uv_dc;
    int8_t delta_q_uv_ac;

    // Segmentation params.
    uint8_t segmentation_enabled;
    uint8_t segmentation_update_map;
    uint8_t segmentation_tree_probs[7];
    uint8_t segmentation_temporal_update;
    uint8_t segmentation_pred_prob[3];
    uint8_t segmentation_update_data;
    uint8_t segmentation_abs_or_delta_update;
    uint8_t feature_enabled[VP9_MAX_SEGMENTS][VP9_SEG_LVL_MAX];
    uint8_t feature_value[VP9_MAX_SEGMENTS][VP9_SEG_LVL_MAX];
    uint8_t feature_sign[VP9_MAX_SEGMENTS][VP9_SEG_LVL_MAX];

    // Tile info.
    uint8_t tile_cols_log2;
    uint8_t tile_rows_log2;

    uint16_t header_size_in_bytes;
} VP9RawFrameHeader;

typedef struct VP9RawFrame {
    VP9RawFrameHeader header;

    uint8_t     *data;
    AVBufferRef *data_ref;
    size_t       data_size;
} VP9RawFrame;

typedef struct VP9RawSuperframeIndex {
    uint8_t superframe_marker;
    uint8_t bytes_per_framesize_minus_1;
    uint8_t frames_in_superframe_minus_1;
    uint32_t frame_sizes[VP9_MAX_FRAMES_IN_SUPERFRAME];
} VP9RawSuperframeIndex;

typedef struct VP9RawSuperframe {
    VP9RawFrame frames[VP9_MAX_FRAMES_IN_SUPERFRAME];
    VP9RawSuperframeIndex index;
} VP9RawSuperframe;

typedef struct VP9ReferenceFrameState {
    int frame_width;    // RefFrameWidth
    int frame_height;   // RefFrameHeight
    int subsampling_x;  // RefSubsamplingX
    int subsampling_y;  // RefSubsamplingY
    int bit_depth;      // RefBitDepth
} VP9ReferenceFrameState;

typedef struct CodedBitstreamVP9Context {
    int profile;

    // Frame dimensions in 8x8 mode info blocks.
    uint16_t mi_cols;
    uint16_t mi_rows;
    // Frame dimensions in 64x64 superblocks.
    uint16_t sb64_cols;
    uint16_t sb64_rows;

    int frame_width;
    int frame_height;

    uint8_t subsampling_x;
    uint8_t subsampling_y;
    int bit_depth;

    VP9ReferenceFrameState ref[VP9_NUM_REF_FRAMES];
} CodedBitstreamVP9Context;


#endif /* AVCODEC_CBS_VP9_H */
