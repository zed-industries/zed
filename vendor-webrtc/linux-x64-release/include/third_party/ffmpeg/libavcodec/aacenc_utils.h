/*
 * AAC encoder utilities
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
 * AAC encoder utilities
 * @author Rostislav Pehlivanov ( atomnuker gmail com )
 */

#ifndef AVCODEC_AACENC_UTILS_H
#define AVCODEC_AACENC_UTILS_H

#include "libavutil/ffmath.h"
#include "aacenc.h"
#include "aacenctab.h"
#include "aactab.h"

#define ROUND_STANDARD 0.4054f
#define ROUND_TO_ZERO 0.1054f
#define C_QUANT 0.4054f

static inline float pos_pow34(float a)
{
    return sqrtf(a * sqrtf(a));
}

/**
 * Quantize one coefficient.
 * @return absolute value of the quantized coefficient
 * @see 3GPP TS26.403 5.6.2 "Scalefactor determination"
 */
static inline int quant(float coef, const float Q, const float rounding)
{
    float a = coef * Q;
    return sqrtf(a * sqrtf(a)) + rounding;
}

static inline float find_max_val(int group_len, int swb_size, const float *scaled)
{
    float maxval = 0.0f;
    int w2, i;
    for (w2 = 0; w2 < group_len; w2++) {
        for (i = 0; i < swb_size; i++) {
            maxval = FFMAX(maxval, scaled[w2*128+i]);
        }
    }
    return maxval;
}

static inline int find_min_book(float maxval, int sf)
{
    float Q34 = ff_aac_pow34sf_tab[POW_SF2_ZERO - sf + SCALE_ONE_POS - SCALE_DIV_512];
    int qmaxval, cb;
    qmaxval = maxval * Q34 + C_QUANT;
    if (qmaxval >= (FF_ARRAY_ELEMS(aac_maxval_cb)))
        cb = 11;
    else
        cb = aac_maxval_cb[qmaxval];
    return cb;
}

static inline float find_form_factor(int group_len, int swb_size, float thresh,
                                     const float *scaled, float nzslope) {
    const float iswb_size = 1.0f / swb_size;
    const float iswb_sizem1 = 1.0f / (swb_size - 1);
    const float ethresh = thresh;
    float form = 0.0f, weight = 0.0f;
    int w2, i;
    for (w2 = 0; w2 < group_len; w2++) {
        float e = 0.0f, e2 = 0.0f, var = 0.0f, maxval = 0.0f;
        float nzl = 0;
        for (i = 0; i < swb_size; i++) {
            float s = fabsf(scaled[w2*128+i]);
            maxval = FFMAX(maxval, s);
            e += s;
            e2 += s *= s;
            /* We really don't want a hard non-zero-line count, since
             * even below-threshold lines do add up towards band spectral power.
             * So, fall steeply towards zero, but smoothly
             */
            if (s >= ethresh) {
                nzl += 1.0f;
            } else {
                if (nzslope == 2.f)
                    nzl += (s / ethresh) * (s / ethresh);
                else
                    nzl += ff_fast_powf(s / ethresh, nzslope);
            }
        }
        if (e2 > thresh) {
            float frm;
            e *= iswb_size;

            /** compute variance */
            for (i = 0; i < swb_size; i++) {
                float d = fabsf(scaled[w2*128+i]) - e;
                var += d*d;
            }
            var = sqrtf(var * iswb_sizem1);

            e2 *= iswb_size;
            frm = e / FFMIN(e+4*var,maxval);
            form += e2 * sqrtf(frm) / FFMAX(0.5f,nzl);
            weight += e2;
        }
    }
    if (weight > 0) {
        return form / weight;
    } else {
        return 1.0f;
    }
}

/** Return the minimum scalefactor where the quantized coef does not clip. */
static inline uint8_t coef2minsf(float coef)
{
    return av_clip_uint8(log2f(coef)*4 - 69 + SCALE_ONE_POS - SCALE_DIV_512);
}

/** Return the maximum scalefactor where the quantized coef is not zero. */
static inline uint8_t coef2maxsf(float coef)
{
    return av_clip_uint8(log2f(coef)*4 +  6 + SCALE_ONE_POS - SCALE_DIV_512);
}

/*
 * Returns the closest possible index to an array of float values, given a value.
 */
static inline int quant_array_idx(const float val, const float *arr, const int num)
{
    int i, index = 0;
    float quant_min_err = INFINITY;
    for (i = 0; i < num; i++) {
        float error = (val - arr[i])*(val - arr[i]);
        if (error < quant_min_err) {
            quant_min_err = error;
            index = i;
        }
    }
    return index;
}

/**
 * approximates exp10f(-3.0f*(0.5f + 0.5f * cosf(FFMIN(b,15.5f) / 15.5f)))
 */
static av_always_inline float bval2bmax(float b)
{
    return 0.001f + 0.0035f * (b*b*b) / (15.5f*15.5f*15.5f);
}

/*
 * Compute a nextband map to be used with SF delta constraint utilities.
 * The nextband array should contain 128 elements, and positions that don't
 * map to valid, nonzero bands of the form w*16+g (with w being the initial
 * window of the window group, only) are left indetermined.
 */
static inline void ff_init_nextband_map(const SingleChannelElement *sce, uint8_t *nextband)
{
    unsigned char prevband = 0;
    int w, g;
    /** Just a safe default */
    for (g = 0; g < 128; g++)
        nextband[g] = g;

    /** Now really navigate the nonzero band chain */
    for (w = 0; w < sce->ics.num_windows; w += sce->ics.group_len[w]) {
        for (g = 0; g < sce->ics.num_swb; g++) {
            if (!sce->zeroes[w*16+g] && sce->band_type[w*16+g] < RESERVED_BT)
                prevband = nextband[prevband] = w*16+g;
        }
    }
    nextband[prevband] = prevband; /* terminate */
}

/*
 * Updates nextband to reflect a removed band (equivalent to
 * calling ff_init_nextband_map after marking a band as zero)
 */
static inline void ff_nextband_remove(uint8_t *nextband, int prevband, int band)
{
    nextband[prevband] = nextband[band];
}

/*
 * Checks whether the specified band could be removed without inducing
 * scalefactor delta that violates SF delta encoding constraints.
 * prev_sf has to be the scalefactor of the previous nonzero, nonspecial
 * band, in encoding order, or negative if there was no such band.
 */
static inline int ff_sfdelta_can_remove_band(const SingleChannelElement *sce,
    const uint8_t *nextband, int prev_sf, int band)
{
    return prev_sf >= 0
        && sce->sf_idx[nextband[band]] >= (prev_sf - SCALE_MAX_DIFF)
        && sce->sf_idx[nextband[band]] <= (prev_sf + SCALE_MAX_DIFF);
}

/*
 * Checks whether the specified band's scalefactor could be replaced
 * with another one without violating SF delta encoding constraints.
 * prev_sf has to be the scalefactor of the previous nonzero, nonsepcial
 * band, in encoding order, or negative if there was no such band.
 */
static inline int ff_sfdelta_can_replace(const SingleChannelElement *sce,
    const uint8_t *nextband, int prev_sf, int new_sf, int band)
{
    return new_sf >= (prev_sf - SCALE_MAX_DIFF)
        && new_sf <= (prev_sf + SCALE_MAX_DIFF)
        && sce->sf_idx[nextband[band]] >= (new_sf - SCALE_MAX_DIFF)
        && sce->sf_idx[nextband[band]] <= (new_sf + SCALE_MAX_DIFF);
}

/**
 * linear congruential pseudorandom number generator
 *
 * @param   previous_val    pointer to the current state of the generator
 *
 * @return  Returns a 32-bit pseudorandom integer
 */
static av_always_inline int lcg_random(unsigned previous_val)
{
    union { unsigned u; int s; } v = { previous_val * 1664525u + 1013904223 };
    return v.s;
}

#define ERROR_IF(cond, ...) \
    if (cond) { \
        av_log(avctx, AV_LOG_ERROR, __VA_ARGS__); \
        return AVERROR(EINVAL); \
    }

#define WARN_IF(cond, ...) \
    if (cond) { \
        av_log(avctx, AV_LOG_WARNING, __VA_ARGS__); \
    }

#endif /* AVCODEC_AACENC_UTILS_H */
