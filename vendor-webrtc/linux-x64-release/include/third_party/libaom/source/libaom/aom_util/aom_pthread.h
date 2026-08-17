/*
 * Copyright (c) 2024, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */
//
// pthread.h wrapper

#ifndef AOM_AOM_UTIL_AOM_PTHREAD_H_
#define AOM_AOM_UTIL_AOM_PTHREAD_H_

#include "config/aom_config.h"

#if CONFIG_MULTITHREAD

#ifdef __cplusplus
extern "C" {
#endif

#if defined(_WIN32) && !HAVE_PTHREAD_H
// Prevent leaking max/min macros.
#undef NOMINMAX
#define NOMINMAX
#undef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#include <errno.h>    // NOLINT
#include <process.h>  // NOLINT
#include <stddef.h>   // NOLINT
#include <windows.h>  // NOLINT
typedef HANDLE pthread_t;
typedef int pthread_attr_t;
typedef CRITICAL_SECTION pthread_mutex_t;

#if _WIN32_WINNT < 0x0600
#error _WIN32_WINNT must target Windows Vista / Server 2008 or newer.
#endif
typedef CONDITION_VARIABLE pthread_cond_t;

#ifndef WINAPI_FAMILY_PARTITION
#define WINAPI_PARTITION_DESKTOP 1
#define WINAPI_FAMILY_PARTITION(x) x
#endif

#if !WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#define USE_CREATE_THREAD
#endif

//------------------------------------------------------------------------------
// simplistic pthread emulation layer

// _beginthreadex requires __stdcall
#if defined(__GNUC__) && \
    (__GNUC__ > 4 || (__GNUC__ == 4 && __GNUC_MINOR__ >= 2))
#define THREADFN __attribute__((force_align_arg_pointer)) unsigned int __stdcall
#else
#define THREADFN unsigned int __stdcall
#endif
#define THREAD_EXIT_SUCCESS 0

static inline int pthread_attr_init(pthread_attr_t *attr) {
  (void)attr;
  return 0;
}

static inline int pthread_attr_destroy(pthread_attr_t *attr) {
  (void)attr;
  return 0;
}

static inline int pthread_attr_getstacksize(const pthread_attr_t *attr,
                                            size_t *stacksize) {
  (void)attr;
  (void)stacksize;
  return EINVAL;
}

static inline int pthread_attr_setstacksize(pthread_attr_t *attr,
                                            size_t stacksize) {
  (void)attr;
  (void)stacksize;
  return EINVAL;
}

static inline int pthread_create(pthread_t *const thread,
                                 const pthread_attr_t *attr,
                                 unsigned int(__stdcall *start)(void *),
                                 void *arg) {
  (void)attr;
#ifdef USE_CREATE_THREAD
  *thread = CreateThread(NULL,          /* lpThreadAttributes */
                         0,             /* dwStackSize */
                         start, arg, 0, /* dwStackSize */
                         NULL);         /* lpThreadId */
#else
  *thread = (pthread_t)_beginthreadex(NULL,          /* void *security */
                                      0,             /* unsigned stack_size */
                                      start, arg, 0, /* unsigned initflag */
                                      NULL);         /* unsigned *thrdaddr */
#endif
  if (*thread == NULL) return 1;
  SetThreadPriority(*thread, THREAD_PRIORITY_ABOVE_NORMAL);
  return 0;
}

static inline int pthread_join(pthread_t thread, void **value_ptr) {
  (void)value_ptr;
  return (WaitForSingleObjectEx(thread, INFINITE, FALSE /*bAlertable*/) !=
              WAIT_OBJECT_0 ||
          CloseHandle(thread) == 0);
}

// Mutex
static inline int pthread_mutex_init(pthread_mutex_t *const mutex,
                                     void *mutexattr) {
  (void)mutexattr;
  InitializeCriticalSectionEx(mutex, 0 /*dwSpinCount*/, 0 /*Flags*/);
  return 0;
}

static inline int pthread_mutex_trylock(pthread_mutex_t *const mutex) {
  return TryEnterCriticalSection(mutex) ? 0 : EBUSY;
}

static inline int pthread_mutex_lock(pthread_mutex_t *const mutex) {
  EnterCriticalSection(mutex);
  return 0;
}

static inline int pthread_mutex_unlock(pthread_mutex_t *const mutex) {
  LeaveCriticalSection(mutex);
  return 0;
}

static inline int pthread_mutex_destroy(pthread_mutex_t *const mutex) {
  DeleteCriticalSection(mutex);
  return 0;
}

// Condition
static inline int pthread_cond_destroy(pthread_cond_t *const condition) {
  (void)condition;
  return 0;
}

static inline int pthread_cond_init(pthread_cond_t *const condition,
                                    void *cond_attr) {
  (void)cond_attr;
  InitializeConditionVariable(condition);
  return 0;
}

static inline int pthread_cond_signal(pthread_cond_t *const condition) {
  WakeConditionVariable(condition);
  return 0;
}

static inline int pthread_cond_broadcast(pthread_cond_t *const condition) {
  WakeAllConditionVariable(condition);
  return 0;
}

static inline int pthread_cond_wait(pthread_cond_t *const condition,
                                    pthread_mutex_t *const mutex) {
  int ok;
  ok = SleepConditionVariableCS(condition, mutex, INFINITE);
  return !ok;
}
#else                 // _WIN32
#include <pthread.h>  // NOLINT
#define THREADFN void *
#define THREAD_EXIT_SUCCESS NULL
#endif

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // CONFIG_MULTITHREAD

#endif  // AOM_AOM_UTIL_AOM_PTHREAD_H_
