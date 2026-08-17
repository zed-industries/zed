/*
 * Immersive Audio Model and Formats demuxing utils
 * Copyright (c) 2024 James Almer <jamrial@gmail.com>
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

#ifndef AVFORMAT_IAMF_READER_H
#define AVFORMAT_IAMF_READER_H

#include <stdint.h>

#include "libavcodec/packet.h"
#include "avformat.h"
#include "avio.h"
#include "iamf.h"

typedef struct IAMFDemuxContext {
    IAMFContext iamf;

    // Packet side data
    AVIAMFParamDefinition *mix;
    size_t mix_size;
    AVIAMFParamDefinition *demix;
    size_t demix_size;
    AVIAMFParamDefinition *recon;
    size_t recon_size;
} IAMFDemuxContext;

int ff_iamf_read_packet(AVFormatContext *s, IAMFDemuxContext *c,
                        AVIOContext *pb, int max_size, int stream_id_offset, AVPacket *pkt);

void ff_iamf_read_deinit(IAMFDemuxContext *c);

#endif /* AVFORMAT_IAMF_READER_H */
