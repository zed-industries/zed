/*
 * Copyright © 2018, VideoLAN and dav1d authors
 * Copyright © 2023, Nathan Egge
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
#include "src/itx.h"

#define decl_itx_fns(ext) \
decl_itx17_fns( 4,  4, ext); \
decl_itx16_fns( 4,  8, ext); \
decl_itx16_fns( 4, 16, ext); \
decl_itx16_fns( 8,  4, ext); \
decl_itx16_fns( 8,  8, ext); \
decl_itx16_fns( 8, 16, ext); \
decl_itx16_fns(16,  4, ext); \
decl_itx16_fns(16,  8, ext); \
decl_itx16_fns(16, 16, ext)

decl_itx_fns(rvv);

static ALWAYS_INLINE void itx_dsp_init_riscv(Dav1dInvTxfmDSPContext *const c, int const bpc) {
  const unsigned flags = dav1d_get_cpu_flags();

  if (!(flags & DAV1D_RISCV_CPU_FLAG_V)) return;

#if BITDEPTH == 8
  assign_itx17_fn( ,  4,  4, rvv);
  assign_itx16_fn(R,  4,  8, rvv);
  assign_itx16_fn(R,  4, 16, rvv);
  assign_itx16_fn(R,  8,  4, rvv);
  assign_itx16_fn( ,  8,  8, rvv);
  assign_itx16_fn(R,  8, 16, rvv);
  assign_itx16_fn(R, 16,  4, rvv);
  assign_itx16_fn(R, 16,  8, rvv);
  assign_itx12_fn( , 16, 16, rvv);
#endif
}
