/*
 * Copyright © 2018-2021, VideoLAN and dav1d authors
 * Copyright © 2018-2021, Two Orioles, LLC
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

#include "src/cpu.h"
#include "src/mc.h"

#define decl_fn(type, name) \
    decl_##type##_fn(BF(name, ssse3)); \
    decl_##type##_fn(BF(name, avx2)); \
    decl_##type##_fn(BF(name, avx512icl));
#define init_mc_fn(type, name, suffix) \
    c->mc[type] = BF(dav1d_put_##name, suffix)
#define init_mct_fn(type, name, suffix) \
    c->mct[type] = BF(dav1d_prep_##name, suffix)
#define init_mc_scaled_fn(type, name, suffix) \
    c->mc_scaled[type] = BF(dav1d_put_##name, suffix)
#define init_mct_scaled_fn(type, name, suffix) \
    c->mct_scaled[type] = BF(dav1d_prep_##name, suffix)

decl_8tap_fns(ssse3);
decl_8tap_fns(avx2);
decl_8tap_fns(avx512icl);

decl_fn(mc, dav1d_put_bilin);
decl_fn(mct, dav1d_prep_bilin);

decl_fn(mc_scaled, dav1d_put_8tap_scaled_regular);
decl_fn(mc_scaled, dav1d_put_8tap_scaled_regular_smooth);
decl_fn(mc_scaled, dav1d_put_8tap_scaled_regular_sharp);
decl_fn(mc_scaled, dav1d_put_8tap_scaled_smooth);
decl_fn(mc_scaled, dav1d_put_8tap_scaled_smooth_regular);
decl_fn(mc_scaled, dav1d_put_8tap_scaled_smooth_sharp);
decl_fn(mc_scaled, dav1d_put_8tap_scaled_sharp);
decl_fn(mc_scaled, dav1d_put_8tap_scaled_sharp_regular);
decl_fn(mc_scaled, dav1d_put_8tap_scaled_sharp_smooth);
decl_fn(mc_scaled, dav1d_put_bilin_scaled);

decl_fn(mct_scaled, dav1d_prep_8tap_scaled_regular);
decl_fn(mct_scaled, dav1d_prep_8tap_scaled_regular_smooth);
decl_fn(mct_scaled, dav1d_prep_8tap_scaled_regular_sharp);
decl_fn(mct_scaled, dav1d_prep_8tap_scaled_smooth);
decl_fn(mct_scaled, dav1d_prep_8tap_scaled_smooth_regular);
decl_fn(mct_scaled, dav1d_prep_8tap_scaled_smooth_sharp);
decl_fn(mct_scaled, dav1d_prep_8tap_scaled_sharp);
decl_fn(mct_scaled, dav1d_prep_8tap_scaled_sharp_regular);
decl_fn(mct_scaled, dav1d_prep_8tap_scaled_sharp_smooth);
decl_fn(mct_scaled, dav1d_prep_bilin_scaled);

decl_fn(avg, dav1d_avg);
decl_fn(w_avg, dav1d_w_avg);
decl_fn(mask, dav1d_mask);
decl_fn(w_mask, dav1d_w_mask_420);
decl_fn(w_mask, dav1d_w_mask_422);
decl_fn(w_mask, dav1d_w_mask_444);
decl_fn(blend, dav1d_blend);
decl_fn(blend_dir, dav1d_blend_v);
decl_fn(blend_dir, dav1d_blend_h);

decl_fn(warp8x8, dav1d_warp_affine_8x8);
decl_warp8x8_fn(BF(dav1d_warp_affine_8x8, sse4));
decl_fn(warp8x8t, dav1d_warp_affine_8x8t);
decl_warp8x8t_fn(BF(dav1d_warp_affine_8x8t, sse4));

decl_fn(emu_edge, dav1d_emu_edge);

decl_fn(resize, dav1d_resize);

static ALWAYS_INLINE void mc_dsp_init_x86(Dav1dMCDSPContext *const c) {
    const unsigned flags = dav1d_get_cpu_flags();

    if(!(flags & DAV1D_X86_CPU_FLAG_SSSE3))
        return;

    init_8tap_fns(ssse3);

    init_mc_fn(FILTER_2D_BILINEAR,             bilin,               ssse3);
    init_mct_fn(FILTER_2D_BILINEAR,            bilin,               ssse3);

    init_mc_scaled_fn(FILTER_2D_8TAP_REGULAR,        8tap_scaled_regular,        ssse3);
    init_mc_scaled_fn(FILTER_2D_8TAP_REGULAR_SMOOTH, 8tap_scaled_regular_smooth, ssse3);
    init_mc_scaled_fn(FILTER_2D_8TAP_REGULAR_SHARP,  8tap_scaled_regular_sharp,  ssse3);
    init_mc_scaled_fn(FILTER_2D_8TAP_SMOOTH_REGULAR, 8tap_scaled_smooth_regular, ssse3);
    init_mc_scaled_fn(FILTER_2D_8TAP_SMOOTH,         8tap_scaled_smooth,         ssse3);
    init_mc_scaled_fn(FILTER_2D_8TAP_SMOOTH_SHARP,   8tap_scaled_smooth_sharp,   ssse3);
    init_mc_scaled_fn(FILTER_2D_8TAP_SHARP_REGULAR,  8tap_scaled_sharp_regular,  ssse3);
    init_mc_scaled_fn(FILTER_2D_8TAP_SHARP_SMOOTH,   8tap_scaled_sharp_smooth,   ssse3);
    init_mc_scaled_fn(FILTER_2D_8TAP_SHARP,          8tap_scaled_sharp,          ssse3);
    init_mc_scaled_fn(FILTER_2D_BILINEAR,            bilin_scaled,               ssse3);

    init_mct_scaled_fn(FILTER_2D_8TAP_REGULAR,        8tap_scaled_regular,        ssse3);
    init_mct_scaled_fn(FILTER_2D_8TAP_REGULAR_SMOOTH, 8tap_scaled_regular_smooth, ssse3);
    init_mct_scaled_fn(FILTER_2D_8TAP_REGULAR_SHARP,  8tap_scaled_regular_sharp,  ssse3);
    init_mct_scaled_fn(FILTER_2D_8TAP_SMOOTH_REGULAR, 8tap_scaled_smooth_regular, ssse3);
    init_mct_scaled_fn(FILTER_2D_8TAP_SMOOTH,         8tap_scaled_smooth,         ssse3);
    init_mct_scaled_fn(FILTER_2D_8TAP_SMOOTH_SHARP,   8tap_scaled_smooth_sharp,   ssse3);
    init_mct_scaled_fn(FILTER_2D_8TAP_SHARP_REGULAR,  8tap_scaled_sharp_regular,  ssse3);
    init_mct_scaled_fn(FILTER_2D_8TAP_SHARP_SMOOTH,   8tap_scaled_sharp_smooth,   ssse3);
    init_mct_scaled_fn(FILTER_2D_8TAP_SHARP,          8tap_scaled_sharp,          ssse3);
    init_mct_scaled_fn(FILTER_2D_BILINEAR,            bilin_scaled,               ssse3);

    c->avg = BF(dav1d_avg, ssse3);
    c->w_avg = BF(dav1d_w_avg, ssse3);
    c->mask = BF(dav1d_mask, ssse3);
    c->w_mask[0] = BF(dav1d_w_mask_444, ssse3);
    c->w_mask[1] = BF(dav1d_w_mask_422, ssse3);
    c->w_mask[2] = BF(dav1d_w_mask_420, ssse3);
    c->blend = BF(dav1d_blend, ssse3);
    c->blend_v = BF(dav1d_blend_v, ssse3);
    c->blend_h = BF(dav1d_blend_h, ssse3);
    c->warp8x8  = BF(dav1d_warp_affine_8x8, ssse3);
    c->warp8x8t = BF(dav1d_warp_affine_8x8t, ssse3);
    c->emu_edge = BF(dav1d_emu_edge, ssse3);
    c->resize = BF(dav1d_resize, ssse3);

    if(!(flags & DAV1D_X86_CPU_FLAG_SSE41))
        return;

#if BITDEPTH == 8
    c->warp8x8  = BF(dav1d_warp_affine_8x8, sse4);
    c->warp8x8t = BF(dav1d_warp_affine_8x8t, sse4);
#endif

#if ARCH_X86_64
    if (!(flags & DAV1D_X86_CPU_FLAG_AVX2))
        return;

    init_8tap_fns(avx2);

    init_mc_fn(FILTER_2D_BILINEAR,            bilin,               avx2);
    init_mct_fn(FILTER_2D_BILINEAR,           bilin,               avx2);

    init_mc_scaled_fn(FILTER_2D_8TAP_REGULAR,        8tap_scaled_regular,        avx2);
    init_mc_scaled_fn(FILTER_2D_8TAP_REGULAR_SMOOTH, 8tap_scaled_regular_smooth, avx2);
    init_mc_scaled_fn(FILTER_2D_8TAP_REGULAR_SHARP,  8tap_scaled_regular_sharp,  avx2);
    init_mc_scaled_fn(FILTER_2D_8TAP_SMOOTH_REGULAR, 8tap_scaled_smooth_regular, avx2);
    init_mc_scaled_fn(FILTER_2D_8TAP_SMOOTH,         8tap_scaled_smooth,         avx2);
    init_mc_scaled_fn(FILTER_2D_8TAP_SMOOTH_SHARP,   8tap_scaled_smooth_sharp,   avx2);
    init_mc_scaled_fn(FILTER_2D_8TAP_SHARP_REGULAR,  8tap_scaled_sharp_regular,  avx2);
    init_mc_scaled_fn(FILTER_2D_8TAP_SHARP_SMOOTH,   8tap_scaled_sharp_smooth,   avx2);
    init_mc_scaled_fn(FILTER_2D_8TAP_SHARP,          8tap_scaled_sharp,          avx2);
    init_mc_scaled_fn(FILTER_2D_BILINEAR,            bilin_scaled,               avx2);

    init_mct_scaled_fn(FILTER_2D_8TAP_REGULAR,        8tap_scaled_regular,        avx2);
    init_mct_scaled_fn(FILTER_2D_8TAP_REGULAR_SMOOTH, 8tap_scaled_regular_smooth, avx2);
    init_mct_scaled_fn(FILTER_2D_8TAP_REGULAR_SHARP,  8tap_scaled_regular_sharp,  avx2);
    init_mct_scaled_fn(FILTER_2D_8TAP_SMOOTH_REGULAR, 8tap_scaled_smooth_regular, avx2);
    init_mct_scaled_fn(FILTER_2D_8TAP_SMOOTH,         8tap_scaled_smooth,         avx2);
    init_mct_scaled_fn(FILTER_2D_8TAP_SMOOTH_SHARP,   8tap_scaled_smooth_sharp,   avx2);
    init_mct_scaled_fn(FILTER_2D_8TAP_SHARP_REGULAR,  8tap_scaled_sharp_regular,  avx2);
    init_mct_scaled_fn(FILTER_2D_8TAP_SHARP_SMOOTH,   8tap_scaled_sharp_smooth,   avx2);
    init_mct_scaled_fn(FILTER_2D_8TAP_SHARP,          8tap_scaled_sharp,          avx2);
    init_mct_scaled_fn(FILTER_2D_BILINEAR,            bilin_scaled,               avx2);

    c->avg = BF(dav1d_avg, avx2);
    c->w_avg = BF(dav1d_w_avg, avx2);
    c->mask = BF(dav1d_mask, avx2);
    c->w_mask[0] = BF(dav1d_w_mask_444, avx2);
    c->w_mask[1] = BF(dav1d_w_mask_422, avx2);
    c->w_mask[2] = BF(dav1d_w_mask_420, avx2);
    c->blend = BF(dav1d_blend, avx2);
    c->blend_v = BF(dav1d_blend_v, avx2);
    c->blend_h = BF(dav1d_blend_h, avx2);
    c->warp8x8  = BF(dav1d_warp_affine_8x8, avx2);
    c->warp8x8t = BF(dav1d_warp_affine_8x8t, avx2);
    c->emu_edge = BF(dav1d_emu_edge, avx2);
    c->resize = BF(dav1d_resize, avx2);

    if (!(flags & DAV1D_X86_CPU_FLAG_AVX512ICL))
        return;

    init_8tap_fns(avx512icl);

    init_mc_fn (FILTER_2D_BILINEAR,            bilin,               avx512icl);
    init_mct_fn(FILTER_2D_BILINEAR,            bilin,               avx512icl);

    c->avg = BF(dav1d_avg, avx512icl);
    c->w_avg = BF(dav1d_w_avg, avx512icl);
    c->mask = BF(dav1d_mask, avx512icl);
    c->w_mask[0] = BF(dav1d_w_mask_444, avx512icl);
    c->w_mask[1] = BF(dav1d_w_mask_422, avx512icl);
    c->w_mask[2] = BF(dav1d_w_mask_420, avx512icl);
    c->blend = BF(dav1d_blend, avx512icl);
    c->blend_v = BF(dav1d_blend_v, avx512icl);
    c->blend_h = BF(dav1d_blend_h, avx512icl);

    if (!(flags & DAV1D_X86_CPU_FLAG_SLOW_GATHER)) {
        c->resize = BF(dav1d_resize, avx512icl);
        c->warp8x8  = BF(dav1d_warp_affine_8x8, avx512icl);
        c->warp8x8t = BF(dav1d_warp_affine_8x8t, avx512icl);
    }
#endif
}
