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

#ifndef AVCODEC_HW_BASE_ENCODE_H264_H
#define AVCODEC_HW_BASE_ENCODE_H264_H

#include "hw_base_encode.h"
#include "cbs_h264.h"

typedef struct FFHWBaseEncodeH264 {
    H264RawSPS raw_sps;
    H264RawPPS raw_pps;

    H264RawSEIBufferingPeriod sei_buffering_period;

    int dpb_frames;
} FFHWBaseEncodeH264;

typedef struct FFHWBaseEncodeH264Opts {
    int flags;
#define FF_HW_H264_SEI_TIMING (1 << 0)

    int mb_width;
    int mb_height;
    int64_t bit_rate;
    int cabac;
    int fixed_qp_idr;
    uint64_t hrd_buffer_size;
    uint64_t initial_buffer_fullness;
} FFHWBaseEncodeH264Opts;

int ff_hw_base_encode_init_params_h264(FFHWBaseEncodeContext *base_ctx,
                                       AVCodecContext *avctx,
                                       FFHWBaseEncodeH264 *common,
                                       FFHWBaseEncodeH264Opts *opts);

#endif /* AVCODEC_HW_BASE_ENCODE_H264_H */
