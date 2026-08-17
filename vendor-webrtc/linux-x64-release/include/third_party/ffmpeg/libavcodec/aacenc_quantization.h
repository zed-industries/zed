/*
 * AAC encoder quantizer
 * Copyright (C) 2015 Rostislav Pehlivanov
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

/**
 * @file
 * AAC encoder quantizer
 * @author Rostislav Pehlivanov ( atomnuker gmail com )
 */

#ifndef AVCODEC_AACENC_QUANTIZATION_H
#define AVCODEC_AACENC_QUANTIZATION_H

#include <stddef.h>

#include "aacenc.h"
#include "put_bits.h"


float ff_quantize_and_encode_band_cost(AACEncContext *s, PutBitContext *pb,
                                       const float *in, float *quant, const float *scaled,
                                       int size, int scale_idx, int cb,
                                       const float lambda, const float uplim,
                                       int *bits, float *energy);

static inline float quantize_band_cost(struct AACEncContext *s, const float *in,
                                const float *scaled, int size, int scale_idx,
                                int cb, const float lambda, const float uplim,
                                int *bits, float *energy)
{
    return ff_quantize_and_encode_band_cost(s, NULL, in, NULL, scaled, size, scale_idx,
                                            cb, lambda, uplim, bits, energy);
}

static inline int quantize_band_cost_bits(struct AACEncContext *s, const float *in,
                                const float *scaled, int size, int scale_idx,
                                int cb, const float lambda, const float uplim,
                                int *bits, float *energy)
{
    int auxbits;
    ff_quantize_and_encode_band_cost(s, NULL, in, NULL, scaled, size, scale_idx,
                                     cb, 0.0f, uplim, &auxbits, energy);
    if (bits) {
        *bits = auxbits;
    }
    return auxbits;
}

#include "aacenc_quantization_misc.h"

#endif /* AVCODEC_AACENC_QUANTIZATION_H */
