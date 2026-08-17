/*
 * Android MediaCodec software buffer copy functions
 *
 * Copyright (c) 2015-2016 Matthieu Bouron <matthieu.bouron stupeflix.com>
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

#ifndef AVCODEC_MEDIACODEC_SW_BUFFER_H
#define AVCODEC_MEDIACODEC_SW_BUFFER_H

#include <sys/types.h>

#include "libavutil/frame.h"

#include "avcodec.h"
#include "mediacodec_wrapper.h"
#include "mediacodecdec_common.h"

void ff_mediacodec_sw_buffer_copy_yuv420_planar(AVCodecContext *avctx,
                                                MediaCodecDecContext *s,
                                                uint8_t *data,
                                                size_t size,
                                                FFAMediaCodecBufferInfo *info,
                                                AVFrame *frame);

void ff_mediacodec_sw_buffer_copy_yuv420_semi_planar(AVCodecContext *avctx,
                                                     MediaCodecDecContext *s,
                                                     uint8_t *data,
                                                     size_t size,
                                                     FFAMediaCodecBufferInfo *info,
                                                     AVFrame *frame);

void ff_mediacodec_sw_buffer_copy_yuv420_packed_semi_planar(AVCodecContext *avctx,
                                                     MediaCodecDecContext *s,
                                                     uint8_t *data,
                                                     size_t size,
                                                     FFAMediaCodecBufferInfo *info,
                                                     AVFrame *frame);

void ff_mediacodec_sw_buffer_copy_yuv420_packed_semi_planar_64x32Tile2m8ka(AVCodecContext *avctx,
                                                     MediaCodecDecContext *s,
                                                     uint8_t *data,
                                                     size_t size,
                                                     FFAMediaCodecBufferInfo *info,
                                                     AVFrame *frame);

#endif /* AVCODEC_MEDIACODEC_SW_BUFFER_H */
