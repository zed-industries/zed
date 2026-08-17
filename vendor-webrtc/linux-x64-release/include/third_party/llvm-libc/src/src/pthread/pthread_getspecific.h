//===-- Implementation header for pthread_getspecific function --*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_PTHREAD_PTHREAD_GETSPECIFIC_H
#define LLVM_LIBC_SRC_PTHREAD_PTHREAD_GETSPECIFIC_H

#include "src/__support/macros/config.h"
#include <pthread.h>
#include <stddef.h>

namespace LIBC_NAMESPACE_DECL {

void *pthread_getspecific(pthread_key_t);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_PTHREAD_PTHREAD_GETSPECIFIC_H
