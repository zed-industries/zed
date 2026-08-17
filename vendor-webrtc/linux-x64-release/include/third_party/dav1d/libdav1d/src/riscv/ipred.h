/*
 * Copyright © 2018, VideoLAN and dav1d authors
 * Copyright © 2024, Bogdan Gligorijevic
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
#include "src/ipred.h"

decl_cfl_pred_fn(BF(dav1d_ipred_cfl, rvv));
decl_cfl_pred_fn(BF(dav1d_ipred_cfl_128, rvv));
decl_cfl_pred_fn(BF(dav1d_ipred_cfl_top, rvv));
decl_cfl_pred_fn(BF(dav1d_ipred_cfl_left, rvv));

decl_angular_ipred_fn(BF(dav1d_ipred_paeth, rvv));
decl_angular_ipred_fn(BF(dav1d_ipred_smooth, rvv));
decl_angular_ipred_fn(BF(dav1d_ipred_smooth_v, rvv));
decl_angular_ipred_fn(BF(dav1d_ipred_smooth_h, rvv));


decl_pal_pred_fn(BF(dav1d_pal_pred, rvv));

static ALWAYS_INLINE void intra_pred_dsp_init_riscv(Dav1dIntraPredDSPContext *const c) {
  const unsigned flags = dav1d_get_cpu_flags();

  if (!(flags & DAV1D_RISCV_CPU_FLAG_V)) return;

#if BITDEPTH == 8
    c->cfl_pred[DC_PRED     ] = dav1d_ipred_cfl_8bpc_rvv;
    c->cfl_pred[DC_128_PRED ] = dav1d_ipred_cfl_128_8bpc_rvv;
    c->cfl_pred[TOP_DC_PRED ] = dav1d_ipred_cfl_top_8bpc_rvv;
    c->cfl_pred[LEFT_DC_PRED] = dav1d_ipred_cfl_left_8bpc_rvv;

    c->intra_pred[PAETH_PRED   ] = dav1d_ipred_paeth_8bpc_rvv;
    c->intra_pred[SMOOTH_PRED  ] = dav1d_ipred_smooth_8bpc_rvv;
    c->intra_pred[SMOOTH_V_PRED] = dav1d_ipred_smooth_v_8bpc_rvv;
    c->intra_pred[SMOOTH_H_PRED] = dav1d_ipred_smooth_h_8bpc_rvv;

    c->pal_pred = dav1d_pal_pred_8bpc_rvv;
#elif BITDEPTH == 16
    c->cfl_pred[DC_PRED     ] = dav1d_ipred_cfl_16bpc_rvv;
    c->cfl_pred[DC_128_PRED ] = dav1d_ipred_cfl_128_16bpc_rvv;
    c->cfl_pred[TOP_DC_PRED ] = dav1d_ipred_cfl_top_16bpc_rvv;
    c->cfl_pred[LEFT_DC_PRED] = dav1d_ipred_cfl_left_16bpc_rvv;

    c->intra_pred[PAETH_PRED   ] = dav1d_ipred_paeth_16bpc_rvv;
    c->intra_pred[SMOOTH_PRED  ] = dav1d_ipred_smooth_16bpc_rvv;
    c->intra_pred[SMOOTH_V_PRED] = dav1d_ipred_smooth_v_16bpc_rvv;
    c->intra_pred[SMOOTH_H_PRED] = dav1d_ipred_smooth_h_16bpc_rvv;

    c->pal_pred = dav1d_pal_pred_16bpc_rvv;
#endif
}
