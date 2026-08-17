/*
 * Copyright (c) 2015 Shivraj Patil (Shivraj.Patil@imgtec.com)
 *                    Zhou Xiaoyong <zhouxiaoyong@loongson.cn>
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

#ifndef AVCODEC_MIPS_PIXBLOCKDSP_MIPS_H
#define AVCODEC_MIPS_PIXBLOCKDSP_MIPS_H

#include "../mpegvideo.h"

void ff_diff_pixels_msa(int16_t *restrict block, const uint8_t *src1,
                        const uint8_t *src2, ptrdiff_t stride);
void ff_get_pixels_16_msa(int16_t *restrict dst, const uint8_t *src,
                          ptrdiff_t stride);
void ff_get_pixels_8_msa(int16_t *restrict dst, const uint8_t *src,
                         ptrdiff_t stride);

void ff_get_pixels_8_mmi(int16_t *restrict block, const uint8_t *pixels,
                         ptrdiff_t stride);
void ff_diff_pixels_mmi(int16_t *restrict block, const uint8_t *src1,
                        const uint8_t *src2, ptrdiff_t stride);

#endif  // #ifndef AVCODEC_MIPS_PIXBLOCKDSP_MIPS_H
