/*
 * simple math operations
 * Copyright (c) 2006 Michael Niedermayer <michaelni@gmx.at> et al
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

#ifndef AVCODEC_ARM_MATHOPS_H
#define AVCODEC_ARM_MATHOPS_H

#include <stdint.h>
#include "config.h"
#include "libavutil/common.h"

#if HAVE_INLINE_ASM

#if HAVE_ARMV6_INLINE
#define MULH MULH
static inline av_const int MULH(int a, int b)
{
    int r;
    __asm__ ("smmul %0, %1, %2" : "=r"(r) : "r"(a), "r"(b));
    return r;
}

#define FASTDIV FASTDIV
static av_always_inline av_const int FASTDIV(int a, int b)
{
    int r;
    __asm__ ("cmp     %2, #2               \n\t"
             "ldr     %0, [%3, %2, lsl #2] \n\t"
             "ite     le                   \n\t"
             "lsrle   %0, %1, #1           \n\t"
             "smmulgt %0, %0, %1           \n\t"
             : "=&r"(r) : "r"(a), "r"(b), "r"(ff_inverse) : "cc");
    return r;
}

#else /* HAVE_ARMV6_INLINE */

#define FASTDIV FASTDIV
static av_always_inline av_const int FASTDIV(int a, int b)
{
    int r, t;
    __asm__ ("umull %1, %0, %2, %3"
             : "=&r"(r), "=&r"(t) : "r"(a), "r"(ff_inverse[b]));
    return r;
}
#endif

#define MLS64(d, a, b) MAC64(d, -(a), b)

#if HAVE_ARMV5TE_INLINE

/* signed 16x16 -> 32 multiply add accumulate */
#   define MAC16(rt, ra, rb)                                            \
    __asm__ ("smlabb %0, %1, %2, %0" : "+r"(rt) : "r"(ra), "r"(rb));

/* signed 16x16 -> 32 multiply */
#   define MUL16 MUL16
static inline av_const int MUL16(int ra, int rb)
{
    int rt;
    __asm__ ("smulbb %0, %1, %2" : "=r"(rt) : "r"(ra), "r"(rb));
    return rt;
}

#endif

#define mid_pred mid_pred
static inline av_const int mid_pred(int a, int b, int c)
{
    int m;
    __asm__ (
        "mov   %0, %2  \n\t"
        "cmp   %1, %2  \n\t"
        "itt   gt      \n\t"
        "movgt %0, %1  \n\t"
        "movgt %1, %2  \n\t"
        "cmp   %1, %3  \n\t"
        "it    le      \n\t"
        "movle %1, %3  \n\t"
        "cmp   %0, %1  \n\t"
        "it    gt      \n\t"
        "movgt %0, %1  \n\t"
        : "=&r"(m), "+r"(a)
        : "r"(b), "r"(c)
        : "cc");
    return m;
}

#endif /* HAVE_INLINE_ASM */

#endif /* AVCODEC_ARM_MATHOPS_H */
