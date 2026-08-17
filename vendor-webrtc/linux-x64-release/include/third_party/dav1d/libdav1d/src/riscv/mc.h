/*
 * Copyright © 2024, VideoLAN and dav1d authors
 * Copyright © 2024, Nathan Egge
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
#include "src/mc.h"

decl_blend_fn(BF(dav1d_blend, rvv));
decl_blend_dir_fn(BF(dav1d_blend_h, rvv));
decl_blend_dir_fn(BF(dav1d_blend_v, rvv));

decl_blend_fn(BF(dav1d_blend_vl256, rvv));
decl_blend_dir_fn(BF(dav1d_blend_h_vl256, rvv));
decl_blend_dir_fn(BF(dav1d_blend_v_vl256, rvv));

decl_avg_fn(BF(dav1d_avg, rvv));
decl_w_avg_fn(BF(dav1d_w_avg, rvv));
decl_mask_fn(BF(dav1d_mask, rvv));

decl_warp8x8_fn(BF(dav1d_warp_8x8, rvv));
decl_warp8x8t_fn(BF(dav1d_warp_8x8t, rvv));

static ALWAYS_INLINE void mc_dsp_init_riscv(Dav1dMCDSPContext *const c) {
  const unsigned flags = dav1d_get_cpu_flags();

  if (!(flags & DAV1D_RISCV_CPU_FLAG_V)) return;

  c->blend = BF(dav1d_blend, rvv);
  c->blend_v = BF(dav1d_blend_v, rvv);

  if (dav1d_get_vlen() >= 256) {
    c->blend = BF(dav1d_blend_vl256, rvv);
    c->blend_v = BF(dav1d_blend_v_vl256, rvv);
  }

#if BITDEPTH == 8
  c->blend_h = BF(dav1d_blend_h, rvv);

  if (dav1d_get_vlen() >= 256) {
    c->blend_h = BF(dav1d_blend_h_vl256, rvv);
  }

  c->avg     = BF(dav1d_avg, rvv);
  c->w_avg   = BF(dav1d_w_avg, rvv);
  c->mask    = BF(dav1d_mask, rvv);

  c->warp8x8 = BF(dav1d_warp_8x8, rvv);
  c->warp8x8t = BF(dav1d_warp_8x8t, rvv);
#endif
}
