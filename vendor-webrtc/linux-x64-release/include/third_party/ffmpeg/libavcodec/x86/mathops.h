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

#ifndef AVCODEC_X86_MATHOPS_H
#define AVCODEC_X86_MATHOPS_H

#include "config.h"

#include "libavutil/common.h"
#include "libavutil/x86/asm.h"

#if HAVE_INLINE_ASM

#if ARCH_X86_32

#define MULL MULL
static av_always_inline av_const int MULL(int a, int b, unsigned shift)
{
    int rt, dummy;
    if (__builtin_constant_p(shift))
    __asm__ (
        "imull %3               \n\t"
        "shrdl %4, %%edx, %%eax \n\t"
        :"=a"(rt), "=d"(dummy)
        :"a"(a), "rm"(b), "i"(shift & 0x1F)
    );
    else
        __asm__ (
            "imull %3               \n\t"
            "shrdl %4, %%edx, %%eax \n\t"
            :"=a"(rt), "=d"(dummy)
            :"a"(a), "rm"(b), "c"((uint8_t)shift)
        );
    return rt;
}

#define MULH MULH
static av_always_inline av_const int MULH(int a, int b)
{
    int rt, dummy;
    __asm__ (
        "imull %3"
        :"=d"(rt), "=a"(dummy)
        :"a"(a), "rm"(b)
    );
    return rt;
}

#define MUL64 MUL64
static av_always_inline av_const int64_t MUL64(int a, int b)
{
    int64_t rt;
    __asm__ (
        "imull %2"
        :"=A"(rt)
        :"a"(a), "rm"(b)
    );
    return rt;
}

#endif /* ARCH_X86_32 */

#if HAVE_I686
/* median of 3 */
#define mid_pred mid_pred
static inline av_const int mid_pred(int a, int b, int c)
{
    int i=b;
    __asm__ (
        "cmp    %2, %1 \n\t"
        "cmovg  %1, %0 \n\t"
        "cmovg  %2, %1 \n\t"
        "cmp    %3, %1 \n\t"
        "cmovl  %3, %1 \n\t"
        "cmp    %1, %0 \n\t"
        "cmovg  %1, %0 \n\t"
        :"+&r"(i), "+&r"(a)
        :"r"(b), "r"(c)
    );
    return i;
}

#if HAVE_6REGS
#define COPY3_IF_LT(x, y, a, b, c, d)\
__asm__ volatile(\
    "cmpl  %0, %3       \n\t"\
    "cmovl %3, %0       \n\t"\
    "cmovl %4, %1       \n\t"\
    "cmovl %5, %2       \n\t"\
    : "+&r" (x), "+&r" (a), "+r" (c)\
    : "r" (y), "r" (b), "r" (d)\
);
#endif /* HAVE_6REGS */

#endif /* HAVE_I686 */

#define MASK_ABS(mask, level)                   \
    __asm__ ("cdq                    \n\t"      \
             "xorl %1, %0            \n\t"      \
             "subl %1, %0            \n\t"      \
             : "+a"(level), "=&d"(mask))

// avoid +32 for shift optimization (gcc should do that ...)
#define NEG_SSR32 NEG_SSR32
static inline  int32_t NEG_SSR32( int32_t a, int8_t s){
    if (__builtin_constant_p(s))
    __asm__ ("sarl %1, %0\n\t"
         : "+r" (a)
         : "i" (-s & 0x1F)
    );
    else
        __asm__ ("sarl %1, %0\n\t"
               : "+r" (a)
               : "c" ((uint8_t)(-s))
        );
    return a;
}

#define NEG_USR32 NEG_USR32
static inline uint32_t NEG_USR32(uint32_t a, int8_t s){
    if (__builtin_constant_p(s))
    __asm__ ("shrl %1, %0\n\t"
         : "+r" (a)
         : "i" (-s & 0x1F)
    );
    else
        __asm__ ("shrl %1, %0\n\t"
               : "+r" (a)
               : "c" ((uint8_t)(-s))
        );
    return a;
}

#endif /* HAVE_INLINE_ASM */
#endif /* AVCODEC_X86_MATHOPS_H */
