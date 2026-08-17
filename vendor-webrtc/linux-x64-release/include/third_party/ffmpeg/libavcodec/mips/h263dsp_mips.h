/*
 * Copyright (c) 2015 Manojkumar Bhosale (Manojkumar.Bhosale@imgtec.com)
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

#ifndef AVCODEC_MIPS_H263DSP_MIPS_H
#define AVCODEC_MIPS_H263DSP_MIPS_H

#include "libavcodec/mpegvideo.h"

void ff_h263_h_loop_filter_msa(uint8_t *src, int stride, int q_scale);
void ff_h263_v_loop_filter_msa(uint8_t *src, int stride, int q_scale);
void ff_dct_unquantize_mpeg2_inter_msa(MpegEncContext *s, int16_t *block,
                                       int32_t index, int32_t q_scale);
void ff_dct_unquantize_h263_inter_msa(MpegEncContext *s, int16_t *block,
                                      int32_t index, int32_t q_scale);
void ff_dct_unquantize_h263_intra_msa(MpegEncContext *s, int16_t *block,
                                      int32_t index, int32_t q_scale);
int ff_pix_sum_msa(const uint8_t *pix, ptrdiff_t line_size);

#endif  // #ifndef AVCODEC_MIPS_H263DSP_MIPS_H
