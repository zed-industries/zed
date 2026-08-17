/*
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

#ifndef AVCODEC_MIPS_VC1DSP_MIPS_H
#define AVCODEC_MIPS_VC1DSP_MIPS_H

#include "libavcodec/vc1dsp.h"

void ff_put_vc1_mspel_mc00_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc01_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc02_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc03_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc10_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc11_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc12_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc13_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc20_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc21_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc22_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc23_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc30_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc31_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc32_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc33_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);

void ff_avg_vc1_mspel_mc00_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc01_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc02_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc03_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc10_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc11_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc12_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc13_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc20_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc21_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc22_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc23_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc30_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc31_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc32_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc33_mmi(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride, int rnd);


void ff_put_vc1_mspel_mc00_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc01_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc02_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc03_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc10_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc11_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc12_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc13_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc20_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc21_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc22_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc23_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc30_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc31_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc32_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_put_vc1_mspel_mc33_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);

void ff_avg_vc1_mspel_mc00_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc01_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc02_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc03_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc10_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc11_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc12_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc13_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc20_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc21_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc22_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc23_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc30_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc31_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc32_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);
void ff_avg_vc1_mspel_mc33_16_mmi(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride, int rnd);

void ff_vc1_inv_trans_8x8_mmi(int16_t block[64]);
void ff_vc1_inv_trans_8x4_mmi(uint8_t *dest, ptrdiff_t linesize, int16_t *block);
void ff_vc1_inv_trans_4x8_mmi(uint8_t *dest, ptrdiff_t linesize, int16_t *block);
void ff_vc1_inv_trans_4x4_mmi(uint8_t *dest, ptrdiff_t linesize, int16_t *block);

void ff_vc1_inv_trans_4x4_dc_mmi(uint8_t *dest, ptrdiff_t linesize, int16_t *block);
void ff_vc1_inv_trans_4x8_dc_mmi(uint8_t *dest, ptrdiff_t linesize, int16_t *block);
void ff_vc1_inv_trans_8x4_dc_mmi(uint8_t *dest, ptrdiff_t linesize, int16_t *block);
void ff_vc1_inv_trans_8x8_dc_mmi(uint8_t *dest, ptrdiff_t linesize, int16_t *block);

void ff_vc1_v_overlap_mmi(uint8_t *src, ptrdiff_t stride);
void ff_vc1_h_overlap_mmi(uint8_t *src, ptrdiff_t stride);
void ff_vc1_v_s_overlap_mmi(int16_t *top, int16_t *bottom);
void ff_vc1_h_s_overlap_mmi(int16_t *left, int16_t *right, ptrdiff_t left_stride, ptrdiff_t right_stride, int flags);

void ff_vc1_v_loop_filter4_mmi(uint8_t *src, ptrdiff_t stride, int pq);
void ff_vc1_h_loop_filter4_mmi(uint8_t *src, ptrdiff_t stride, int pq);
void ff_vc1_v_loop_filter8_mmi(uint8_t *src, ptrdiff_t stride, int pq);
void ff_vc1_h_loop_filter8_mmi(uint8_t *src, ptrdiff_t stride, int pq);
void ff_vc1_v_loop_filter16_mmi(uint8_t *src, ptrdiff_t stride, int pq);
void ff_vc1_h_loop_filter16_mmi(uint8_t *src, ptrdiff_t stride, int pq);

void ff_put_no_rnd_vc1_chroma_mc8_mmi(uint8_t *dst /* align 8 */,
                                      const uint8_t *src /* align 1 */,
                                      ptrdiff_t stride, int h, int x, int y);
void ff_put_no_rnd_vc1_chroma_mc4_mmi(uint8_t *dst /* align 8 */,
                                      const uint8_t *src /* align 1 */,
                                      ptrdiff_t stride, int h, int x, int y);
void ff_avg_no_rnd_vc1_chroma_mc8_mmi(uint8_t *dst /* align 8 */,
                                      const uint8_t *src /* align 1 */,
                                      ptrdiff_t stride, int h, int x, int y);
void ff_avg_no_rnd_vc1_chroma_mc4_mmi(uint8_t *dst /* align 8 */,
                                      const uint8_t *src /* align 1 */,
                                      ptrdiff_t stride, int h, int x, int y);

void ff_vc1_inv_trans_8x8_msa(int16_t block[64]);
void ff_vc1_inv_trans_8x4_msa(uint8_t *dest, ptrdiff_t linesize, int16_t *block);
void ff_vc1_inv_trans_4x8_msa(uint8_t *dest, ptrdiff_t linesize, int16_t *block);

#define FF_PUT_VC1_MSPEL_MC_MSA(hmode, vmode)                                 \
void ff_put_vc1_mspel_mc ## hmode ## vmode ## _msa(uint8_t *dst,              \
                                                  const uint8_t *src,         \
                                                  ptrdiff_t stride, int rnd); \
void ff_put_vc1_mspel_mc ## hmode ## vmode ## _16_msa(uint8_t *dst,           \
                                                  const uint8_t *src,         \
                                                  ptrdiff_t stride, int rnd);

FF_PUT_VC1_MSPEL_MC_MSA(1, 1);
FF_PUT_VC1_MSPEL_MC_MSA(1, 2);
FF_PUT_VC1_MSPEL_MC_MSA(1, 3);

FF_PUT_VC1_MSPEL_MC_MSA(2, 1);
FF_PUT_VC1_MSPEL_MC_MSA(2, 2);
FF_PUT_VC1_MSPEL_MC_MSA(2, 3);

FF_PUT_VC1_MSPEL_MC_MSA(3, 1);
FF_PUT_VC1_MSPEL_MC_MSA(3, 2);
FF_PUT_VC1_MSPEL_MC_MSA(3, 3);
#endif /* AVCODEC_MIPS_VC1DSP_MIPS_H */
