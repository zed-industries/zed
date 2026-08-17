//===-- Implementation header for pthread_rwlockattr_setkind_np -*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_PTHREAD_PTHREAD_RWLOCKATTR_SETKIND_NP_H
#define LLVM_LIBC_SRC_PTHREAD_PTHREAD_RWLOCKATTR_SETKIND_NP_H

#include "src/__support/macros/config.h"
#include <pthread.h>

namespace LIBC_NAMESPACE_DECL {

int pthread_rwlockattr_setkind_np(pthread_rwlockattr_t *attr, int pref);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_PTHREAD_PTHREAD_RWLOCKATTR_SETKIND_NP_H
