// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/base.h>

#if defined(__GLIBC__) && !defined(__UCLIBC__)
#define OPENSSL_GLIBC
#endif

// This file isn't built on ARM or Aarch64 because we link statically in those
// builds and trying to override malloc in a static link doesn't work. It also
// requires glibc. It's also disabled on ASan builds as this interferes with
// ASan's malloc interceptor.
//
// TODO(davidben): See if this and ASan's and MSan's interceptors can be made to
// coexist.
#if defined(__linux__) && defined(OPENSSL_GLIBC) && !defined(OPENSSL_ARM) && \
    !defined(OPENSSL_AARCH64) && !defined(OPENSSL_ASAN) &&                   \
    !defined(OPENSSL_MSAN) && !defined(OPENSSL_TSAN)

#include <errno.h>
#include <signal.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#include <new>


// This file defines overrides for the standard allocation functions that allow
// a given allocation to be made to fail for testing. If the program is run
// with MALLOC_NUMBER_TO_FAIL set to a base-10 number then that allocation will
// return NULL. If MALLOC_BREAK_ON_FAIL is also defined then the allocation
// will signal SIGTRAP rather than return NULL.
//
// This code is not thread safe.

static uint64_t current_malloc_count = 0;
static uint64_t malloc_number_to_fail = 0;
static bool failure_enabled = false, break_on_fail = false, in_call = false;

extern "C" {
// These are other names for the standard allocation functions.
extern void *__libc_malloc(size_t size);
extern void *__libc_calloc(size_t num_elems, size_t size);
extern void *__libc_realloc(void *ptr, size_t size);
}

static void exit_handler(void) {
  if (failure_enabled && current_malloc_count > malloc_number_to_fail) {
    _exit(88);
  }
}

static void cpp_new_handler() {
  // Return to try again. It won't fail a second time.
  return;
}

// should_fail_allocation returns true if the current allocation should fail.
static bool should_fail_allocation() {
  static bool init = false;

  if (in_call) {
    return false;
  }

  in_call = true;

  if (!init) {
    const char *env = getenv("MALLOC_NUMBER_TO_FAIL");
    if (env != NULL && env[0] != 0) {
      char *endptr;
      malloc_number_to_fail = strtoull(env, &endptr, 10);
      if (*endptr == 0) {
        failure_enabled = true;
        atexit(exit_handler);
        std::set_new_handler(cpp_new_handler);
      }
    }
    break_on_fail = (NULL != getenv("MALLOC_BREAK_ON_FAIL"));
    init = true;
  }

  in_call = false;

  if (!failure_enabled) {
    return false;
  }

  bool should_fail = (current_malloc_count == malloc_number_to_fail);
  current_malloc_count++;

  if (should_fail && break_on_fail) {
    raise(SIGTRAP);
  }
  return should_fail;
}

extern "C" {

void *malloc(size_t size) {
  if (should_fail_allocation()) {
    errno = ENOMEM;
    return NULL;
  }

  return __libc_malloc(size);
}

void *calloc(size_t num_elems, size_t size) {
  if (should_fail_allocation()) {
    errno = ENOMEM;
    return NULL;
  }

  return __libc_calloc(num_elems, size);
}

void *realloc(void *ptr, size_t size) {
  if (should_fail_allocation()) {
    errno = ENOMEM;
    return NULL;
  }

  return __libc_realloc(ptr, size);
}

}  // extern "C"

#endif  // defined(linux) && GLIBC && !ARM && !AARCH64 && !ASAN && !TSAN
