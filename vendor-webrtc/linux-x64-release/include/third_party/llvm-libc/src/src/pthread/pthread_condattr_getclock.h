//===-- Implementation header for pthread_condattr_getclock -----*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_PTHREAD_PTHREAD_CONDATTR_GETCLOCK_H
#define LLVM_LIBC_SRC_PTHREAD_PTHREAD_CONDATTR_GETCLOCK_H

#include "src/__support/macros/config.h"
#include <pthread.h>   // pthread_condattr_t
#include <sys/types.h> // clockid_t

namespace LIBC_NAMESPACE_DECL {

int pthread_condattr_getclock(const pthread_condattr_t *__restrict attr,
                              clockid_t *__restrict clock_id);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_PTHREAD_PTHREAD_CONDATTR_GETCLOCK_H
