/*
 * Copyright (C) 2010 David Conrad
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

#ifndef AVCODEC_DIRACDSP_H
#define AVCODEC_DIRACDSP_H

#include <stdint.h>
#include <stddef.h>

typedef void (*dirac_weight_func)(uint8_t *block, int stride, int log2_denom, int weight, int h);
typedef void (*dirac_biweight_func)(uint8_t *dst, const uint8_t *src, int stride, int log2_denom, int weightd, int weights, int h);

typedef struct {
    void (*dirac_hpel_filter)(uint8_t *dsth, uint8_t *dstv, uint8_t *dstc, const uint8_t *src, int stride, int width, int height);
    /**
     * dirac_pixels_tab[width][subpel]
     * width is 2 for 32, 1 for 16, 0 for 8
     * subpel is 0 for fpel and hpel (only need to copy from the first plane in src)
     *           1 if an average of the first 2 planes is needed (TODO: worth it?)
     *           2 for general qpel (avg of 4)
     *           3 for general epel (biweight of 4 using the weights in src[4])
     * src[0-3] is each of the hpel planes
     * src[4] is the 1/8 pel weights if needed
     */
    void (*put_dirac_pixels_tab[3][4])(uint8_t *dst, const uint8_t *src[5], int stride, int h);
    void (*avg_dirac_pixels_tab[3][4])(uint8_t *dst, const uint8_t *src[5], int stride, int h);

    void (*put_signed_rect_clamped[3])(uint8_t *dst/*align 16*/, int dst_stride, const uint8_t *src/*align 16*/, int src_stride, int width, int height/*mod 2*/);
    void (*put_rect_clamped)(uint8_t *dst/*align 16*/, int dst_stride, const uint8_t *src/*align 16*/, int src_stride, int width, int height/*mod 2*/);
    void (*add_rect_clamped)(uint8_t *dst/*align 16*/, const uint16_t *src/*align 16*/, int stride, const int16_t *idwt/*align 16*/, int idwt_stride, int width, int height/*mod 2*/);
    void (*add_dirac_obmc[3])(uint16_t *dst, const uint8_t *src, int stride, const uint8_t *obmc_weight, int yblen);

    /* 0-1: int16_t and int32_t asm/c, 2-3: int16 and int32_t, C only */
    void (*dequant_subband[4])(uint8_t *src, uint8_t *dst, ptrdiff_t stride, const int qf, const int qs, int tot_v, int tot_h);

    dirac_weight_func weight_dirac_pixels_tab[3];
    dirac_biweight_func biweight_dirac_pixels_tab[3];
} DiracDSPContext;

#define DECL_DIRAC_PIXOP(PFX, EXT)                                      \
    void ff_ ## PFX ## _dirac_pixels8_ ## EXT(uint8_t *dst, const uint8_t *src[5], int stride, int h); \
    void ff_ ## PFX ## _dirac_pixels16_ ## EXT(uint8_t *dst, const uint8_t *src[5], int stride, int h); \
    void ff_ ## PFX ## _dirac_pixels32_ ## EXT(uint8_t *dst, const uint8_t *src[5], int stride, int h)

DECL_DIRAC_PIXOP(put, c);
DECL_DIRAC_PIXOP(avg, c);
DECL_DIRAC_PIXOP(put, l2_c);
DECL_DIRAC_PIXOP(avg, l2_c);
DECL_DIRAC_PIXOP(put, l4_c);
DECL_DIRAC_PIXOP(avg, l4_c);

void ff_diracdsp_init(DiracDSPContext *c);
void ff_diracdsp_init_x86(DiracDSPContext* c);

#endif /* AVCODEC_DIRACDSP_H */
