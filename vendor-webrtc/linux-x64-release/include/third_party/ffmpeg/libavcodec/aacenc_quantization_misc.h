/*
 * AAC encoder quantization
 * Copyright (C) 2015 Claudio Freire
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
 * AAC encoder quantization misc reusable function templates
 * @author Claudio Freire ( klaussfreire gmail com )
 */

#ifndef AVCODEC_AACENC_QUANTIZATION_MISC_H
#define AVCODEC_AACENC_QUANTIZATION_MISC_H

static inline float quantize_band_cost_cached(struct AACEncContext *s, int w, int g, const float *in,
                                const float *scaled, int size, int scale_idx,
                                int cb, const float lambda, const float uplim,
                                int *bits, float *energy, int rtz)
{
    AACQuantizeBandCostCacheEntry *entry;
    av_assert1(scale_idx >= 0 && scale_idx < 256);
    entry = &s->quantize_band_cost_cache[scale_idx][w*16+g];
    if (entry->generation != s->quantize_band_cost_cache_generation || entry->cb != cb || entry->rtz != rtz) {
        entry->rd = quantize_band_cost(s, in, scaled, size, scale_idx,
                                       cb, lambda, uplim, &entry->bits, &entry->energy);
        entry->cb = cb;
        entry->rtz = rtz;
        entry->generation = s->quantize_band_cost_cache_generation;
    }
    if (bits)
        *bits = entry->bits;
    if (energy)
        *energy = entry->energy;
    return entry->rd;
}

#endif /* AVCODEC_AACENC_QUANTIZATION_MISC_H */
