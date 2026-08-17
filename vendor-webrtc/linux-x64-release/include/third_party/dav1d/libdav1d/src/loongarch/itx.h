/*
 * Copyright © 2023, VideoLAN and dav1d authors
 * Copyright © 2023, Loongson Technology Corporation Limited
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

#ifndef DAV1D_SRC_LOONGARCH_ITX_H
#define DAV1D_SRC_LOONGARCH_ITX_H

#include "src/cpu.h"
#include "src/itx.h"

decl_itx17_fns( 4,  4, lsx);
decl_itx16_fns( 4,  8, lsx);
decl_itx16_fns( 4, 16, lsx);
decl_itx16_fns( 8,  4, lsx);
decl_itx16_fns( 8,  8, lsx);
decl_itx16_fns( 8, 16, lsx);
decl_itx2_fns ( 8, 32, lsx);
decl_itx16_fns(16,  8, lsx);

decl_itx_fn(BF(dav1d_inv_txfm_add_dct_dct_16x4, lsx));
decl_itx_fn(BF(dav1d_inv_txfm_add_identity_identity_16x4, lsx));
decl_itx_fn(BF(dav1d_inv_txfm_add_adst_dct_16x4, lsx));

decl_itx_fn(BF(dav1d_inv_txfm_add_dct_dct_16x16, lsx));
decl_itx_fn(BF(dav1d_inv_txfm_add_adst_adst_16x16, lsx));
decl_itx_fn(BF(dav1d_inv_txfm_add_adst_dct_16x16, lsx));
decl_itx_fn(BF(dav1d_inv_txfm_add_dct_adst_16x16, lsx));
decl_itx_fn(BF(dav1d_inv_txfm_add_flipadst_dct_16x16, lsx));
decl_itx_fn(BF(dav1d_inv_txfm_add_dct_flipadst_16x16, lsx));
decl_itx_fn(BF(dav1d_inv_txfm_add_adst_flipadst_16x16, lsx));
decl_itx_fn(BF(dav1d_inv_txfm_add_flipadst_adst_16x16, lsx));

decl_itx_fn(BF(dav1d_inv_txfm_add_dct_dct_16x32, lsx));

decl_itx_fn(BF(dav1d_inv_txfm_add_dct_dct_32x8, lsx));

decl_itx_fn(BF(dav1d_inv_txfm_add_dct_dct_32x16, lsx));

decl_itx_fn(BF(dav1d_inv_txfm_add_dct_dct_32x32, lsx));

decl_itx_fn(BF(dav1d_inv_txfm_add_dct_dct_64x32, lsx));

decl_itx_fn(BF(dav1d_inv_txfm_add_dct_dct_64x64, lsx));

decl_itx_fn(BF(dav1d_inv_txfm_add_adst_adst_16x16, lasx));

static ALWAYS_INLINE void itx_dsp_init_loongarch(Dav1dInvTxfmDSPContext *const c, int bpc) {
#if BITDEPTH == 8
    const unsigned flags = dav1d_get_cpu_flags();

    if (!(flags & DAV1D_LOONGARCH_CPU_FLAG_LSX)) return;

    if (BITDEPTH != 8 ) return;

    assign_itx17_fn( ,  4,  4, lsx);
    assign_itx16_fn(R,  4,  8, lsx);
    assign_itx16_fn(R,  4, 16, lsx);
    assign_itx16_fn(R,  8,  4, lsx);
    assign_itx16_fn( ,  8,  8, lsx);
    assign_itx16_fn(R,  8, 16, lsx);
    assign_itx2_fn (R,  8, 32, lsx);
    assign_itx16_fn(R, 16,  8, lsx);
    assign_itx1_fn (R, 64, 32, lsx);
    assign_itx1_fn ( , 64, 64, lsx);

    c->itxfm_add[RTX_16X4][DCT_DCT] = dav1d_inv_txfm_add_dct_dct_16x4_8bpc_lsx;
    c->itxfm_add[RTX_16X4][IDTX] = dav1d_inv_txfm_add_identity_identity_16x4_8bpc_lsx;
    c->itxfm_add[RTX_16X4][DCT_ADST] = dav1d_inv_txfm_add_adst_dct_16x4_8bpc_lsx;

    c->itxfm_add[TX_16X16][DCT_DCT] = dav1d_inv_txfm_add_dct_dct_16x16_8bpc_lsx;
    c->itxfm_add[TX_16X16][ADST_ADST] = dav1d_inv_txfm_add_adst_adst_16x16_8bpc_lsx;
    c->itxfm_add[TX_16X16][DCT_ADST] = dav1d_inv_txfm_add_adst_dct_16x16_8bpc_lsx;
    c->itxfm_add[TX_16X16][ADST_DCT] = dav1d_inv_txfm_add_dct_adst_16x16_8bpc_lsx;
    c->itxfm_add[TX_16X16][DCT_FLIPADST] = dav1d_inv_txfm_add_flipadst_dct_16x16_8bpc_lsx;
    c->itxfm_add[TX_16X16][FLIPADST_DCT] = dav1d_inv_txfm_add_dct_flipadst_16x16_8bpc_lsx;
    c->itxfm_add[TX_16X16][FLIPADST_ADST] = dav1d_inv_txfm_add_adst_flipadst_16x16_8bpc_lsx;
    c->itxfm_add[TX_16X16][ADST_FLIPADST] = dav1d_inv_txfm_add_flipadst_adst_16x16_8bpc_lsx;

    c->itxfm_add[RTX_16X32][DCT_DCT] = dav1d_inv_txfm_add_dct_dct_16x32_8bpc_lsx;

    c->itxfm_add[RTX_32X8][DCT_DCT] = dav1d_inv_txfm_add_dct_dct_32x8_8bpc_lsx;

    c->itxfm_add[RTX_32X16][DCT_DCT] = dav1d_inv_txfm_add_dct_dct_32x16_8bpc_lsx;

    c->itxfm_add[TX_32X32][DCT_DCT] = dav1d_inv_txfm_add_dct_dct_32x32_8bpc_lsx;

    if (!(flags & DAV1D_LOONGARCH_CPU_FLAG_LASX)) return;

    if (BITDEPTH != 8 ) return;

    c->itxfm_add[TX_16X16][ADST_ADST] = dav1d_inv_txfm_add_adst_adst_16x16_8bpc_lasx;

#endif
}

#endif /* DAV1D_SRC_LOONGARCH_ITX_H */
