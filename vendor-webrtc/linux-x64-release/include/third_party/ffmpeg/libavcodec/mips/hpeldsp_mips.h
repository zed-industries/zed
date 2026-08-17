/*
 * Copyright (c) 2015 Parag Salasakar (Parag.Salasakar@imgtec.com)
 * Copyright (c) 2016 Zhou Xiaoyong <zhouxiaoyong@loongson.cn>
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

#ifndef AVCODEC_MIPS_HPELDSP_MIPS_H
#define AVCODEC_MIPS_HPELDSP_MIPS_H

#include "libavcodec/bit_depth_template.c"

void ff_put_pixels16_msa(uint8_t *block, const uint8_t *pixels,
                         ptrdiff_t line_size, int32_t h);
void ff_put_pixels16_x2_msa(uint8_t *block, const uint8_t *pixels,
                            ptrdiff_t line_size, int32_t h);
void ff_put_pixels16_y2_msa(uint8_t *block, const uint8_t *pixels,
                            ptrdiff_t line_size, int32_t h);
void ff_put_pixels16_xy2_msa(uint8_t *block, const uint8_t *pixels,
                             ptrdiff_t line_size, int32_t h);
void ff_put_pixels8_msa(uint8_t *block, const uint8_t *pixels,
                        ptrdiff_t line_size, int32_t h);
void ff_put_pixels8_x2_msa(uint8_t *block, const uint8_t *pixels,
                           ptrdiff_t line_size, int32_t h);
void ff_put_pixels8_y2_msa(uint8_t *block, const uint8_t *pixels,
                           ptrdiff_t line_size, int32_t h);
void ff_put_pixels8_xy2_msa(uint8_t *block, const uint8_t *pixels,
                            ptrdiff_t line_size, int32_t h);
void ff_put_pixels4_msa(uint8_t *block, const uint8_t *pixels,
                        ptrdiff_t line_size, int32_t h);
void ff_put_pixels4_x2_msa(uint8_t *block, const uint8_t *pixels,
                           ptrdiff_t line_size, int32_t h);
void ff_put_pixels4_y2_msa(uint8_t *block, const uint8_t *pixels,
                           ptrdiff_t line_size, int32_t h);
void ff_put_pixels4_xy2_msa(uint8_t *block, const uint8_t *pixels,
                            ptrdiff_t line_size, int32_t h);
void ff_put_no_rnd_pixels16_x2_msa(uint8_t *block, const uint8_t *pixels,
                                   ptrdiff_t line_size, int32_t h);
void ff_put_no_rnd_pixels16_y2_msa(uint8_t *block, const uint8_t *pixels,
                                   ptrdiff_t line_size, int32_t h);
void ff_put_no_rnd_pixels16_xy2_msa(uint8_t *block, const uint8_t *pixels,
                                    ptrdiff_t line_size, int32_t h);
void ff_put_no_rnd_pixels8_x2_msa(uint8_t *block, const uint8_t *pixels,
                                  ptrdiff_t line_size, int32_t h);
void ff_put_no_rnd_pixels8_y2_msa(uint8_t *block, const uint8_t *pixels,
                                  ptrdiff_t line_size, int32_t h);
void ff_put_no_rnd_pixels8_xy2_msa(uint8_t *block, const uint8_t *pixels,
                                   ptrdiff_t line_size, int32_t h);
void ff_avg_pixels16_msa(uint8_t *block, const uint8_t *pixels,
                         ptrdiff_t line_size, int32_t h);
void ff_avg_pixels16_x2_msa(uint8_t *block, const uint8_t *pixels,
                            ptrdiff_t line_size, int32_t h);
void ff_avg_pixels16_y2_msa(uint8_t *block, const uint8_t *pixels,
                            ptrdiff_t line_size, int32_t h);
void ff_avg_pixels16_xy2_msa(uint8_t *block, const uint8_t *pixels,
                             ptrdiff_t line_size, int32_t h);
void ff_avg_pixels8_msa(uint8_t *block, const uint8_t *pixels,
                        ptrdiff_t line_size, int32_t h);
void ff_avg_pixels8_x2_msa(uint8_t *block, const uint8_t *pixels,
                           ptrdiff_t line_size, int32_t h);
void ff_avg_pixels8_y2_msa(uint8_t *block, const uint8_t *pixels,
                           ptrdiff_t line_size, int32_t h);
void ff_avg_pixels8_xy2_msa(uint8_t *block, const uint8_t *pixels,
                            ptrdiff_t line_size, int32_t h);
void ff_avg_pixels4_msa(uint8_t *block, const uint8_t *pixels,
                        ptrdiff_t line_size, int32_t h);
void ff_avg_pixels4_x2_msa(uint8_t *block, const uint8_t *pixels,
                           ptrdiff_t line_size, int32_t h);
void ff_avg_pixels4_y2_msa(uint8_t *block, const uint8_t *pixels,
                           ptrdiff_t line_size, int32_t h);
void ff_avg_pixels4_xy2_msa(uint8_t *block, const uint8_t *pixels,
                            ptrdiff_t line_size, int32_t h);

void ff_put_pixels16_l2_8_mmi(uint8_t *dst, const uint8_t *src1,
    const uint8_t *src2, int dst_stride, int src_stride1, int src_stride2,
    int h);
void ff_put_pixels8_l2_8_mmi(uint8_t *dst, const uint8_t *src1,
    const uint8_t *src2, int dst_stride, int src_stride1, int src_stride2,
    int h);
void ff_put_pixels4_l2_8_mmi(uint8_t *dst, const uint8_t *src1,
    const uint8_t *src2, int dst_stride, int src_stride1, int src_stride2,
    int h);
void ff_avg_pixels16_l2_8_mmi(uint8_t *dst, const uint8_t *src1,
    const uint8_t *src2, int dst_stride, int src_stride1, int src_stride2,
    int h);
void ff_avg_pixels8_l2_8_mmi(uint8_t *dst, const uint8_t *src1,
    const uint8_t *src2, int dst_stride, int src_stride1, int src_stride2,
    int h);
void ff_avg_pixels4_l2_8_mmi(uint8_t *dst, const uint8_t *src1,
    const uint8_t *src2, int dst_stride, int src_stride1, int src_stride2,
    int h);
void ff_put_no_rnd_pixels16_l2_8_mmi(uint8_t *dst, const uint8_t *src1,
    const uint8_t *src2, int dst_stride, int src_stride1, int src_stride2,
    int h);
void ff_put_no_rnd_pixels8_l2_8_mmi(uint8_t *dst, const uint8_t *src1,
    const uint8_t *src2, int dst_stride, int src_stride1, int src_stride2,
    int h);

void ff_put_pixels16_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_put_pixels16_x2_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_put_pixels16_y2_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_put_pixels16_xy2_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_put_pixels8_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_put_pixels8_x2_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_put_pixels8_y2_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_put_pixels8_xy2_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_put_pixels4_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_put_pixels4_x2_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_put_pixels4_y2_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_put_pixels4_xy2_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_put_no_rnd_pixels16_x2_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_put_no_rnd_pixels16_y2_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_put_no_rnd_pixels16_xy2_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_put_no_rnd_pixels8_x2_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_put_no_rnd_pixels8_y2_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_put_no_rnd_pixels8_xy2_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_avg_pixels16_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_avg_pixels16_x2_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_avg_pixels16_y2_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_avg_pixels16_xy2_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_avg_pixels8_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_avg_pixels8_x2_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_avg_pixels8_y2_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_avg_pixels8_xy2_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_avg_pixels4_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_avg_pixels4_x2_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_avg_pixels4_y2_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);
void ff_avg_pixels4_xy2_8_mmi(uint8_t *block, const uint8_t *pixels,
    ptrdiff_t line_size, int32_t h);

#endif  // #ifndef AVCODEC_MIPS_HPELDSP_MIPS_H
