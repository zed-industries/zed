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

/**
 * @file
 * high precision timer, useful to profile code
 */

#ifndef AVUTIL_TIMER_H
#define AVUTIL_TIMER_H

#include "config.h"

#if CONFIG_LINUX_PERF
# ifndef _GNU_SOURCE
#  define _GNU_SOURCE
# endif
# include <unistd.h> // read(3)
# include <sys/ioctl.h>
# include <asm/unistd.h>
# include <linux/perf_event.h>
#endif

#include <stdlib.h>
#include <stdint.h>
#include <inttypes.h>

#if CONFIG_MACOS_KPERF
#include "macos_kperf.h"
#endif

#if HAVE_MACH_ABSOLUTE_TIME
#include <mach/mach_time.h>
#elif HAVE_CLOCK_GETTIME
#include <time.h>
#endif

#include "common.h"
#include "log.h"

#if   ARCH_AARCH64
#   include "aarch64/timer.h"
#elif ARCH_ARM
#   include "arm/timer.h"
#elif ARCH_PPC
#   include "ppc/timer.h"
#elif ARCH_X86
#   include "x86/timer.h"
#elif ARCH_LOONGARCH
#   include "loongarch/timer.h"
#endif

#if !defined(AV_READ_TIME)
#   if HAVE_GETHRTIME
#       define AV_READ_TIME gethrtime
#   elif HAVE_MACH_ABSOLUTE_TIME
#       define AV_READ_TIME mach_absolute_time
#   elif HAVE_CLOCK_GETTIME && defined(CLOCK_MONOTONIC)
        static inline int64_t ff_read_time(void)
        {
            struct timespec ts;
            clock_gettime(CLOCK_MONOTONIC, &ts);
            return ts.tv_sec * INT64_C(1000000000) + ts.tv_nsec;
        }
#       define AV_READ_TIME ff_read_time
#       define FF_TIMER_UNITS "ns"
#   endif
#endif

#ifndef FF_TIMER_UNITS
#   define FF_TIMER_UNITS "UNITS"
#endif

#define TIMER_REPORT(id, tdiff)                                           \
    {                                                                     \
        static uint64_t tsum   = 0;                                       \
        static int tcount      = 0;                                       \
        static int tskip_count = 0;                                       \
        static int thistogram[32] = {0};                                  \
        thistogram[av_log2(tdiff)]++;                                     \
        if (tcount < 2                ||                                  \
            (tdiff) < 8 * tsum / tcount ||                                \
            (tdiff) < 2000) {                                             \
            tsum += (tdiff);                                              \
            tcount++;                                                     \
        } else                                                            \
            tskip_count++;                                                \
        if (((tcount + tskip_count) & (tcount + tskip_count - 1)) == 0) { \
            int i;                                                        \
            av_log(NULL, AV_LOG_ERROR,                                    \
                   "%7" PRIu64 " " FF_TIMER_UNITS " in %s,%8d runs,%7d skips",\
                   tsum * 10 / tcount, id, tcount, tskip_count);          \
            for (i = 0; i < 32; i++)                                      \
                av_log(NULL, AV_LOG_VERBOSE, " %2d", av_log2(2*thistogram[i]));\
            av_log(NULL, AV_LOG_ERROR, "\n");                             \
        }                                                                 \
    }

#if CONFIG_LINUX_PERF

#define START_TIMER                                                         \
    static int linux_perf_fd = -1;                                          \
    uint64_t tperf;                                                         \
    if (linux_perf_fd == -1) {                                              \
        struct perf_event_attr attr = {                                     \
            .type           = PERF_TYPE_HARDWARE,                           \
            .size           = sizeof(struct perf_event_attr),               \
            .config         = PERF_COUNT_HW_CPU_CYCLES,                     \
            .disabled       = 1,                                            \
            .exclude_kernel = 1,                                            \
            .exclude_hv     = 1,                                            \
        };                                                                  \
        linux_perf_fd = syscall(__NR_perf_event_open, &attr,                \
                                0, -1, -1, 0);                              \
    }                                                                       \
    if (linux_perf_fd == -1) {                                              \
        av_log(NULL, AV_LOG_ERROR, "perf_event_open failed: %s\n",          \
               av_err2str(AVERROR(errno)));                                 \
    } else {                                                                \
        ioctl(linux_perf_fd, PERF_EVENT_IOC_RESET, 0);                      \
        ioctl(linux_perf_fd, PERF_EVENT_IOC_ENABLE, 0);                     \
    }

#define STOP_TIMER(id)                                                      \
    ioctl(linux_perf_fd, PERF_EVENT_IOC_DISABLE, 0);                        \
    read(linux_perf_fd, &tperf, sizeof(tperf));                             \
    TIMER_REPORT(id, tperf)

#elif CONFIG_MACOS_KPERF

#define START_TIMER                                                         \
    uint64_t tperf;                                                         \
    ff_kperf_init();                                                        \
    tperf = ff_kperf_cycles();

#define STOP_TIMER(id)                                                      \
    TIMER_REPORT(id, ff_kperf_cycles() - tperf);

#elif defined(AV_READ_TIME)
#define START_TIMER                             \
    uint64_t tend;                              \
    uint64_t tstart = AV_READ_TIME();           \

#define STOP_TIMER(id)                                                    \
    tend = AV_READ_TIME();                                                \
    TIMER_REPORT(id, tend - tstart)
#else
#define START_TIMER
#define STOP_TIMER(id) { }
#endif

#endif /* AVUTIL_TIMER_H */
