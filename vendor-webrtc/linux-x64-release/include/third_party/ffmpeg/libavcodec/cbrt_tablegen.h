/*
 * Header file for hardcoded AAC cube-root table
 *
 * Copyright (c) 2010 Reimar Döffinger <Reimar.Doeffinger@gmx.de>
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

#ifndef AVCODEC_CBRT_TABLEGEN_H
#define AVCODEC_CBRT_TABLEGEN_H

#include <stdint.h>
#include <math.h>
#include "libavutil/attributes.h"
#include "libavutil/intfloat.h"
#include "libavcodec/aac_defines.h"

#if USE_FIXED
#define CBRT(x) lrint((x) * 8192)
#else
#define CBRT(x) av_float2int((float)(x))
#endif

uint32_t AAC_RENAME(ff_cbrt_tab)[1 << 13];

av_cold void AAC_RENAME(ff_cbrt_tableinit)(void)
{
    static double cbrt_tab_dbl[1 << 13];
    if (!AAC_RENAME(ff_cbrt_tab)[(1<<13) - 1]) {
        int i, j, k;
        double cbrt_val;

        for (i = 1; i < 1<<13; i++)
            cbrt_tab_dbl[i] = 1;

        /* have to take care of non-squarefree numbers */
        for (i = 2; i < 90; i++) {
            if (cbrt_tab_dbl[i] == 1) {
                cbrt_val = i * cbrt(i);
                for (k = i; k < 1<<13; k *= i)
                    for (j = k; j < 1<<13; j += k)
                        cbrt_tab_dbl[j] *= cbrt_val;
            }
        }

        for (i = 91; i <= 8191; i+= 2) {
            if (cbrt_tab_dbl[i] == 1) {
                cbrt_val = i * cbrt(i);
                for (j = i; j < 1<<13; j += i)
                    cbrt_tab_dbl[j] *= cbrt_val;
            }
        }

        for (i = 0; i < 1<<13; i++)
            AAC_RENAME(ff_cbrt_tab)[i] = CBRT(cbrt_tab_dbl[i]);
    }
}

#endif /* AVCODEC_CBRT_TABLEGEN_H */
