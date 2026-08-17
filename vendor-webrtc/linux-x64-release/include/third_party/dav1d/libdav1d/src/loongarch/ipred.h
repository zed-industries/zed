/*
 * Copyright © 2024, VideoLAN and dav1d authors
 * Copyright © 2024, Loongson Technology Corporation Limited
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

#ifndef DAV1D_SRC_LOONGARCH_IPRED_H
#define DAV1D_SRC_LOONGARCH_IPRED_H

#include "config.h"
#include "src/ipred.h"
#include "src/cpu.h"
#include "src/tables.h"

#define MULTIPLIER_1x2 0x5556
#define MULTIPLIER_1x4 0x3334
#define BASE_SHIFT 16

#define init_fn(type0, type1, name, suffix) \
    c->type0[type1] = BF(dav1d_##name, suffix)

#define init_angular_ipred_fn(type, name, suffix) \
    init_fn(intra_pred, type, name, suffix)
#define init_cfl_pred_fn(type, name, suffix) \
    init_fn(cfl_pred, type, name, suffix)

decl_angular_ipred_fn(BF(dav1d_ipred_dc, lsx));
decl_angular_ipred_fn(BF(dav1d_ipred_dc_128, lsx));
decl_angular_ipred_fn(BF(dav1d_ipred_dc_top, lsx));
decl_angular_ipred_fn(BF(dav1d_ipred_dc_left, lsx));
decl_angular_ipred_fn(BF(dav1d_ipred_h, lsx));
decl_angular_ipred_fn(BF(dav1d_ipred_v, lsx));
decl_angular_ipred_fn(BF(dav1d_ipred_paeth, lsx));
decl_angular_ipred_fn(BF(dav1d_ipred_smooth, lsx));
decl_angular_ipred_fn(BF(dav1d_ipred_smooth_v, lsx));
decl_angular_ipred_fn(BF(dav1d_ipred_smooth_h, lsx));
decl_angular_ipred_fn(BF(dav1d_ipred_filter, lsx));
decl_angular_ipred_fn(BF(dav1d_ipred_z1, lsx));

decl_cfl_pred_fn(BF(dav1d_ipred_cfl, lsx));
decl_cfl_pred_fn(BF(dav1d_ipred_cfl_128, lsx));
decl_cfl_pred_fn(BF(dav1d_ipred_cfl_top, lsx));
decl_cfl_pred_fn(BF(dav1d_ipred_cfl_left, lsx));

decl_pal_pred_fn(BF(dav1d_pal_pred, lsx));

static ALWAYS_INLINE void intra_pred_dsp_init_loongarch(Dav1dIntraPredDSPContext *const c) {
    const unsigned flags = dav1d_get_cpu_flags();

    if (!(flags & DAV1D_LOONGARCH_CPU_FLAG_LSX)) return;

#if BITDEPTH == 8
    init_angular_ipred_fn(DC_PRED,       ipred_dc,       lsx);
    init_angular_ipred_fn(DC_128_PRED,   ipred_dc_128,   lsx);
    init_angular_ipred_fn(TOP_DC_PRED,   ipred_dc_top,   lsx);
    init_angular_ipred_fn(LEFT_DC_PRED,  ipred_dc_left,  lsx);
    init_angular_ipred_fn(HOR_PRED,      ipred_h,        lsx);
    init_angular_ipred_fn(VERT_PRED,     ipred_v,        lsx);
    init_angular_ipred_fn(PAETH_PRED,    ipred_paeth,    lsx);
    init_angular_ipred_fn(SMOOTH_PRED,   ipred_smooth,   lsx);
    init_angular_ipred_fn(SMOOTH_V_PRED, ipred_smooth_v, lsx);
    init_angular_ipred_fn(SMOOTH_H_PRED, ipred_smooth_h, lsx);
    init_angular_ipred_fn(FILTER_PRED,   ipred_filter,   lsx);
    init_angular_ipred_fn(Z1_PRED,       ipred_z1,       lsx);

    init_cfl_pred_fn(DC_PRED,      ipred_cfl,      lsx);
    init_cfl_pred_fn(DC_128_PRED,  ipred_cfl_128,  lsx);
    init_cfl_pred_fn(TOP_DC_PRED,  ipred_cfl_top,  lsx);
    init_cfl_pred_fn(LEFT_DC_PRED, ipred_cfl_left, lsx);

    c->pal_pred = BF(dav1d_pal_pred, lsx);
#endif
}

#endif /* DAV1D_SRC_LOONGARCH_IPRED_H */
