/*
 * Copyright (c) 2021 Loongson Technology Corporation Limited
 * Contributed by Hecai Yuan <yuanhecai@loongson.cn>
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

#ifndef AVCODEC_LOONGARCH_VP8DSP_LOONGARCH_H
#define AVCODEC_LOONGARCH_VP8DSP_LOONGARCH_H

#include "libavcodec/vp8dsp.h"

void ff_put_vp8_pixels8_lsx(uint8_t *dst, ptrdiff_t dst_stride,
                            const uint8_t *src, ptrdiff_t src_stride,
                            int h, int x, int y);
void ff_put_vp8_pixels16_lsx(uint8_t *dst, ptrdiff_t dst_stride,
                             const uint8_t *src, ptrdiff_t src_stride,
                             int h, int x, int y);

void ff_put_vp8_epel16_h6_lsx(uint8_t *dst, ptrdiff_t dst_stride,
                              const uint8_t *src, ptrdiff_t src_stride,
                              int h, int mx, int my);
void ff_put_vp8_epel16_v4_lsx(uint8_t *dst, ptrdiff_t dst_stride,
                              const uint8_t *src, ptrdiff_t src_stride,
                              int h, int mx, int my);
void ff_put_vp8_epel16_v6_lsx(uint8_t *dst, ptrdiff_t dst_stride,
                              const uint8_t *src, ptrdiff_t src_stride,
                              int h, int mx, int my);
void ff_put_vp8_epel16_h6v4_lsx(uint8_t *dst, ptrdiff_t dst_stride,
                                const uint8_t *src, ptrdiff_t src_stride,
                                int h, int mx, int my);
void ff_put_vp8_epel16_h4v6_lsx(uint8_t *dst, ptrdiff_t dst_stride,
                                const uint8_t *src, ptrdiff_t src_stride,
                                int h, int mx, int my);
void ff_put_vp8_epel16_h6v6_lsx(uint8_t *dst, ptrdiff_t dst_stride,
                                const uint8_t *src, ptrdiff_t src_stride,
                                int h, int mx, int my);

void ff_put_vp8_epel8_v4_lsx(uint8_t *dst, ptrdiff_t dst_stride,
                             const uint8_t *src, ptrdiff_t src_stride,
                             int h, int mx, int my);
void ff_put_vp8_epel8_v6_lsx(uint8_t *dst, ptrdiff_t dst_stride,
                             const uint8_t *src, ptrdiff_t src_stride,
                             int h, int mx, int my);
void ff_put_vp8_epel8_h6v4_lsx(uint8_t *dst, ptrdiff_t dst_stride,
                               const uint8_t *src, ptrdiff_t src_stride,
                               int h, int mx, int my);
void ff_put_vp8_epel8_h4v6_lsx(uint8_t *dst, ptrdiff_t dst_stride,
                               const uint8_t *src, ptrdiff_t src_stride,
                               int h, int mx, int my);
void ff_put_vp8_epel8_h6v6_lsx(uint8_t *dst, ptrdiff_t dst_stride,
                               const uint8_t *src, ptrdiff_t src_stride,
                               int h, int mx, int my);

void ff_put_vp8_epel8_h6_lsx(uint8_t *dst, ptrdiff_t dst_stride,
                             const uint8_t *src, ptrdiff_t src_stride,
                             int h, int mx, int my);

/* loop filter */
void ff_vp8_v_loop_filter16_inner_lsx(uint8_t *dst, ptrdiff_t stride,
                                      int32_t e, int32_t i, int32_t h);
void ff_vp8_h_loop_filter16_inner_lsx(uint8_t *src, ptrdiff_t stride,
                                      int32_t e, int32_t i, int32_t h);

void ff_vp8_v_loop_filter16_lsx(uint8_t *dst, ptrdiff_t stride,
                                int flim_e, int flim_i, int hev_thresh);
void ff_vp8_h_loop_filter16_lsx(uint8_t *dst, ptrdiff_t stride,
                                int flim_e, int flim_i, int hev_thresh);
void ff_vp8_h_loop_filter8uv_lsx(uint8_t *dst_u, uint8_t *dst_v,
                                 ptrdiff_t stride,
                                 int flim_e, int flim_i, int hev_thresh);
void ff_vp8_v_loop_filter8uv_lsx(uint8_t *dst_u, uint8_t *dst_v,
                                 ptrdiff_t stride,
                                 int flim_e, int flim_i, int hev_thresh);

#endif  // #ifndef AVCODEC_LOONGARCH_VP8DSP_LOONGARCH_H
