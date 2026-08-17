//===-- Implementation header for Rwlock's clockwrlock function --*- C++-*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_PTHREAD_PTHREAD_RWLOCK_CLOCKWRLOCK_H
#define LLVM_LIBC_SRC_PTHREAD_PTHREAD_RWLOCK_CLOCKWRLOCK_H

#include "src/__support/macros/config.h"
#include <pthread.h>

namespace LIBC_NAMESPACE_DECL {

int pthread_rwlock_clockwrlock(pthread_rwlock_t *__restrict rwlock,
                               clockid_t clockid,
                               const timespec *__restrict abstime);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_PTHREAD_PTHREAD_RWLOCK_CLOCKWRLOCK_H
