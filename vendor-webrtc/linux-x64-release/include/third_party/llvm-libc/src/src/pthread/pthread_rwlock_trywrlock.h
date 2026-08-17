//===-- Implementation header for Rwlock's trywrlock function ----*- C++-*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_PTHREAD_PTHREAD_RWLOCK_TRYWRLOCK_H
#define LLVM_LIBC_SRC_PTHREAD_PTHREAD_RWLOCK_TRYWRLOCK_H

#include "src/__support/macros/config.h"
#include <pthread.h>

namespace LIBC_NAMESPACE_DECL {

int pthread_rwlock_trywrlock(pthread_rwlock_t *rwlock);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_PTHREAD_PTHREAD_RWLOCK_TRYWRLOCK_H
