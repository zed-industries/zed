/*
 * Opus decoder/parser common functions and structures
 * Copyright (c) 2012 Andrew D'Addesio
 * Copyright (c) 2013-2014 Mozilla Corporation
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

#ifndef AVCODEC_OPUS_PARSE_H
#define AVCODEC_OPUS_PARSE_H

#include <stdint.h>

#include "libavcodec/avcodec.h"
#include "opus.h"

typedef struct OpusPacket {
    int packet_size;                /**< packet size */
    int data_size;                  /**< size of the useful data -- packet size - padding */
    int code;                       /**< packet code: specifies the frame layout */
    int stereo;                     /**< whether this packet is mono or stereo */
    int vbr;                        /**< vbr flag */
    int config;                     /**< configuration: tells the audio mode,
                                     **                bandwidth, and frame duration */
    int frame_count;                /**< frame count */
    int frame_offset[OPUS_MAX_FRAMES]; /**< frame offsets */
    int frame_size[OPUS_MAX_FRAMES]; /**< frame sizes */
    int frame_duration;             /**< frame duration, in samples @ 48kHz */
    enum OpusMode mode;             /**< mode */
    enum OpusBandwidth bandwidth;   /**< bandwidth */
} OpusPacket;

// a mapping between an opus stream and an output channel
typedef struct ChannelMap {
    int stream_idx;
    int channel_idx;

    // when a single decoded channel is mapped to multiple output channels, we
    // write to the first output directly and copy from it to the others
    // this field is set to 1 for those copied output channels
    int copy;
    // this is the index of the output channel to copy from
    int copy_idx;

    // this channel is silent
    int silence;
} ChannelMap;

typedef struct OpusParseContext {
    int             nb_streams;
    int      nb_stereo_streams;

    int16_t gain_i;

    ChannelMap *channel_maps;
} OpusParseContext;

int ff_opus_parse_packet(OpusPacket *pkt, const uint8_t *buf, int buf_size,
                         int self_delimited);

int ff_opus_parse_extradata(AVCodecContext *avctx, OpusParseContext *s);

#endif /* AVCODEC_OPUS_PARSE_H */
