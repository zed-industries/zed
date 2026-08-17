/*
 * Copyright (c) 2015 Zhou Xiaoyong <zhouxiaoyong@loongson.cn>
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

#ifndef AVCODEC_MIPS_H264PRED_MIPS_H
#define AVCODEC_MIPS_H264PRED_MIPS_H

#include "constants.h"
#include "libavcodec/h264pred.h"

void ff_pred16x16_vertical_8_mmi(uint8_t *src, ptrdiff_t stride);
void ff_pred16x16_horizontal_8_mmi(uint8_t *src, ptrdiff_t stride);
void ff_pred16x16_dc_8_mmi(uint8_t *src, ptrdiff_t stride);
void ff_pred8x8l_top_dc_8_mmi(uint8_t *src, int has_topleft, int has_topright,
        ptrdiff_t stride);
void ff_pred8x8l_dc_8_mmi(uint8_t *src, int has_topleft, int has_topright,
        ptrdiff_t stride);
void ff_pred8x8l_vertical_8_mmi(uint8_t *src, int has_topleft,
        int has_topright, ptrdiff_t stride);
void ff_pred4x4_dc_8_mmi(uint8_t *src, const uint8_t *topright,
        ptrdiff_t stride);
void ff_pred8x8_vertical_8_mmi(uint8_t *src, ptrdiff_t stride);
void ff_pred8x8_horizontal_8_mmi(uint8_t *src, ptrdiff_t stride);
void ff_pred16x16_plane_svq3_8_mmi(uint8_t *src, ptrdiff_t stride);
void ff_pred16x16_plane_rv40_8_mmi(uint8_t *src, ptrdiff_t stride);
void ff_pred16x16_plane_h264_8_mmi(uint8_t *src, ptrdiff_t stride);
void ff_pred8x8_top_dc_8_mmi(uint8_t *src, ptrdiff_t stride);
void ff_pred8x8_dc_8_mmi(uint8_t *src, ptrdiff_t stride);
void ff_pred8x16_vertical_8_mmi(uint8_t *src, ptrdiff_t stride);
void ff_pred8x16_horizontal_8_mmi(uint8_t *src, ptrdiff_t stride);

#endif  /* AVCODEC_MIPS_H264PRED_MIPS_H */
