/*
 * Copyright © 2018-2022, VideoLAN and dav1d authors
 * Copyright © 2018-2022, Two Orioles, LLC
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
#include "src/filmgrain.h"

#define decl_fg_fns(ext)                                         \
decl_generate_grain_y_fn(BF(dav1d_generate_grain_y, ext));       \
decl_generate_grain_uv_fn(BF(dav1d_generate_grain_uv_420, ext)); \
decl_generate_grain_uv_fn(BF(dav1d_generate_grain_uv_422, ext)); \
decl_generate_grain_uv_fn(BF(dav1d_generate_grain_uv_444, ext)); \
decl_fgy_32x32xn_fn(BF(dav1d_fgy_32x32xn, ext));                 \
decl_fguv_32x32xn_fn(BF(dav1d_fguv_32x32xn_i420, ext));          \
decl_fguv_32x32xn_fn(BF(dav1d_fguv_32x32xn_i422, ext));          \
decl_fguv_32x32xn_fn(BF(dav1d_fguv_32x32xn_i444, ext))

decl_fg_fns(ssse3);
decl_fg_fns(avx2);
decl_fg_fns(avx512icl);

static ALWAYS_INLINE void film_grain_dsp_init_x86(Dav1dFilmGrainDSPContext *const c) {
    const unsigned flags = dav1d_get_cpu_flags();

    if (!(flags & DAV1D_X86_CPU_FLAG_SSSE3)) return;

    c->generate_grain_y = BF(dav1d_generate_grain_y, ssse3);
    c->generate_grain_uv[DAV1D_PIXEL_LAYOUT_I420 - 1] = BF(dav1d_generate_grain_uv_420, ssse3);
    c->fgy_32x32xn = BF(dav1d_fgy_32x32xn, ssse3);
    c->fguv_32x32xn[DAV1D_PIXEL_LAYOUT_I420 - 1] = BF(dav1d_fguv_32x32xn_i420, ssse3);
    c->generate_grain_uv[DAV1D_PIXEL_LAYOUT_I422 - 1] = BF(dav1d_generate_grain_uv_422, ssse3);
    c->generate_grain_uv[DAV1D_PIXEL_LAYOUT_I444 - 1] = BF(dav1d_generate_grain_uv_444, ssse3);
    c->fguv_32x32xn[DAV1D_PIXEL_LAYOUT_I422 - 1] = BF(dav1d_fguv_32x32xn_i422, ssse3);
    c->fguv_32x32xn[DAV1D_PIXEL_LAYOUT_I444 - 1] = BF(dav1d_fguv_32x32xn_i444, ssse3);

#if ARCH_X86_64
    if (!(flags & DAV1D_X86_CPU_FLAG_AVX2)) return;

    c->generate_grain_y = BF(dav1d_generate_grain_y, avx2);
    c->generate_grain_uv[DAV1D_PIXEL_LAYOUT_I420 - 1] = BF(dav1d_generate_grain_uv_420, avx2);
    c->generate_grain_uv[DAV1D_PIXEL_LAYOUT_I422 - 1] = BF(dav1d_generate_grain_uv_422, avx2);
    c->generate_grain_uv[DAV1D_PIXEL_LAYOUT_I444 - 1] = BF(dav1d_generate_grain_uv_444, avx2);

    if (!(flags & DAV1D_X86_CPU_FLAG_SLOW_GATHER)) {
        c->fgy_32x32xn = BF(dav1d_fgy_32x32xn, avx2);
        c->fguv_32x32xn[DAV1D_PIXEL_LAYOUT_I420 - 1] = BF(dav1d_fguv_32x32xn_i420, avx2);
        c->fguv_32x32xn[DAV1D_PIXEL_LAYOUT_I422 - 1] = BF(dav1d_fguv_32x32xn_i422, avx2);
        c->fguv_32x32xn[DAV1D_PIXEL_LAYOUT_I444 - 1] = BF(dav1d_fguv_32x32xn_i444, avx2);
    }

    if (!(flags & DAV1D_X86_CPU_FLAG_AVX512ICL)) return;

    if (BITDEPTH == 8 || !(flags & DAV1D_X86_CPU_FLAG_SLOW_GATHER)) {
        c->fgy_32x32xn = BF(dav1d_fgy_32x32xn, avx512icl);
        c->fguv_32x32xn[DAV1D_PIXEL_LAYOUT_I420 - 1] = BF(dav1d_fguv_32x32xn_i420, avx512icl);
        c->fguv_32x32xn[DAV1D_PIXEL_LAYOUT_I422 - 1] = BF(dav1d_fguv_32x32xn_i422, avx512icl);
        c->fguv_32x32xn[DAV1D_PIXEL_LAYOUT_I444 - 1] = BF(dav1d_fguv_32x32xn_i444, avx512icl);
    }
#endif
}
