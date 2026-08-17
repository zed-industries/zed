/*
 * Copyright © 2021, VideoLAN and dav1d authors
 * Copyright © 2021, Two Orioles, LLC
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
#include "src/refmvs.h"

decl_load_tmvs_fn(dav1d_load_tmvs_sse4);

decl_save_tmvs_fn(dav1d_save_tmvs_ssse3);
decl_save_tmvs_fn(dav1d_save_tmvs_avx2);
decl_save_tmvs_fn(dav1d_save_tmvs_avx512icl);

decl_splat_mv_fn(dav1d_splat_mv_sse2);
decl_splat_mv_fn(dav1d_splat_mv_avx2);
decl_splat_mv_fn(dav1d_splat_mv_avx512icl);

static ALWAYS_INLINE void refmvs_dsp_init_x86(Dav1dRefmvsDSPContext *const c) {
    const unsigned flags = dav1d_get_cpu_flags();

    if (!(flags & DAV1D_X86_CPU_FLAG_SSE2)) return;

    c->splat_mv = dav1d_splat_mv_sse2;

    if (!(flags & DAV1D_X86_CPU_FLAG_SSSE3)) return;

    c->save_tmvs = dav1d_save_tmvs_ssse3;

    if (!(flags & DAV1D_X86_CPU_FLAG_SSE41)) return;
#if ARCH_X86_64
    c->load_tmvs = dav1d_load_tmvs_sse4;

    if (!(flags & DAV1D_X86_CPU_FLAG_AVX2)) return;

    c->save_tmvs = dav1d_save_tmvs_avx2;
    c->splat_mv = dav1d_splat_mv_avx2;

    if (!(flags & DAV1D_X86_CPU_FLAG_AVX512ICL)) return;

    c->save_tmvs = dav1d_save_tmvs_avx512icl;
    c->splat_mv = dav1d_splat_mv_avx512icl;
#endif
}
