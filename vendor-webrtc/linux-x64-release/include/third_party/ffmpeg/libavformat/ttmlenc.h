/*
 * Generic TTML helpers
 * Copyright (c) 2021 24i
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

#ifndef AVFORMAT_TTMLENC_H
#define AVFORMAT_TTMLENC_H

#include "libavcodec/codec_par.h"
#include "libavcodec/ttmlenc.h"

static inline unsigned int ff_is_ttml_stream_paragraph_based(const AVCodecParameters *codecpar)
{
    // Not perfect, but decide whether the packet is a document or not
    // by the existence of the lavc ttmlenc extradata.
    return (codecpar->extradata &&
            codecpar->extradata_size >= TTMLENC_EXTRADATA_SIGNATURE_SIZE &&
            !memcmp(codecpar->extradata,
                    TTMLENC_EXTRADATA_SIGNATURE,
                    TTMLENC_EXTRADATA_SIGNATURE_SIZE));
}

#endif /* AVFORMAT_TTMLENC_H */
