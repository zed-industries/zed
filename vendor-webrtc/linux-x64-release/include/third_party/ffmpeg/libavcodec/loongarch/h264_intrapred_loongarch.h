/*
 * Copyright (c) 2023 Loongson Technology Corporation Limited
 * Contributed by Hao Chen <chenhao@loongson.cn>
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

#ifndef AVCODEC_LOONGARCH_H264_INTRAPRED_LOONGARCH_H
#define AVCODEC_LOONGARCH_H264_INTRAPRED_LOONGARCH_H

#include "libavcodec/avcodec.h"

void ff_h264_pred16x16_plane_h264_8_lsx(uint8_t *src, ptrdiff_t stride);
void ff_h264_pred16x16_plane_rv40_8_lsx(uint8_t *src, ptrdiff_t stride);
void ff_h264_pred16x16_plane_svq3_8_lsx(uint8_t *src, ptrdiff_t stride);

void ff_h264_pred16x16_plane_h264_8_lasx(uint8_t *src, ptrdiff_t stride);
void ff_h264_pred16x16_plane_rv40_8_lasx(uint8_t *src, ptrdiff_t stride);
void ff_h264_pred16x16_plane_svq3_8_lasx(uint8_t *src, ptrdiff_t stride);

#endif  // #ifndef AVCODEC_LOONGARCH_H264_INTRAPRED_LOONGARCH_H
