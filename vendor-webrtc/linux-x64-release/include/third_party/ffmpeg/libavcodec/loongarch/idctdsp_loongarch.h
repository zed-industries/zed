/*
 * Copyright (c) 2021 Loongson Technology Corporation Limited
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

#ifndef AVCODEC_LOONGARCH_IDCTDSP_LOONGARCH_H
#define AVCODEC_LOONGARCH_IDCTDSP_LOONGARCH_H

#include <stdint.h>
#include "libavcodec/mpegvideo.h"

void ff_simple_idct_lasx(int16_t *block);
void ff_simple_idct_put_lasx(uint8_t *dest, ptrdiff_t stride_dst, int16_t *block);
void ff_simple_idct_add_lasx(uint8_t *dest, ptrdiff_t stride_dst, int16_t *block);
void ff_put_pixels_clamped_lasx(const int16_t *block,
                                uint8_t *restrict pixels,
                                ptrdiff_t line_size);
void ff_put_signed_pixels_clamped_lasx(const int16_t *block,
                                       uint8_t *restrict pixels,
                                       ptrdiff_t line_size);
void ff_add_pixels_clamped_lasx(const int16_t *block,
                                uint8_t *restrict pixels,
                                ptrdiff_t line_size);

#endif /* AVCODEC_LOONGARCH_IDCTDSP_LOONGARCH_H */
