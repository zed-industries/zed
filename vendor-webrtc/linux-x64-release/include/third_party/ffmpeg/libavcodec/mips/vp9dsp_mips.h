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

#ifndef AVCODEC_MIPS_VP9DSP_MIPS_H
#define AVCODEC_MIPS_VP9DSP_MIPS_H

#include <stddef.h>
#include <stdint.h>

#define VP9_8TAP_MIPS_MSA_FUNC(SIZE, type, type_idx)                         \
void ff_put_8tap_##type##_##SIZE##h_msa(uint8_t *dst, ptrdiff_t dststride,   \
                                        const uint8_t *src,                  \
                                        ptrdiff_t srcstride,                 \
                                        int h, int mx, int my);              \
                                                                             \
void ff_put_8tap_##type##_##SIZE##v_msa(uint8_t *dst, ptrdiff_t dststride,   \
                                        const uint8_t *src,                  \
                                        ptrdiff_t srcstride,                 \
                                        int h, int mx, int my);              \
                                                                             \
void ff_put_8tap_##type##_##SIZE##hv_msa(uint8_t *dst, ptrdiff_t dststride,  \
                                         const uint8_t *src,                 \
                                         ptrdiff_t srcstride,                \
                                         int h, int mx, int my);             \
                                                                             \
void ff_avg_8tap_##type##_##SIZE##h_msa(uint8_t *dst, ptrdiff_t dststride,   \
                                        const uint8_t *src,                  \
                                        ptrdiff_t srcstride,                 \
                                        int h, int mx, int my);              \
                                                                             \
void ff_avg_8tap_##type##_##SIZE##v_msa(uint8_t *dst, ptrdiff_t dststride,   \
                                        const uint8_t *src,                  \
                                        ptrdiff_t srcstride,                 \
                                        int h, int mx, int my);              \
                                                                             \
void ff_avg_8tap_##type##_##SIZE##hv_msa(uint8_t *dst, ptrdiff_t dststride,  \
                                         const uint8_t *src,                 \
                                         ptrdiff_t srcstride,                \
                                         int h, int mx, int my);

#define VP9_BILINEAR_MIPS_MSA_FUNC(SIZE)                                   \
void ff_put_bilin_##SIZE##h_msa(uint8_t *dst, ptrdiff_t dststride,         \
                                const uint8_t *src, ptrdiff_t srcstride,   \
                                int h, int mx, int my);                    \
                                                                           \
void ff_put_bilin_##SIZE##v_msa(uint8_t *dst, ptrdiff_t dststride,         \
                                const uint8_t *src, ptrdiff_t srcstride,   \
                                int h, int mx, int my);                    \
                                                                           \
void ff_put_bilin_##SIZE##hv_msa(uint8_t *dst, ptrdiff_t dststride,        \
                                 const uint8_t *src, ptrdiff_t srcstride,  \
                                 int h, int mx, int my);                   \
                                                                           \
void ff_avg_bilin_##SIZE##h_msa(uint8_t *dst, ptrdiff_t dststride,         \
                                const uint8_t *src, ptrdiff_t srcstride,   \
                                int h, int mx, int my);                    \
                                                                           \
void ff_avg_bilin_##SIZE##v_msa(uint8_t *dst, ptrdiff_t dststride,         \
                                const uint8_t *src, ptrdiff_t srcstride,   \
                                int h, int mx, int my);                    \
                                                                           \
void ff_avg_bilin_##SIZE##hv_msa(uint8_t *dst, ptrdiff_t dststride,        \
                                 const uint8_t *src, ptrdiff_t srcstride,  \
                                 int h, int mx, int my);

#define VP9_COPY_AVG_MIPS_MSA_FUNC(SIZE)                           \
void ff_copy##SIZE##_msa(uint8_t *dst, ptrdiff_t dststride,        \
                         const uint8_t *src, ptrdiff_t srcstride,  \
                         int h, int mx, int my);                   \
                                                                   \
void ff_avg##SIZE##_msa(uint8_t *dst, ptrdiff_t dststride,         \
                        const uint8_t *src, ptrdiff_t srcstride,   \
                        int h, int mx, int my);

VP9_8TAP_MIPS_MSA_FUNC(64, regular, FILTER_8TAP_REGULAR);
VP9_8TAP_MIPS_MSA_FUNC(32, regular, FILTER_8TAP_REGULAR);
VP9_8TAP_MIPS_MSA_FUNC(16, regular, FILTER_8TAP_REGULAR);
VP9_8TAP_MIPS_MSA_FUNC(8, regular, FILTER_8TAP_REGULAR);
VP9_8TAP_MIPS_MSA_FUNC(4, regular, FILTER_8TAP_REGULAR);

VP9_8TAP_MIPS_MSA_FUNC(64, sharp, FILTER_8TAP_SHARP);
VP9_8TAP_MIPS_MSA_FUNC(32, sharp, FILTER_8TAP_SHARP);
VP9_8TAP_MIPS_MSA_FUNC(16, sharp, FILTER_8TAP_SHARP);
VP9_8TAP_MIPS_MSA_FUNC(8, sharp, FILTER_8TAP_SHARP);
VP9_8TAP_MIPS_MSA_FUNC(4, sharp, FILTER_8TAP_SHARP);

VP9_8TAP_MIPS_MSA_FUNC(64, smooth, FILTER_8TAP_SMOOTH);
VP9_8TAP_MIPS_MSA_FUNC(32, smooth, FILTER_8TAP_SMOOTH);
VP9_8TAP_MIPS_MSA_FUNC(16, smooth, FILTER_8TAP_SMOOTH);
VP9_8TAP_MIPS_MSA_FUNC(8, smooth, FILTER_8TAP_SMOOTH);
VP9_8TAP_MIPS_MSA_FUNC(4, smooth, FILTER_8TAP_SMOOTH);

VP9_BILINEAR_MIPS_MSA_FUNC(64);
VP9_BILINEAR_MIPS_MSA_FUNC(32);
VP9_BILINEAR_MIPS_MSA_FUNC(16);
VP9_BILINEAR_MIPS_MSA_FUNC(8);
VP9_BILINEAR_MIPS_MSA_FUNC(4);

VP9_COPY_AVG_MIPS_MSA_FUNC(64);
VP9_COPY_AVG_MIPS_MSA_FUNC(32);
VP9_COPY_AVG_MIPS_MSA_FUNC(16);
VP9_COPY_AVG_MIPS_MSA_FUNC(8);
VP9_COPY_AVG_MIPS_MSA_FUNC(4);

#undef VP9_8TAP_MIPS_MSA_FUNC
#undef VP9_BILINEAR_MIPS_MSA_FUNC
#undef VP9_COPY_AVG_MIPS_MSA_FUNC

void ff_loop_filter_h_4_8_msa(uint8_t *dst, ptrdiff_t stride, int32_t e,
                              int32_t i, int32_t h);
void ff_loop_filter_h_8_8_msa(uint8_t *dst, ptrdiff_t stride, int32_t e,
                              int32_t i, int32_t h);
void ff_loop_filter_h_16_8_msa(uint8_t *dst, ptrdiff_t stride, int32_t e,
                               int32_t i, int32_t h);
void ff_loop_filter_v_4_8_msa(uint8_t *dst, ptrdiff_t stride, int32_t e,
                              int32_t i, int32_t h);
void ff_loop_filter_v_8_8_msa(uint8_t *dst, ptrdiff_t stride, int32_t e,
                              int32_t i, int32_t h);
void ff_loop_filter_v_16_8_msa(uint8_t *dst, ptrdiff_t stride, int32_t e,
                               int32_t i, int32_t h);
void ff_loop_filter_h_44_16_msa(uint8_t *dst, ptrdiff_t stride, int32_t e,
                                int32_t i, int32_t h);
void ff_loop_filter_h_88_16_msa(uint8_t *dst, ptrdiff_t stride, int32_t e,
                                int32_t i, int32_t h);
void ff_loop_filter_h_16_16_msa(uint8_t *dst, ptrdiff_t stride, int32_t e,
                                int32_t i, int32_t h);
void ff_loop_filter_v_44_16_msa(uint8_t *dst, ptrdiff_t stride, int32_t e,
                                int32_t i, int32_t h);
void ff_loop_filter_v_88_16_msa(uint8_t *dst, ptrdiff_t stride, int32_t e,
                                int32_t i, int32_t h);
void ff_loop_filter_v_16_16_msa(uint8_t *dst, ptrdiff_t stride, int32_t e,
                                int32_t i, int32_t h);
void ff_loop_filter_h_48_16_msa(uint8_t *dst, ptrdiff_t stride, int32_t e,
                                int32_t i, int32_t h);
void ff_loop_filter_h_84_16_msa(uint8_t *dst, ptrdiff_t stride, int32_t e,
                                int32_t i, int32_t h);
void ff_loop_filter_v_48_16_msa(uint8_t *dst, ptrdiff_t stride, int32_t e,
                                int32_t i, int32_t h);
void ff_loop_filter_v_84_16_msa(uint8_t *dst, ptrdiff_t stride, int32_t e,
                                int32_t i, int32_t h);
void ff_idct_idct_4x4_add_msa(uint8_t *dst, ptrdiff_t stride,
                              int16_t *block, int eob);
void ff_idct_idct_8x8_add_msa(uint8_t *dst, ptrdiff_t stride,
                              int16_t *block, int eob);
void ff_idct_idct_16x16_add_msa(uint8_t *dst, ptrdiff_t stride,
                                int16_t *block, int eob);
void ff_idct_idct_32x32_add_msa(uint8_t *dst, ptrdiff_t stride,
                                int16_t *block, int eob);
void ff_iadst_iadst_4x4_add_msa(uint8_t *dst, ptrdiff_t stride,
                                int16_t *block, int eob);
void ff_iadst_iadst_8x8_add_msa(uint8_t *dst, ptrdiff_t stride,
                                int16_t *block, int eob);
void ff_iadst_iadst_16x16_add_msa(uint8_t *dst, ptrdiff_t stride,
                                  int16_t *block, int eob);
void ff_iadst_idct_4x4_add_msa(uint8_t *dst, ptrdiff_t stride,
                               int16_t *block, int eob);
void ff_iadst_idct_8x8_add_msa(uint8_t *dst, ptrdiff_t stride,
                               int16_t *block, int eob);
void ff_iadst_idct_16x16_add_msa(uint8_t *dst, ptrdiff_t stride,
                                 int16_t *block, int eob);
void ff_idct_iadst_4x4_add_msa(uint8_t *pu8Dest, ptrdiff_t stride,
                               int16_t *block, int eob);
void ff_idct_iadst_8x8_add_msa(uint8_t *pu8Dest, ptrdiff_t stride,
                               int16_t *block, int eob);
void ff_idct_iadst_16x16_add_msa(uint8_t *pu8Dest, ptrdiff_t stride,
                                 int16_t *block, int eob);
void ff_iwht_iwht_4x4_add_msa(uint8_t *dst, ptrdiff_t stride,
                              int16_t *block, int eob);

void ff_vert_16x16_msa(uint8_t *dst, ptrdiff_t stride, const uint8_t *left,
                       const uint8_t *top);
void ff_vert_32x32_msa(uint8_t *dst, ptrdiff_t stride, const uint8_t *left,
                       const uint8_t *top);
void ff_hor_16x16_msa(uint8_t *dst, ptrdiff_t stride, const uint8_t *left,
                      const uint8_t *top);
void ff_hor_32x32_msa(uint8_t *dst, ptrdiff_t stride, const uint8_t *left,
                      const uint8_t *top);
void ff_dc_4x4_msa(uint8_t *dst, ptrdiff_t stride, const uint8_t *left,
                   const uint8_t *top);
void ff_dc_8x8_msa(uint8_t *dst, ptrdiff_t stride, const uint8_t *left,
                   const uint8_t *top);
void ff_dc_16x16_msa(uint8_t *dst, ptrdiff_t stride, const uint8_t *left,
                     const uint8_t *top);
void ff_dc_32x32_msa(uint8_t *dst, ptrdiff_t stride, const uint8_t *left,
                     const uint8_t *top);
void ff_dc_left_4x4_msa(uint8_t *dst, ptrdiff_t stride, const uint8_t *left,
                        const uint8_t *top);
void ff_dc_left_8x8_msa(uint8_t *dst, ptrdiff_t stride, const uint8_t *left,
                        const uint8_t *top);
void ff_dc_left_16x16_msa(uint8_t *dst, ptrdiff_t stride,
                          const uint8_t *left, const uint8_t *top);
void ff_dc_left_32x32_msa(uint8_t *dst, ptrdiff_t stride,
                          const uint8_t *left, const uint8_t *top);
void ff_dc_top_4x4_msa(uint8_t *dst, ptrdiff_t stride, const uint8_t *left,
                       const uint8_t *top);
void ff_dc_top_8x8_msa(uint8_t *dst, ptrdiff_t stride, const uint8_t *left,
                       const uint8_t *top);
void ff_dc_top_16x16_msa(uint8_t *dst, ptrdiff_t stride,
                         const uint8_t *left, const uint8_t *top);
void ff_dc_top_32x32_msa(uint8_t *dst, ptrdiff_t stride,
                         const uint8_t *left, const uint8_t *top);
void ff_dc_128_16x16_msa(uint8_t *dst, ptrdiff_t stride,
                         const uint8_t *left, const uint8_t *top);
void ff_dc_128_32x32_msa(uint8_t *dst, ptrdiff_t stride,
                         const uint8_t *left, const uint8_t *top);
void ff_dc_127_16x16_msa(uint8_t *dst, ptrdiff_t stride,
                         const uint8_t *left, const uint8_t *top);
void ff_dc_127_32x32_msa(uint8_t *dst, ptrdiff_t stride,
                         const uint8_t *left, const uint8_t *top);
void ff_dc_129_16x16_msa(uint8_t *dst, ptrdiff_t stride,
                         const uint8_t *left, const uint8_t *top);
void ff_dc_129_32x32_msa(uint8_t *dst, ptrdiff_t stride,
                         const uint8_t *left, const uint8_t *top);
void ff_tm_4x4_msa(uint8_t *dst, ptrdiff_t stride, const uint8_t *left,
                   const uint8_t *top);
void ff_tm_8x8_msa(uint8_t *dst, ptrdiff_t stride, const uint8_t *left,
                   const uint8_t *top);
void ff_tm_16x16_msa(uint8_t *dst, ptrdiff_t stride, const uint8_t *left,
                     const uint8_t *top);
void ff_tm_32x32_msa(uint8_t *dst, ptrdiff_t stride, const uint8_t *left,
                     const uint8_t *top);

#define VP9_8TAP_MIPS_MMI_FUNC(SIZE, type, type_idx)                         \
void ff_put_8tap_##type##_##SIZE##h_mmi(uint8_t *dst, ptrdiff_t dststride,   \
                                        const uint8_t *src,                  \
                                        ptrdiff_t srcstride,                 \
                                        int h, int mx, int my);              \
                                                                             \
void ff_put_8tap_##type##_##SIZE##v_mmi(uint8_t *dst, ptrdiff_t dststride,   \
                                        const uint8_t *src,                  \
                                        ptrdiff_t srcstride,                 \
                                        int h, int mx, int my);              \
                                                                             \
void ff_put_8tap_##type##_##SIZE##hv_mmi(uint8_t *dst, ptrdiff_t dststride,  \
                                         const uint8_t *src,                 \
                                         ptrdiff_t srcstride,                \
                                         int h, int mx, int my);             \
                                                                             \
void ff_avg_8tap_##type##_##SIZE##h_mmi(uint8_t *dst, ptrdiff_t dststride,   \
                                        const uint8_t *src,                  \
                                        ptrdiff_t srcstride,                 \
                                        int h, int mx, int my);              \
                                                                             \
void ff_avg_8tap_##type##_##SIZE##v_mmi(uint8_t *dst, ptrdiff_t dststride,   \
                                        const uint8_t *src,                  \
                                        ptrdiff_t srcstride,                 \
                                        int h, int mx, int my);              \
                                                                             \
void ff_avg_8tap_##type##_##SIZE##hv_mmi(uint8_t *dst, ptrdiff_t dststride,  \
                                         const uint8_t *src,                 \
                                         ptrdiff_t srcstride,                \
                                         int h, int mx, int my);

VP9_8TAP_MIPS_MMI_FUNC(64, regular, FILTER_8TAP_REGULAR);
VP9_8TAP_MIPS_MMI_FUNC(32, regular, FILTER_8TAP_REGULAR);
VP9_8TAP_MIPS_MMI_FUNC(16, regular, FILTER_8TAP_REGULAR);
VP9_8TAP_MIPS_MMI_FUNC(8, regular, FILTER_8TAP_REGULAR);
VP9_8TAP_MIPS_MMI_FUNC(4, regular, FILTER_8TAP_REGULAR);

VP9_8TAP_MIPS_MMI_FUNC(64, sharp, FILTER_8TAP_SHARP);
VP9_8TAP_MIPS_MMI_FUNC(32, sharp, FILTER_8TAP_SHARP);
VP9_8TAP_MIPS_MMI_FUNC(16, sharp, FILTER_8TAP_SHARP);
VP9_8TAP_MIPS_MMI_FUNC(8, sharp, FILTER_8TAP_SHARP);
VP9_8TAP_MIPS_MMI_FUNC(4, sharp, FILTER_8TAP_SHARP);

VP9_8TAP_MIPS_MMI_FUNC(64, smooth, FILTER_8TAP_SMOOTH);
VP9_8TAP_MIPS_MMI_FUNC(32, smooth, FILTER_8TAP_SMOOTH);
VP9_8TAP_MIPS_MMI_FUNC(16, smooth, FILTER_8TAP_SMOOTH);
VP9_8TAP_MIPS_MMI_FUNC(8, smooth, FILTER_8TAP_SMOOTH);
VP9_8TAP_MIPS_MMI_FUNC(4, smooth, FILTER_8TAP_SMOOTH);
#undef VP9_8TAP_MIPS_MMI_FUNC

#endif  // #ifndef AVCODEC_MIPS_VP9DSP_MIPS_H
