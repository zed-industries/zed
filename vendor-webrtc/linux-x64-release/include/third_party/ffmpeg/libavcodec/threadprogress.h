/*
 * Copyright (c) 2022 Andreas Rheinhardt <andreas.rheinhardt@outlook.com>
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

#ifndef AVCODEC_THREADPROGRESS_H
#define AVCODEC_THREADPROGRESS_H

/**
 * ThreadProgress is an API to easily notify other threads about progress
 * of any kind as long as it can be packaged into an int and is consistent
 * with the natural ordering of integers.
 *
 * Each initialized ThreadProgress can be in one of two modes: No-op mode
 * or ordinary mode. In the former mode, ff_thread_report_progress() and
 * ff_thread_await_progress() are no-ops to simply support usecases like
 * non-frame-threading. Only in the latter case perform these functions
 * what their name already implies.
 */

#include <limits.h>
#include <stdatomic.h>
#include "libavutil/thread.h"

/**
 * This struct should be treated as opaque by users.
 */
typedef struct ThreadProgress {
    atomic_int progress;
    unsigned   init;
    AVMutex progress_mutex;
    AVCond  progress_cond;
} ThreadProgress;

/**
 * Initialize a ThreadProgress.
 *
 * @param init_mode If zero, the ThreadProgress will be initialized
 *                  to be in no-op mode as described above. Otherwise
 *                  it is initialized to be in ordinary mode.
 */
int ff_thread_progress_init(ThreadProgress *pro, int init_mode);

/**
 * Destroy a ThreadProgress. Can be called on a ThreadProgress that
 * has never been initialized provided that the ThreadProgress struct
 * has been initially zeroed. Must be called even if ff_thread_progress_init()
 * failed.
 */
void ff_thread_progress_destroy(ThreadProgress *pro);

/**
 * Reset the ::ThreadProgress.progress counter; must only be called
 * if the ThreadProgress is not in use in any way (e.g. no thread
 * may wait on it via ff_thread_progress_await()).
 */
static inline void ff_thread_progress_reset(ThreadProgress *pro)
{
    atomic_init(&pro->progress, pro->init ? -1 : INT_MAX);
}

/**
 * This function is a no-op in no-op mode; otherwise it notifies
 * other threads that a certain level of progress has been reached.
 * Later calls with lower values of progress have no effect.
 */
void ff_thread_progress_report(ThreadProgress *pro, int progress);

/**
 * This function is a no-op in no-op mode; otherwise it waits
 * until other threads have reached a certain level of progress:
 * This function will return after another thread has called
 * ff_thread_progress_report() with the same or higher value for progress.
 */
void ff_thread_progress_await(const ThreadProgress *pro, int progress);

#endif /* AVCODEC_THREADPROGRESS_H */
