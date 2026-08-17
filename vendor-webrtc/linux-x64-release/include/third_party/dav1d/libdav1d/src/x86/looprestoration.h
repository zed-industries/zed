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

#include "src/cpu.h"
#include "src/looprestoration.h"

#include "common/intops.h"

#define decl_wiener_filter_fns(ext) \
decl_lr_filter_fn(BF(dav1d_wiener_filter7, ext)); \
decl_lr_filter_fn(BF(dav1d_wiener_filter5, ext))

#define decl_sgr_filter_fns(ext) \
decl_lr_filter_fn(BF(dav1d_sgr_filter_5x5, ext)); \
decl_lr_filter_fn(BF(dav1d_sgr_filter_3x3, ext)); \
decl_lr_filter_fn(BF(dav1d_sgr_filter_mix, ext))

decl_wiener_filter_fns(sse2);
decl_wiener_filter_fns(ssse3);
decl_wiener_filter_fns(avx2);
decl_wiener_filter_fns(avx512icl);
decl_sgr_filter_fns(ssse3);
decl_sgr_filter_fns(avx2);
decl_sgr_filter_fns(avx512icl);

static ALWAYS_INLINE void loop_restoration_dsp_init_x86(Dav1dLoopRestorationDSPContext *const c, const int bpc) {
    const unsigned flags = dav1d_get_cpu_flags();

    if (!(flags & DAV1D_X86_CPU_FLAG_SSE2)) return;
#if BITDEPTH == 8
    c->wiener[0] = BF(dav1d_wiener_filter7, sse2);
    c->wiener[1] = BF(dav1d_wiener_filter5, sse2);
#endif

    if (!(flags & DAV1D_X86_CPU_FLAG_SSSE3)) return;
    c->wiener[0] = BF(dav1d_wiener_filter7, ssse3);
    c->wiener[1] = BF(dav1d_wiener_filter5, ssse3);
    if (BITDEPTH == 8 || bpc == 10) {
        c->sgr[0] = BF(dav1d_sgr_filter_5x5, ssse3);
        c->sgr[1] = BF(dav1d_sgr_filter_3x3, ssse3);
        c->sgr[2] = BF(dav1d_sgr_filter_mix, ssse3);
    }

#if ARCH_X86_64
    if (!(flags & DAV1D_X86_CPU_FLAG_AVX2)) return;

    c->wiener[0] = BF(dav1d_wiener_filter7, avx2);
    c->wiener[1] = BF(dav1d_wiener_filter5, avx2);
    if (BITDEPTH == 8 || bpc == 10) {
        c->sgr[0] = BF(dav1d_sgr_filter_5x5, avx2);
        c->sgr[1] = BF(dav1d_sgr_filter_3x3, avx2);
        c->sgr[2] = BF(dav1d_sgr_filter_mix, avx2);
    }

    if (!(flags & DAV1D_X86_CPU_FLAG_AVX512ICL)) return;

    c->wiener[0] = BF(dav1d_wiener_filter7, avx512icl);
#if BITDEPTH == 8
    /* With VNNI we don't need a 5-tap version. */
    c->wiener[1] = c->wiener[0];
#else
    c->wiener[1] = BF(dav1d_wiener_filter5, avx512icl);
#endif
    if (BITDEPTH == 8 || bpc == 10) {
        c->sgr[0] = BF(dav1d_sgr_filter_5x5, avx512icl);
        c->sgr[1] = BF(dav1d_sgr_filter_3x3, avx512icl);
        c->sgr[2] = BF(dav1d_sgr_filter_mix, avx512icl);
    }
#endif
}
