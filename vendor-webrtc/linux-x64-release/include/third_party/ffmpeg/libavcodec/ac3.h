/*
 * Common code between the AC-3 encoder and decoder
 * Copyright (c) 2000, 2001, 2002 Fabrice Bellard
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
 * Common code between the AC-3 encoder and decoder.
 */

#ifndef AVCODEC_AC3_H
#define AVCODEC_AC3_H

#include <math.h>
#include <stdint.h>

#include "ac3tab.h"

#ifndef USE_FIXED
#define USE_FIXED 0
#endif

#if USE_FIXED

#define FIXR(a)                 ((int)((a) * 0 + 0.5))
#define FIXR12(a)               ((int)((a) * 4096 + 0.5))
#define FIXR15(a)               ((int)((a) * 32768 + 0.5))
#define ROUND15(x)              ((x) + 16384) >> 15

#define AC3_RENAME(x)           x ## _fixed
#define AC3_NORM(norm)          (1<<24)/(norm)
#define AC3_MUL(a,b)            ((((int64_t) (a)) * (b))>>12)
#define AC3_RANGE(x)            ((x)|(((x)&128)<<1))
#define AC3_HEAVY_RANGE(x)      ((x)<<1)
#define AC3_DYNAMIC_RANGE(x)    (x)
#define AC3_SPX_BLEND(x)        (x)
#define AC3_DYNAMIC_RANGE1      0

typedef int                     INTFLOAT;
typedef unsigned int            UINTFLOAT;
typedef int16_t                 SHORTFLOAT;

#else /* USE_FIXED */
#include "libavutil/libm.h"

#define FIXR(x)                 ((float)(x))
#define FIXR12(x)               ((float)(x))
#define FIXR15(x)               ((float)(x))
#define ROUND15(x)              (x)

#define AC3_RENAME(x)           x
#define AC3_NORM(norm)          (1.0f/(norm))
#define AC3_MUL(a,b)            ((a) * (b))
#define AC3_RANGE(x)            (dynamic_range_tab[(x)])
#define AC3_HEAVY_RANGE(x)      (ff_ac3_heavy_dynamic_range_tab[(x)])
#define AC3_DYNAMIC_RANGE(x)    (powf(x,  s->drc_scale))
#define AC3_SPX_BLEND(x)        (x)* (1.0f/32)
#define AC3_DYNAMIC_RANGE1      1.0f

typedef float                   INTFLOAT;
typedef float                   UINTFLOAT;
typedef float                   SHORTFLOAT;

#endif /* USE_FIXED */

#define AC3_LEVEL(x)            ROUND15((x) * FIXR15(M_SQRT1_2))

/* pre-defined gain values */
#define LEVEL_PLUS_3DB          M_SQRT2
#define LEVEL_PLUS_1POINT5DB    1.1892071150027209
#define LEVEL_MINUS_1POINT5DB   0.8408964152537145
#define LEVEL_MINUS_3DB         M_SQRT1_2
#define LEVEL_MINUS_4POINT5DB   0.5946035575013605
#define LEVEL_MINUS_6DB         0.5000000000000000
#define LEVEL_MINUS_9DB         0.3535533905932738
#define LEVEL_ZERO              0.0000000000000000
#define LEVEL_ONE               1.0000000000000000

typedef struct AC3BitAllocParameters {
    int sr_code;
    int sr_shift;
    int slow_gain, slow_decay, fast_decay, db_per_bit, floor;
    int cpl_fast_leak, cpl_slow_leak;
} AC3BitAllocParameters;

/**
 * Calculate the log power-spectral density of the input signal.
 * This gives a rough estimate of signal power in the frequency domain by using
 * the spectral envelope (exponents).  The psd is also separately grouped
 * into critical bands for use in the calculating the masking curve.
 * 128 units in psd = -6 dB.  The dbknee parameter in AC3BitAllocParameters
 * determines the reference level.
 *
 * @param[in]  exp        frequency coefficient exponents
 * @param[in]  start      starting bin location
 * @param[in]  end        ending bin location
 * @param[out] psd        signal power for each frequency bin
 * @param[out] band_psd   signal power for each critical band
 */
void ff_ac3_bit_alloc_calc_psd(int8_t *exp, int start, int end, int16_t *psd,
                               int16_t *band_psd);

/**
 * Calculate the masking curve.
 * First, the excitation is calculated using parameters in s and the signal
 * power in each critical band.  The excitation is compared with a predefined
 * hearing threshold table to produce the masking curve.  If delta bit
 * allocation information is provided, it is used for adjusting the masking
 * curve, usually to give a closer match to a better psychoacoustic model.
 *
 * @param[in]  s            adjustable bit allocation parameters
 * @param[in]  band_psd     signal power for each critical band
 * @param[in]  start        starting bin location
 * @param[in]  end          ending bin location
 * @param[in]  fast_gain    fast gain (estimated signal-to-mask ratio)
 * @param[in]  is_lfe       whether or not the channel being processed is the LFE
 * @param[in]  dba_mode     delta bit allocation mode (none, reuse, or new)
 * @param[in]  dba_nsegs    number of delta segments
 * @param[in]  dba_offsets  location offsets for each segment
 * @param[in]  dba_lengths  length of each segment
 * @param[in]  dba_values   delta bit allocation for each segment
 * @param[out] mask         calculated masking curve
 * @return returns 0 for success, non-zero for error
 */
int ff_ac3_bit_alloc_calc_mask(AC3BitAllocParameters *s, int16_t *band_psd,
                               int start, int end, int fast_gain, int is_lfe,
                               int dba_mode, int dba_nsegs, uint8_t *dba_offsets,
                               uint8_t *dba_lengths, uint8_t *dba_values,
                               int16_t *mask);

#endif /* AVCODEC_AC3_H */
