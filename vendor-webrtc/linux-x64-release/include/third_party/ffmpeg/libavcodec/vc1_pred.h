/*
 * VC-1 and WMV3 decoder
 * Copyright (c) 2006-2007 Konstantin Shishkov
 * Partly based on vc9.c (c) 2005 Anonymous, Alex Beregszaszi, Michael Niedermayer
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

#ifndef AVCODEC_VC1_PRED_H
#define AVCODEC_VC1_PRED_H

#include "vc1.h"
#include "vc1data.h"

void ff_vc1_pred_mv(VC1Context *v, int n, int dmv_x, int dmv_y,
                    int mv1, int r_x, int r_y, uint8_t* is_intra,
                    int pred_flag, int dir);
void ff_vc1_pred_mv_intfr(VC1Context *v, int n, int dmv_x, int dmv_y,
                          int mvn, int r_x, int r_y, int dir);
void ff_vc1_pred_b_mv(VC1Context *v, int dmv_x[2], int dmv_y[2],
                      int direct, int mvtype);
void ff_vc1_pred_b_mv_intfi(VC1Context *v, int n, int *dmv_x, int *dmv_y,
                            int mv1, int *pred_flag);

static av_always_inline int scale_mv(int value, int bfrac, int inv, int qs)
{
    int n = bfrac;

#if B_FRACTION_DEN==256
    if (inv)
        n -= 256;
    if (!qs)
        return 2 * ((value * n + 255) >> 9);
    return (value * n + 128) >> 8;
#else
    if (inv)
        n -= B_FRACTION_DEN;
    if (!qs)
        return 2 * ((value * n + B_FRACTION_DEN - 1) / (2 * B_FRACTION_DEN));
    return (value * n + B_FRACTION_DEN/2) / B_FRACTION_DEN;
#endif
}

#endif /* AVCODEC_VC1_PRED_H */
