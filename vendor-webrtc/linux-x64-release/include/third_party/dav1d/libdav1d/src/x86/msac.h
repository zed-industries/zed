/*
 * Copyright © 2019, VideoLAN and dav1d authors
 * Copyright © 2019, Two Orioles, LLC
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

#ifndef DAV1D_SRC_X86_MSAC_H
#define DAV1D_SRC_X86_MSAC_H

#include "src/cpu.h"

unsigned dav1d_msac_decode_symbol_adapt4_sse2(MsacContext *s, uint16_t *cdf,
                                              size_t n_symbols);
unsigned dav1d_msac_decode_symbol_adapt8_sse2(MsacContext *s, uint16_t *cdf,
                                              size_t n_symbols);
unsigned dav1d_msac_decode_symbol_adapt16_sse2(MsacContext *s, uint16_t *cdf,
                                               size_t n_symbols);
unsigned dav1d_msac_decode_symbol_adapt16_avx2(MsacContext *s, uint16_t *cdf,
                                               size_t n_symbols);
unsigned dav1d_msac_decode_bool_adapt_sse2(MsacContext *s, uint16_t *cdf);
unsigned dav1d_msac_decode_bool_equi_sse2(MsacContext *s);
unsigned dav1d_msac_decode_bool_sse2(MsacContext *s, unsigned f);
unsigned dav1d_msac_decode_hi_tok_sse2(MsacContext *s, uint16_t *cdf);

#if ARCH_X86_64 || defined(__SSE2__) || (defined(_M_IX86_FP) && _M_IX86_FP >= 2)
#define dav1d_msac_decode_symbol_adapt4  dav1d_msac_decode_symbol_adapt4_sse2
#define dav1d_msac_decode_symbol_adapt8  dav1d_msac_decode_symbol_adapt8_sse2
#define dav1d_msac_decode_hi_tok         dav1d_msac_decode_hi_tok_sse2
#endif

#define dav1d_msac_decode_bool_adapt     dav1d_msac_decode_bool_adapt_sse2
#define dav1d_msac_decode_bool_equi      dav1d_msac_decode_bool_equi_sse2
#define dav1d_msac_decode_bool           dav1d_msac_decode_bool_sse2

#if ARCH_X86_64
#define dav1d_msac_decode_symbol_adapt16(ctx, cdf, symb) ((ctx)->symbol_adapt16(ctx, cdf, symb))

static ALWAYS_INLINE void msac_init_x86(MsacContext *const s) {
    const unsigned flags = dav1d_get_cpu_flags();

    if (flags & DAV1D_X86_CPU_FLAG_SSE2) {
        s->symbol_adapt16 = dav1d_msac_decode_symbol_adapt16_sse2;
    }

    if (flags & DAV1D_X86_CPU_FLAG_AVX2) {
        s->symbol_adapt16 = dav1d_msac_decode_symbol_adapt16_avx2;
    }
}

#elif defined(__SSE2__) || (defined(_M_IX86_FP) && _M_IX86_FP >= 2)
#define dav1d_msac_decode_symbol_adapt16 dav1d_msac_decode_symbol_adapt16_sse2
#endif

#endif /* DAV1D_SRC_X86_MSAC_H */
