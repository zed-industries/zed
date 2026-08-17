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

#ifndef DAV1D_SRC_LOONGARCH_LOOPRESTORATION_H
#define DAV1D_SRC_LOONGARCH_LOOPRESTORATION_H

#include "common/intops.h"
#include "src/cpu.h"
#include "src/looprestoration.h"

void dav1d_wiener_filter_lsx(uint8_t *p, const ptrdiff_t stride,
                             const uint8_t (*const left)[4],
                             const uint8_t *lpf,
                             const int w, const int h,
                             const LooprestorationParams *const params,
                             const enum LrEdgeFlags edges HIGHBD_DECL_SUFFIX);

void dav1d_wiener_filter_lasx(uint8_t *p, const ptrdiff_t stride,
                             const uint8_t (*const left)[4],
                             const uint8_t *lpf,
                             const int w, const int h,
                             const LooprestorationParams *const params,
                             const enum LrEdgeFlags edges HIGHBD_DECL_SUFFIX);

void dav1d_sgr_filter_3x3_lsx(pixel *p, const ptrdiff_t p_stride,
                              const pixel (*const left)[4],
                              const pixel *lpf,
                              const int w, const int h,
                              const LooprestorationParams *const params,
                              const enum LrEdgeFlags edges HIGHBD_DECL_SUFFIX);

void dav1d_sgr_filter_3x3_lasx(pixel *p, const ptrdiff_t p_stride,
                              const pixel (*const left)[4],
                              const pixel *lpf,
                              const int w, const int h,
                              const LooprestorationParams *const params,
                              const enum LrEdgeFlags edges HIGHBD_DECL_SUFFIX);

void dav1d_sgr_filter_5x5_lsx(pixel *p, const ptrdiff_t p_stride,
                              const pixel (*const left)[4],
                              const pixel *lpf,
                              const int w, const int h,
                              const LooprestorationParams *const params,
                              const enum LrEdgeFlags edges HIGHBD_DECL_SUFFIX);

void dav1d_sgr_filter_mix_lsx(pixel *p, const ptrdiff_t p_stride,
                              const pixel (*const left)[4],
                              const pixel *lpf,
                              const int w, const int h,
                              const LooprestorationParams *const params,
                              const enum LrEdgeFlags edges HIGHBD_DECL_SUFFIX);

void dav1d_sgr_filter_mix_lasx(pixel *p, const ptrdiff_t p_stride,
                               const pixel (*const left)[4],
                               const pixel *lpf,
                               const int w, const int h,
                               const LooprestorationParams *const params,
                               const enum LrEdgeFlags edges HIGHBD_DECL_SUFFIX);

static ALWAYS_INLINE void loop_restoration_dsp_init_loongarch(Dav1dLoopRestorationDSPContext *const c, int bpc)
{
    const unsigned flags = dav1d_get_cpu_flags();

    if (!(flags & DAV1D_LOONGARCH_CPU_FLAG_LSX)) return;

#if BITDEPTH == 8
    c->wiener[0] = c->wiener[1] = dav1d_wiener_filter_lsx;

    c->sgr[0] = dav1d_sgr_filter_5x5_lsx;
    c->sgr[1] = dav1d_sgr_filter_3x3_lsx;
    c->sgr[2] = dav1d_sgr_filter_mix_lsx;
#endif

    if (!(flags & DAV1D_LOONGARCH_CPU_FLAG_LASX)) return;

#if BITDEPTH == 8
    c->wiener[0] = c->wiener[1] = dav1d_wiener_filter_lasx;

    c->sgr[1] = dav1d_sgr_filter_3x3_lasx;
#endif
}

#endif /* DAV1D_SRC_LOONGARCH_LOOPRESTORATION_H */
