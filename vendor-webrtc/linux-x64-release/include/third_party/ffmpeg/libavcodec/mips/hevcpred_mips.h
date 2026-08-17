/*
 * Copyright (c) 2015 Shivraj Patil (Shivraj.Patil@imgtec.com)
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

#ifndef AVCODEC_MIPS_HEVCPRED_MIPS_H
#define AVCODEC_MIPS_HEVCPRED_MIPS_H

#include "libavcodec/hevc/pred.h"

void ff_hevc_intra_pred_planar_0_msa(uint8_t *dst,
                                     const uint8_t *src_top,
                                     const uint8_t *src_left,
                                     ptrdiff_t stride);

void ff_hevc_intra_pred_planar_1_msa(uint8_t *dst,
                                     const uint8_t *src_top,
                                     const uint8_t *src_left,
                                     ptrdiff_t stride);

void ff_hevc_intra_pred_planar_2_msa(uint8_t *dst,
                                     const uint8_t *src_top,
                                     const uint8_t *src_left,
                                     ptrdiff_t stride);

void ff_hevc_intra_pred_planar_3_msa(uint8_t *dst,
                                     const uint8_t *src_top,
                                     const uint8_t *src_left,
                                     ptrdiff_t stride);

void ff_hevc_intra_pred_dc_msa(uint8_t *dst, const uint8_t *src_top,
                               const uint8_t *src_left,
                               ptrdiff_t stride, int log2, int c_idx);

void ff_pred_intra_pred_angular_0_msa(uint8_t *dst,
                                      const uint8_t *src_top,
                                      const uint8_t *src_left,
                                      ptrdiff_t stride, int c_idx, int mode);

void ff_pred_intra_pred_angular_1_msa(uint8_t *dst,
                                      const uint8_t *src_top,
                                      const uint8_t *src_left,
                                      ptrdiff_t stride, int c_idx, int mode);

void ff_pred_intra_pred_angular_2_msa(uint8_t *dst,
                                      const uint8_t *src_top,
                                      const uint8_t *src_left,
                                      ptrdiff_t stride, int c_idx, int mode);

void ff_pred_intra_pred_angular_3_msa(uint8_t *dst,
                                      const uint8_t *src_top,
                                      const uint8_t *src_left,
                                      ptrdiff_t stride, int c_idx, int mode);

void ff_intra_pred_8_16x16_msa(struct HEVCLocalContext *s, const struct HEVCPPS *pps, int x0, int y0, int c_idx);
void ff_intra_pred_8_32x32_msa(struct HEVCLocalContext *s, const struct HEVCPPS *pps, int x0, int y0, int c_idx);

#endif  // #ifndef AVCODEC_MIPS_HEVCPRED_MIPS_H
