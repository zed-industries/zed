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
 * Author:  Bojan Zivkovic (bojan@mips.com)
 *
 * Compute antialias function optimised for MIPS fixed-point architecture
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
 * Reference: libavcodec/mpegaudiodec.c
 */

#ifndef AVCODEC_MIPS_COMPUTE_ANTIALIAS_FIXED_H
#define AVCODEC_MIPS_COMPUTE_ANTIALIAS_FIXED_H

#if HAVE_INLINE_ASM
#if !HAVE_MIPS32R6 && !HAVE_MIPS64R6
static void compute_antialias_mips_fixed(MPADecodeContext *s,
                                        GranuleDef *g)
{
    const int32_t *csa;
    int32_t *ptr;
    int n, i;
    int MAX_lo = 0xffffffff;

    /* we antialias only "long" bands */
    if (g->block_type == 2) {
        if (!g->switch_point)
            return;
        /* XXX: check this for 8000Hz case */
        n = 1;
    } else {
        n = SBLIMIT - 1;
    }


    ptr = g->sb_hybrid + 18;

    for(i = n;i > 0;i--) {
        int tmp0, tmp1, tmp2, tmp00, tmp11;
        int temp_reg1, temp_reg2, temp_reg3, temp_reg4, temp_reg5, temp_reg6;
        csa = &csa_table[0][0];

        /**
         * instructions are scheduled to minimize pipeline stall.
         */
        __asm__ volatile (
            "lw   %[tmp0],      -1*4(%[ptr])                            \n\t"
            "lw   %[tmp1],      0*4(%[ptr])                             \n\t"
            "lw   %[temp_reg1], 0*4(%[csa])                             \n\t"
            "lw   %[temp_reg2], 2*4(%[csa])                             \n\t"
            "add  %[tmp2],      %[tmp0],      %[tmp1]                   \n\t"
            "lw   %[temp_reg3], 3*4(%[csa])                             \n\t"
            "mult $ac0,         %[tmp2],      %[temp_reg1]              \n\t"
            "mult $ac1,         %[tmp2],      %[temp_reg1]              \n\t"
            "lw   %[tmp00],     -2*4(%[ptr])                            \n\t"
            "lw   %[tmp11],     1*4(%[ptr])                             \n\t"
            "lw   %[temp_reg4], 4*4(%[csa])                             \n\t"
            "mtlo %[MAX_lo],    $ac0                                    \n\t"
            "mtlo $zero,        $ac1                                    \n\t"
            "msub $ac0,         %[tmp1],      %[temp_reg2]              \n\t"
            "madd $ac1,         %[tmp0],      %[temp_reg3]              \n\t"
            "add  %[tmp2],      %[tmp00],     %[tmp11]                  \n\t"
            "lw   %[temp_reg5], 6*4(%[csa])                             \n\t"
            "mult $ac2,         %[tmp2],      %[temp_reg4]              \n\t"
            "mult $ac3,         %[tmp2],      %[temp_reg4]              \n\t"
            "mfhi %[temp_reg1], $ac0                                    \n\t"
            "mfhi %[temp_reg2], $ac1                                    \n\t"
            "lw   %[temp_reg6], 7*4(%[csa])                             \n\t"
            "mtlo %[MAX_lo],    $ac2                                    \n\t"
            "msub $ac2,         %[tmp11],     %[temp_reg5]              \n\t"
            "mtlo $zero,        $ac3                                    \n\t"
            "madd $ac3,         %[tmp00],     %[temp_reg6]              \n\t"
            "sll  %[temp_reg1], %[temp_reg1], 2                         \n\t"
            "sw   %[temp_reg1], -1*4(%[ptr])                            \n\t"
            "mfhi %[temp_reg4], $ac2                                    \n\t"
            "sll  %[temp_reg2], %[temp_reg2], 2                         \n\t"
            "mfhi %[temp_reg5], $ac3                                    \n\t"
            "sw   %[temp_reg2], 0*4(%[ptr])                             \n\t"
            "lw   %[tmp0],      -3*4(%[ptr])                            \n\t"
            "lw   %[tmp1],      2*4(%[ptr])                             \n\t"
            "lw   %[temp_reg1], 8*4(%[csa])                             \n\t"
            "sll  %[temp_reg4], %[temp_reg4], 2                         \n\t"
            "add  %[tmp2],      %[tmp0],      %[tmp1]                   \n\t"
            "sll  %[temp_reg5], %[temp_reg5], 2                         \n\t"
            "mult $ac0,         %[tmp2],      %[temp_reg1]              \n\t"
            "mult $ac1,         %[tmp2],      %[temp_reg1]              \n\t"
            "sw   %[temp_reg4], -2*4(%[ptr])                            \n\t"
            "sw   %[temp_reg5], 1*4(%[ptr])                             \n\t"
            "lw   %[temp_reg2], 10*4(%[csa])                            \n\t"
            "mtlo %[MAX_lo],    $ac0                                    \n\t"
            "lw   %[temp_reg3], 11*4(%[csa])                            \n\t"
            "msub $ac0,         %[tmp1],      %[temp_reg2]              \n\t"
            "mtlo $zero,        $ac1                                    \n\t"
            "madd $ac1,         %[tmp0],      %[temp_reg3]              \n\t"
            "lw   %[tmp00],     -4*4(%[ptr])                            \n\t"
            "lw   %[tmp11],     3*4(%[ptr])                             \n\t"
            "mfhi %[temp_reg1], $ac0                                    \n\t"
            "lw   %[temp_reg4], 12*4(%[csa])                            \n\t"
            "mfhi %[temp_reg2], $ac1                                    \n\t"
            "add  %[tmp2],      %[tmp00],     %[tmp11]                  \n\t"
            "mult $ac2,         %[tmp2],      %[temp_reg4]              \n\t"
            "mult $ac3,         %[tmp2],      %[temp_reg4]              \n\t"
            "lw   %[temp_reg5], 14*4(%[csa])                            \n\t"
            "lw   %[temp_reg6], 15*4(%[csa])                            \n\t"
            "sll  %[temp_reg1], %[temp_reg1], 2                         \n\t"
            "mtlo %[MAX_lo],    $ac2                                    \n\t"
            "msub $ac2,         %[tmp11],     %[temp_reg5]              \n\t"
            "mtlo $zero,        $ac3                                    \n\t"
            "madd $ac3,         %[tmp00],     %[temp_reg6]              \n\t"
            "sll  %[temp_reg2], %[temp_reg2], 2                         \n\t"
            "sw   %[temp_reg1], -3*4(%[ptr])                            \n\t"
            "mfhi %[temp_reg4], $ac2                                    \n\t"
            "sw   %[temp_reg2], 2*4(%[ptr])                             \n\t"
            "mfhi %[temp_reg5], $ac3                                    \n\t"
            "lw   %[tmp0],      -5*4(%[ptr])                            \n\t"
            "lw   %[tmp1],      4*4(%[ptr])                             \n\t"
            "lw   %[temp_reg1], 16*4(%[csa])                            \n\t"
            "lw   %[temp_reg2], 18*4(%[csa])                            \n\t"
            "add  %[tmp2],      %[tmp0],      %[tmp1]                   \n\t"
            "lw   %[temp_reg3], 19*4(%[csa])                            \n\t"
            "mult $ac0,         %[tmp2],      %[temp_reg1]              \n\t"
            "mult $ac1,         %[tmp2],      %[temp_reg1]              \n\t"
            "sll  %[temp_reg4], %[temp_reg4], 2                         \n\t"
            "sll  %[temp_reg5], %[temp_reg5], 2                         \n\t"
            "sw   %[temp_reg4], -4*4(%[ptr])                            \n\t"
            "mtlo %[MAX_lo],    $ac0                                    \n\t"
            "msub $ac0,         %[tmp1],      %[temp_reg2]              \n\t"
            "mtlo $zero,        $ac1                                    \n\t"
            "madd $ac1,         %[tmp0],      %[temp_reg3]              \n\t"
            "sw   %[temp_reg5], 3*4(%[ptr])                             \n\t"
            "lw   %[tmp00],     -6*4(%[ptr])                            \n\t"
            "mfhi %[temp_reg1], $ac0                                    \n\t"
            "lw   %[tmp11],     5*4(%[ptr])                             \n\t"
            "mfhi %[temp_reg2], $ac1                                    \n\t"
            "lw   %[temp_reg4], 20*4(%[csa])                            \n\t"
            "add  %[tmp2],      %[tmp00],     %[tmp11]                  \n\t"
            "lw   %[temp_reg5], 22*4(%[csa])                            \n\t"
            "mult $ac2,         %[tmp2],      %[temp_reg4]              \n\t"
            "mult $ac3,         %[tmp2],      %[temp_reg4]              \n\t"
            "lw   %[temp_reg6], 23*4(%[csa])                            \n\t"
            "sll  %[temp_reg1], %[temp_reg1], 2                         \n\t"
            "sll  %[temp_reg2], %[temp_reg2], 2                         \n\t"
            "mtlo %[MAX_lo],    $ac2                                    \n\t"
            "msub $ac2,         %[tmp11],     %[temp_reg5]              \n\t"
            "mtlo $zero,        $ac3                                    \n\t"
            "madd $ac3,         %[tmp00],     %[temp_reg6]              \n\t"
            "sw   %[temp_reg1], -5*4(%[ptr])                            \n\t"
            "sw   %[temp_reg2], 4*4(%[ptr])                             \n\t"
            "mfhi %[temp_reg4], $ac2                                    \n\t"
            "lw   %[tmp0],      -7*4(%[ptr])                            \n\t"
            "mfhi %[temp_reg5], $ac3                                    \n\t"
            "lw   %[tmp1],      6*4(%[ptr])                             \n\t"
            "lw   %[temp_reg1], 24*4(%[csa])                            \n\t"
            "lw   %[temp_reg2], 26*4(%[csa])                            \n\t"
            "add  %[tmp2],      %[tmp0],      %[tmp1]                   \n\t"
            "lw   %[temp_reg3], 27*4(%[csa])                            \n\t"
            "mult $ac0,         %[tmp2],      %[temp_reg1]              \n\t"
            "mult $ac1,         %[tmp2],      %[temp_reg1]              \n\t"
            "sll  %[temp_reg4], %[temp_reg4], 2                         \n\t"
            "sll  %[temp_reg5], %[temp_reg5], 2                         \n\t"
            "sw   %[temp_reg4], -6*4(%[ptr])                            \n\t"
            "mtlo %[MAX_lo],    $ac0                                    \n\t"
            "msub $ac0,         %[tmp1],      %[temp_reg2]              \n\t"
            "mtlo $zero,        $ac1                                    \n\t"
            "madd $ac1,         %[tmp0],      %[temp_reg3]              \n\t"
            "sw   %[temp_reg5], 5*4(%[ptr])                             \n\t"
            "lw   %[tmp00],     -8*4(%[ptr])                            \n\t"
            "mfhi %[temp_reg1], $ac0                                    \n\t"
            "lw   %[tmp11],     7*4(%[ptr])                             \n\t"
            "mfhi %[temp_reg2], $ac1                                    \n\t"
            "lw   %[temp_reg4], 28*4(%[csa])                            \n\t"
            "add  %[tmp2],      %[tmp00],     %[tmp11]                  \n\t"
            "lw   %[temp_reg5], 30*4(%[csa])                            \n\t"
            "mult $ac2,         %[tmp2],      %[temp_reg4]              \n\t"
            "mult $ac3,         %[tmp2],      %[temp_reg4]              \n\t"
            "lw   %[temp_reg6], 31*4(%[csa])                            \n\t"
            "sll  %[temp_reg1], %[temp_reg1], 2                         \n\t"
            "sll  %[temp_reg2], %[temp_reg2], 2                         \n\t"
            "mtlo %[MAX_lo],    $ac2                                    \n\t"
            "msub $ac2,         %[tmp11],     %[temp_reg5]              \n\t"
            "mtlo $zero,        $ac3                                    \n\t"
            "madd $ac3,         %[tmp00],     %[temp_reg6]              \n\t"
            "sw   %[temp_reg1], -7*4(%[ptr])                            \n\t"
            "sw   %[temp_reg2], 6*4(%[ptr])                             \n\t"
            "mfhi %[temp_reg4], $ac2                                    \n\t"
            "mfhi %[temp_reg5], $ac3                                    \n\t"
            "sll  %[temp_reg4], %[temp_reg4], 2                         \n\t"
            "sll  %[temp_reg5], %[temp_reg5], 2                         \n\t"
            "sw   %[temp_reg4], -8*4(%[ptr])                            \n\t"
            "sw   %[temp_reg5], 7*4(%[ptr])                             \n\t"

            : [tmp0] "=&r" (tmp0), [tmp1] "=&r" (tmp1), [tmp2] "=&r" (tmp2),
              [tmp00] "=&r" (tmp00), [tmp11] "=&r" (tmp11),
              [temp_reg1] "=&r" (temp_reg1), [temp_reg2] "=&r" (temp_reg2),
              [temp_reg3] "=&r" (temp_reg3), [temp_reg4] "=&r" (temp_reg4),
              [temp_reg5] "=&r" (temp_reg5), [temp_reg6] "=&r" (temp_reg6)
            : [csa] "r" (csa), [ptr] "r" (ptr),
              [MAX_lo] "r" (MAX_lo)
            : "memory", "hi", "lo", "$ac1hi", "$ac1lo", "$ac2hi", "$ac2lo",
              "$ac3hi", "$ac3lo"
         );

        ptr += 18;
    }
}
#define compute_antialias compute_antialias_mips_fixed
#endif /* !HAVE_MIPS32R6 && !HAVE_MIPS64R6 */
#endif /* HAVE_INLINE_ASM */

#endif /* AVCODEC_MIPS_COMPUTE_ANTIALIAS_FIXED_H */
