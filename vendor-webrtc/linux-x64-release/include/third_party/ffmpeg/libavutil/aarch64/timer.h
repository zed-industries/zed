/*
 * Copyright (c) 2015 Janne Grunau <janne-libav@jannau.net>
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

#ifndef AVUTIL_AARCH64_TIMER_H
#define AVUTIL_AARCH64_TIMER_H

#include <stdint.h>
#include "config.h"

#if HAVE_INLINE_ASM

#define AV_READ_TIME read_time

static inline uint64_t read_time(void)
{
    uint64_t cycle_counter;
    __asm__ volatile(
        "isb                   \t\n"
#if defined(__ANDROID__) || defined(__APPLE__)
        // cntvct_el0 has lower resolution than pmccntr_el0, but is usually
        // accessible from user space by default.
        "mrs %0, cntvct_el0        "
#else
        // pmccntr_el0 has higher resolution, but is usually not accessible
        // from user space by default (but access can be enabled with a custom
        // kernel module).
        "mrs %0, pmccntr_el0       "
#endif
        : "=r"(cycle_counter) :: "memory" );

    return cycle_counter;
}

#endif /* HAVE_INLINE_ASM */

#endif /* AVUTIL_AARCH64_TIMER_H */
