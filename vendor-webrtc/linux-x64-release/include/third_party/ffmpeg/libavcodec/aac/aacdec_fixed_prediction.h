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

#ifndef AVCODEC_AAC_AACDEC_FIXED_PREDICTION_H
#define AVCODEC_AAC_AACDEC_FIXED_PREDICTION_H

static av_always_inline SoftFloat flt16_round(SoftFloat pf)
{
    SoftFloat tmp;
    int s;

    tmp.exp = pf.exp;
    s = pf.mant >> 31;
    tmp.mant = (pf.mant ^ s) - s;
    tmp.mant = (tmp.mant + 0x00200000U) & 0xFFC00000U;
    tmp.mant = (tmp.mant ^ s) - s;

    return tmp;
}

static av_always_inline SoftFloat flt16_even(SoftFloat pf)
{
    SoftFloat tmp;
    int s;

    tmp.exp = pf.exp;
    s = pf.mant >> 31;
    tmp.mant = (pf.mant ^ s) - s;
    tmp.mant = (tmp.mant + 0x001FFFFFU + (tmp.mant & 0x00400000U >> 16)) & 0xFFC00000U;
    tmp.mant = (tmp.mant ^ s) - s;

    return tmp;
}

static av_always_inline SoftFloat flt16_trunc(SoftFloat pf)
{
    SoftFloat pun;
    int s;

    pun.exp = pf.exp;
    s = pf.mant >> 31;
    pun.mant = (pf.mant ^ s) - s;
    pun.mant = pun.mant & 0xFFC00000U;
    pun.mant = (pun.mant ^ s) - s;

    return pun;
}

static av_always_inline void predict(PredictorState *ps, int *coef,
                                     int output_enable)
{
    const SoftFloat a     = { 1023410176, 0 };  // 61.0 / 64
    const SoftFloat alpha = {  973078528, 0 };  // 29.0 / 32
    SoftFloat e0, e1;
    SoftFloat pv;
    SoftFloat k1, k2;
    SoftFloat   r0 = ps->r0,     r1 = ps->r1;
    SoftFloat cor0 = ps->cor0, cor1 = ps->cor1;
    SoftFloat var0 = ps->var0, var1 = ps->var1;
    SoftFloat tmp;

    if (var0.exp > 1 || (var0.exp == 1 && var0.mant > 0x20000000)) {
        k1 = av_mul_sf(cor0, flt16_even(av_div_sf(a, var0)));
    }
    else {
        k1.mant = 0;
        k1.exp = 0;
    }

    if (var1.exp > 1 || (var1.exp == 1 && var1.mant > 0x20000000)) {
        k2 = av_mul_sf(cor1, flt16_even(av_div_sf(a, var1)));
    }
    else {
        k2.mant = 0;
        k2.exp = 0;
    }

    tmp = av_mul_sf(k1, r0);
    pv = flt16_round(av_add_sf(tmp, av_mul_sf(k2, r1)));
    if (output_enable) {
        int shift = 28 - pv.exp;

        if (shift < 31) {
            if (shift > 0) {
                *coef += (unsigned)((pv.mant + (1 << (shift - 1))) >> shift);
            } else
                *coef += (unsigned)pv.mant << -shift;
        }
    }

    e0 = av_int2sf(*coef, 2);
    e1 = av_sub_sf(e0, tmp);

    ps->cor1 = flt16_trunc(av_add_sf(av_mul_sf(alpha, cor1), av_mul_sf(r1, e1)));
    tmp = av_add_sf(av_mul_sf(r1, r1), av_mul_sf(e1, e1));
    tmp.exp--;
    ps->var1 = flt16_trunc(av_add_sf(av_mul_sf(alpha, var1), tmp));
    ps->cor0 = flt16_trunc(av_add_sf(av_mul_sf(alpha, cor0), av_mul_sf(r0, e0)));
    tmp = av_add_sf(av_mul_sf(r0, r0), av_mul_sf(e0, e0));
    tmp.exp--;
    ps->var0 = flt16_trunc(av_add_sf(av_mul_sf(alpha, var0), tmp));

    ps->r1 = flt16_trunc(av_mul_sf(a, av_sub_sf(r0, av_mul_sf(k1, e0))));
    ps->r0 = flt16_trunc(av_mul_sf(a, e0));
}

static av_always_inline void reset_predict_state(PredictorState *ps)
{
    ps->r0.mant   = 0;
    ps->r0.exp   = 0;
    ps->r1.mant   = 0;
    ps->r1.exp   = 0;
    ps->cor0.mant = 0;
    ps->cor0.exp = 0;
    ps->cor1.mant = 0;
    ps->cor1.exp = 0;
    ps->var0.mant = 0x20000000;
    ps->var0.exp = 1;
    ps->var1.mant = 0x20000000;
    ps->var1.exp = 1;
}

#endif /* AVCODEC_AAC_AACDEC_FIXED_PREDICTION_H */
