/*
 * Copyright (c) 2015 Parag Salasakar (Parag.Salasakar@imgtec.com)
                      Zhou Xiaoyong <zhouxiaoyong@loongson.cn>
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

#ifndef AVCODEC_MIPS_H264DSP_MIPS_H
#define AVCODEC_MIPS_H264DSP_MIPS_H

#include "libavcodec/h264dec.h"
#include "constants.h"

void ff_h264_h_lpf_luma_inter_msa(uint8_t *src, ptrdiff_t stride,
                                  int alpha, int beta, int8_t *tc0);
void ff_h264_v_lpf_luma_inter_msa(uint8_t *src, ptrdiff_t stride,
                                  int alpha, int beta, int8_t *tc0);
void ff_h264_h_lpf_chroma_inter_msa(uint8_t *src, ptrdiff_t stride,
                                    int alpha, int beta, int8_t *tc0);
void ff_h264_v_lpf_chroma_inter_msa(uint8_t *src, ptrdiff_t stride,
                                    int alpha, int beta, int8_t *tc0);
void ff_h264_h_loop_filter_chroma422_msa(uint8_t *src, ptrdiff_t stride,
                                         int32_t alpha, int32_t beta,
                                         int8_t *tc0);
void ff_h264_h_loop_filter_chroma422_mbaff_msa(uint8_t *src, ptrdiff_t stride,
                                               int32_t alpha, int32_t beta,
                                               int8_t *tc0);
void ff_h264_h_loop_filter_luma_mbaff_msa(uint8_t *src, ptrdiff_t stride,
                                          int32_t alpha, int32_t beta,
                                          int8_t *tc0);

void ff_h264_idct_add_msa(uint8_t *dst, int16_t *src, int32_t dst_stride);
void ff_h264_idct4x4_addblk_dc_msa(uint8_t *dst, int16_t *src,
                                   int32_t dst_stride);
void ff_h264_deq_idct_luma_dc_msa(int16_t *dst, int16_t *src,
                                  int32_t de_q_val);
void ff_h264_idct_add16_msa(uint8_t *dst, const int32_t *blk_offset,
                            int16_t *block, int32_t stride,
                            const uint8_t nnzc[5 * 8]);
void ff_h264_idct_add16_intra_msa(uint8_t *dst, const int32_t *blk_offset,
                                  int16_t *block, int32_t dst_stride,
                                  const uint8_t nnzc[5 * 8]);
void ff_h264_idct_add8_msa(uint8_t **dst, const int32_t *blk_offset,
                           int16_t *block, int32_t dst_stride,
                           const uint8_t nnzc[15 * 8]);
void ff_h264_idct_add8_422_msa(uint8_t **dst, const int32_t *blk_offset,
                               int16_t *block, int32_t dst_stride,
                               const uint8_t nnzc[15 * 8]);
void ff_h264_idct8_addblk_msa(uint8_t *dst, int16_t *src, int32_t dst_stride);
void ff_h264_idct8_dc_addblk_msa(uint8_t *dst, int16_t *src,
                                 int32_t dst_stride);
void ff_h264_idct8_add4_msa(uint8_t *dst, const int *blk_offset,
                            int16_t *blk, int dst_stride,
                            const uint8_t nnzc[5 * 8]);

void ff_h264_h_lpf_luma_intra_msa(uint8_t *src, ptrdiff_t stride,
                                  int alpha, int beta);
void ff_h264_v_lpf_luma_intra_msa(uint8_t *src, ptrdiff_t stride,
                                  int alpha, int beta);
void ff_h264_h_lpf_chroma_intra_msa(uint8_t *src, ptrdiff_t stride,
                                    int alpha, int beta);
void ff_h264_v_lpf_chroma_intra_msa(uint8_t *src, ptrdiff_t stride,
                                    int alpha, int beta);
void ff_h264_h_loop_filter_luma_mbaff_intra_msa(uint8_t *src, ptrdiff_t stride,
                                                int alpha, int beta);

void ff_biweight_h264_pixels16_8_msa(uint8_t *dst, uint8_t *src,
                                     ptrdiff_t stride, int height, int log2_denom,
                                     int weightd, int weights, int offset);
void ff_biweight_h264_pixels8_8_msa(uint8_t *dst, uint8_t *src,
                                    ptrdiff_t stride, int height, int log2_denom,
                                    int weightd, int weights, int offset);
void ff_biweight_h264_pixels4_8_msa(uint8_t *dst, uint8_t *src,
                                    ptrdiff_t stride, int height, int log2_denom,
                                    int weightd, int weights, int offset);
void ff_weight_h264_pixels16_8_msa(uint8_t *src, ptrdiff_t stride, int height,
                                   int log2_denom, int weight, int offset);
void ff_weight_h264_pixels8_8_msa(uint8_t *src, ptrdiff_t stride, int height,
                                  int log2_denom, int weight, int offset);
void ff_weight_h264_pixels4_8_msa(uint8_t *src, ptrdiff_t stride, int height,
                                  int log2_denom, int weight, int offset);

void ff_put_h264_qpel16_mc00_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc10_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc20_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc30_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc01_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc11_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc21_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc31_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc02_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc12_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc22_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc32_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc03_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc13_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc23_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc33_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);

void ff_put_h264_qpel8_mc00_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc10_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc20_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc30_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc01_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc11_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc21_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc31_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc02_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc12_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc22_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc32_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc03_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc13_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc23_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc33_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);

void ff_put_h264_qpel4_mc00_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc10_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc20_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc30_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc01_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc11_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc21_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc31_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc02_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc12_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc22_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc32_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc03_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc13_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc23_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc33_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);

void ff_avg_h264_qpel16_mc00_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc10_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc20_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc30_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc01_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc11_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc21_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc31_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc02_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc12_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc22_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc32_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc03_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc13_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc23_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc33_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t dst_stride);

void ff_avg_h264_qpel8_mc00_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc10_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc20_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc30_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc01_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc11_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc21_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc31_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc02_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc12_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc22_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc32_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc03_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc13_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc23_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc33_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);

void ff_avg_h264_qpel4_mc00_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc10_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc20_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc30_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc01_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc11_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc21_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc31_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc02_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc12_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc22_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc32_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc03_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc13_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc23_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc33_msa(uint8_t *dst, const uint8_t *src,
                                ptrdiff_t dst_stride);

void ff_h264_intra_predict_plane_8x8_msa(uint8_t *src, ptrdiff_t stride);
void ff_h264_intra_predict_dc_4blk_8x8_msa(uint8_t *src, ptrdiff_t stride);
void ff_h264_intra_predict_hor_dc_8x8_msa(uint8_t *src, ptrdiff_t stride);
void ff_h264_intra_predict_vert_dc_8x8_msa(uint8_t *src, ptrdiff_t stride);
void ff_h264_intra_predict_mad_cow_dc_l0t_8x8_msa(uint8_t *src,
                                                  ptrdiff_t stride);
void ff_h264_intra_predict_mad_cow_dc_0lt_8x8_msa(uint8_t *src,
                                                  ptrdiff_t stride);
void ff_h264_intra_predict_mad_cow_dc_l00_8x8_msa(uint8_t *src,
                                                  ptrdiff_t stride);
void ff_h264_intra_predict_mad_cow_dc_0l0_8x8_msa(uint8_t *src,
                                                  ptrdiff_t stride);
void ff_h264_intra_predict_plane_16x16_msa(uint8_t *src, ptrdiff_t stride);
void ff_h264_intra_pred_vert_8x8_msa(uint8_t *src, ptrdiff_t stride);
void ff_h264_intra_pred_horiz_8x8_msa(uint8_t *src, ptrdiff_t stride);
void ff_h264_intra_pred_dc_16x16_msa(uint8_t *src, ptrdiff_t stride);
void ff_h264_intra_pred_vert_16x16_msa(uint8_t *src, ptrdiff_t stride);
void ff_h264_intra_pred_horiz_16x16_msa(uint8_t *src, ptrdiff_t stride);
void ff_h264_intra_pred_dc_left_16x16_msa(uint8_t *src, ptrdiff_t stride);
void ff_h264_intra_pred_dc_top_16x16_msa(uint8_t *src, ptrdiff_t stride);
void ff_h264_intra_pred_dc_128_8x8_msa(uint8_t *src, ptrdiff_t stride);
void ff_h264_intra_pred_dc_128_16x16_msa(uint8_t *src, ptrdiff_t stride);
void ff_vp8_pred8x8_127_dc_8_msa(uint8_t *src, ptrdiff_t stride);
void ff_vp8_pred8x8_129_dc_8_msa(uint8_t *src, ptrdiff_t stride);
void ff_vp8_pred16x16_127_dc_8_msa(uint8_t *src, ptrdiff_t stride);
void ff_vp8_pred16x16_129_dc_8_msa(uint8_t *src, ptrdiff_t stride);

void ff_h264_loop_filter_strength_msa(int16_t bS[2][4][4], uint8_t nnz[40],
        int8_t ref[2][40], int16_t mv[2][40][2], int bidir, int edges,
        int step, int mask_mv0, int mask_mv1, int field);

void ff_h264_add_pixels4_8_mmi(uint8_t *_dst, int16_t *_src, int stride);
void ff_h264_idct_add_8_mmi(uint8_t *dst, int16_t *block, int stride);
void ff_h264_idct8_add_8_mmi(uint8_t *dst, int16_t *block, int stride);
void ff_h264_idct_dc_add_8_mmi(uint8_t *dst, int16_t *block, int stride);
void ff_h264_idct8_dc_add_8_mmi(uint8_t *dst, int16_t *block, int stride);
void ff_h264_idct_add16_8_mmi(uint8_t *dst, const int *block_offset,
        int16_t *block, int stride, const uint8_t nnzc[5 * 8]);
void ff_h264_idct_add16intra_8_mmi(uint8_t *dst, const int *block_offset,
        int16_t *block, int stride, const uint8_t nnzc[5 * 8]);
void ff_h264_idct8_add4_8_mmi(uint8_t *dst, const int *block_offset,
        int16_t *block, int stride, const uint8_t nnzc[5 * 8]);
void ff_h264_idct_add8_8_mmi(uint8_t **dest, const int *block_offset,
        int16_t *block, int stride, const uint8_t nnzc[15*8]);
void ff_h264_idct_add8_422_8_mmi(uint8_t **dest, const int *block_offset,
        int16_t *block, int stride, const uint8_t nnzc[15*8]);
void ff_h264_luma_dc_dequant_idct_8_mmi(int16_t *output, int16_t *input,
        int qmul);

void ff_h264_weight_pixels16_8_mmi(uint8_t *block, ptrdiff_t stride, int height,
        int log2_denom, int weight, int offset);
void ff_h264_biweight_pixels16_8_mmi(uint8_t *dst, uint8_t *src,
        ptrdiff_t stride, int height, int log2_denom, int weightd, int weights,
        int offset);
void ff_h264_weight_pixels8_8_mmi(uint8_t *block, ptrdiff_t stride, int height,
        int log2_denom, int weight, int offset);
void ff_h264_biweight_pixels8_8_mmi(uint8_t *dst, uint8_t *src,
        ptrdiff_t stride, int height, int log2_denom, int weightd, int weights,
        int offset);
void ff_h264_weight_pixels4_8_mmi(uint8_t *block, ptrdiff_t stride, int height,
        int log2_denom, int weight, int offset);
void ff_h264_biweight_pixels4_8_mmi(uint8_t *dst, uint8_t *src,
        ptrdiff_t stride, int height, int log2_denom, int weightd, int weights,
        int offset);

void ff_deblock_v_chroma_8_mmi(uint8_t *pix, ptrdiff_t stride, int alpha, int beta,
        int8_t *tc0);
void ff_deblock_v_chroma_intra_8_mmi(uint8_t *pix, ptrdiff_t stride, int alpha,
        int beta);
void ff_deblock_h_chroma_8_mmi(uint8_t *pix, ptrdiff_t stride, int alpha, int beta,
        int8_t *tc0);
void ff_deblock_h_chroma_intra_8_mmi(uint8_t *pix, ptrdiff_t stride, int alpha,
        int beta);
void ff_deblock_v_luma_8_mmi(uint8_t *pix, ptrdiff_t stride, int alpha, int beta,
        int8_t *tc0);
void ff_deblock_v_luma_intra_8_mmi(uint8_t *pix, ptrdiff_t stride, int alpha,
        int beta);
void ff_deblock_h_luma_8_mmi(uint8_t *pix, ptrdiff_t stride, int alpha, int beta,
        int8_t *tc0);
void ff_deblock_h_luma_intra_8_mmi(uint8_t *pix, ptrdiff_t stride, int alpha,
        int beta);
void ff_deblock_v8_luma_8_mmi(uint8_t *pix, ptrdiff_t stride, int alpha, int beta,
        int8_t *tc0);
void ff_deblock_v8_luma_intra_8_mmi(uint8_t *pix, ptrdiff_t stride, int alpha,
        int beta);

void ff_put_h264_qpel16_mc00_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc10_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc20_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc30_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc01_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc11_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc21_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc31_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc02_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc12_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc22_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc32_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc03_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc13_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc23_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel16_mc33_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);

void ff_put_h264_qpel8_mc00_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc10_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc20_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc30_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc01_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc11_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc21_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc31_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc02_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc12_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc22_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc32_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc03_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc13_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc23_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel8_mc33_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);

void ff_put_h264_qpel4_mc00_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc10_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc20_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc30_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc01_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc11_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc21_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc31_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc02_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc12_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc22_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc32_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc03_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc13_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc23_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_put_h264_qpel4_mc33_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);

void ff_avg_h264_qpel16_mc00_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc10_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc20_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc30_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc01_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc11_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc21_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc31_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc02_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc12_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc22_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc32_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc03_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc13_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc23_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel16_mc33_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);

void ff_avg_h264_qpel8_mc00_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc10_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc20_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc30_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc01_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc11_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc21_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc31_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc02_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc12_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc22_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc32_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc03_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc13_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc23_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel8_mc33_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);

void ff_avg_h264_qpel4_mc00_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc10_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc20_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc30_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc01_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc11_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc21_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc31_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc02_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc12_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc22_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc32_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc03_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc13_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc23_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);
void ff_avg_h264_qpel4_mc33_mmi(uint8_t *dst, const uint8_t *src,
        ptrdiff_t dst_stride);

#endif  // #ifndef AVCODEC_MIPS_H264DSP_MIPS_H
