/* Copyright 2018 Google LLC
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google LLC nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

/*
 * Make sure it's defined before including anything else.  A number of syscalls
 * are GNU extensions and rely on being exported by glibc.
 */
#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif

/*
 * Make sure the assert checks aren't removed as all the unittests are based
 * on them.
 */
#undef NDEBUG

#include <assert.h>
#include <fcntl.h>
#include <sched.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/prctl.h>
#include <sys/stat.h>
#include <sys/vfs.h>
#include <sys/wait.h>
#include <unistd.h>

#include <linux/capability.h>

#include "linux_syscall_support.h"

#define SKIP_TEST_EXIT_STATUS 77

void assert_buffers_eq_len(const void *buf1, const void *buf2, size_t len) {
  const uint8_t *u8_1 = (const uint8_t *)buf1;
  const uint8_t *u8_2 = (const uint8_t *)buf2;
  size_t i;

  for (i = 0; i < len; ++i) {
    if (u8_1[i] != u8_2[i])
      printf("offset %zu: %02x != %02x\n", i, u8_1[i], u8_2[i]);
  }
}
#define assert_buffers_eq(obj1, obj2) assert_buffers_eq_len(obj1, obj2, sizeof(*obj1))

// Returns true iff pointed timevals are equal.
static inline bool kernel_timeval_eq(const struct kernel_timeval* lhs,
                                     const struct kernel_timeval* rhs) {
  return (lhs->tv_sec == rhs->tv_sec) && (lhs->tv_usec == rhs->tv_usec);
}

// Returns true iff pointed itimervals are equal.
static inline bool kernel_itimerval_eq(const struct kernel_itimerval* lhs,
                                       const struct kernel_itimerval* rhs) {
  return kernel_timeval_eq(&lhs->it_interval, &rhs->it_interval) &&
         kernel_timeval_eq(&lhs->it_value, &rhs->it_value);
}

