//===-- Implementation header for pthread_mutexattr_gettype -----*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_PTHREAD_PTHREAD_MUTEXATTR_GETTYPE_H
#define LLVM_LIBC_SRC_PTHREAD_PTHREAD_MUTEXATTR_GETTYPE_H

#include "src/__support/macros/config.h"
#include <pthread.h>

namespace LIBC_NAMESPACE_DECL {

int pthread_mutexattr_gettype(const pthread_mutexattr_t *__restrict attr,
                              int *__restrict type);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_PTHREAD_PTHREAD_MUTEXATTR_GETTYPE_H
