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

#include "src/arm/asm-offsets.h"
#include "src/cpu.h"
#include "src/refmvs.h"

#if ARCH_AARCH64
CHECK_OFFSET(refmvs_frame, iw8, RMVSF_IW8);
CHECK_OFFSET(refmvs_frame, ih8, RMVSF_IH8);
CHECK_OFFSET(refmvs_frame, mfmv_ref, RMVSF_MFMV_REF);
CHECK_OFFSET(refmvs_frame, mfmv_ref2cur, RMVSF_MFMV_REF2CUR);
CHECK_OFFSET(refmvs_frame, mfmv_ref2ref, RMVSF_MFMV_REF2REF);
CHECK_OFFSET(refmvs_frame, n_mfmvs, RMVSF_N_MFMVS);
CHECK_OFFSET(refmvs_frame, rp_ref, RMVSF_RP_REF);
CHECK_OFFSET(refmvs_frame, rp_proj, RMVSF_RP_PROJ);
CHECK_OFFSET(refmvs_frame, rp_stride, RMVSF_RP_STRIDE);
CHECK_OFFSET(refmvs_frame, n_tile_threads, RMVSF_N_TILE_THREADS);
#endif

decl_load_tmvs_fn(dav1d_load_tmvs_neon);
decl_save_tmvs_fn(dav1d_save_tmvs_neon);
decl_splat_mv_fn(dav1d_splat_mv_neon);

static ALWAYS_INLINE void refmvs_dsp_init_arm(Dav1dRefmvsDSPContext *const c) {
    const unsigned flags = dav1d_get_cpu_flags();

    if (!(flags & DAV1D_ARM_CPU_FLAG_NEON)) return;

#if ARCH_AARCH64
    c->load_tmvs = dav1d_load_tmvs_neon;
#endif
    c->save_tmvs = dav1d_save_tmvs_neon;
    c->splat_mv = dav1d_splat_mv_neon;
}
