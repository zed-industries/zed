/*
 * RTJpeg decoding functions
 * copyright (c) 2006 Reimar Doeffinger
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

#ifndef AVCODEC_RTJPEG_H
#define AVCODEC_RTJPEG_H

#include <stdint.h>

#include "libavutil/frame.h"
#include "libavutil/mem_internal.h"
#include "idctdsp.h"

#define RTJPEG_FILE_VERSION 0
#define RTJPEG_HEADER_SIZE 12

typedef struct RTJpegContext {
    int w, h;
    IDCTDSPContext idsp;
    uint8_t scan[64];
    uint32_t lquant[64];
    uint32_t cquant[64];
    DECLARE_ALIGNED(16, int16_t, block)[64];
} RTJpegContext;

struct AVCodecContext;
void ff_rtjpeg_init(RTJpegContext *c, struct AVCodecContext *avctx);

void ff_rtjpeg_decode_init(RTJpegContext *c, int width, int height,
                           const uint32_t *lquant, const uint32_t *cquant);

int ff_rtjpeg_decode_frame_yuv420(RTJpegContext *c, AVFrame *f,
                               const uint8_t *buf, int buf_size);
#endif /* AVCODEC_RTJPEG_H */
