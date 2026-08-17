/*
 * Copyright (c) 2021 Loongson Technology Corporation Limited
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

#ifndef AVCODEC_LOONGARCH_HPELDSP_LASX_H
#define AVCODEC_LOONGARCH_HPELDSP_LASX_H

#include <stdint.h>
#include <stddef.h>
#include "libavutil/attributes.h"

void ff_put_pixels8_8_lasx(uint8_t *block, const uint8_t *pixels,
                           ptrdiff_t line_size, int h);
void ff_put_pixels8_x2_8_lasx(uint8_t *block, const uint8_t *pixels,
                              ptrdiff_t line_size, int32_t h);
void ff_put_pixels8_y2_8_lasx(uint8_t *block, const uint8_t *pixels,
                              ptrdiff_t line_size, int32_t h);
void ff_put_pixels16_8_lsx(uint8_t *block, const uint8_t *pixels,
                           ptrdiff_t line_size, int h);
void ff_put_pixels16_x2_8_lasx(uint8_t *block, const uint8_t *pixels,
                               ptrdiff_t line_size, int32_t h);
void ff_put_pixels16_y2_8_lasx(uint8_t *block, const uint8_t *pixels,
                               ptrdiff_t line_size, int32_t h);
void ff_put_no_rnd_pixels16_x2_8_lasx(uint8_t *block, const uint8_t *pixels,
                                      ptrdiff_t line_size, int h);
void ff_put_no_rnd_pixels16_y2_8_lasx(uint8_t *block, const uint8_t *pixels,
                                      ptrdiff_t line_size, int h);
void ff_put_no_rnd_pixels16_xy2_8_lasx(uint8_t *block,
                                       const uint8_t *pixels,
                                       ptrdiff_t line_size, int h);
void ff_put_no_rnd_pixels8_x2_8_lasx(uint8_t *block, const uint8_t *pixels,
                                     ptrdiff_t line_size, int h);
void ff_put_no_rnd_pixels8_y2_8_lasx(uint8_t *block, const uint8_t *pixels,
                                     ptrdiff_t line_size, int h);
void ff_put_no_rnd_pixels8_xy2_8_lasx(uint8_t *block, const uint8_t *pixels,
                                      ptrdiff_t line_size, int h);
void ff_put_pixels8_xy2_8_lasx(uint8_t *block, const uint8_t *pixels,
                               ptrdiff_t line_size, int h);
void ff_put_pixels16_xy2_8_lasx(uint8_t *block, const uint8_t *pixels,
                                ptrdiff_t line_size, int h);
#endif
