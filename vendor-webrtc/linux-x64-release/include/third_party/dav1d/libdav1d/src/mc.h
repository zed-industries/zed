/*
 * Copyright © 2018, VideoLAN and dav1d authors
 * Copyright © 2018, Two Orioles, LLC
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef DAV1D_SRC_MC_H
#define DAV1D_SRC_MC_H

#include <stdint.h>
#include <stddef.h>

#include "common/bitdepth.h"

#include "src/levels.h"

#define decl_mc_fn(name) \
void (name)(pixel *dst, ptrdiff_t dst_stride, \
            const pixel *src, ptrdiff_t src_stride, \
            int w, int h, int mx, int my HIGHBD_DECL_SUFFIX)
typedef decl_mc_fn(*mc_fn);

#define decl_mc_scaled_fn(name) \
void (name)(pixel *dst, ptrdiff_t dst_stride, \
            const pixel *src, ptrdiff_t src_stride, \
            int w, int h, int mx, int my, int dx, int dy HIGHBD_DECL_SUFFIX)
typedef decl_mc_scaled_fn(*mc_scaled_fn);

#define decl_warp8x8_fn(name) \
void (name)(pixel *dst, ptrdiff_t dst_stride, \
            const pixel *src, ptrdiff_t src_stride, \
            const int16_t *abcd, int mx, int my HIGHBD_DECL_SUFFIX)
typedef decl_warp8x8_fn(*warp8x8_fn);

#define decl_mct_fn(name) \
void (name)(int16_t *tmp, const pixel *src, ptrdiff_t src_stride, \
            int w, int h, int mx, int my HIGHBD_DECL_SUFFIX)
typedef decl_mct_fn(*mct_fn);

#define decl_mct_scaled_fn(name) \
void (name)(int16_t *tmp, const pixel *src, ptrdiff_t src_stride, \
            int w, int h, int mx, int my, int dx, int dy HIGHBD_DECL_SUFFIX)
typedef decl_mct_scaled_fn(*mct_scaled_fn);

#define decl_warp8x8t_fn(name) \
void (name)(int16_t *tmp, const ptrdiff_t tmp_stride, \
            const pixel *src, ptrdiff_t src_stride, \
            const int16_t *abcd, int mx, int my HIGHBD_DECL_SUFFIX)
typedef decl_warp8x8t_fn(*warp8x8t_fn);

#define decl_avg_fn(name) \
void (name)(pixel *dst, ptrdiff_t dst_stride, \
            const int16_t *tmp1, const int16_t *tmp2, int w, int h \
            HIGHBD_DECL_SUFFIX)
typedef decl_avg_fn(*avg_fn);

#define decl_w_avg_fn(name) \
void (name)(pixel *dst, ptrdiff_t dst_stride, \
            const int16_t *tmp1, const int16_t *tmp2, int w, int h, int weight \
            HIGHBD_DECL_SUFFIX)
typedef decl_w_avg_fn(*w_avg_fn);

#define decl_mask_fn(name) \
void (name)(pixel *dst, ptrdiff_t dst_stride, \
            const int16_t *tmp1, const int16_t *tmp2, int w, int h, \
            const uint8_t *mask HIGHBD_DECL_SUFFIX)
typedef decl_mask_fn(*mask_fn);

#define decl_w_mask_fn(name) \
void (name)(pixel *dst, ptrdiff_t dst_stride, \
            const int16_t *tmp1, const int16_t *tmp2, int w, int h, \
            uint8_t *mask, int sign HIGHBD_DECL_SUFFIX)
typedef decl_w_mask_fn(*w_mask_fn);

#define decl_blend_fn(name) \
void (name)(pixel *dst, ptrdiff_t dst_stride, const pixel *tmp, \
            int w, int h, const uint8_t *mask)
typedef decl_blend_fn(*blend_fn);

#define decl_blend_dir_fn(name) \
void (name)(pixel *dst, ptrdiff_t dst_stride, const pixel *tmp, int w, int h)
typedef decl_blend_dir_fn(*blend_dir_fn);

#define decl_emu_edge_fn(name) \
void (name)(intptr_t bw, intptr_t bh, intptr_t iw, intptr_t ih, intptr_t x, intptr_t y, \
            pixel *dst, ptrdiff_t dst_stride, const pixel *src, ptrdiff_t src_stride)
typedef decl_emu_edge_fn(*emu_edge_fn);

#define decl_resize_fn(name) \
void (name)(pixel *dst, ptrdiff_t dst_stride, \
            const pixel *src, ptrdiff_t src_stride, \
            int dst_w, int h, int src_w, int dx, int mx HIGHBD_DECL_SUFFIX)
typedef decl_resize_fn(*resize_fn);

#define decl_8tap_gen(decl_name, fn_name, opt) \
    decl_##decl_name##_fn(BF(dav1d_##fn_name##_8tap_regular,        opt)); \
    decl_##decl_name##_fn(BF(dav1d_##fn_name##_8tap_regular_smooth, opt)); \
    decl_##decl_name##_fn(BF(dav1d_##fn_name##_8tap_regular_sharp,  opt)); \
    decl_##decl_name##_fn(BF(dav1d_##fn_name##_8tap_smooth_regular, opt)); \
    decl_##decl_name##_fn(BF(dav1d_##fn_name##_8tap_smooth,         opt)); \
    decl_##decl_name##_fn(BF(dav1d_##fn_name##_8tap_smooth_sharp,   opt)); \
    decl_##decl_name##_fn(BF(dav1d_##fn_name##_8tap_sharp_regular,  opt)); \
    decl_##decl_name##_fn(BF(dav1d_##fn_name##_8tap_sharp_smooth,   opt)); \
    decl_##decl_name##_fn(BF(dav1d_##fn_name##_8tap_sharp,          opt))

#define decl_8tap_fns(opt) \
    decl_8tap_gen(mc,  put,  opt); \
    decl_8tap_gen(mct, prep, opt)

#define init_8tap_gen(name, opt) \
    init_##name##_fn(FILTER_2D_8TAP_REGULAR,        8tap_regular,        opt); \
    init_##name##_fn(FILTER_2D_8TAP_REGULAR_SMOOTH, 8tap_regular_smooth, opt); \
    init_##name##_fn(FILTER_2D_8TAP_REGULAR_SHARP,  8tap_regular_sharp,  opt); \
    init_##name##_fn(FILTER_2D_8TAP_SMOOTH_REGULAR, 8tap_smooth_regular, opt); \
    init_##name##_fn(FILTER_2D_8TAP_SMOOTH,         8tap_smooth,         opt); \
    init_##name##_fn(FILTER_2D_8TAP_SMOOTH_SHARP,   8tap_smooth_sharp,   opt); \
    init_##name##_fn(FILTER_2D_8TAP_SHARP_REGULAR,  8tap_sharp_regular,  opt); \
    init_##name##_fn(FILTER_2D_8TAP_SHARP_SMOOTH,   8tap_sharp_smooth,   opt); \
    init_##name##_fn(FILTER_2D_8TAP_SHARP,          8tap_sharp,          opt)

#define init_8tap_fns(opt) \
    init_8tap_gen(mc,  opt); \
    init_8tap_gen(mct, opt)

typedef struct Dav1dMCDSPContext {
    mc_fn mc[N_2D_FILTERS];
    mc_scaled_fn mc_scaled[N_2D_FILTERS];
    mct_fn mct[N_2D_FILTERS];
    mct_scaled_fn mct_scaled[N_2D_FILTERS];
    avg_fn avg;
    w_avg_fn w_avg;
    mask_fn mask;
    w_mask_fn w_mask[3 /* 444, 422, 420 */];
    blend_fn blend;
    blend_dir_fn blend_v;
    blend_dir_fn blend_h;
    warp8x8_fn warp8x8;
    warp8x8t_fn warp8x8t;
    emu_edge_fn emu_edge;
    resize_fn resize;
} Dav1dMCDSPContext;

bitfn_decls(void dav1d_mc_dsp_init, Dav1dMCDSPContext *c);

#endif /* DAV1D_SRC_MC_H */
