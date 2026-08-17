/*
 * Copyright (c) 2024 Institue of Software Chinese Academy of Sciences (ISCAS).
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

#ifndef AVCODEC_RISCV_VP9DSP_H
#define AVCODEC_RISCV_VP9DSP_H

#include <stddef.h>
#include <stdint.h>

void ff_dc_32x32_rvv(uint8_t *dst, ptrdiff_t stride, const uint8_t *l,
                     const uint8_t *a);
void ff_dc_16x16_rvv(uint8_t *dst, ptrdiff_t stride, const uint8_t *l,
                     const uint8_t *a);
void ff_dc_8x8_rvv(uint8_t *dst, ptrdiff_t stride, const uint8_t *l,
                   const uint8_t *a);
void ff_dc_top_32x32_rvv(uint8_t *dst, ptrdiff_t stride, const uint8_t *l,
                         const uint8_t *a);
void ff_dc_top_16x16_rvv(uint8_t *dst, ptrdiff_t stride, const uint8_t *l,
                         const uint8_t *a);
void ff_dc_top_8x8_rvv(uint8_t *dst, ptrdiff_t stride, const uint8_t *l,
                       const uint8_t *a);
void ff_dc_left_32x32_rvv(uint8_t *dst, ptrdiff_t stride, const uint8_t *l,
                          const uint8_t *a);
void ff_dc_left_16x16_rvv(uint8_t *dst, ptrdiff_t stride, const uint8_t *l,
                          const uint8_t *a);
void ff_dc_left_8x8_rvv(uint8_t *dst, ptrdiff_t stride, const uint8_t *l,
                        const uint8_t *a);
void ff_dc_127_32x32_rvv(uint8_t *dst, ptrdiff_t stride, const uint8_t *l,
                         const uint8_t *a);
void ff_dc_127_16x16_rvv(uint8_t *dst, ptrdiff_t stride, const uint8_t *l,
                         const uint8_t *a);
void ff_dc_127_8x8_rvv(uint8_t *dst, ptrdiff_t stride, const uint8_t *l,
                       const uint8_t *a);
void ff_dc_128_32x32_rvv(uint8_t *dst, ptrdiff_t stride, const uint8_t *l,
                         const uint8_t *a);
void ff_dc_128_16x16_rvv(uint8_t *dst, ptrdiff_t stride, const uint8_t *l,
                         const uint8_t *a);
void ff_dc_128_8x8_rvv(uint8_t *dst, ptrdiff_t stride, const uint8_t *l,
                       const uint8_t *a);
void ff_dc_129_32x32_rvv(uint8_t *dst, ptrdiff_t stride, const uint8_t *l,
                         const uint8_t *a);
void ff_dc_129_16x16_rvv(uint8_t *dst, ptrdiff_t stride, const uint8_t *l,
                         const uint8_t *a);
void ff_dc_129_8x8_rvv(uint8_t *dst, ptrdiff_t stride, const uint8_t *l,
                       const uint8_t *a);
void ff_h_32x32_rvv(uint8_t *dst, ptrdiff_t stride, const uint8_t *l,
                    const uint8_t *a);
void ff_h_16x16_rvv(uint8_t *dst, ptrdiff_t stride, const uint8_t *l,
                    const uint8_t *a);
void ff_h_8x8_rvv(uint8_t *dst, ptrdiff_t stride, const uint8_t *l,
                  const uint8_t *a);
void ff_tm_32x32_rvv(uint8_t *dst, ptrdiff_t stride, const uint8_t *l,
                     const uint8_t *a);
void ff_tm_16x16_rvv(uint8_t *dst, ptrdiff_t stride, const uint8_t *l,
                     const uint8_t *a);
void ff_tm_8x8_rvv(uint8_t *dst, ptrdiff_t stride, const uint8_t *l,
                   const uint8_t *a);
void ff_tm_4x4_rvv(uint8_t *dst, ptrdiff_t stride, const uint8_t *l,
                   const uint8_t *a);

#define VP9_8TAP_RISCV_RVV_FUNC(SIZE, type, type_idx)                         \
void ff_put_8tap_##type##_##SIZE##h_rvv(uint8_t *dst, ptrdiff_t dststride,   \
                                        const uint8_t *src,                  \
                                        ptrdiff_t srcstride,                 \
                                        int h, int mx, int my);              \
                                                                             \
void ff_put_8tap_##type##_##SIZE##v_rvv(uint8_t *dst, ptrdiff_t dststride,   \
                                        const uint8_t *src,                  \
                                        ptrdiff_t srcstride,                 \
                                        int h, int mx, int my);              \
                                                                             \
void ff_put_8tap_##type##_##SIZE##hv_rvv(uint8_t *dst, ptrdiff_t dststride,  \
                                         const uint8_t *src,                 \
                                         ptrdiff_t srcstride,                \
                                         int h, int mx, int my);             \
                                                                             \
void ff_avg_8tap_##type##_##SIZE##h_rvv(uint8_t *dst, ptrdiff_t dststride,   \
                                        const uint8_t *src,                  \
                                        ptrdiff_t srcstride,                 \
                                        int h, int mx, int my);              \
                                                                             \
void ff_avg_8tap_##type##_##SIZE##v_rvv(uint8_t *dst, ptrdiff_t dststride,   \
                                        const uint8_t *src,                  \
                                        ptrdiff_t srcstride,                 \
                                        int h, int mx, int my);              \
                                                                             \
void ff_avg_8tap_##type##_##SIZE##hv_rvv(uint8_t *dst, ptrdiff_t dststride,  \
                                         const uint8_t *src,                 \
                                         ptrdiff_t srcstride,                \
                                         int h, int mx, int my);

#define VP9_BILINEAR_RISCV_RVV_FUNC(SIZE)                                   \
void ff_put_vp9_bilin_##SIZE##h_rvv(uint8_t *dst, ptrdiff_t dststride,     \
                                const uint8_t *src, ptrdiff_t srcstride,   \
                                int h, int mx, int my);                    \
                                                                           \
void ff_put_vp9_bilin_##SIZE##v_rvv(uint8_t *dst, ptrdiff_t dststride,     \
                                const uint8_t *src, ptrdiff_t srcstride,   \
                                int h, int mx, int my);                    \
                                                                           \
void ff_put_vp9_bilin_##SIZE##hv_rvv(uint8_t *dst, ptrdiff_t dststride,    \
                                 const uint8_t *src, ptrdiff_t srcstride,  \
                                 int h, int mx, int my);                   \
                                                                           \
void ff_avg_vp9_bilin_##SIZE##h_rvv(uint8_t *dst, ptrdiff_t dststride,     \
                                const uint8_t *src, ptrdiff_t srcstride,   \
                                int h, int mx, int my);                    \
                                                                           \
void ff_avg_vp9_bilin_##SIZE##v_rvv(uint8_t *dst, ptrdiff_t dststride,     \
                                const uint8_t *src, ptrdiff_t srcstride,   \
                                int h, int mx, int my);                    \
                                                                           \
void ff_avg_vp9_bilin_##SIZE##hv_rvv(uint8_t *dst, ptrdiff_t dststride,    \
                                 const uint8_t *src, ptrdiff_t srcstride,  \
                                 int h, int mx, int my);

#define VP9_COPY_AVG_RISCV_RVV_FUNC(SIZE)                           \
void ff_vp9_copy##SIZE##_rvv(uint8_t *dst, ptrdiff_t dststride,    \
                         const uint8_t *src, ptrdiff_t srcstride,  \
                         int h, int mx, int my);                   \
                                                                   \
void ff_vp9_avg##SIZE##_rvv(uint8_t *dst, ptrdiff_t dststride,     \
                        const uint8_t *src, ptrdiff_t srcstride,   \
                        int h, int mx, int my);

VP9_8TAP_RISCV_RVV_FUNC(64, regular, FILTER_8TAP_REGULAR);
VP9_8TAP_RISCV_RVV_FUNC(32, regular, FILTER_8TAP_REGULAR);
VP9_8TAP_RISCV_RVV_FUNC(16, regular, FILTER_8TAP_REGULAR);
VP9_8TAP_RISCV_RVV_FUNC(8, regular, FILTER_8TAP_REGULAR);
VP9_8TAP_RISCV_RVV_FUNC(4, regular, FILTER_8TAP_REGULAR);

VP9_8TAP_RISCV_RVV_FUNC(64, sharp, FILTER_8TAP_SHARP);
VP9_8TAP_RISCV_RVV_FUNC(32, sharp, FILTER_8TAP_SHARP);
VP9_8TAP_RISCV_RVV_FUNC(16, sharp, FILTER_8TAP_SHARP);
VP9_8TAP_RISCV_RVV_FUNC(8, sharp, FILTER_8TAP_SHARP);
VP9_8TAP_RISCV_RVV_FUNC(4, sharp, FILTER_8TAP_SHARP);

VP9_8TAP_RISCV_RVV_FUNC(64, smooth, FILTER_8TAP_SMOOTH);
VP9_8TAP_RISCV_RVV_FUNC(32, smooth, FILTER_8TAP_SMOOTH);
VP9_8TAP_RISCV_RVV_FUNC(16, smooth, FILTER_8TAP_SMOOTH);
VP9_8TAP_RISCV_RVV_FUNC(8, smooth, FILTER_8TAP_SMOOTH);
VP9_8TAP_RISCV_RVV_FUNC(4, smooth, FILTER_8TAP_SMOOTH);

VP9_BILINEAR_RISCV_RVV_FUNC(64);
VP9_BILINEAR_RISCV_RVV_FUNC(32);
VP9_BILINEAR_RISCV_RVV_FUNC(16);
VP9_BILINEAR_RISCV_RVV_FUNC(8);
VP9_BILINEAR_RISCV_RVV_FUNC(4);

VP9_COPY_AVG_RISCV_RVV_FUNC(64);
VP9_COPY_AVG_RISCV_RVV_FUNC(32);
VP9_COPY_AVG_RISCV_RVV_FUNC(16);
VP9_COPY_AVG_RISCV_RVV_FUNC(8);
VP9_COPY_AVG_RISCV_RVV_FUNC(4);

#define VP9_COPY_RISCV_RVI_FUNC(SIZE)                           \
void ff_copy##SIZE##_rvi(uint8_t *dst, ptrdiff_t dststride,        \
                         const uint8_t *src, ptrdiff_t srcstride,  \
                         int h, int mx, int my);

VP9_COPY_RISCV_RVI_FUNC(64);
VP9_COPY_RISCV_RVI_FUNC(32);
VP9_COPY_RISCV_RVI_FUNC(16);
VP9_COPY_RISCV_RVI_FUNC(8);
VP9_COPY_RISCV_RVI_FUNC(4);

#undef VP9_8TAP_RISCV_RVV_FUNC
#undef VP9_BILINEAR_RISCV_RVV_FUNC
#undef VP9_COPY_AVG_RISCV_RVV_FUNC

#endif  // #ifndef AVCODEC_RISCV_VP9DSP_H
