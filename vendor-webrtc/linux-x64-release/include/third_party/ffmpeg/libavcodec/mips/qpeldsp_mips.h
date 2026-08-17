/*
 * Copyright (c) 2015 Parag Salasakar (Parag.Salasakar@imgtec.com)
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

#ifndef AVCODEC_MIPS_QPELDSP_MIPS_H
#define AVCODEC_MIPS_QPELDSP_MIPS_H

#include "../mpegvideo.h"

void ff_copy_8x8_msa(uint8_t *dst, const uint8_t *src, ptrdiff_t stride);
void ff_copy_16x16_msa(uint8_t *dst, const uint8_t *src, ptrdiff_t stride);
void ff_avg_width8_msa(uint8_t *dst, const uint8_t *src, ptrdiff_t stride);
void ff_avg_width16_msa(uint8_t *dst, const uint8_t *src, ptrdiff_t stride);

void ff_horiz_mc_qpel_aver_src0_8width_msa(uint8_t *dst, const uint8_t *src,
                                           ptrdiff_t stride);
void ff_horiz_mc_qpel_aver_src0_16width_msa(uint8_t *dst, const uint8_t *src,
                                            ptrdiff_t stride);
void ff_horiz_mc_qpel_8width_msa(uint8_t *dst, const uint8_t *src,
                                 ptrdiff_t stride);
void ff_horiz_mc_qpel_16width_msa(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride);
void ff_horiz_mc_qpel_aver_src1_8width_msa(uint8_t *dst, const uint8_t *src,
                                           ptrdiff_t stride);
void ff_horiz_mc_qpel_aver_src1_16width_msa(uint8_t *dst, const uint8_t *src,
                                            ptrdiff_t stride);
void ff_horiz_mc_qpel_no_rnd_aver_src0_8width_msa(uint8_t *dst,
                                                  const uint8_t *src,
                                                  ptrdiff_t stride);
void ff_horiz_mc_qpel_no_rnd_aver_src0_16width_msa(uint8_t *dst,
                                                   const uint8_t *src,
                                                   ptrdiff_t stride);
void ff_horiz_mc_qpel_no_rnd_8width_msa(uint8_t *dst, const uint8_t *src,
                                        ptrdiff_t stride);
void ff_horiz_mc_qpel_no_rnd_16width_msa(uint8_t *dst, const uint8_t *src,
                                         ptrdiff_t stride);
void ff_horiz_mc_qpel_no_rnd_aver_src1_8width_msa(uint8_t *dst,
                                                  const uint8_t *src,
                                                  ptrdiff_t stride);
void ff_horiz_mc_qpel_no_rnd_aver_src1_16width_msa(uint8_t *dst,
                                                   const uint8_t *src,
                                                   ptrdiff_t stride);
void ff_horiz_mc_qpel_avg_dst_aver_src0_8width_msa(uint8_t *dst,
                                                   const uint8_t *src,
                                                   ptrdiff_t stride);
void ff_horiz_mc_qpel_avg_dst_aver_src0_16width_msa(uint8_t *dst,
                                                    const uint8_t *src,
                                                    ptrdiff_t stride);
void ff_horiz_mc_qpel_avg_dst_8width_msa(uint8_t *dst, const uint8_t *src,
                                         ptrdiff_t stride);
void ff_horiz_mc_qpel_avg_dst_16width_msa(uint8_t *dst, const uint8_t *src,
                                          ptrdiff_t stride);
void ff_horiz_mc_qpel_avg_dst_aver_src1_8width_msa(uint8_t *dst,
                                                   const uint8_t *src,
                                                   ptrdiff_t stride);
void ff_horiz_mc_qpel_avg_dst_aver_src1_16width_msa(uint8_t *dst,
                                                    const uint8_t *src,
                                                    ptrdiff_t stride);

void ff_vert_mc_qpel_aver_src0_8x8_msa(uint8_t *dst, const uint8_t *src,
                                       ptrdiff_t stride);
void ff_vert_mc_qpel_aver_src0_16x16_msa(uint8_t *dst, const uint8_t *src,
                                         ptrdiff_t stride);
void ff_vert_mc_qpel_8x8_msa(uint8_t *dst, const uint8_t *src,
                             ptrdiff_t stride);
void ff_vert_mc_qpel_16x16_msa(uint8_t *dst, const uint8_t *src,
                               ptrdiff_t stride);
void ff_vert_mc_qpel_aver_src1_8x8_msa(uint8_t *dst, const uint8_t *src,
                                       ptrdiff_t stride);
void ff_vert_mc_qpel_aver_src1_16x16_msa(uint8_t *dst, const uint8_t *src,
                                         ptrdiff_t stride);
void ff_vert_mc_qpel_no_rnd_aver_src0_8x8_msa(uint8_t *dst,
                                              const uint8_t *src,
                                              ptrdiff_t stride);
void ff_vert_mc_qpel_no_rnd_aver_src0_16x16_msa(uint8_t *dst,
                                                const uint8_t *src,
                                                ptrdiff_t stride);
void ff_vert_mc_qpel_no_rnd_8x8_msa(uint8_t *dst, const uint8_t *src,
                                    ptrdiff_t stride);
void ff_vert_mc_qpel_no_rnd_16x16_msa(uint8_t *dst, const uint8_t *src,
                                      ptrdiff_t stride);
void ff_vert_mc_qpel_no_rnd_aver_src1_8x8_msa(uint8_t *dst,
                                              const uint8_t *src,
                                              ptrdiff_t stride);
void ff_vert_mc_qpel_no_rnd_aver_src1_16x16_msa(uint8_t *dst,
                                                const uint8_t *src,
                                                ptrdiff_t stride);
void ff_vert_mc_qpel_avg_dst_aver_src0_8x8_msa(uint8_t *dst,
                                               const uint8_t *src,
                                               ptrdiff_t stride);
void ff_vert_mc_qpel_avg_dst_aver_src0_16x16_msa(uint8_t *dst,
                                                 const uint8_t *src,
                                                 ptrdiff_t stride);
void ff_vert_mc_qpel_avg_dst_8x8_msa(uint8_t *dst, const uint8_t *src,
                                     ptrdiff_t stride);
void ff_vert_mc_qpel_avg_dst_16x16_msa(uint8_t *dst, const uint8_t *src,
                                       ptrdiff_t stride);
void ff_vert_mc_qpel_avg_dst_aver_src1_8x8_msa(uint8_t *dst,
                                               const uint8_t *src,
                                               ptrdiff_t stride);
void ff_vert_mc_qpel_avg_dst_aver_src1_16x16_msa(uint8_t *dst,
                                                 const uint8_t *src,
                                                 ptrdiff_t stride);

void ff_hv_mc_qpel_aver_hv_src00_16x16_msa(uint8_t *dst, const uint8_t *src,
                                           ptrdiff_t stride);
void ff_hv_mc_qpel_aver_hv_src00_8x8_msa(uint8_t *dst, const uint8_t *src,
                                         ptrdiff_t stride);
void ff_hv_mc_qpel_aver_v_src0_16x16_msa(uint8_t *dst, const uint8_t *src,
                                         ptrdiff_t stride);
void ff_hv_mc_qpel_aver_v_src0_8x8_msa(uint8_t *dst, const uint8_t *src,
                                       ptrdiff_t stride);
void ff_hv_mc_qpel_aver_hv_src10_16x16_msa(uint8_t *dst, const uint8_t *src,
                                           ptrdiff_t stride);
void ff_hv_mc_qpel_aver_hv_src10_8x8_msa(uint8_t *dst, const uint8_t *src,
                                         ptrdiff_t stride);
void ff_hv_mc_qpel_aver_h_src0_16x16_msa(uint8_t *dst, const uint8_t *src,
                                         ptrdiff_t stride);
void ff_hv_mc_qpel_aver_h_src0_8x8_msa(uint8_t *dst, const uint8_t *src,
                                       ptrdiff_t stride);
void ff_hv_mc_qpel_16x16_msa(uint8_t *dst, const uint8_t *src,
                             ptrdiff_t stride);
void ff_hv_mc_qpel_8x8_msa(uint8_t *dst, const uint8_t *src, ptrdiff_t stride);
void ff_hv_mc_qpel_aver_h_src1_16x16_msa(uint8_t *dst, const uint8_t *src,
                                         ptrdiff_t stride);
void ff_hv_mc_qpel_aver_h_src1_8x8_msa(uint8_t *dst, const uint8_t *src,
                                       ptrdiff_t stride);
void ff_hv_mc_qpel_aver_hv_src01_16x16_msa(uint8_t *dst, const uint8_t *src,
                                           ptrdiff_t stride);
void ff_hv_mc_qpel_aver_hv_src01_8x8_msa(uint8_t *dst, const uint8_t *src,
                                         ptrdiff_t stride);
void ff_hv_mc_qpel_aver_v_src1_16x16_msa(uint8_t *dst, const uint8_t *src,
                                         ptrdiff_t stride);
void ff_hv_mc_qpel_aver_v_src1_8x8_msa(uint8_t *dst, const uint8_t *src,
                                       ptrdiff_t stride);
void ff_hv_mc_qpel_aver_hv_src11_16x16_msa(uint8_t *dst, const uint8_t *src,
                                           ptrdiff_t stride);
void ff_hv_mc_qpel_aver_hv_src11_8x8_msa(uint8_t *dst, const uint8_t *src,
                                         ptrdiff_t stride);
void ff_hv_mc_qpel_avg_dst_aver_hv_src00_16x16_msa(uint8_t *dst,
                                                   const uint8_t *src,
                                                   ptrdiff_t stride);
void ff_hv_mc_qpel_avg_dst_aver_hv_src00_8x8_msa(uint8_t *dst,
                                                 const uint8_t *src,
                                                 ptrdiff_t stride);
void ff_hv_mc_qpel_avg_dst_aver_v_src0_16x16_msa(uint8_t *dst,
                                                 const uint8_t *src,
                                                 ptrdiff_t stride);
void ff_hv_mc_qpel_avg_dst_aver_v_src0_8x8_msa(uint8_t *dst,
                                               const uint8_t *src,
                                               ptrdiff_t stride);
void ff_hv_mc_qpel_avg_dst_aver_hv_src10_16x16_msa(uint8_t *dst,
                                                   const uint8_t *src,
                                                   ptrdiff_t stride);
void ff_hv_mc_qpel_avg_dst_aver_hv_src10_8x8_msa(uint8_t *dst,
                                                 const uint8_t *src,
                                                 ptrdiff_t stride);
void ff_hv_mc_qpel_avg_dst_aver_h_src0_16x16_msa(uint8_t *dst,
                                                 const uint8_t *src,
                                                 ptrdiff_t stride);
void ff_hv_mc_qpel_avg_dst_aver_h_src0_8x8_msa(uint8_t *dst,
                                               const uint8_t *src,
                                               ptrdiff_t stride);
void ff_hv_mc_qpel_avg_dst_16x16_msa(uint8_t *dst, const uint8_t *src,
                                     ptrdiff_t stride);
void ff_hv_mc_qpel_avg_dst_8x8_msa(uint8_t *dst, const uint8_t *src,
                                   ptrdiff_t stride);
void ff_hv_mc_qpel_avg_dst_aver_h_src1_16x16_msa(uint8_t *dst,
                                                 const uint8_t *src,
                                                 ptrdiff_t stride);
void ff_hv_mc_qpel_avg_dst_aver_h_src1_8x8_msa(uint8_t *dst,
                                               const uint8_t *src,
                                               ptrdiff_t stride);
void ff_hv_mc_qpel_avg_dst_aver_hv_src01_16x16_msa(uint8_t *dst,
                                                   const uint8_t *src,
                                                   ptrdiff_t stride);
void ff_hv_mc_qpel_avg_dst_aver_hv_src01_8x8_msa(uint8_t *dst,
                                                 const uint8_t *src,
                                                 ptrdiff_t stride);
void ff_hv_mc_qpel_avg_dst_aver_v_src1_16x16_msa(uint8_t *dst,
                                                 const uint8_t *src,
                                                 ptrdiff_t stride);
void ff_hv_mc_qpel_avg_dst_aver_v_src1_8x8_msa(uint8_t *dst,
                                               const uint8_t *src,
                                               ptrdiff_t stride);
void ff_hv_mc_qpel_avg_dst_aver_hv_src11_16x16_msa(uint8_t *dst,
                                                   const uint8_t *src,
                                                   ptrdiff_t stride);
void ff_hv_mc_qpel_avg_dst_aver_hv_src11_8x8_msa(uint8_t *dst,
                                                 const uint8_t *src,
                                                 ptrdiff_t stride);
void ff_hv_mc_qpel_no_rnd_aver_hv_src00_16x16_msa(uint8_t *dst,
                                                  const uint8_t *src,
                                                  ptrdiff_t stride);
void ff_hv_mc_qpel_no_rnd_aver_hv_src00_8x8_msa(uint8_t *dst,
                                                const uint8_t *src,
                                                ptrdiff_t stride);
void ff_hv_mc_qpel_no_rnd_aver_v_src0_16x16_msa(uint8_t *dst,
                                                const uint8_t *src,
                                                ptrdiff_t stride);
void ff_hv_mc_qpel_no_rnd_aver_v_src0_8x8_msa(uint8_t *dst,
                                              const uint8_t *src,
                                              ptrdiff_t stride);
void ff_hv_mc_qpel_no_rnd_aver_hv_src10_16x16_msa(uint8_t *dst,
                                                  const uint8_t *src,
                                                  ptrdiff_t stride);
void ff_hv_mc_qpel_no_rnd_aver_hv_src10_8x8_msa(uint8_t *dst,
                                                const uint8_t *src,
                                                ptrdiff_t stride);
void ff_hv_mc_qpel_no_rnd_aver_h_src0_16x16_msa(uint8_t *dst,
                                                const uint8_t *src,
                                                ptrdiff_t stride);
void ff_hv_mc_qpel_no_rnd_aver_h_src0_8x8_msa(uint8_t *dst,
                                              const uint8_t *src,
                                              ptrdiff_t stride);
void ff_hv_mc_qpel_no_rnd_16x16_msa(uint8_t *dst, const uint8_t *src,
                                    ptrdiff_t stride);
void ff_hv_mc_qpel_no_rnd_8x8_msa(uint8_t *dst, const uint8_t *src,
                                  ptrdiff_t stride);
void ff_hv_mc_qpel_no_rnd_aver_h_src1_16x16_msa(uint8_t *dst,
                                                const uint8_t *src,
                                                ptrdiff_t stride);
void ff_hv_mc_qpel_no_rnd_aver_h_src1_8x8_msa(uint8_t *dst,
                                              const uint8_t *src,
                                              ptrdiff_t stride);
void ff_hv_mc_qpel_no_rnd_aver_hv_src01_16x16_msa(uint8_t *dst,
                                                  const uint8_t *src,
                                                  ptrdiff_t stride);
void ff_hv_mc_qpel_no_rnd_aver_hv_src01_8x8_msa(uint8_t *dst,
                                                const uint8_t *src,
                                                ptrdiff_t stride);
void ff_hv_mc_qpel_no_rnd_aver_v_src1_16x16_msa(uint8_t *dst,
                                                const uint8_t *src,
                                                ptrdiff_t stride);
void ff_hv_mc_qpel_no_rnd_aver_v_src1_8x8_msa(uint8_t *dst,
                                              const uint8_t *src,
                                              ptrdiff_t stride);
void ff_hv_mc_qpel_no_rnd_aver_hv_src11_16x16_msa(uint8_t *dst,
                                                  const uint8_t *src,
                                                  ptrdiff_t stride);
void ff_hv_mc_qpel_no_rnd_aver_hv_src11_8x8_msa(uint8_t *dst,
                                                const uint8_t *src,
                                                ptrdiff_t stride);

#endif  // #ifndef AVCODEC_MIPS_QPELDSP_MIPS_H
