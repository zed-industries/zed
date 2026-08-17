/*
 * AVC helper functions for muxers
 * Copyright (c) 2008 Aurelien Jacobs <aurel@gnuage.org>
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

#ifndef AVFORMAT_AVC_H
#define AVFORMAT_AVC_H

#include <stdint.h>
#include "libavutil/rational.h"
#include "avio.h"

int ff_isom_write_avcc(AVIOContext *pb, const uint8_t *data, int len);
int ff_avc_write_annexb_extradata(const uint8_t *in, uint8_t **buf, int *size);

typedef struct {
    uint8_t id;
    uint8_t profile_idc;
    uint8_t level_idc;
    uint8_t constraint_set_flags;
    uint8_t chroma_format_idc;
    uint8_t bit_depth_luma;
    uint8_t bit_depth_chroma;
    uint8_t frame_mbs_only_flag;
    AVRational sar;
} H264SPS;

int ff_avc_decode_sps(H264SPS *sps, const uint8_t *buf, int buf_size);

#endif /* AVFORMAT_AVC_H */
