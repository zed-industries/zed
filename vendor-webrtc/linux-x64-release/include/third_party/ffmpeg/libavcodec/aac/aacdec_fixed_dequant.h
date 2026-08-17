/*
 * AAC decoder
 * Copyright (c) 2005-2006 Oded Shimon ( ods15 ods15 dyndns org )
 * Copyright (c) 2006-2007 Maxim Gavrilov ( maxim.gavrilov gmail com )
 * Copyright (c) 2008-2013 Alex Converse <alex.converse@gmail.com>
 *
 * AAC LATM decoder
 * Copyright (c) 2008-2010 Paul Kendall <paul@kcbbs.gen.nz>
 * Copyright (c) 2010      Janne Grunau <janne-libav@jannau.net>
 *
 * AAC decoder fixed-point implementation
 * Copyright (c) 2013
 *      MIPS Technologies, Inc., California.
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

#ifndef AVCODEC_AAC_AACDEC_FIXED_DEQUANT_H
#define AVCODEC_AAC_AACDEC_FIXED_DEQUANT_H

#include "aacdec_tab.h"

static void inline vector_pow43(int *coefs, int len)
{
    int i, coef;

    for (i=0; i<len; i++) {
        coef = coefs[i];
        if (coef < 0)
            coef = -(int)ff_cbrt_tab_fixed[(-coef) & 8191];
        else
            coef =  (int)ff_cbrt_tab_fixed[  coef  & 8191];
        coefs[i] = coef;
    }
}

/* 2^0, 2^0.25, 2^0.5, 2^0.75 */
static const int exp2tab[4] = {
    Q31(1.0000000000/2), Q31(1.1892071150/2),
    Q31(1.4142135624/2), Q31(1.6817928305/2)
};

static void inline subband_scale(int *dst, int *src, int scale,
                                 int offset, int len, void *log_context)
{
    int ssign = scale < 0 ? -1 : 1;
    int s = FFABS(scale);
    unsigned int round;
    int i, out, c = exp2tab[s & 3];

    s = offset - (s >> 2);

    if (s > 31) {
        for (i=0; i<len; i++) {
            dst[i] = 0;
        }
    } else if (s > 0) {
        round = 1 << (s-1);
        for (i=0; i<len; i++) {
            out = (int)(((int64_t)src[i] * c) >> 32);
            dst[i] = ((int)(out+round) >> s) * ssign;
        }
    } else if (s > -32) {
        s = s + 32;
        round = 1U << (s-1);
        for (i=0; i<len; i++) {
            out = (int)((int64_t)((int64_t)src[i] * c + round) >> s);
            dst[i] = out * (unsigned)ssign;
        }
    } else {
        av_log(log_context, AV_LOG_ERROR, "Overflow in subband_scale()\n");
    }
}

static void noise_scale(int *coefs, int scale, int band_energy, int len)
{
    int s = -scale;
    unsigned int round;
    int i, out, c = exp2tab[s & 3];
    int nlz = 0;

    av_assert0(s >= 0);
    while (band_energy > 0x7fff) {
        band_energy >>= 1;
        nlz++;
    }
    c /= band_energy;
    s = 21 + nlz - (s >> 2);

    if (s > 31) {
        for (i=0; i<len; i++) {
            coefs[i] = 0;
        }
    } else if (s >= 0) {
        round = s ? 1 << (s-1) : 0;
        for (i=0; i<len; i++) {
            out = (int)(((int64_t)coefs[i] * c) >> 32);
            coefs[i] = -((int)(out+round) >> s);
        }
    }
    else {
        s = s + 32;
        if (s > 0) {
            round = 1 << (s-1);
            for (i=0; i<len; i++) {
                out = (int)((int64_t)((int64_t)coefs[i] * c + round) >> s);
                coefs[i] = -out;
            }
        } else {
            for (i=0; i<len; i++)
                coefs[i] = -(int64_t)coefs[i] * c * (1 << -s);
        }
    }
}

static inline int *DEC_SPAIR(int *dst, unsigned idx)
{
    dst[0] = (idx & 15) - 4;
    dst[1] = (idx >> 4 & 15) - 4;

    return dst + 2;
}

static inline int *DEC_SQUAD(int *dst, unsigned idx)
{
    dst[0] = (idx & 3) - 1;
    dst[1] = (idx >> 2 & 3) - 1;
    dst[2] = (idx >> 4 & 3) - 1;
    dst[3] = (idx >> 6 & 3) - 1;

    return dst + 4;
}

static inline int *DEC_UPAIR(int *dst, unsigned idx, unsigned sign)
{
    dst[0] = (idx & 15) * (1 - (sign & 0xFFFFFFFE));
    dst[1] = (idx >> 4 & 15) * (1 - ((sign & 1) * 2));

    return dst + 2;
}

static inline int *DEC_UQUAD(int *dst, unsigned idx, unsigned sign)
{
    unsigned nz = idx >> 12;

    dst[0] = (idx & 3) * (1 + (((int)sign >> 31) * 2));
    sign <<= nz & 1;
    nz >>= 1;
    dst[1] = (idx >> 2 & 3) * (1 + (((int)sign >> 31) * 2));
    sign <<= nz & 1;
    nz >>= 1;
    dst[2] = (idx >> 4 & 3) * (1 + (((int)sign >> 31) * 2));
    sign <<= nz & 1;
    nz >>= 1;
    dst[3] = (idx >> 6 & 3) * (1 + (((int)sign >> 31) * 2));

    return dst + 4;
}

#endif /* AVCODEC_AAC_AACDEC_FIXED_DEQUANT_H */
