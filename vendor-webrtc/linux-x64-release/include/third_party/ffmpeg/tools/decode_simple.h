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

/* shared code for simple demux/decode tools */

#ifndef DECODE_SIMPLE_H
#define DECODE_SIMPLE_H

#include "libavformat/avformat.h"

#include "libavcodec/avcodec.h"
#include "libavcodec/packet.h"

#include "libavutil/dict.h"
#include "libavutil/frame.h"


typedef struct DecodeContext {
    AVFormatContext *demuxer;
    AVStream        *stream;
    AVCodecContext  *decoder;

    AVPacket        *pkt;
    AVFrame         *frame;

    int (*process_frame)(struct DecodeContext *dc, AVFrame *frame);
    void            *opaque;

    AVDictionary    *decoder_opts;
    int              max_frames;
} DecodeContext;

int ds_open(DecodeContext *dc, const char *url, int stream_idx);
void ds_free(DecodeContext *dc);

int ds_run(DecodeContext *dc);

#endif /* DECODE_SIMPLE_H */
