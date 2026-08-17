/*
 * Copyright (c) 2011-2017 KO Myung-Hun <komh@chollian.net>
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
 * os2threads to pthreads wrapper
 */

#ifndef COMPAT_OS2THREADS_H
#define COMPAT_OS2THREADS_H

#define INCL_DOS
#define INCL_DOSERRORS
#include <os2.h>

#undef __STRICT_ANSI__          /* for _beginthread() */
#include <stdlib.h>
#include <time.h>

#include <sys/builtin.h>
#include <sys/fmutex.h>

#include "libavutil/attributes.h"
#include "libavutil/common.h"
#include "libavutil/time.h"

typedef struct {
    TID tid;
    void *(*start_routine)(void *);
    void *arg;
    void *result;
} pthread_t;

typedef void pthread_attr_t;

typedef _fmutex pthread_mutex_t;
typedef void pthread_mutexattr_t;

#define PTHREAD_MUTEX_INITIALIZER _FMUTEX_INITIALIZER

typedef struct {
    HEV event_sem;
    HEV ack_sem;
    volatile unsigned  wait_count;
} pthread_cond_t;

typedef void pthread_condattr_t;

typedef struct {
    volatile int done;
    _fmutex mtx;
} pthread_once_t;

#define PTHREAD_ONCE_INIT {0, _FMUTEX_INITIALIZER}

static void thread_entry(void *arg)
{
    pthread_t *thread = arg;

    thread->result = thread->start_routine(thread->arg);
}

static av_always_inline int pthread_create(pthread_t *thread,
                                           const pthread_attr_t *attr,
                                           void *(*start_routine)(void*),
                                           void *arg)
{
    thread->start_routine = start_routine;
    thread->arg = arg;
    thread->result = NULL;

    thread->tid = _beginthread(thread_entry, NULL, 1024 * 1024, thread);

    return 0;
}

static av_always_inline int pthread_join(pthread_t thread, void **value_ptr)
{
    DosWaitThread(&thread.tid, DCWW_WAIT);

    if (value_ptr)
        *value_ptr = thread.result;

    return 0;
}

static av_always_inline int pthread_mutex_init(pthread_mutex_t *mutex,
                                               const pthread_mutexattr_t *attr)
{
    _fmutex_create(mutex, 0);

    return 0;
}

static av_always_inline int pthread_mutex_destroy(pthread_mutex_t *mutex)
{
    _fmutex_close(mutex);

    return 0;
}

static av_always_inline int pthread_mutex_lock(pthread_mutex_t *mutex)
{
    _fmutex_request(mutex, 0);

    return 0;
}

static av_always_inline int pthread_mutex_unlock(pthread_mutex_t *mutex)
{
    _fmutex_release(mutex);

    return 0;
}

static av_always_inline int pthread_cond_init(pthread_cond_t *cond,
                                              const pthread_condattr_t *attr)
{
    DosCreateEventSem(NULL, &cond->event_sem, DCE_POSTONE, FALSE);
    DosCreateEventSem(NULL, &cond->ack_sem, DCE_POSTONE, FALSE);

    cond->wait_count = 0;

    return 0;
}

static av_always_inline int pthread_cond_destroy(pthread_cond_t *cond)
{
    DosCloseEventSem(cond->event_sem);
    DosCloseEventSem(cond->ack_sem);

    return 0;
}

static av_always_inline int pthread_cond_signal(pthread_cond_t *cond)
{
    if (!__atomic_cmpxchg32(&cond->wait_count, 0, 0)) {
        DosPostEventSem(cond->event_sem);
        DosWaitEventSem(cond->ack_sem, SEM_INDEFINITE_WAIT);
    }

    return 0;
}

static av_always_inline int pthread_cond_broadcast(pthread_cond_t *cond)
{
    while (!__atomic_cmpxchg32(&cond->wait_count, 0, 0))
        pthread_cond_signal(cond);

    return 0;
}

static av_always_inline int pthread_cond_timedwait(pthread_cond_t *cond,
                                                   pthread_mutex_t *mutex,
                                                   const struct timespec *abstime)
{
    int64_t abs_milli = abstime->tv_sec * 1000LL + abstime->tv_nsec / 1000000;
    ULONG t = av_clip64(abs_milli - av_gettime() / 1000, 0, ULONG_MAX);

    __atomic_increment(&cond->wait_count);

    pthread_mutex_unlock(mutex);

    APIRET ret = DosWaitEventSem(cond->event_sem, t);

    __atomic_decrement(&cond->wait_count);

    DosPostEventSem(cond->ack_sem);

    pthread_mutex_lock(mutex);

    return (ret == ERROR_TIMEOUT) ? ETIMEDOUT : 0;
}

static av_always_inline int pthread_cond_wait(pthread_cond_t *cond,
                                              pthread_mutex_t *mutex)
{
    __atomic_increment(&cond->wait_count);

    pthread_mutex_unlock(mutex);

    DosWaitEventSem(cond->event_sem, SEM_INDEFINITE_WAIT);

    __atomic_decrement(&cond->wait_count);

    DosPostEventSem(cond->ack_sem);

    pthread_mutex_lock(mutex);

    return 0;
}

static av_always_inline int pthread_once(pthread_once_t *once_control,
                                         void (*init_routine)(void))
{
    if (!once_control->done)
    {
        _fmutex_request(&once_control->mtx, 0);

        if (!once_control->done)
        {
            init_routine();

            once_control->done = 1;
        }

        _fmutex_release(&once_control->mtx);
    }

    return 0;
}
#endif /* COMPAT_OS2THREADS_H */
