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

#ifndef AVCODEC_H264_SEI_H
#define AVCODEC_H264_SEI_H

#include "get_bits.h"
#include "h2645_sei.h"
#include "h264_ps.h"
#include "sei.h"


/**
 * pic_struct in picture timing SEI message
 */
typedef enum {
    H264_SEI_PIC_STRUCT_FRAME             = 0, ///<  0: %frame
    H264_SEI_PIC_STRUCT_TOP_FIELD         = 1, ///<  1: top field
    H264_SEI_PIC_STRUCT_BOTTOM_FIELD      = 2, ///<  2: bottom field
    H264_SEI_PIC_STRUCT_TOP_BOTTOM        = 3, ///<  3: top field, bottom field, in that order
    H264_SEI_PIC_STRUCT_BOTTOM_TOP        = 4, ///<  4: bottom field, top field, in that order
    H264_SEI_PIC_STRUCT_TOP_BOTTOM_TOP    = 5, ///<  5: top field, bottom field, top field repeated, in that order
    H264_SEI_PIC_STRUCT_BOTTOM_TOP_BOTTOM = 6, ///<  6: bottom field, top field, bottom field repeated, in that order
    H264_SEI_PIC_STRUCT_FRAME_DOUBLING    = 7, ///<  7: %frame doubling
    H264_SEI_PIC_STRUCT_FRAME_TRIPLING    = 8  ///<  8: %frame tripling
} H264_SEI_PicStructType;

typedef struct H264SEITimeCode {
    /* When not continuously receiving full timecodes, we have to reference
       the previous timecode received */
    int full;
    int frame;
    int seconds;
    int minutes;
    int hours;
    int dropframe;
} H264SEITimeCode;

typedef struct H264SEIPictureTiming {
    // maximum size of pic_timing according to the spec should be 274 bits
    uint8_t payload[40];
    int     payload_size_bytes;

    int present;
    H264_SEI_PicStructType pic_struct;

    /**
     * Bit set of clock types for fields/frames in picture timing SEI message.
     * For each found ct_type, appropriate bit is set (e.g., bit 1 for
     * interlaced).
     */
    int ct_type;

    /**
     * dpb_output_delay in picture timing SEI message, see H.264 C.2.2
     */
    int dpb_output_delay;

    /**
     * cpb_removal_delay in picture timing SEI message, see H.264 C.1.2
     */
    int cpb_removal_delay;

    /**
     * Maximum three timecodes in a pic_timing SEI.
     */
    H264SEITimeCode timecode[3];

    /**
     * Number of timecode in use
     */
    int timecode_cnt;
} H264SEIPictureTiming;

typedef struct H264SEIRecoveryPoint {
    /**
     * recovery_frame_cnt
     *
     * Set to -1 if no recovery point SEI message found or to number of frames
     * before playback synchronizes. Frames having recovery point are key
     * frames.
     */
    int recovery_frame_cnt;
} H264SEIRecoveryPoint;

typedef struct H264SEIBufferingPeriod {
    int present;   ///< Buffering period SEI flag
    int initial_cpb_removal_delay[32];  ///< Initial timestamps for CPBs
} H264SEIBufferingPeriod;

typedef struct H264SEIGreenMetaData {
    uint8_t green_metadata_type;
    uint8_t period_type;
    uint16_t num_seconds;
    uint16_t num_pictures;
    uint8_t percent_non_zero_macroblocks;
    uint8_t percent_intra_coded_macroblocks;
    uint8_t percent_six_tap_filtering;
    uint8_t percent_alpha_point_deblocking_instance;
    uint8_t xsd_metric_type;
    uint16_t xsd_metric_value;
} H264SEIGreenMetaData;

typedef struct H264SEIContext {
    H2645SEI common;
    H264SEIPictureTiming picture_timing;
    H264SEIRecoveryPoint recovery_point;
    H264SEIBufferingPeriod buffering_period;
    H264SEIGreenMetaData green_metadata;
} H264SEIContext;

struct H264ParamSets;

int ff_h264_sei_decode(H264SEIContext *h, GetBitContext *gb,
                       const struct H264ParamSets *ps, void *logctx);

/**
 * Reset SEI values at the beginning of the frame.
 */
void ff_h264_sei_uninit(H264SEIContext *h);

/**
 * Get stereo_mode string from the h264 frame_packing_arrangement
 */
const char *ff_h264_sei_stereo_mode(const H2645SEIFramePacking *h);

/**
 * Parse the contents of a picture timing message given an active SPS.
 */
int ff_h264_sei_process_picture_timing(H264SEIPictureTiming *h, const SPS *sps,
                                       void *logctx);

#endif /* AVCODEC_H264_SEI_H */
