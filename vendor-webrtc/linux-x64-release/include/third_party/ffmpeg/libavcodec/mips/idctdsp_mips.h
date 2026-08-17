/*
 * Copyright (c) 2015 Manojkumar Bhosale (Manojkumar.Bhosale@imgtec.com)
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

#ifndef AVCODEC_MIPS_IDCTDSP_MIPS_H
#define AVCODEC_MIPS_IDCTDSP_MIPS_H

#include "../mpegvideo.h"

void ff_put_pixels_clamped_msa(const int16_t *block,
                               uint8_t *restrict pixels,
                               ptrdiff_t line_size);
void ff_put_signed_pixels_clamped_msa(const int16_t *block,
                                      uint8_t *restrict pixels,
                                      ptrdiff_t line_size);
void ff_add_pixels_clamped_msa(const int16_t *block,
                               uint8_t *restrict pixels,
                               ptrdiff_t line_size);
void ff_j_rev_dct_msa(int16_t *data);
void ff_jref_idct_put_msa(uint8_t *dest, ptrdiff_t stride, int16_t *block);
void ff_jref_idct_add_msa(uint8_t *dest, ptrdiff_t stride, int16_t *block);
void ff_simple_idct_msa(int16_t *block);
void ff_simple_idct_put_msa(uint8_t *dest, ptrdiff_t stride_dst, int16_t *block);
void ff_simple_idct_add_msa(uint8_t *dest, ptrdiff_t stride_dst, int16_t *block);

void ff_put_pixels_clamped_mmi(const int16_t *block,
        uint8_t *restrict pixels, ptrdiff_t line_size);
void ff_put_signed_pixels_clamped_mmi(const int16_t *block,
        uint8_t *restrict pixels, ptrdiff_t line_size);
void ff_add_pixels_clamped_mmi(const int16_t *block,
        uint8_t *restrict pixels, ptrdiff_t line_size);
void ff_simple_idct_8_mmi(int16_t *block);
void ff_simple_idct_put_8_mmi(uint8_t *dest, ptrdiff_t line_size, int16_t *block);
void ff_simple_idct_add_8_mmi(uint8_t *dest, ptrdiff_t line_size, int16_t *block);

#endif  // #ifndef AVCODEC_MIPS_IDCTDSP_MIPS_H
