/*
 * Copyright (c) 2023 Loongson Technology Corporation Limited
 * Contributed by Shiyou Yin <yinshiyou-hf@loongson.cn>
 *                Xiwei  Gu  <guxiwei-hf@loongson.cn>
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

#ifndef AVCODEC_LOONGARCH_H264DSP_LOONGARCH_H
#define AVCODEC_LOONGARCH_H264DSP_LOONGARCH_H

#include "libavcodec/h264dec.h"
#include "config.h"

void ff_h264_idct_add_8_lsx(uint8_t *dst, int16_t *src, int dst_stride);
void ff_h264_idct8_add_8_lsx(uint8_t *dst, int16_t *src, int dst_stride);
void ff_h264_idct_dc_add_8_lsx(uint8_t *dst, int16_t *src, int dst_stride);
void ff_h264_idct8_dc_add_8_lsx(uint8_t *dst, int16_t *src, int dst_stride);
void ff_h264_luma_dc_dequant_idct_8_lsx(int16_t *_output, int16_t *_input, int qmul);
void ff_h264_idct_add16_8_lsx(uint8_t *dst, const int32_t *blk_offset,
                              int16_t *block, int32_t dst_stride,
                              const uint8_t nzc[15 * 8]);
void ff_h264_idct8_add4_8_lsx(uint8_t *dst, const int32_t *blk_offset,
                              int16_t *block, int32_t dst_stride,
                              const uint8_t nzc[15 * 8]);
void ff_h264_idct_add8_8_lsx(uint8_t **dst, const int32_t *blk_offset,
                             int16_t *block, int32_t dst_stride,
                             const uint8_t nzc[15 * 8]);
void ff_h264_idct_add8_422_8_lsx(uint8_t **dst, const int32_t *blk_offset,
                                 int16_t *block, int32_t dst_stride,
                                 const uint8_t nzc[15 * 8]);
void ff_h264_idct_add16_intra_8_lsx(uint8_t *dst, const int32_t *blk_offset,
                                    int16_t *block, int32_t dst_stride,
                                    const uint8_t nzc[15 * 8]);

void ff_h264_h_lpf_luma_8_lsx(uint8_t *src, ptrdiff_t stride,
                              int alpha, int beta, int8_t *tc0);
void ff_h264_v_lpf_luma_8_lsx(uint8_t *src, ptrdiff_t stride,
                              int alpha, int beta, int8_t *tc0);
void ff_h264_h_lpf_luma_intra_8_lsx(uint8_t *src, ptrdiff_t stride,
                                    int alpha, int beta);
void ff_h264_v_lpf_luma_intra_8_lsx(uint8_t *src, ptrdiff_t stride,
                                    int alpha, int beta);
void ff_h264_h_lpf_chroma_8_lsx(uint8_t *src, ptrdiff_t stride,
                                int alpha, int beta, int8_t *tc0);
void ff_h264_v_lpf_chroma_8_lsx(uint8_t *src, ptrdiff_t stride,
                                int alpha, int beta, int8_t *tc0);
void ff_h264_h_lpf_chroma_intra_8_lsx(uint8_t *src, ptrdiff_t stride,
                                      int alpha, int beta);
void ff_h264_v_lpf_chroma_intra_8_lsx(uint8_t *src, ptrdiff_t stride,
                                      int alpha, int beta);
void ff_biweight_h264_pixels16_8_lsx(uint8_t *dst, uint8_t *src,
                                     ptrdiff_t stride, int height,
                                     int log2_denom, int weight_dst,
                                     int weight_src, int offset_in);
void ff_biweight_h264_pixels8_8_lsx(uint8_t *dst, uint8_t *src,
                                    ptrdiff_t stride, int height,
                                    int log2_denom, int weight_dst,
                                    int weight_src, int offset);
void ff_biweight_h264_pixels4_8_lsx(uint8_t *dst, uint8_t *src,
                                    ptrdiff_t stride, int height,
                                    int log2_denom, int weight_dst,
                                    int weight_src, int offset);
void ff_weight_h264_pixels16_8_lsx(uint8_t *src, ptrdiff_t stride,
                                   int height, int log2_denom,
                                   int weight_src, int offset_in);
void ff_weight_h264_pixels8_8_lsx(uint8_t *src, ptrdiff_t stride,
                                  int height, int log2_denom,
                                  int weight_src, int offset);
void ff_weight_h264_pixels4_8_lsx(uint8_t *src, ptrdiff_t stride,
                                  int height, int log2_denom,
                                  int weight_src, int offset);
void ff_h264_add_pixels4_8_lsx(uint8_t *_dst, int16_t *_src, int stride);
void ff_h264_add_pixels8_8_lsx(uint8_t *_dst, int16_t *_src, int stride);
void ff_h264_loop_filter_strength_lsx(int16_t bS[2][4][4], uint8_t nnz[40],
                                      int8_t ref[2][40], int16_t mv[2][40][2],
                                      int bidir, int edges, int step,
                                      int mask_mv0, int mask_mv1, int field);

#if HAVE_LASX
void ff_h264_h_lpf_luma_8_lasx(uint8_t *src, ptrdiff_t stride,
                               int alpha, int beta, int8_t *tc0);
void ff_h264_v_lpf_luma_8_lasx(uint8_t *src, ptrdiff_t stride,
                               int alpha, int beta, int8_t *tc0);
void ff_h264_h_lpf_luma_intra_8_lasx(uint8_t *src, ptrdiff_t stride,
                                     int alpha, int beta);
void ff_h264_v_lpf_luma_intra_8_lasx(uint8_t *src, ptrdiff_t stride,
                                     int alpha, int beta);
void ff_biweight_h264_pixels16_8_lasx(unsigned char *dst, unsigned char *src,
                                      long int stride, int height,
                                      int log2_denom, int weight_dst,
                                      int weight_src, int offset_in);
void ff_biweight_h264_pixels8_8_lasx(unsigned char *dst, unsigned char *src,
                                     long int stride, int height,
                                     int log2_denom, int weight_dst,
                                     int weight_src, int offset);
void ff_weight_h264_pixels16_8_lasx(uint8_t *src, ptrdiff_t stride,
                                    int height, int log2_denom,
                                    int weight_src, int offset_in);
void ff_weight_h264_pixels8_8_lasx(uint8_t *src, ptrdiff_t stride,
                                   int height, int log2_denom,
                                   int weight_src, int offset);
void ff_h264_add_pixels4_8_lasx(uint8_t *_dst, int16_t *_src, int stride);

void ff_h264_add_pixels8_8_lasx(uint8_t *_dst, int16_t *_src, int stride);
void ff_h264_idct8_add_8_lasx(uint8_t *dst, int16_t *src, int32_t dst_stride);
void ff_h264_idct8_dc_add_8_lasx(uint8_t *dst, int16_t *src,
                                  int32_t dst_stride);
void ff_h264_idct8_add4_8_lasx(uint8_t *dst, const int32_t *blk_offset,
                               int16_t *block, int32_t dst_stride,
                               const uint8_t nzc[15 * 8]);
void ff_h264_loop_filter_strength_lasx(int16_t bS[2][4][4], uint8_t nnz[40],
                                       int8_t ref[2][40], int16_t mv[2][40][2],
                                       int bidir, int edges, int step,
                                       int mask_mv0, int mask_mv1, int field);
#endif // #if HAVE_LASX

#endif  // #ifndef AVCODEC_LOONGARCH_H264DSP_LOONGARCH_H
