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

/**
 * @file
 * AV1 common definitions
 */

#ifndef AVCODEC_AV1_H
#define AVCODEC_AV1_H

// OBU types (section 6.2.2).
typedef enum {
    // 0 reserved.
    AV1_OBU_SEQUENCE_HEADER        = 1,
    AV1_OBU_TEMPORAL_DELIMITER     = 2,
    AV1_OBU_FRAME_HEADER           = 3,
    AV1_OBU_TILE_GROUP             = 4,
    AV1_OBU_METADATA               = 5,
    AV1_OBU_FRAME                  = 6,
    AV1_OBU_REDUNDANT_FRAME_HEADER = 7,
    AV1_OBU_TILE_LIST              = 8,
    // 9-14 reserved.
    AV1_OBU_PADDING                = 15,
} AV1_OBU_Type;

// Metadata types (section 6.7.1).
enum {
    AV1_METADATA_TYPE_HDR_CLL     = 1,
    AV1_METADATA_TYPE_HDR_MDCV    = 2,
    AV1_METADATA_TYPE_SCALABILITY = 3,
    AV1_METADATA_TYPE_ITUT_T35    = 4,
    AV1_METADATA_TYPE_TIMECODE    = 5,
};

// Frame types (section 6.8.2).
enum {
    AV1_FRAME_KEY        = 0,
    AV1_FRAME_INTER      = 1,
    AV1_FRAME_INTRA_ONLY = 2,
    AV1_FRAME_SWITCH     = 3,
};

// Reference frames (section 6.10.24).
enum {
    AV1_REF_FRAME_NONE    = -1,
    AV1_REF_FRAME_INTRA   = 0,
    AV1_REF_FRAME_LAST    = 1,
    AV1_REF_FRAME_LAST2   = 2,
    AV1_REF_FRAME_LAST3   = 3,
    AV1_REF_FRAME_GOLDEN  = 4,
    AV1_REF_FRAME_BWDREF  = 5,
    AV1_REF_FRAME_ALTREF2 = 6,
    AV1_REF_FRAME_ALTREF  = 7,
};

// Constants (section 3).
enum {
    AV1_MAX_OPERATING_POINTS = 32,

    AV1_MAX_SB_SIZE    = 128,
    AV1_MI_SIZE        = 4,

    AV1_MAX_TILE_WIDTH = 4096,
    AV1_MAX_TILE_AREA  = 4096 * 2304,
    AV1_MAX_TILE_ROWS  = 64,
    AV1_MAX_TILE_COLS  = 64,

    AV1_NUM_REF_FRAMES       = 8,
    AV1_REFS_PER_FRAME       = 7,
    AV1_TOTAL_REFS_PER_FRAME = 8,
    AV1_PRIMARY_REF_NONE     = 7,

    AV1_MAX_SEGMENTS = 8,
    AV1_SEG_LVL_MAX  = 8,

    AV1_SEG_LVL_ALT_Q      = 0,
    AV1_SEG_LVL_ALT_LF_Y_V = 1,
    AV1_SEG_LVL_REF_FRAME  = 5,
    AV1_SEG_LVL_SKIP       = 6,
    AV1_SEG_LVL_GLOBAL_MV  = 7,

    AV1_SELECT_SCREEN_CONTENT_TOOLS = 2,
    AV1_SELECT_INTEGER_MV           = 2,

    AV1_SUPERRES_NUM       = 8,
    AV1_SUPERRES_DENOM_MIN = 9,

    AV1_INTERPOLATION_FILTER_SWITCHABLE = 4,

    AV1_GM_ABS_ALPHA_BITS       = 12,
    AV1_GM_ALPHA_PREC_BITS      = 15,
    AV1_GM_ABS_TRANS_ONLY_BITS  = 9,
    AV1_GM_TRANS_ONLY_PREC_BITS = 3,
    AV1_GM_ABS_TRANS_BITS       = 12,
    AV1_GM_TRANS_PREC_BITS      = 6,
    AV1_WARPEDMODEL_PREC_BITS   = 16,

    AV1_WARP_MODEL_IDENTITY    = 0,
    AV1_WARP_MODEL_TRANSLATION = 1,
    AV1_WARP_MODEL_ROTZOOM     = 2,
    AV1_WARP_MODEL_AFFINE      = 3,
    AV1_WARP_PARAM_REDUCE_BITS = 6,

    AV1_DIV_LUT_BITS      = 8,
    AV1_DIV_LUT_PREC_BITS = 14,
    AV1_DIV_LUT_NUM       = 257,

    AV1_MAX_LOOP_FILTER = 63,
};


// The main colour configuration information uses the same ISO/IEC 23001-8
// (H.273) enums as FFmpeg does, so separate definitions are not required.

// Chroma sample position.
enum {
    AV1_CSP_UNKNOWN   = 0,
    AV1_CSP_VERTICAL  = 1, // -> AVCHROMA_LOC_LEFT.
    AV1_CSP_COLOCATED = 2, // -> AVCHROMA_LOC_TOPLEFT.
};

// Scalability modes (section 6.7.5)
enum {
    AV1_SCALABILITY_L1T2 = 0,
    AV1_SCALABILITY_L1T3 = 1,
    AV1_SCALABILITY_L2T1 = 2,
    AV1_SCALABILITY_L2T2 = 3,
    AV1_SCALABILITY_L2T3 = 4,
    AV1_SCALABILITY_S2T1 = 5,
    AV1_SCALABILITY_S2T2 = 6,
    AV1_SCALABILITY_S2T3 = 7,
    AV1_SCALABILITY_L2T1h = 8,
    AV1_SCALABILITY_L2T2h = 9,
    AV1_SCALABILITY_L2T3h = 10,
    AV1_SCALABILITY_S2T1h = 11,
    AV1_SCALABILITY_S2T2h = 12,
    AV1_SCALABILITY_S2T3h = 13,
    AV1_SCALABILITY_SS = 14,
    AV1_SCALABILITY_L3T1 = 15,
    AV1_SCALABILITY_L3T2 = 16,
    AV1_SCALABILITY_L3T3 = 17,
    AV1_SCALABILITY_S3T1 = 18,
    AV1_SCALABILITY_S3T2 = 19,
    AV1_SCALABILITY_S3T3 = 20,
    AV1_SCALABILITY_L3T2_KEY = 21,
    AV1_SCALABILITY_L3T3_KEY = 22,
    AV1_SCALABILITY_L4T5_KEY = 23,
    AV1_SCALABILITY_L4T7_KEY = 24,
    AV1_SCALABILITY_L3T2_KEY_SHIFT = 25,
    AV1_SCALABILITY_L3T3_KEY_SHIFT = 26,
    AV1_SCALABILITY_L4T5_KEY_SHIFT = 27,
    AV1_SCALABILITY_L4T7_KEY_SHIFT = 28,
};

// Frame Restoration types (section 6.10.15)
enum {
    AV1_RESTORE_NONE       = 0,
    AV1_RESTORE_WIENER     = 1,
    AV1_RESTORE_SGRPROJ    = 2,
    AV1_RESTORE_SWITCHABLE = 3,
};

// TX mode (section 6.8.21)
enum {
    AV1_ONLY_4X4        = 0,
    AV1_TX_MODE_LARGEST = 1,
    AV1_TX_MODE_SELECT  = 2,
};

// Sequence Headers are actually unbounded because one can use
// an arbitrary number of leading zeroes when encoding via uvlc.
// The following estimate is based around using the lowest number
// of bits for uvlc encoding.
#define AV1_SANE_SEQUENCE_HEADER_MAX_BITS           3138

#endif /* AVCODEC_AV1_H */
