/*
 * Copyright (c) 2016 Google Inc.
 * Copyright (c) 2016 KongQun Yang (kqyang@google.com)
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

/**
 * @file
 * internal header for VPx codec configuration utilities.
 */

#ifndef AVFORMAT_VPCC_H
#define AVFORMAT_VPCC_H

#include "libavutil/rational.h"
#include "libavcodec/codec_par.h"
#include "avio.h"
#include "avformat.h"

typedef struct VPCC {
    int profile;
    int level;
    int bitdepth;
    int chroma_subsampling;
    int full_range_flag;
} VPCC;

/**
 * Writes VP codec configuration to the provided AVIOContext.
 *
 * @param s address of the AVFormatContext for the logging context.
 * @param pb address of the AVIOContext where the vpcC shall be written.
 * @param data address of a data array which contains coded bitstream data from
 *             which codec information can be extracted. May be NULL.
 * @param len length of the data array.
 * @param par address of the AVCodecParameters which contains codec information.
 * @return >=0 in case of success, a negative value corresponding to an AVERROR
 *         code in case of failure
 */
int ff_isom_write_vpcc(AVFormatContext *s, AVIOContext *pb,
                       const uint8_t *data, int len,
                       AVCodecParameters *par);

int ff_isom_get_vpcc_features(AVFormatContext *s, AVCodecParameters *par,
                              const uint8_t *data, int len,
                              AVRational *frame_rate, VPCC *vpcc);

#endif /* AVFORMAT_VPCC_H */
