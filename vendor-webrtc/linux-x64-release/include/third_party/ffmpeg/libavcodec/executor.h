/*
 * Copyright (C) 2024 Nuo Mi
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

/*
 * We still need several refactors to improve the current VVC decoder's performance,
 * which will frequently break the API/ABI. To mitigate this, we've copied the executor from
 * avutil to avcodec. Once the API/ABI is stable, we will move this class back to avutil
 */

#ifndef AVCODEC_EXECUTOR_H
#define AVCODEC_EXECUTOR_H

typedef struct FFExecutor FFExecutor;
typedef struct FFTask FFTask;

struct FFTask {
    FFTask *next;
    int priority;   // task priority should >= 0 and < AVTaskCallbacks.priorities
};

typedef struct FFTaskCallbacks {
    void *user_data;

    int local_context_size;

    // how many priorities do we have？
    int priorities;

    // run the task
    int (*run)(FFTask *t, void *local_context, void *user_data);
} FFTaskCallbacks;

/**
 * Alloc executor
 * @param callbacks callback structure for executor
 * @param thread_count worker thread number, 0 for run on caller's thread directly
 * @return return the executor
 */
FFExecutor* ff_executor_alloc(const FFTaskCallbacks *callbacks, int thread_count);

/**
 * Free executor
 * @param e  pointer to executor
 */
void ff_executor_free(FFExecutor **e);

/**
 * Add task to executor
 * @param e pointer to executor
 * @param t pointer to task. If NULL, it will wakeup one work thread
 */
void ff_executor_execute(FFExecutor *e, FFTask *t);

#endif //AVCODEC_EXECUTOR_H
