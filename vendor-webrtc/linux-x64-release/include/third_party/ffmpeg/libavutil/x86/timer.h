/*
 * copyright (c) 2006 Michael Niedermayer <michaelni@gmx.at>
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

#ifndef AVUTIL_X86_TIMER_H
#define AVUTIL_X86_TIMER_H

#include <stdint.h>

#if HAVE_INLINE_ASM

#define FF_TIMER_UNITS "decicycles"
#define AV_READ_TIME read_time

static inline uint64_t read_time(void)
{
    uint32_t a, d;
    __asm__ volatile(
#if ARCH_X86_64 || defined(__SSE2__)
                     "lfence \n\t"
#endif
                     "rdtsc  \n\t"
                     : "=a" (a), "=d" (d));
    return ((uint64_t)d << 32) + a;
}

#elif HAVE_RDTSC

#include <intrin.h>
#define AV_READ_TIME __rdtsc

#endif /* HAVE_INLINE_ASM */

#endif /* AVUTIL_X86_TIMER_H */
