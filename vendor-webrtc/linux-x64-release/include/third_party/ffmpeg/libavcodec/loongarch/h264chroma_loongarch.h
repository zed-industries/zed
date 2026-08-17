/*
 * Copyright (c) 2023 Loongson Technology Corporation Limited
 * Contributed by Shiyou Yin <yinshiyou-hf@loongson.cn>
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

#ifndef AVCODEC_LOONGARCH_H264CHROMA_LOONGARCH_H
#define AVCODEC_LOONGARCH_H264CHROMA_LOONGARCH_H

#include "libavcodec/h264.h"

void ff_put_h264_chroma_mc8_lsx(unsigned char *dst, const unsigned char *src,
                                long int stride, int h, int x, int y);
void ff_avg_h264_chroma_mc8_lsx(unsigned char *dst, const unsigned char *src,
                                long int stride, int h, int x, int y);
void ff_put_h264_chroma_mc4_lsx(unsigned char *dst, const unsigned char *src,
                                long int stride, int h, int x, int y);

void ff_put_h264_chroma_mc4_lasx(unsigned char *dst, const unsigned char *src,
                                 long int stride, int h, int x, int y);
void ff_put_h264_chroma_mc8_lasx(unsigned char *dst, const unsigned char *src,
                                 long int stride, int h, int x, int y);
void ff_avg_h264_chroma_mc8_lasx(unsigned char *dst, const unsigned char *src,
                                 long int stride, int h, int x, int y);

#endif /* AVCODEC_LOONGARCH_H264CHROMA_LOONGARCH_H */
