/*
 * FLAC (Free Lossless Audio Codec) common stuff
 * Copyright (c) 2008 Justin Ruggles
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
 * FLAC (Free Lossless Audio Codec) common stuff
 */

#ifndef AVCODEC_FLAC_H
#define AVCODEC_FLAC_H

#include "libavutil/intreadwrite.h"

#define FLAC_STREAMINFO_SIZE   34
#define FLAC_MAX_CHANNELS       8
#define FLAC_MIN_BLOCKSIZE     16
#define FLAC_MAX_BLOCKSIZE  65535
#define FLAC_MIN_FRAME_SIZE    10

enum {
    FLAC_CHMODE_INDEPENDENT = 0,
    FLAC_CHMODE_LEFT_SIDE   = 1,
    FLAC_CHMODE_RIGHT_SIDE  = 2,
    FLAC_CHMODE_MID_SIDE    = 3,
};

enum {
    FLAC_METADATA_TYPE_STREAMINFO = 0,
    FLAC_METADATA_TYPE_PADDING,
    FLAC_METADATA_TYPE_APPLICATION,
    FLAC_METADATA_TYPE_SEEKTABLE,
    FLAC_METADATA_TYPE_VORBIS_COMMENT,
    FLAC_METADATA_TYPE_CUESHEET,
    FLAC_METADATA_TYPE_PICTURE,
    FLAC_METADATA_TYPE_INVALID = 127
};

/**
 * Parse the metadata block parameters from the header.
 * @param[in]  block_header header data, at least 4 bytes
 * @param[out] last indicator for last metadata block
 * @param[out] type metadata block type
 * @param[out] size metadata block size
 */
static av_always_inline void flac_parse_block_header(const uint8_t *block_header,
                                                      int *last, int *type, int *size)
{
    int tmp = *block_header;
    if (last)
        *last = tmp & 0x80;
    if (type)
        *type = tmp & 0x7F;
    if (size)
        *size = AV_RB24(block_header + 1);
}

#endif /* AVCODEC_FLAC_H */
