/*
 * VP9 compatible video decoder
 *
 * Copyright (C) 2013 Ronald S. Bultje <rsbultje gmail com>
 * Copyright (C) 2013 Clément Bœsch <u pkh me>
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

#ifndef AVCODEC_VP9DSP_H
#define AVCODEC_VP9DSP_H

#include <stddef.h>
#include <stdint.h>

#include "libavcodec/vp9.h"
#include "libavutil/attributes_internal.h"

typedef void (*vp9_mc_func)(uint8_t *dst, ptrdiff_t dst_stride,
                            const uint8_t *ref, ptrdiff_t ref_stride,
                            int h, int mx, int my);
typedef void (*vp9_scaled_mc_func)(uint8_t *dst, ptrdiff_t dst_stride,
                                   const uint8_t *ref, ptrdiff_t ref_stride,
                                   int h, int mx, int my, int dx, int dy);

typedef struct VP9DSPContext {
    /*
     * dimension 1: 0=4x4, 1=8x8, 2=16x16, 3=32x32
     * dimension 2: intra prediction modes
     *
     * dst/left/top is aligned by transform-size (i.e. 4, 8, 16 or 32 pixels)
     * stride is aligned by 16 pixels
     * top[-1] is top/left; top[4,7] is top-right for 4x4
     */
    // FIXME(rbultje) maybe replace left/top pointers with HAVE_TOP/
    // HAVE_LEFT/HAVE_TOPRIGHT flags instead, and then handle it in-place?
    // also needs to fit in with what H.264/VP8/etc do
    void (*intra_pred[N_TXFM_SIZES][N_INTRA_PRED_MODES])(uint8_t *dst,
                                                         ptrdiff_t stride,
                                                         const uint8_t *left,
                                                         const uint8_t *top);

    /*
     * dimension 1: 0=4x4, 1=8x8, 2=16x16, 3=32x32, 4=lossless (3-4=dct only)
     * dimension 2: 0=dct/dct, 1=dct/adst, 2=adst/dct, 3=adst/adst
     *
     * dst is aligned by transform-size (i.e. 4, 8, 16 or 32 pixels)
     * stride is aligned by 16 pixels
     * block is 16-byte aligned
     * eob indicates the position (+1) of the last non-zero coefficient,
     * in scan-order. This can be used to write faster versions, e.g. a
     * dc-only 4x4/8x8/16x16/32x32, or a 4x4-only (eob<10) 8x8/16x16/32x32,
     * etc.
     */
    // FIXME also write idct_add_block() versions for whole (inter) pred
    // blocks, so we can do 2 4x4s at once
    void (*itxfm_add[N_TXFM_SIZES + 1][N_TXFM_TYPES])(uint8_t *dst,
                                                      ptrdiff_t stride,
                                                      int16_t *block, int eob);

    /*
     * dimension 1: width of filter (0=4, 1=8, 2=16)
     * dimension 2: 0=col-edge filter (h), 1=row-edge filter (v)
     *
     * dst/stride are aligned by 8
     */
    void (*loop_filter_8[3][2])(uint8_t *dst, ptrdiff_t stride,
                                int mb_lim, int lim, int hev_thr);

    /*
     * dimension 1: 0=col-edge filter (h), 1=row-edge filter (v)
     *
     * The width of filter is assumed to be 16; dst/stride are aligned by 16
     */
    void (*loop_filter_16[2])(uint8_t *dst, ptrdiff_t stride,
                              int mb_lim, int lim, int hev_thr);

    /*
     * dimension 1/2: width of filter (0=4, 1=8) for each filter half
     * dimension 3: 0=col-edge filter (h), 1=row-edge filter (v)
     *
     * dst/stride are aligned by operation size
     * this basically calls loop_filter[d1][d3][0](), followed by
     * loop_filter[d2][d3][0]() on the next 8 pixels
     * mb_lim/lim/hev_thr contain two values in the lowest two bytes of the
     * integer.
     */
    // FIXME perhaps a mix4 that operates on 32px (for AVX2)
    void (*loop_filter_mix2[2][2][2])(uint8_t *dst, ptrdiff_t stride,
                                      int mb_lim, int lim, int hev_thr);

    /*
     * dimension 1: hsize (0: 64, 1: 32, 2: 16, 3: 8, 4: 4)
     * dimension 2: filter type (0: smooth, 1: regular, 2: sharp, 3: bilin)
     * dimension 3: averaging type (0: put, 1: avg)
     * dimension 4: x subpel interpolation (0: none, 1: 8tap/bilin)
     * dimension 5: y subpel interpolation (0: none, 1: 8tap/bilin)
     *
     * dst/stride are aligned by hsize
     */
    vp9_mc_func mc[5][N_FILTERS][2][2][2];

    /*
     * for scalable MC, first 3 dimensions identical to above, the other two
     * don't exist since it changes per stepsize.
     */
    vp9_scaled_mc_func smc[5][N_FILTERS][2];
} VP9DSPContext;

EXTERN const int16_t ff_vp9_subpel_filters[3][16][8];

void ff_vp9dsp_init(VP9DSPContext *dsp, int bpp, int bitexact);

void ff_vp9dsp_init_8(VP9DSPContext *dsp);
void ff_vp9dsp_init_10(VP9DSPContext *dsp);
void ff_vp9dsp_init_12(VP9DSPContext *dsp);

void ff_vp9dsp_init_aarch64(VP9DSPContext *dsp, int bpp);
void ff_vp9dsp_init_arm(VP9DSPContext *dsp, int bpp);
void ff_vp9dsp_init_riscv(VP9DSPContext *dsp, int bpp, int bitexact);
void ff_vp9dsp_init_x86(VP9DSPContext *dsp, int bpp, int bitexact);
void ff_vp9dsp_init_mips(VP9DSPContext *dsp, int bpp);
void ff_vp9dsp_init_loongarch(VP9DSPContext *dsp, int bpp);

#endif /* AVCODEC_VP9DSP_H */
