/*
 * simple math operations
 * Copyright (c) 2001, 2002 Fabrice Bellard
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

#ifndef AVCODEC_PPC_MATHOPS_H
#define AVCODEC_PPC_MATHOPS_H

#include <stdint.h>
#include "config.h"
#include "libavutil/common.h"

#if HAVE_PPC4XX
/* signed 16x16 -> 32 multiply add accumulate */
#define MAC16(rt, ra, rb) \
    __asm__ ("maclhw %0, %2, %3" : "=r" (rt) : "0" (rt), "r" (ra), "r" (rb));

/* signed 16x16 -> 32 multiply */
#define MUL16(ra, rb) \
    ({ int __rt; \
    __asm__ ("mullhw %0, %1, %2" : "=r" (__rt) : "r" (ra), "r" (rb)); \
    __rt; })
#endif

#define MULH MULH
static inline av_const int MULH(int a, int b){
    int r;
    __asm__ ("mulhw %0, %1, %2" : "=r"(r) : "r"(a), "r"(b));
    return r;
}

#if !ARCH_PPC64
static inline av_const int64_t MAC64(int64_t d, int a, int b)
{
    union { uint64_t x; unsigned hl[2]; } x = { d };
    int h, l;
    __asm__ ("mullw %3, %4, %5   \n\t"
             "mulhw %2, %4, %5   \n\t"
             "addc  %1, %1, %3   \n\t"
             "adde  %0, %0, %2   \n\t"
             : "+r"(x.hl[0]), "+r"(x.hl[1]), "=&r"(h), "=&r"(l)
             : "r"(a), "r"(b));
    return x.x;
}
#define MAC64(d, a, b) ((d) = MAC64(d, a, b))

static inline av_const int64_t MLS64(int64_t d, int a, int b)
{
    union { uint64_t x; unsigned hl[2]; } x = { d };
    int h, l;
    __asm__ ("mullw %3, %4, %5   \n\t"
             "mulhw %2, %4, %5   \n\t"
             "subfc %1, %3, %1   \n\t"
             "subfe %0, %2, %0   \n\t"
             : "+r"(x.hl[0]), "+r"(x.hl[1]), "=&r"(h), "=&r"(l)
             : "r"(a), "r"(b));
    return x.x;
}
#define MLS64(d, a, b) ((d) = MLS64(d, a, b))
#endif

#endif /* AVCODEC_PPC_MATHOPS_H */
