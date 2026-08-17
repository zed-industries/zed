/*
 * Copyright © 2018-2021, VideoLAN and dav1d authors
 * Copyright © 2018, Two Orioles, LLC
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef DAV1D_SRC_THREAD_H
#define DAV1D_SRC_THREAD_H

#if defined(_WIN32)

#include <limits.h>
#include <windows.h>

#define PTHREAD_MUTEX_INITIALIZER SRWLOCK_INIT
#define PTHREAD_ONCE_INIT INIT_ONCE_STATIC_INIT

typedef struct {
    HANDLE h;
    void *(*func)(void*);
    void *arg;
} pthread_t;

typedef struct {
    unsigned stack_size;
} pthread_attr_t;

typedef SRWLOCK pthread_mutex_t;
typedef CONDITION_VARIABLE pthread_cond_t;
typedef INIT_ONCE pthread_once_t;

void dav1d_init_thread(void);
void dav1d_set_thread_name(const wchar_t *name);
#define dav1d_set_thread_name(name) dav1d_set_thread_name(L##name)

int dav1d_pthread_create(pthread_t *thread, const pthread_attr_t *attr,
                         void *(*func)(void*), void *arg);
int dav1d_pthread_join(pthread_t *thread, void **res);
int dav1d_pthread_once(pthread_once_t *once_control,
                       void (*init_routine)(void));

#define pthread_create dav1d_pthread_create
#define pthread_join(thread, res) dav1d_pthread_join(&(thread), res)
#define pthread_once   dav1d_pthread_once

static inline int pthread_attr_init(pthread_attr_t *const attr) {
    attr->stack_size = 0;
    return 0;
}

static inline int pthread_attr_destroy(pthread_attr_t *const attr) {
    return 0;
}

static inline int pthread_attr_setstacksize(pthread_attr_t *const attr,
                                            const size_t stack_size)
{
    if (stack_size > UINT_MAX) return 1;
    attr->stack_size = (unsigned) stack_size;
    return 0;
}

static inline int pthread_mutex_init(pthread_mutex_t *const mutex,
                                     const void *const attr)
{
    InitializeSRWLock(mutex);
    return 0;
}

static inline int pthread_mutex_destroy(pthread_mutex_t *const mutex) {
    return 0;
}

static inline int pthread_mutex_lock(pthread_mutex_t *const mutex) {
    AcquireSRWLockExclusive(mutex);
    return 0;
}

static inline int pthread_mutex_unlock(pthread_mutex_t *const mutex) {
    ReleaseSRWLockExclusive(mutex);
    return 0;
}

static inline int pthread_cond_init(pthread_cond_t *const cond,
                                    const void *const attr)
{
    InitializeConditionVariable(cond);
    return 0;
}

static inline int pthread_cond_destroy(pthread_cond_t *const cond) {
    return 0;
}

static inline int pthread_cond_wait(pthread_cond_t *const cond,
                                    pthread_mutex_t *const mutex)
{
    return !SleepConditionVariableSRW(cond, mutex, INFINITE, 0);
}

static inline int pthread_cond_signal(pthread_cond_t *const cond) {
    WakeConditionVariable(cond);
    return 0;
}

static inline int pthread_cond_broadcast(pthread_cond_t *const cond) {
    WakeAllConditionVariable(cond);
    return 0;
}

#else

#include <pthread.h>
#if defined(__FreeBSD__)
 /* ALIGN from <sys/param.h> conflicts with ALIGN from "common/attributes.h" */
#define _SYS_PARAM_H_
#include <sys/types.h>
#endif
#if HAVE_PTHREAD_NP_H
#include <pthread_np.h>
#endif

#define dav1d_init_thread() do {} while (0)

/* Thread naming support */

#ifdef __linux__

#include <sys/prctl.h>

static inline void dav1d_set_thread_name(const char *const name) {
    prctl(PR_SET_NAME, name);
}

#elif HAVE_PTHREAD_SETNAME_NP && defined(__APPLE__)

static inline void dav1d_set_thread_name(const char *const name) {
    pthread_setname_np(name);
}

#elif HAVE_PTHREAD_SETNAME_NP && defined(__NetBSD__)

static inline void dav1d_set_thread_name(const char *const name) {
    pthread_setname_np(pthread_self(), "%s", (void*)name);
}

#elif HAVE_PTHREAD_SETNAME_NP

static inline void dav1d_set_thread_name(const char *const name) {
    pthread_setname_np(pthread_self(), name);
}

#elif HAVE_PTHREAD_SET_NAME_NP

static inline void dav1d_set_thread_name(const char *const name) {
    pthread_set_name_np(pthread_self(), name);
}

#elif defined(__HAIKU__)

#include <os/kernel/OS.h>

static inline void dav1d_set_thread_name(const char *const name) {
    rename_thread(find_thread(NULL), name);
}

#else

#define dav1d_set_thread_name(name) do {} while (0)

#endif

#endif

#endif /* DAV1D_SRC_THREAD_H */
