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

#ifndef DAV1D_SRC_MSAC_H
#define DAV1D_SRC_MSAC_H

#include <stdint.h>
#include <stdlib.h>

#include "common/intops.h"

typedef size_t ec_win;

typedef struct MsacContext {
    const uint8_t *buf_pos;
    const uint8_t *buf_end;
    ec_win dif;
    unsigned rng;
    int cnt;
    int allow_update_cdf;

#if ARCH_X86_64 && HAVE_ASM
    unsigned (*symbol_adapt16)(struct MsacContext *s, uint16_t *cdf, size_t n_symbols);
#endif
} MsacContext;

#if HAVE_ASM
#if ARCH_AARCH64 || ARCH_ARM
#include "src/arm/msac.h"
#elif ARCH_LOONGARCH64
#include "src/loongarch/msac.h"
#elif ARCH_X86
#include "src/x86/msac.h"
#endif
#endif

void dav1d_msac_init(MsacContext *s, const uint8_t *data, size_t sz,
                     int disable_cdf_update_flag);
unsigned dav1d_msac_decode_symbol_adapt_c(MsacContext *s, uint16_t *cdf,
                                          size_t n_symbols);
unsigned dav1d_msac_decode_bool_adapt_c(MsacContext *s, uint16_t *cdf);
unsigned dav1d_msac_decode_bool_equi_c(MsacContext *s);
unsigned dav1d_msac_decode_bool_c(MsacContext *s, unsigned f);
unsigned dav1d_msac_decode_hi_tok_c(MsacContext *s, uint16_t *cdf);
int dav1d_msac_decode_subexp(MsacContext *s, int ref, int n, unsigned k);

/* Supported n_symbols ranges: adapt4: 1-3, adapt8: 1-7, adapt16: 3-15 */
#ifndef dav1d_msac_decode_symbol_adapt4
#define dav1d_msac_decode_symbol_adapt4  dav1d_msac_decode_symbol_adapt_c
#endif
#ifndef dav1d_msac_decode_symbol_adapt8
#define dav1d_msac_decode_symbol_adapt8  dav1d_msac_decode_symbol_adapt_c
#endif
#ifndef dav1d_msac_decode_symbol_adapt16
#define dav1d_msac_decode_symbol_adapt16 dav1d_msac_decode_symbol_adapt_c
#endif
#ifndef dav1d_msac_decode_bool_adapt
#define dav1d_msac_decode_bool_adapt     dav1d_msac_decode_bool_adapt_c
#endif
#ifndef dav1d_msac_decode_bool_equi
#define dav1d_msac_decode_bool_equi      dav1d_msac_decode_bool_equi_c
#endif
#ifndef dav1d_msac_decode_bool
#define dav1d_msac_decode_bool           dav1d_msac_decode_bool_c
#endif
#ifndef dav1d_msac_decode_hi_tok
#define dav1d_msac_decode_hi_tok         dav1d_msac_decode_hi_tok_c
#endif

static inline unsigned dav1d_msac_decode_bools(MsacContext *const s, unsigned n) {
    unsigned v = 0;
    while (n--)
        v = (v << 1) | dav1d_msac_decode_bool_equi(s);
    return v;
}

static inline int dav1d_msac_decode_uniform(MsacContext *const s, const unsigned n) {
    assert(n > 0);
    const int l = ulog2(n) + 1;
    assert(l > 1);
    const unsigned m = (1 << l) - n;
    const unsigned v = dav1d_msac_decode_bools(s, l - 1);
    return v < m ? v : (v << 1) - m + dav1d_msac_decode_bool_equi(s);
}

#endif /* DAV1D_SRC_MSAC_H */
