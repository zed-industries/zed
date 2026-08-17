/*
 * quarterpel DSP functions
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

/**
 * @file
 * quarterpel DSP functions
 */

#ifndef AVCODEC_QPELDSP_H
#define AVCODEC_QPELDSP_H

#include <stddef.h>
#include <stdint.h>

void ff_put_pixels8x8_c(uint8_t *dst, const uint8_t *src, ptrdiff_t stride);
void ff_avg_pixels8x8_c(uint8_t *dst, const uint8_t *src, ptrdiff_t stride);
void ff_put_pixels16x16_c(uint8_t *dst, const uint8_t *src, ptrdiff_t stride);
void ff_avg_pixels16x16_c(uint8_t *dst, const uint8_t *src, ptrdiff_t stride);

void ff_put_pixels8_l2_8(uint8_t *dst, const uint8_t *src1, const uint8_t *src2,
                         int dst_stride, int src_stride1, int src_stride2,
                         int h);

#define DEF_OLD_QPEL(name)                                              \
void ff_put_        ## name(uint8_t *dst /* align width (8 or 16) */,   \
                            const uint8_t *src /* align 1 */,           \
                            ptrdiff_t stride);                          \
void ff_put_no_rnd_ ## name(uint8_t *dst /* align width (8 or 16) */,   \
                            const uint8_t *src /* align 1 */,           \
                            ptrdiff_t stride);                          \
void ff_avg_        ## name(uint8_t *dst /* align width (8 or 16) */,   \
                            const uint8_t *src /* align 1 */,           \
                            ptrdiff_t stride);

DEF_OLD_QPEL(qpel16_mc11_old_c)
DEF_OLD_QPEL(qpel16_mc31_old_c)
DEF_OLD_QPEL(qpel16_mc12_old_c)
DEF_OLD_QPEL(qpel16_mc32_old_c)
DEF_OLD_QPEL(qpel16_mc13_old_c)
DEF_OLD_QPEL(qpel16_mc33_old_c)
DEF_OLD_QPEL(qpel8_mc11_old_c)
DEF_OLD_QPEL(qpel8_mc31_old_c)
DEF_OLD_QPEL(qpel8_mc12_old_c)
DEF_OLD_QPEL(qpel8_mc32_old_c)
DEF_OLD_QPEL(qpel8_mc13_old_c)
DEF_OLD_QPEL(qpel8_mc33_old_c)

typedef void (*qpel_mc_func)(uint8_t *dst /* align width (8 or 16) */,
                             const uint8_t *src /* align 1 */,
                             ptrdiff_t stride);

/**
 * quarterpel DSP context
 */
typedef struct QpelDSPContext {
    qpel_mc_func put_qpel_pixels_tab[2][16];
    qpel_mc_func avg_qpel_pixels_tab[2][16];
    qpel_mc_func put_no_rnd_qpel_pixels_tab[2][16];
} QpelDSPContext;

void ff_qpeldsp_init(QpelDSPContext *c);

void ff_qpeldsp_init_x86(QpelDSPContext *c);
void ff_qpeldsp_init_mips(QpelDSPContext *c);

#endif /* AVCODEC_QPELDSP_H */
