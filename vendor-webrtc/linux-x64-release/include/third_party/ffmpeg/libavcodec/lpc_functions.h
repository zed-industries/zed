/*
 * LPC utility functions
 * Copyright (c) 2006  Justin Ruggles <justin.ruggles@gmail.com>
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

#ifndef AVCODEC_LPC_FUNCTIONS_H
#define AVCODEC_LPC_FUNCTIONS_H

#include "libavutil/avassert.h"

#ifndef LPC_USE_FIXED
#define LPC_USE_FIXED 0
#endif

#if LPC_USE_FIXED
typedef int LPC_TYPE;
typedef unsigned LPC_TYPE_U;
#else
#ifndef LPC_SRA_R
#define LPC_SRA_R(x, y) (x)
#define LPC_MUL26(x, y) ((x) * (y))
#define LPC_FIXR(x)     ((float)(x))
#endif

#ifdef LPC_USE_DOUBLE
typedef double LPC_TYPE;
typedef double LPC_TYPE_U;
#else
typedef float LPC_TYPE;
typedef float LPC_TYPE_U;
#endif
#endif // USE_FIXED

/**
 * Levinson-Durbin recursion.
 * Produce LPC coefficients from autocorrelation data.
 */
static inline int compute_lpc_coefs(const LPC_TYPE *autoc, int max_order,
                                    LPC_TYPE *lpc, int lpc_stride, int fail,
                                    int normalize)
{
    LPC_TYPE err = 0;
    LPC_TYPE *lpc_last = lpc;

    av_assert2(normalize || !fail);

    if (normalize)
        err = *autoc++;

    if (fail && (autoc[max_order - 1] == 0 || err <= 0))
        return -1;

    for(int i = 0; i < max_order; i++) {
        LPC_TYPE r = LPC_SRA_R(-autoc[i], 5);

        if (normalize) {
            for(int j = 0; j < i; j++)
                r -= lpc_last[j] * autoc[i-j-1];

            if (err)
                r /= err;
            err *= LPC_FIXR(1.0) - (r * r);
        }

        lpc[i] = r;

        for(int j = 0; j < (i + 1) >> 1; j++) {
            LPC_TYPE f = lpc_last[    j];
            LPC_TYPE b = lpc_last[i-1-j];
            lpc[    j] = f + (LPC_TYPE_U)LPC_MUL26(r, b);
            lpc[i-1-j] = b + (LPC_TYPE_U)LPC_MUL26(r, f);
        }

        if (fail && err < 0)
            return -1;

        lpc_last = lpc;
        lpc += lpc_stride;
    }

    return 0;
}

#endif /* AVCODEC_LPC_FUNCTIONS_H */
