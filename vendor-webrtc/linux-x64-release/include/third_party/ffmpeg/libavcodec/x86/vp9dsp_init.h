/*
 * VP9 SIMD optimizations
 *
 * Copyright (c) 2013 Ronald S. Bultje <rsbultje gmail com>
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

#ifndef AVCODEC_X86_VP9DSP_INIT_H
#define AVCODEC_X86_VP9DSP_INIT_H

#include "libavutil/attributes.h"
#include "libavutil/mem_internal.h"

#include "libavcodec/vp9dsp.h"

// hack to force-expand BPC
#define cat(a, bpp, b) a##bpp##b

#define decl_fpel_func(avg, sz, bpp, opt) \
void ff_vp9_##avg##sz##bpp##_##opt(uint8_t *dst, ptrdiff_t dst_stride, \
                                   const uint8_t *src, ptrdiff_t src_stride, \
                                   int h, int mx, int my)

#define decl_mc_func(avg, sz, dir, opt, type, f_sz, bpp) \
void ff_vp9_##avg##_8tap_1d_##dir##_##sz##_##bpp##_##opt(uint8_t *dst, ptrdiff_t dst_stride, \
                                                         const uint8_t *src, ptrdiff_t src_stride, \
                                                         int h, const type (*filter)[f_sz])

#define decl_mc_funcs(sz, opt, type, fsz, bpp) \
decl_mc_func(put, sz, h, opt, type, fsz, bpp); \
decl_mc_func(avg, sz, h, opt, type, fsz, bpp); \
decl_mc_func(put, sz, v, opt, type, fsz, bpp); \
decl_mc_func(avg, sz, v, opt, type, fsz, bpp)

#define decl_ipred_fn(type, sz, bpp, opt) \
void ff_vp9_ipred_##type##_##sz##x##sz##_##bpp##_##opt(uint8_t *dst, \
                                                       ptrdiff_t stride, \
                                                       const uint8_t *l, \
                                                       const uint8_t *a)

#define decl_ipred_fns(type, bpp, opt4, opt8_16_32) \
decl_ipred_fn(type,  4, bpp, opt4); \
decl_ipred_fn(type,  8, bpp, opt8_16_32); \
decl_ipred_fn(type, 16, bpp, opt8_16_32); \
decl_ipred_fn(type, 32, bpp, opt8_16_32)

#define decl_itxfm_func(typea, typeb, size, bpp, opt) \
void cat(ff_vp9_##typea##_##typeb##_##size##x##size##_add_, bpp, _##opt)(uint8_t *dst, \
                                                                         ptrdiff_t stride, \
                                                                         int16_t *block, \
                                                                         int eob)

#define decl_itxfm_funcs(size, bpp, opt) \
decl_itxfm_func(idct,  idct,  size, bpp, opt); \
decl_itxfm_func(iadst, idct,  size, bpp, opt); \
decl_itxfm_func(idct,  iadst, size, bpp, opt); \
decl_itxfm_func(iadst, iadst, size, bpp, opt)

#define mc_rep_func(avg, sz, hsz, hszb, dir, opt, type, f_sz, bpp) \
static av_always_inline void \
ff_vp9_##avg##_8tap_1d_##dir##_##sz##_##bpp##_##opt(uint8_t *dst, ptrdiff_t dst_stride, \
                                                    const uint8_t *src, ptrdiff_t src_stride, \
                                                    int h, const type (*filter)[f_sz]) \
{ \
    ff_vp9_##avg##_8tap_1d_##dir##_##hsz##_##bpp##_##opt(dst,        dst_stride, src, \
                                                         src_stride, h, filter); \
    ff_vp9_##avg##_8tap_1d_##dir##_##hsz##_##bpp##_##opt(dst + hszb, dst_stride, src + hszb, \
                                                         src_stride, h, filter); \
}

#define mc_rep_funcs(sz, hsz, hszb, opt, type, fsz, bpp) \
mc_rep_func(put, sz, hsz, hszb, h, opt, type, fsz, bpp) \
mc_rep_func(avg, sz, hsz, hszb, h, opt, type, fsz, bpp) \
mc_rep_func(put, sz, hsz, hszb, v, opt, type, fsz, bpp) \
mc_rep_func(avg, sz, hsz, hszb, v, opt, type, fsz, bpp)

#define filter_8tap_1d_fn(op, sz, f, f_opt, fname, dir, dvar, bpp, opt) \
static void op##_8tap_##fname##_##sz##dir##_##bpp##_##opt(uint8_t *dst, ptrdiff_t dst_stride, \
                                                          const uint8_t *src, ptrdiff_t src_stride, \
                                                          int h, int mx, int my) \
{ \
    ff_vp9_##op##_8tap_1d_##dir##_##sz##_##bpp##_##opt(dst, dst_stride, src, src_stride, \
                                                       h, ff_filters_##f_opt[f][dvar - 1]); \
}

#define filters_8tap_1d_fn(op, sz, dir, dvar, bpp, opt, f_opt) \
filter_8tap_1d_fn(op, sz, FILTER_8TAP_REGULAR, f_opt, regular, dir, dvar, bpp, opt) \
filter_8tap_1d_fn(op, sz, FILTER_8TAP_SHARP,   f_opt, sharp,   dir, dvar, bpp, opt) \
filter_8tap_1d_fn(op, sz, FILTER_8TAP_SMOOTH,  f_opt, smooth,  dir, dvar, bpp, opt)

#define filters_8tap_1d_fn2(op, sz, bpp, opt, f_opt) \
filters_8tap_1d_fn(op, sz, h, mx, bpp, opt, f_opt) \
filters_8tap_1d_fn(op, sz, v, my, bpp, opt, f_opt)

#define filters_8tap_1d_fn3(op, bpp, opt4, opt8, f_opt) \
filters_8tap_1d_fn2(op, 64, bpp, opt8, f_opt) \
filters_8tap_1d_fn2(op, 32, bpp, opt8, f_opt) \
filters_8tap_1d_fn2(op, 16, bpp, opt8, f_opt) \
filters_8tap_1d_fn2(op, 8, bpp, opt8, f_opt) \
filters_8tap_1d_fn2(op, 4, bpp, opt4, f_opt)

#define filter_8tap_2d_fn(op, sz, f, f_opt, fname, align, bpp, bytes, opt) \
static void op##_8tap_##fname##_##sz##hv_##bpp##_##opt(uint8_t *dst, ptrdiff_t dst_stride, \
                                                       const uint8_t *src, ptrdiff_t src_stride, \
                                                       int h, int mx, int my) \
{ \
    LOCAL_ALIGNED_##align(uint8_t, temp, [71 * 64 * bytes]); \
    ff_vp9_put_8tap_1d_h_##sz##_##bpp##_##opt(temp, 64 * bytes, src - 3 * src_stride, \
                                              src_stride,  h + 7, \
                                              ff_filters_##f_opt[f][mx - 1]); \
    ff_vp9_##op##_8tap_1d_v_##sz##_##bpp##_##opt(dst, dst_stride, temp + 3 * bytes * 64, \
                                                 64 * bytes, h, \
                                                 ff_filters_##f_opt[f][my - 1]); \
}

#define filters_8tap_2d_fn(op, sz, align, bpp, bytes, opt, f_opt) \
filter_8tap_2d_fn(op, sz, FILTER_8TAP_REGULAR, f_opt, regular, align, bpp, bytes, opt) \
filter_8tap_2d_fn(op, sz, FILTER_8TAP_SHARP,   f_opt, sharp, align, bpp, bytes, opt) \
filter_8tap_2d_fn(op, sz, FILTER_8TAP_SMOOTH,  f_opt, smooth, align, bpp, bytes, opt)

#define filters_8tap_2d_fn2(op, align, bpp, bytes, opt4, opt8, f_opt) \
filters_8tap_2d_fn(op, 64, align, bpp, bytes, opt8, f_opt) \
filters_8tap_2d_fn(op, 32, align, bpp, bytes, opt8, f_opt) \
filters_8tap_2d_fn(op, 16, align, bpp, bytes, opt8, f_opt) \
filters_8tap_2d_fn(op, 8, align, bpp, bytes, opt8, f_opt) \
filters_8tap_2d_fn(op, 4, align, bpp, bytes, opt4, f_opt)

#define init_fpel_func(idx1, idx2, sz, type, bpp, opt) \
    dsp->mc[idx1][FILTER_8TAP_SMOOTH ][idx2][0][0] = \
    dsp->mc[idx1][FILTER_8TAP_REGULAR][idx2][0][0] = \
    dsp->mc[idx1][FILTER_8TAP_SHARP  ][idx2][0][0] = \
    dsp->mc[idx1][FILTER_BILINEAR    ][idx2][0][0] = ff_vp9_##type##sz##bpp##_##opt

#define init_subpel1(idx1, idx2, idxh, idxv, sz, dir, type, bpp, opt) \
    dsp->mc[idx1][FILTER_8TAP_SMOOTH ][idx2][idxh][idxv] = \
        type##_8tap_smooth_##sz##dir##_##bpp##_##opt; \
    dsp->mc[idx1][FILTER_8TAP_REGULAR][idx2][idxh][idxv] = \
        type##_8tap_regular_##sz##dir##_##bpp##_##opt; \
    dsp->mc[idx1][FILTER_8TAP_SHARP  ][idx2][idxh][idxv] = \
        type##_8tap_sharp_##sz##dir##_##bpp##_##opt

#define init_subpel2(idx1, idx2, sz, type, bpp, opt) \
    init_subpel1(idx1, idx2, 1, 1, sz, hv, type, bpp, opt); \
    init_subpel1(idx1, idx2, 0, 1, sz, v,  type, bpp, opt); \
    init_subpel1(idx1, idx2, 1, 0, sz, h,  type, bpp, opt)

#define init_subpel3_32_64(idx, type, bpp, opt) \
    init_subpel2(0, idx, 64, type, bpp, opt); \
    init_subpel2(1, idx, 32, type, bpp, opt)

#define init_subpel3_8to64(idx, type, bpp, opt) \
    init_subpel3_32_64(idx, type, bpp, opt); \
    init_subpel2(2, idx, 16, type, bpp, opt); \
    init_subpel2(3, idx,  8, type, bpp, opt)

#define init_subpel3(idx, type, bpp, opt) \
    init_subpel3_8to64(idx, type, bpp, opt); \
    init_subpel2(4, idx,  4, type, bpp, opt)

#define init_ipred_func(type, enum, sz, bpp, opt) \
    dsp->intra_pred[TX_##sz##X##sz][enum##_PRED] = \
        cat(ff_vp9_ipred_##type##_##sz##x##sz##_, bpp, _##opt)

#define init_8_16_32_ipred_funcs(type, enum, bpp, opt) \
    init_ipred_func(type, enum,  8, bpp, opt); \
    init_ipred_func(type, enum, 16, bpp, opt); \
    init_ipred_func(type, enum, 32, bpp, opt)

#define init_ipred_funcs(type, enum, bpp, opt) \
    init_ipred_func(type, enum,  4, bpp, opt); \
    init_8_16_32_ipred_funcs(type, enum, bpp, opt)

void ff_vp9dsp_init_10bpp_x86(VP9DSPContext *dsp, int bitexact);
void ff_vp9dsp_init_12bpp_x86(VP9DSPContext *dsp, int bitexact);
void ff_vp9dsp_init_16bpp_x86(VP9DSPContext *dsp);

#endif /* AVCODEC_X86_VP9DSP_INIT_H */
