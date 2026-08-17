/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AOM_PORTS_AOM_ONCE_H_
#define AOM_AOM_PORTS_AOM_ONCE_H_

#include "config/aom_config.h"

/* Implement a function wrapper to guarantee initialization
 * thread-safety for library singletons.
 *
 * NOTE: This function uses static locks, and can only be
 * used with one common argument per compilation unit. So
 *
 * file1.c:
 *   aom_once(foo);
 *   ...
 *   aom_once(foo);
 *
 * file2.c:
 *   aom_once(bar);
 *
 * will ensure foo() and bar() are each called only once, but in
 *
 * file1.c:
 *   aom_once(foo);
 *   aom_once(bar):
 *
 * bar() will never be called because the lock is used up
 * by the call to foo().
 */

#if CONFIG_MULTITHREAD && defined(_WIN32)
#undef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
/* Declare a per-compilation-unit state variable to track the progress
 * of calling func() only once. This must be at global scope because
 * local initializers are not thread-safe in MSVC prior to Visual
 * Studio 2015.
 */
static INIT_ONCE aom_init_once = INIT_ONCE_STATIC_INIT;

static void aom_once(void (*func)(void)) {
  BOOL pending;
  InitOnceBeginInitialize(&aom_init_once, 0, &pending, NULL);
  if (!pending) {
    // Initialization has already completed.
    return;
  }
  func();
  InitOnceComplete(&aom_init_once, 0, NULL);
}

#elif CONFIG_MULTITHREAD && HAVE_PTHREAD_H
#include <pthread.h>
static void aom_once(void (*func)(void)) {
  static pthread_once_t lock = PTHREAD_ONCE_INIT;
  pthread_once(&lock, func);
}

#else
/* Default version that performs no synchronization. */

static void aom_once(void (*func)(void)) {
  static volatile int done;

  if (!done) {
    func();
    done = 1;
  }
}
#endif

#endif  // AOM_AOM_PORTS_AOM_ONCE_H_
