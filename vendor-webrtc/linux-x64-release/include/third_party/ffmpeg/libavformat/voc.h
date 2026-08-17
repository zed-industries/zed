/*
 * Creative Voice File demuxer.
 * Copyright (c) 2006  Aurelien Jacobs <aurel@gnuage.org>
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

#ifndef AVFORMAT_VOC_H
#define AVFORMAT_VOC_H

#include "avformat.h"
#include "internal.h"

typedef struct voc_dec_context {
    int64_t remaining_size;
    int64_t pts;
} VocDecContext;

typedef enum voc_type {
    VOC_TYPE_EOF              = 0x00,
    VOC_TYPE_VOICE_DATA       = 0x01,
    VOC_TYPE_VOICE_DATA_CONT  = 0x02,
    VOC_TYPE_SILENCE          = 0x03,
    VOC_TYPE_MARKER           = 0x04,
    VOC_TYPE_ASCII            = 0x05,
    VOC_TYPE_REPETITION_START = 0x06,
    VOC_TYPE_REPETITION_END   = 0x07,
    VOC_TYPE_EXTENDED         = 0x08,
    VOC_TYPE_NEW_VOICE_DATA   = 0x09,
} VocType;

extern const unsigned char ff_voc_magic[21];
extern const AVCodecTag ff_voc_codec_tags[];
extern const AVCodecTag *const ff_voc_codec_tags_list[];

int ff_voc_get_packet(AVFormatContext *s, AVPacket *pkt,
                      AVStream *st, int max_size);

#endif /* AVFORMAT_VOC_H */
