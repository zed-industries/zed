/*
 * Copyright (c) 2009 Mans Rullgard <mans@mansr.com>
 * Copyright (c) 2015 Zhou Xiaoyong <zhouxiaoyong@loongson.cn>
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

#ifndef AVCODEC_MIPS_MATHOPS_H
#define AVCODEC_MIPS_MATHOPS_H

#include <stdint.h>
#include "config.h"
#include "libavutil/common.h"

#if HAVE_INLINE_ASM

#if HAVE_LOONGSON3

#define MULH MULH
static inline av_const int MULH(int a, int b)
{
    int c;
    __asm__ ("dmult %1, %2      \n\t"
             "mflo %0           \n\t"
             "dsrl %0, %0, 32   \n\t"
             : "=r"(c)
             : "r"(a),"r"(b)
             : "hi", "lo");
    return c;
}

#define mid_pred mid_pred
static inline av_const int mid_pred(int a, int b, int c)
{
    int t = b;
    __asm__ ("sgt $8, %1, %2    \n\t"
             "movn %0, %1, $8   \n\t"
             "movn %1, %2, $8   \n\t"
             "sgt $8, %1, %3    \n\t"
             "movz %1, %3, $8   \n\t"
             "sgt $8, %0, %1    \n\t"
             "movn %0, %1, $8   \n\t"
             : "+&r"(t),"+&r"(a)
             : "r"(b),"r"(c)
             : "$8");
    return t;
}

#endif /* HAVE_LOONGSON3 */

#endif /* HAVE_INLINE_ASM */

#endif /* AVCODEC_MIPS_MATHOPS_H */
