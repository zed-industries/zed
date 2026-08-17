/*
 * Copyright (c) 2012
 *      MIPS Technologies, Inc., California.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the MIPS Technologies, Inc., nor the names of its
 *    contributors may be used to endorse or promote products derived from
 *    this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE MIPS TECHNOLOGIES, INC. ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE MIPS TECHNOLOGIES, INC. BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * Author:  Nedeljko Babic (nbabic@mips.com)
 *
 * LSP routines for ACELP-based codecs optimized for MIPS
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
 * Reference: libavcodec/lsp.c
 */
#ifndef AVCODEC_MIPS_LSP_MIPS_H
#define AVCODEC_MIPS_LSP_MIPS_H

#include "config.h"

#if HAVE_MIPSFPU && HAVE_INLINE_ASM
#if !HAVE_MIPS32R6 && !HAVE_MIPS64R6
#include "libavutil/attributes.h"
#include "libavutil/mips/asmdefs.h"

static av_always_inline void lsp2polyf_mips(const double *lsp, double *f, int lp_half_order)
{
    int i, j = 0;
    double * p_fi = f;
    double * p_f = 0;

    f[0] = 1.0;
    f[1] = -2 * lsp[0];
    lsp -= 2;

    for(i=2; i<=lp_half_order; i++)
    {
        double tmp, f_j_2, f_j_1, f_j;
        double val = lsp[2*i];

        __asm__ volatile(
            "move   %[p_f],     %[p_fi]                         \n\t"
            "add.d  %[val],     %[val],     %[val]              \n\t"
            PTR_ADDIU "%[p_fi], 8                               \n\t"
            "ldc1   %[f_j_1],   0(%[p_f])                       \n\t"
            "ldc1   %[f_j],     8(%[p_f])                       \n\t"
            "neg.d  %[val],     %[val]                          \n\t"
            "add.d  %[tmp],     %[f_j_1],   %[f_j_1]            \n\t"
            "madd.d %[tmp],     %[tmp],     %[f_j], %[val]      \n\t"
            "addiu  %[j],       %[i], -2                        \n\t"
            "ldc1   %[f_j_2],   -8(%[p_f])                      \n\t"
            "sdc1   %[tmp],     16(%[p_f])                      \n\t"
            "beqz   %[j],       lsp2polyf_lp_j_end%=            \n\t"
            "lsp2polyf_lp_j%=:                                  \n\t"
            "add.d  %[tmp],     %[f_j],     %[f_j_2]            \n\t"
            "madd.d %[tmp],     %[tmp],     %[f_j_1], %[val]    \n\t"
            "mov.d  %[f_j],     %[f_j_1]                        \n\t"
            "addiu  %[j],       -1                              \n\t"
            "mov.d  %[f_j_1],   %[f_j_2]                        \n\t"
            "ldc1   %[f_j_2],   -16(%[p_f])                     \n\t"
            "sdc1   %[tmp],     8(%[p_f])                       \n\t"
            PTR_ADDIU "%[p_f], -8                              \n\t"
            "bgtz   %[j],       lsp2polyf_lp_j%=                \n\t"
            "lsp2polyf_lp_j_end%=:                              \n\t"

            : [f_j_2]"=&f"(f_j_2), [f_j_1]"=&f"(f_j_1), [val]"+f"(val),
              [tmp]"=&f"(tmp), [f_j]"=&f"(f_j), [p_f]"+r"(p_f),
              [j]"+r"(j), [p_fi]"+r"(p_fi)
            : [i]"r"(i)
            : "memory"
        );
        f[1] += val;
    }
}
#define lsp2polyf lsp2polyf_mips
#endif /* !HAVE_MIPS32R6 && !HAVE_MIPS64R6 */
#endif /* HAVE_MIPSFPU && HAVE_INLINE_ASM */
#endif /* AVCODEC_MIPS_LSP_MIPS_H */
