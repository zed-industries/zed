/*
 * Copyright (c) 2015 Parag Salasakar (parag.salasakar@imgtec.com)
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

#ifndef AVCODEC_MIPS_BLOCKDSP_MIPS_H
#define AVCODEC_MIPS_BLOCKDSP_MIPS_H

#include "../mpegvideo.h"

void ff_fill_block16_msa(uint8_t *src, uint8_t val, ptrdiff_t stride, int height);
void ff_fill_block8_msa(uint8_t *src, uint8_t val, ptrdiff_t stride, int height);
void ff_clear_block_msa(int16_t *block);
void ff_clear_blocks_msa(int16_t *block);

void ff_fill_block16_mmi(uint8_t *block, uint8_t value, ptrdiff_t line_size, int h);
void ff_fill_block8_mmi(uint8_t *block, uint8_t value, ptrdiff_t line_size, int h);
void ff_clear_block_mmi(int16_t *block);
void ff_clear_blocks_mmi(int16_t *block);

#endif  // #ifndef AVCODEC_MIPS_BLOCKDSP_MIPS_H
