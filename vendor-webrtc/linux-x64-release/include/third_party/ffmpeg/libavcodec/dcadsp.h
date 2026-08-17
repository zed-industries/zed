/*
 * Copyright (C) 2016 foo86
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef AVCODEC_DCADSP_H
#define AVCODEC_DCADSP_H

#include "libavutil/common.h"
#include "libavutil/tx.h"

#include "dcadct.h"
#include "synth_filter.h"

typedef struct DCADSPContext {
    void (*decode_hf)(int32_t **dst,
                      const int32_t *vq_index,
                      const int8_t hf_vq[1024][32],
                      int32_t scale_factors[32][2],
                      ptrdiff_t sb_start, ptrdiff_t sb_end,
                      ptrdiff_t ofs, ptrdiff_t len);

    void (*decode_joint)(int32_t **dst, int32_t **src,
                         const int32_t *scale_factors,
                         ptrdiff_t sb_start, ptrdiff_t sb_end,
                         ptrdiff_t ofs, ptrdiff_t len);

    void (*lfe_fir_float[2])(float *pcm_samples, int32_t *lfe_samples,
                             const float *filter_coeff, ptrdiff_t npcmblocks);

    void (*lfe_x96_float)(float *dst, const float *src,
                          float *hist, ptrdiff_t len);

    void (*sub_qmf_float[2])(SynthFilterContext *synth,
                             AVTXContext *imdct,
                             av_tx_fn imdct_fn,
                             float *pcm_samples,
                             int32_t **subband_samples_lo,
                             int32_t **subband_samples_hi,
                             float *hist1, int *offset, float *hist2,
                             const float *filter_coeff, ptrdiff_t npcmblocks,
                             float scale);

    void (*lfe_fir_fixed)(int32_t *pcm_samples, int32_t *lfe_samples,
                          const int32_t *filter_coeff, ptrdiff_t npcmblocks);

    void (*lfe_x96_fixed)(int32_t *dst, const int32_t *src,
                          int32_t *hist, ptrdiff_t len);

    void (*sub_qmf_fixed[2])(SynthFilterContext *synth,
                             DCADCTContext *imdct,
                             int32_t *pcm_samples,
                             int32_t **subband_samples_lo,
                             int32_t **subband_samples_hi,
                             int32_t *hist1, int *offset, int32_t *hist2,
                             const int32_t *filter_coeff, ptrdiff_t npcmblocks);

    void (*decor)(int32_t *dst, const int32_t *src, int coeff, ptrdiff_t len);

    void (*dmix_sub_xch)(int32_t *dst1, int32_t *dst2,
                         const int32_t *src, ptrdiff_t len);

    void (*dmix_sub)(int32_t *dst, const int32_t *src, int coeff, ptrdiff_t len);

    void (*dmix_add)(int32_t *dst, const int32_t *src, int coeff, ptrdiff_t len);

    void (*dmix_scale)(int32_t *dst, int scale, ptrdiff_t len);

    void (*dmix_scale_inv)(int32_t *dst, int scale_inv, ptrdiff_t len);

    void (*assemble_freq_bands)(int32_t *dst, int32_t *src0, int32_t *src1,
                                const int32_t *coeff, ptrdiff_t len);

    void (*lbr_bank)(float output[32][4], float **input,
                     const float *coeff, ptrdiff_t ofs, ptrdiff_t len);

    void (*lfe_iir)(float *output, const float *input,
                    const float iir[5][4], float hist[5][2],
                    ptrdiff_t factor);
} DCADSPContext;

av_cold void ff_dcadsp_init(DCADSPContext *s);
av_cold void ff_dcadsp_init_x86(DCADSPContext *s);

#endif
