/*
 * Copyright © 2018-2021, VideoLAN and dav1d authors
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
#include "src/ipred.h"

#define decl_fn(type, name) \
    decl_##type##_fn(BF(dav1d_##name, ssse3)); \
    decl_##type##_fn(BF(dav1d_##name, avx2)); \
    decl_##type##_fn(BF(dav1d_##name, avx512icl))
#define init_fn(type0, type1, name, suffix) \
    c->type0[type1] = BF(dav1d_##name, suffix)

#define init_angular_ipred_fn(type, name, suffix) \
    init_fn(intra_pred, type, name, suffix)
#define init_cfl_pred_fn(type, name, suffix) \
    init_fn(cfl_pred, type, name, suffix)
#define init_cfl_ac_fn(type, name, suffix) \
    init_fn(cfl_ac, type, name, suffix)

decl_fn(angular_ipred, ipred_dc);
decl_fn(angular_ipred, ipred_dc_128);
decl_fn(angular_ipred, ipred_dc_top);
decl_fn(angular_ipred, ipred_dc_left);
decl_fn(angular_ipred, ipred_h);
decl_fn(angular_ipred, ipred_v);
decl_fn(angular_ipred, ipred_paeth);
decl_fn(angular_ipred, ipred_smooth);
decl_fn(angular_ipred, ipred_smooth_h);
decl_fn(angular_ipred, ipred_smooth_v);
decl_fn(angular_ipred, ipred_z1);
decl_fn(angular_ipred, ipred_z2);
decl_fn(angular_ipred, ipred_z3);
decl_fn(angular_ipred, ipred_filter);

decl_fn(cfl_pred, ipred_cfl);
decl_fn(cfl_pred, ipred_cfl_128);
decl_fn(cfl_pred, ipred_cfl_top);
decl_fn(cfl_pred, ipred_cfl_left);

decl_fn(cfl_ac, ipred_cfl_ac_420);
decl_fn(cfl_ac, ipred_cfl_ac_422);
decl_fn(cfl_ac, ipred_cfl_ac_444);

decl_fn(pal_pred, pal_pred);

static ALWAYS_INLINE void intra_pred_dsp_init_x86(Dav1dIntraPredDSPContext *const c) {
    const unsigned flags = dav1d_get_cpu_flags();

    if (!(flags & DAV1D_X86_CPU_FLAG_SSSE3)) return;

    init_angular_ipred_fn(DC_PRED,       ipred_dc,       ssse3);
    init_angular_ipred_fn(DC_128_PRED,   ipred_dc_128,   ssse3);
    init_angular_ipred_fn(TOP_DC_PRED,   ipred_dc_top,   ssse3);
    init_angular_ipred_fn(LEFT_DC_PRED,  ipred_dc_left,  ssse3);
    init_angular_ipred_fn(HOR_PRED,      ipred_h,        ssse3);
    init_angular_ipred_fn(VERT_PRED,     ipred_v,        ssse3);
    init_angular_ipred_fn(PAETH_PRED,    ipred_paeth,    ssse3);
    init_angular_ipred_fn(SMOOTH_PRED,   ipred_smooth,   ssse3);
    init_angular_ipred_fn(SMOOTH_H_PRED, ipred_smooth_h, ssse3);
    init_angular_ipred_fn(SMOOTH_V_PRED, ipred_smooth_v, ssse3);
    init_angular_ipred_fn(Z1_PRED,       ipred_z1,       ssse3);
    init_angular_ipred_fn(Z2_PRED,       ipred_z2,       ssse3);
    init_angular_ipred_fn(Z3_PRED,       ipred_z3,       ssse3);
    init_angular_ipred_fn(FILTER_PRED,   ipred_filter,   ssse3);

    init_cfl_pred_fn(DC_PRED,      ipred_cfl,      ssse3);
    init_cfl_pred_fn(DC_128_PRED,  ipred_cfl_128,  ssse3);
    init_cfl_pred_fn(TOP_DC_PRED,  ipred_cfl_top,  ssse3);
    init_cfl_pred_fn(LEFT_DC_PRED, ipred_cfl_left, ssse3);

    init_cfl_ac_fn(DAV1D_PIXEL_LAYOUT_I420 - 1, ipred_cfl_ac_420, ssse3);
    init_cfl_ac_fn(DAV1D_PIXEL_LAYOUT_I422 - 1, ipred_cfl_ac_422, ssse3);
    init_cfl_ac_fn(DAV1D_PIXEL_LAYOUT_I444 - 1, ipred_cfl_ac_444, ssse3);

    c->pal_pred = BF(dav1d_pal_pred, ssse3);

#if ARCH_X86_64
    if (!(flags & DAV1D_X86_CPU_FLAG_AVX2)) return;

    init_angular_ipred_fn(DC_PRED,       ipred_dc,       avx2);
    init_angular_ipred_fn(DC_128_PRED,   ipred_dc_128,   avx2);
    init_angular_ipred_fn(TOP_DC_PRED,   ipred_dc_top,   avx2);
    init_angular_ipred_fn(LEFT_DC_PRED,  ipred_dc_left,  avx2);
    init_angular_ipred_fn(HOR_PRED,      ipred_h,        avx2);
    init_angular_ipred_fn(VERT_PRED,     ipred_v,        avx2);
    init_angular_ipred_fn(PAETH_PRED,    ipred_paeth,    avx2);
    init_angular_ipred_fn(SMOOTH_PRED,   ipred_smooth,   avx2);
    init_angular_ipred_fn(SMOOTH_H_PRED, ipred_smooth_h, avx2);
    init_angular_ipred_fn(SMOOTH_V_PRED, ipred_smooth_v, avx2);
    init_angular_ipred_fn(Z1_PRED,       ipred_z1,       avx2);
    init_angular_ipred_fn(Z2_PRED,       ipred_z2,       avx2);
    init_angular_ipred_fn(Z3_PRED,       ipred_z3,       avx2);
    init_angular_ipred_fn(FILTER_PRED,   ipred_filter,   avx2);

    init_cfl_pred_fn(DC_PRED,      ipred_cfl,      avx2);
    init_cfl_pred_fn(DC_128_PRED,  ipred_cfl_128,  avx2);
    init_cfl_pred_fn(TOP_DC_PRED,  ipred_cfl_top,  avx2);
    init_cfl_pred_fn(LEFT_DC_PRED, ipred_cfl_left, avx2);

    init_cfl_ac_fn(DAV1D_PIXEL_LAYOUT_I420 - 1, ipred_cfl_ac_420, avx2);
    init_cfl_ac_fn(DAV1D_PIXEL_LAYOUT_I422 - 1, ipred_cfl_ac_422, avx2);
    init_cfl_ac_fn(DAV1D_PIXEL_LAYOUT_I444 - 1, ipred_cfl_ac_444, avx2);

    c->pal_pred = BF(dav1d_pal_pred, avx2);

    if (!(flags & DAV1D_X86_CPU_FLAG_AVX512ICL)) return;

#if BITDEPTH == 8
    init_angular_ipred_fn(DC_PRED,       ipred_dc,       avx512icl);
    init_angular_ipred_fn(DC_128_PRED,   ipred_dc_128,   avx512icl);
    init_angular_ipred_fn(TOP_DC_PRED,   ipred_dc_top,   avx512icl);
    init_angular_ipred_fn(LEFT_DC_PRED,  ipred_dc_left,  avx512icl);
    init_angular_ipred_fn(HOR_PRED,      ipred_h,        avx512icl);
    init_angular_ipred_fn(VERT_PRED,     ipred_v,        avx512icl);
    init_angular_ipred_fn(Z2_PRED,       ipred_z2,       avx512icl);
#endif
    init_angular_ipred_fn(PAETH_PRED,    ipred_paeth,    avx512icl);
    init_angular_ipred_fn(SMOOTH_PRED,   ipred_smooth,   avx512icl);
    init_angular_ipred_fn(SMOOTH_H_PRED, ipred_smooth_h, avx512icl);
    init_angular_ipred_fn(SMOOTH_V_PRED, ipred_smooth_v, avx512icl);
    init_angular_ipred_fn(Z1_PRED,       ipred_z1,       avx512icl);
    init_angular_ipred_fn(Z2_PRED,       ipred_z2,       avx512icl);
    init_angular_ipred_fn(Z3_PRED,       ipred_z3,       avx512icl);
    init_angular_ipred_fn(FILTER_PRED,   ipred_filter,   avx512icl);

    c->pal_pred = BF(dav1d_pal_pred, avx512icl);
#endif
}
