//===-- Implementation header for pthread_attr_getstack ---------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_PTHREAD_PTHREAD_ATTR_GETSTACK_H
#define LLVM_LIBC_SRC_PTHREAD_PTHREAD_ATTR_GETSTACK_H

#include "src/__support/macros/config.h"
#include <pthread.h>

namespace LIBC_NAMESPACE_DECL {

int pthread_attr_getstack(const pthread_attr_t *__restrict attr,
                          void **__restrict stack,
                          size_t *__restrict stacksize);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_PTHREAD_PTHREAD_ATTR_GETSTACK_H
