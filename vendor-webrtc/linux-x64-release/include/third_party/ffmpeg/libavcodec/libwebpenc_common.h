/*
 * WebP encoding support via libwebp
 * Copyright (c) 2013 Justin Ruggles <justin.ruggles@gmail.com>
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
 * WebP encoder using libwebp: common structs and methods.
 */

#ifndef AVCODEC_LIBWEBPENC_COMMON_H
#define AVCODEC_LIBWEBPENC_COMMON_H

#include <webp/encode.h>

#include "libavutil/attributes.h"
#include "libavutil/frame.h"
#include "libavutil/log.h"
#include "libavutil/pixfmt.h"
#include "avcodec.h"
#include "codec_internal.h"

typedef struct LibWebPContextCommon {
    AVClass *class;         // class for AVOptions
    float quality;          // lossy quality 0 - 100
    int lossless;           // use lossless encoding
    int preset;             // configuration preset
    int chroma_warning;     // chroma linesize mismatch warning has been printed
    int conversion_warning; // pixel format conversion warning has been printed
    WebPConfig config;      // libwebp configuration
    AVFrame *ref;
    int cr_size;
    int cr_threshold;
} LibWebPContextCommon;

int ff_libwebp_error_to_averror(int err);

av_cold int ff_libwebp_encode_init_common(AVCodecContext *avctx);

int ff_libwebp_get_frame(AVCodecContext *avctx, LibWebPContextCommon *s,
                         const AVFrame *frame, AVFrame **alt_frame_ptr,
                         WebPPicture **pic_ptr);

extern const enum AVPixelFormat ff_libwebpenc_pix_fmts[];
extern const AVClass ff_libwebpenc_class;
extern const FFCodecDefault ff_libwebp_defaults[];

#endif /* AVCODEC_LIBWEBPENC_COMMON_H */
