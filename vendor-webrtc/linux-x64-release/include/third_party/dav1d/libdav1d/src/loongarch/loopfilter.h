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

#ifndef DAV1D_SRC_LOONGARCH_LOOPFILTER_H
#define DAV1D_SRC_LOONGARCH_LOOPFILTER_H

#include "src/cpu.h"
#include "src/loopfilter.h"

decl_loopfilter_sb_fn(BF(dav1d_lpf_h_sb_y, lsx));
decl_loopfilter_sb_fn(BF(dav1d_lpf_v_sb_y, lsx));
decl_loopfilter_sb_fn(BF(dav1d_lpf_h_sb_uv, lsx));
decl_loopfilter_sb_fn(BF(dav1d_lpf_v_sb_uv, lsx));

static ALWAYS_INLINE void loop_filter_dsp_init_loongarch(Dav1dLoopFilterDSPContext *const c) {
    const unsigned flags = dav1d_get_cpu_flags();

    if (!(flags & DAV1D_LOONGARCH_CPU_FLAG_LSX)) return;

#if BITDEPTH == 8
    c->loop_filter_sb[0][0] = BF(dav1d_lpf_h_sb_y, lsx);
    c->loop_filter_sb[0][1] = BF(dav1d_lpf_v_sb_y, lsx);
    c->loop_filter_sb[1][0] = BF(dav1d_lpf_h_sb_uv, lsx);
    c->loop_filter_sb[1][1] = BF(dav1d_lpf_v_sb_uv, lsx);
#endif
}

#endif /* DAV1D_SRC_LOONGARCH_LOOPFILTER_H */
