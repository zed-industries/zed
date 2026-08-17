/*
 * DCA ADPCM engine
 * Copyright (C) 2017 Daniil Cherednik
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

#ifndef AVCODEC_DCAADPCM_H
#define AVCODEC_DCAADPCM_H

#include "dcamath.h"
#include "dcadata.h"
#include "dcaenc.h"

typedef struct DCAADPCMEncContext {
    void *private_data;
} DCAADPCMEncContext;

static inline int64_t ff_dcaadpcm_predict(int pred_vq_index, const int32_t *input)
{
    int i;
    const int16_t *coeff = ff_dca_adpcm_vb[pred_vq_index];
    int64_t pred = 0;
    for (i = 0; i < DCA_ADPCM_COEFFS; i++)
        pred += (int64_t)input[DCA_ADPCM_COEFFS - 1 - i] * coeff[i];

    return clip23(norm13(pred));
}

int ff_dcaadpcm_subband_analysis(const DCAADPCMEncContext *s, const int32_t *input, int len, int *diff);

int ff_dcaadpcm_do_real(int pred_vq_index,
                        softfloat quant, int32_t scale_factor, int32_t step_size,
                        const int32_t *prev_hist, const int32_t *in, int32_t *next_hist, int32_t *out,
                        int len, int32_t peak);

av_cold int ff_dcaadpcm_init(DCAADPCMEncContext *s);
av_cold void ff_dcaadpcm_free(DCAADPCMEncContext *s);

#endif /* AVCODEC_DCAADPCM_H */
