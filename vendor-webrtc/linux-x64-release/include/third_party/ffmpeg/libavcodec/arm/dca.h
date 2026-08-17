/*
 * Copyright (c) 2011 Mans Rullgard <mans@mansr.com>
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

#ifndef AVCODEC_ARM_DCA_H
#define AVCODEC_ARM_DCA_H

#include <stdint.h>

#include "config.h"
#include "libavcodec/mathops.h"

#if HAVE_ARMV6_INLINE && AV_GCC_VERSION_AT_LEAST(4,4) && !CONFIG_THUMB

#define decode_blockcodes decode_blockcodes
static inline int decode_blockcodes(int code1, int code2, int levels,
                                    int32_t *values)
{
    int32_t v0, v1, v2, v3, v4, v5;

    __asm__ ("smmul   %0,  %6,  %10           \n"
             "smmul   %3,  %7,  %10           \n"
             "smlabb  %6,  %0,  %9,  %6       \n"
             "smlabb  %7,  %3,  %9,  %7       \n"
             "smmul   %1,  %0,  %10           \n"
             "smmul   %4,  %3,  %10           \n"
             "sub     %6,  %6,  %8,  lsr #1   \n"
             "sub     %7,  %7,  %8,  lsr #1   \n"
             "smlabb  %0,  %1,  %9,  %0       \n"
             "smlabb  %3,  %4,  %9,  %3       \n"
             "smmul   %2,  %1,  %10           \n"
             "smmul   %5,  %4,  %10           \n"
             "str     %6,  [%11, #0]          \n"
             "str     %7,  [%11, #16]         \n"
             "sub     %0,  %0,  %8,  lsr #1   \n"
             "sub     %3,  %3,  %8,  lsr #1   \n"
             "smlabb  %1,  %2,  %9,  %1       \n"
             "smlabb  %4,  %5,  %9,  %4       \n"
             "smmul   %6,  %2,  %10           \n"
             "smmul   %7,  %5,  %10           \n"
             "str     %0,  [%11, #4]          \n"
             "str     %3,  [%11, #20]         \n"
             "sub     %1,  %1,  %8,  lsr #1   \n"
             "sub     %4,  %4,  %8,  lsr #1   \n"
             "smlabb  %2,  %6,  %9,  %2       \n"
             "smlabb  %5,  %7,  %9,  %5       \n"
             "str     %1,  [%11, #8]          \n"
             "str     %4,  [%11, #24]         \n"
             "sub     %2,  %2,  %8,  lsr #1   \n"
             "sub     %5,  %5,  %8,  lsr #1   \n"
             "str     %2,  [%11, #12]         \n"
             "str     %5,  [%11, #28]         \n"
             : "=&r"(v0), "=&r"(v1), "=&r"(v2),
               "=&r"(v3), "=&r"(v4), "=&r"(v5),
               "+&r"(code1), "+&r"(code2)
             : "r"(levels - 1), "r"(-levels),
               "r"(ff_inverse[levels]), "r"(values)
             : "memory");

    return code1 | code2;
}

#endif

#endif /* AVCODEC_ARM_DCA_H */
