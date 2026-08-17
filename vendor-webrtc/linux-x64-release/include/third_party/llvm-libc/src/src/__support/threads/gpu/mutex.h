//===--- Implementation of a GPU mutex class --------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC___SUPPORT_THREADS_GPU_MUTEX_H
#define LLVM_LIBC_SRC___SUPPORT_THREADS_GPU_MUTEX_H

#include "src/__support/macros/attributes.h"
#include "src/__support/macros/config.h"
#include "src/__support/threads/mutex_common.h"

namespace LIBC_NAMESPACE_DECL {

/// Implementation of a simple passthrough mutex which guards nothing. A
/// complete Mutex locks in general cannot be implemented on the GPU. We simply
/// define the Mutex interface and require that only a single thread executes
/// code requiring a mutex lock.
struct Mutex {
  LIBC_INLINE constexpr Mutex(bool, bool, bool, bool) {}

  LIBC_INLINE MutexError lock() { return MutexError::NONE; }
  LIBC_INLINE MutexError unlock() { return MutexError::NONE; }
  LIBC_INLINE MutexError reset() { return MutexError::NONE; }
};

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC___SUPPORT_THREADS_GPU_MUTEX_H
