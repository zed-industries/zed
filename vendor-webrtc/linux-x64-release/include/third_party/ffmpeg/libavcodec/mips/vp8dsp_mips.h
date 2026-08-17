/*
 * Copyright (c) 2015 Manojkumar Bhosale (Manojkumar.Bhosale@imgtec.com)
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

#ifndef AVCODEC_MIPS_VP8DSP_MIPS_H
#define AVCODEC_MIPS_VP8DSP_MIPS_H

#include "libavutil/mem.h"
#include "libavcodec/vp8dsp.h"
#include "libavcodec/mathops.h"
#include "constants.h"

void ff_put_vp8_pixels4_msa(uint8_t *dst, ptrdiff_t dststride,
                            const uint8_t *src, ptrdiff_t srcstride,
                            int h, int x, int y);
void ff_put_vp8_pixels8_msa(uint8_t *dst, ptrdiff_t dststride,
                            const uint8_t *src, ptrdiff_t srcstride,
                            int h, int x, int y);
void ff_put_vp8_pixels16_msa(uint8_t *dst, ptrdiff_t dststride,
                             const uint8_t *src, ptrdiff_t srcstride,
                             int h, int x, int y);

void ff_put_vp8_epel16_h4_msa(uint8_t *dst, ptrdiff_t dststride,
                              const uint8_t *src, ptrdiff_t srcstride,
                              int h, int mx, int my);
void ff_put_vp8_epel16_h6_msa(uint8_t *dst, ptrdiff_t dststride,
                              const uint8_t *src, ptrdiff_t srcstride,
                              int h, int mx, int my);
void ff_put_vp8_epel16_v4_msa(uint8_t *dst, ptrdiff_t dststride,
                              const uint8_t *src, ptrdiff_t srcstride,
                              int h, int mx, int my);
void ff_put_vp8_epel16_v6_msa(uint8_t *dst, ptrdiff_t dststride,
                              const uint8_t *src, ptrdiff_t srcstride,
                              int h, int mx, int my);
void ff_put_vp8_epel16_h4v4_msa(uint8_t *dst, ptrdiff_t dststride,
                                const uint8_t *src, ptrdiff_t srcstride,
                                int h, int mx, int my);
void ff_put_vp8_epel16_h6v4_msa(uint8_t *dst, ptrdiff_t dststride,
                                const uint8_t *src, ptrdiff_t srcstride,
                                int h, int mx, int my);
void ff_put_vp8_epel16_h4v6_msa(uint8_t *dst, ptrdiff_t dststride,
                                const uint8_t *src, ptrdiff_t srcstride,
                                int h, int mx, int my);
void ff_put_vp8_epel16_h6v6_msa(uint8_t *dst, ptrdiff_t dststride,
                                const uint8_t *src, ptrdiff_t srcstride,
                                int h, int mx, int my);

void ff_put_vp8_epel8_h4_msa(uint8_t *dst, ptrdiff_t dststride,
                             const uint8_t *src, ptrdiff_t srcstride,
                             int h, int mx, int my);
void ff_put_vp8_epel8_h6_msa(uint8_t *dst, ptrdiff_t dststride,
                             const uint8_t *src, ptrdiff_t srcstride,
                             int h, int mx, int my);
void ff_put_vp8_epel8_v4_msa(uint8_t *dst, ptrdiff_t dststride,
                             const uint8_t *src, ptrdiff_t srcstride,
                             int h, int mx, int my);
void ff_put_vp8_epel8_v6_msa(uint8_t *dst, ptrdiff_t dststride,
                             const uint8_t *src, ptrdiff_t srcstride,
                             int h, int mx, int my);
void ff_put_vp8_epel8_h4v4_msa(uint8_t *dst, ptrdiff_t dststride,
                               const uint8_t *src, ptrdiff_t srcstride,
                               int h, int mx, int my);
void ff_put_vp8_epel8_h6v4_msa(uint8_t *dst, ptrdiff_t dststride,
                               const uint8_t *src, ptrdiff_t srcstride,
                               int h, int mx, int my);
void ff_put_vp8_epel8_h4v6_msa(uint8_t *dst, ptrdiff_t dststride,
                               const uint8_t *src, ptrdiff_t srcstride,
                               int h, int mx, int my);
void ff_put_vp8_epel8_h6v6_msa(uint8_t *dst, ptrdiff_t dststride,
                               const uint8_t *src, ptrdiff_t srcstride,
                               int h, int mx, int my);

void ff_put_vp8_epel4_h4_msa(uint8_t *dst, ptrdiff_t dststride,
                             const uint8_t *src, ptrdiff_t srcstride,
                             int h, int mx, int my);
void ff_put_vp8_epel4_h6_msa(uint8_t *dst, ptrdiff_t dststride,
                             const uint8_t *src, ptrdiff_t srcstride,
                             int h, int mx, int my);
void ff_put_vp8_epel4_v4_msa(uint8_t *dst, ptrdiff_t dststride,
                             const uint8_t *src, ptrdiff_t srcstride,
                             int h, int mx, int my);
void ff_put_vp8_epel4_v6_msa(uint8_t *dst, ptrdiff_t dststride,
                             const uint8_t *src, ptrdiff_t srcstride,
                             int h, int mx, int my);
void ff_put_vp8_epel4_h4v4_msa(uint8_t *dst, ptrdiff_t dststride,
                               const uint8_t *src, ptrdiff_t srcstride,
                               int h, int mx, int my);
void ff_put_vp8_epel4_h6v4_msa(uint8_t *dst, ptrdiff_t dststride,
                               const uint8_t *src, ptrdiff_t srcstride,
                               int h, int mx, int my);
void ff_put_vp8_epel4_h4v6_msa(uint8_t *dst, ptrdiff_t dststride,
                               const uint8_t *src, ptrdiff_t srcstride,
                               int h, int mx, int my);
void ff_put_vp8_epel4_h6v6_msa(uint8_t *dst, ptrdiff_t dststride,
                               const uint8_t *src, ptrdiff_t srcstride,
                               int h, int mx, int my);

void ff_put_vp8_bilinear16_h_msa(uint8_t *dst, ptrdiff_t dststride,
                                 const uint8_t *src, ptrdiff_t srcstride,
                                 int h, int mx, int my);
void ff_put_vp8_bilinear16_v_msa(uint8_t *dst, ptrdiff_t dststride,
                                 const uint8_t *src, ptrdiff_t srcstride,
                                 int h, int mx, int my);
void ff_put_vp8_bilinear16_hv_msa(uint8_t *dst, ptrdiff_t dststride,
                                  const uint8_t *src, ptrdiff_t srcstride,
                                  int h, int mx, int my);

void ff_put_vp8_bilinear8_h_msa(uint8_t *dst, ptrdiff_t dststride,
                                const uint8_t *src, ptrdiff_t srcstride,
                                int h, int mx, int my);
void ff_put_vp8_bilinear8_v_msa(uint8_t *dst, ptrdiff_t dststride,
                                const uint8_t *src, ptrdiff_t srcstride,
                                int h, int mx, int my);
void ff_put_vp8_bilinear8_hv_msa(uint8_t *dst, ptrdiff_t dststride,
                                 const uint8_t *src, ptrdiff_t srcstride,
                                 int h, int mx, int my);

void ff_put_vp8_bilinear4_h_msa(uint8_t *dst, ptrdiff_t dststride,
                                const uint8_t *src, ptrdiff_t srcstride,
                                int h, int mx, int my);
void ff_put_vp8_bilinear4_v_msa(uint8_t *dst, ptrdiff_t dststride,
                                const uint8_t *src, ptrdiff_t srcstride,
                                int h, int mx, int my);
void ff_put_vp8_bilinear4_hv_msa(uint8_t *dst, ptrdiff_t dststride,
                                 const uint8_t *src, ptrdiff_t srcstride,
                                 int h, int mx, int my);

/* loop filter */
void ff_vp8_h_loop_filter16_inner_msa(uint8_t *dst, ptrdiff_t stride,
                                      int32_t e, int32_t i, int32_t h);
void ff_vp8_v_loop_filter16_inner_msa(uint8_t *dst, ptrdiff_t stride,
                                      int32_t e, int32_t i, int32_t h);
void ff_vp8_h_loop_filter8uv_inner_msa(uint8_t *dst_u, uint8_t *dst_v,
                                       ptrdiff_t stride,
                                       int flim_e, int flim_i, int hev_thresh);
void ff_vp8_v_loop_filter8uv_inner_msa(uint8_t *dst_u, uint8_t *dst_v,
                                       ptrdiff_t stride,
                                       int flim_e, int flim_i, int hev_thresh);
void ff_vp8_h_loop_filter16_msa(uint8_t *dst, ptrdiff_t stride,
                                int flim_e, int flim_i, int hev_thresh);
void ff_vp8_v_loop_filter16_msa(uint8_t *dst, ptrdiff_t stride,
                                int flim_e, int flim_i, int hev_thresh);
void ff_vp8_h_loop_filter8uv_msa(uint8_t *dst_u, uint8_t *dst_v,
                                 ptrdiff_t stride,
                                 int flim_e, int flim_i, int hev_thresh);
void ff_vp8_v_loop_filter8uv_msa(uint8_t *dst_u, uint8_t *dst_v,
                                 ptrdiff_t stride,
                                 int flim_e, int flim_i, int hev_thresh);
void ff_vp8_h_loop_filter_simple_msa(uint8_t *dst, ptrdiff_t stride, int flim);
void ff_vp8_v_loop_filter_simple_msa(uint8_t *dst, ptrdiff_t stride, int flim);

/* Idct functions */
void ff_vp8_luma_dc_wht_msa(int16_t block[4][4][16], int16_t dc[16]);
void ff_vp8_idct_add_msa(uint8_t *dst, int16_t block[16], ptrdiff_t stride);
void ff_vp8_idct_dc_add_msa(uint8_t *dst, int16_t block[16], ptrdiff_t stride);
void ff_vp8_idct_dc_add4uv_msa(uint8_t *dst, int16_t block[4][16],
                               ptrdiff_t stride);
void ff_vp8_idct_dc_add4y_msa(uint8_t *dst, int16_t block[4][16],
                              ptrdiff_t stride);

void ff_vp8_luma_dc_wht_mmi(int16_t block[4][4][16], int16_t dc[16]);
void ff_vp8_luma_dc_wht_dc_mmi(int16_t block[4][4][16], int16_t dc[16]);
void ff_vp8_idct_add_mmi(uint8_t *dst, int16_t block[16], ptrdiff_t stride);
void ff_vp8_idct_dc_add_mmi(uint8_t *dst, int16_t block[16], ptrdiff_t stride);
void ff_vp8_idct_dc_add4y_mmi(uint8_t *dst, int16_t block[4][16],
        ptrdiff_t stride);
void ff_vp8_idct_dc_add4uv_mmi(uint8_t *dst, int16_t block[4][16],
        ptrdiff_t stride);

void ff_put_vp8_pixels4_mmi(uint8_t *dst, ptrdiff_t dststride, const uint8_t *src,
        ptrdiff_t srcstride, int h, int x, int y);
void ff_put_vp8_pixels8_mmi(uint8_t *dst, ptrdiff_t dststride, const uint8_t *src,
        ptrdiff_t srcstride, int h, int x, int y);
void ff_put_vp8_pixels16_mmi(uint8_t *dst, ptrdiff_t dststride, const uint8_t *src,
        ptrdiff_t srcstride, int h, int x, int y);

void ff_put_vp8_epel16_h4_mmi(uint8_t *dst, ptrdiff_t dststride, const uint8_t *src,
        ptrdiff_t srcstride, int h, int mx, int my);
void ff_put_vp8_epel16_h6_mmi(uint8_t *dst, ptrdiff_t dststride, const uint8_t *src,
        ptrdiff_t srcstride, int h, int mx, int my);
void ff_put_vp8_epel16_v4_mmi(uint8_t *dst, ptrdiff_t dststride, const uint8_t *src,
        ptrdiff_t srcstride, int h, int mx, int my);
void ff_put_vp8_epel16_v6_mmi(uint8_t *dst, ptrdiff_t dststride, const uint8_t *src,
        ptrdiff_t srcstride, int h, int mx, int my);
void ff_put_vp8_epel16_h4v4_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);
void ff_put_vp8_epel16_h6v4_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);
void ff_put_vp8_epel16_h4v6_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);
void ff_put_vp8_epel16_h6v6_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);

void ff_put_vp8_epel8_h4_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);
void ff_put_vp8_epel8_h6_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);
void ff_put_vp8_epel8_v4_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);
void ff_put_vp8_epel8_v6_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);
void ff_put_vp8_epel8_h4v4_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);
void ff_put_vp8_epel8_h6v4_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);
void ff_put_vp8_epel8_h4v6_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);
void ff_put_vp8_epel8_h6v6_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);

void ff_put_vp8_epel4_h4_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);
void ff_put_vp8_epel4_h6_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);
void ff_put_vp8_epel4_v4_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);
void ff_put_vp8_epel4_v6_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);
void ff_put_vp8_epel4_h4v4_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);
void ff_put_vp8_epel4_h6v4_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);
void ff_put_vp8_epel4_h4v6_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);
void ff_put_vp8_epel4_h6v6_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);

void ff_put_vp8_bilinear16_h_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);
void ff_put_vp8_bilinear16_v_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);
void ff_put_vp8_bilinear16_hv_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);

void ff_put_vp8_bilinear8_h_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);
void ff_put_vp8_bilinear8_v_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);
void ff_put_vp8_bilinear8_hv_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);

void ff_put_vp8_bilinear4_h_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);
void ff_put_vp8_bilinear4_v_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);
void ff_put_vp8_bilinear4_hv_mmi(uint8_t *dst, ptrdiff_t dststride,
        const uint8_t *src, ptrdiff_t srcstride, int h, int mx, int my);

// loop filter applied to edges between macroblocks
void ff_vp8_v_loop_filter16_mmi(uint8_t *dst, ptrdiff_t stride, int flim_E,
        int flim_I, int hev_thresh);
void ff_vp8_h_loop_filter16_mmi(uint8_t *dst, ptrdiff_t stride, int flim_E,
        int flim_I, int hev_thresh);
void ff_vp8_v_loop_filter8uv_mmi(uint8_t *dstU, uint8_t *dstV, ptrdiff_t stride,
        int flim_E, int flim_I, int hev_thresh);
void ff_vp8_h_loop_filter8uv_mmi(uint8_t *dstU, uint8_t *dstV, ptrdiff_t stride,
        int flim_E, int flim_I, int hev_thresh);

// loop filter applied to inner macroblock edges
void ff_vp8_v_loop_filter16_inner_mmi(uint8_t *dst, ptrdiff_t stride,
        int flim_E, int flim_I, int hev_thresh);
void ff_vp8_h_loop_filter16_inner_mmi(uint8_t *dst, ptrdiff_t stride,
        int flim_E, int flim_I, int hev_thresh);
void ff_vp8_v_loop_filter8uv_inner_mmi(uint8_t *dstU, uint8_t *dstV,
        ptrdiff_t stride, int flim_E, int flim_I, int hev_thresh);
void ff_vp8_h_loop_filter8uv_inner_mmi(uint8_t *dstU, uint8_t *dstV,
        ptrdiff_t stride, int flim_E, int flim_I, int hev_thresh);

void ff_vp8_v_loop_filter_simple_mmi(uint8_t *dst, ptrdiff_t stride, int flim);
void ff_vp8_h_loop_filter_simple_mmi(uint8_t *dst, ptrdiff_t stride, int flim);

#endif  // #ifndef AVCODEC_MIPS_VP8DSP_MIPS_H
