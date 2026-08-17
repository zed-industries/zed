/*
 * Copyright © 2018, VideoLAN and dav1d authors
 * Copyright © 2019, Martin Storsjo
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

decl_itx17_fns( 4,  4, neon);
decl_itx16_fns( 4,  8, neon);
decl_itx16_fns( 4, 16, neon);
decl_itx16_fns( 8,  4, neon);
decl_itx16_fns( 8,  8, neon);
decl_itx16_fns( 8, 16, neon);
decl_itx2_fns ( 8, 32, neon);
decl_itx16_fns(16,  4, neon);
decl_itx16_fns(16,  8, neon);
decl_itx12_fns(16, 16, neon);
decl_itx2_fns (16, 32, neon);
decl_itx2_fns (32,  8, neon);
decl_itx2_fns (32, 16, neon);
decl_itx2_fns (32, 32, neon);

decl_itx_fn(BF(dav1d_inv_txfm_add_dct_dct_16x64, neon));
decl_itx_fn(BF(dav1d_inv_txfm_add_dct_dct_32x64, neon));
decl_itx_fn(BF(dav1d_inv_txfm_add_dct_dct_64x16, neon));
decl_itx_fn(BF(dav1d_inv_txfm_add_dct_dct_64x32, neon));
decl_itx_fn(BF(dav1d_inv_txfm_add_dct_dct_64x64, neon));

static ALWAYS_INLINE void itx_dsp_init_arm(Dav1dInvTxfmDSPContext *const c, int bpc,
                                           int *const all_simd)
{
    const unsigned flags = dav1d_get_cpu_flags();

    if (!(flags & DAV1D_ARM_CPU_FLAG_NEON)) return;

    assign_itx_fn( , 4, 4, wht_wht,           WHT_WHT,           neon);

    if (BITDEPTH == 16 && bpc != 10) return;

    assign_itx16_fn( ,  4,  4, neon);
    assign_itx16_fn(R,  4,  8, neon);
    assign_itx16_fn(R,  4, 16, neon);
    assign_itx16_fn(R,  8,  4, neon);
    assign_itx16_fn( ,  8,  8, neon);
    assign_itx16_fn(R,  8, 16, neon);
    assign_itx2_fn (R,  8, 32, neon);
    assign_itx16_fn(R, 16,  4, neon);
    assign_itx16_fn(R, 16,  8, neon);
    assign_itx12_fn( , 16, 16, neon);
    assign_itx2_fn (R, 16, 32, neon);
    assign_itx1_fn (R, 16, 64, neon);
    assign_itx2_fn (R, 32,  8, neon);
    assign_itx2_fn (R, 32, 16, neon);
    assign_itx2_fn ( , 32, 32, neon);
    assign_itx1_fn (R, 32, 64, neon);
    assign_itx1_fn (R, 64, 16, neon);
    assign_itx1_fn (R, 64, 32, neon);
    assign_itx1_fn ( , 64, 64, neon);
    *all_simd = 1;
}
