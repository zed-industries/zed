//===--- Implementation of a Linux mutex class ------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC___SUPPORT_THREADS_LINUX_MUTEX_H
#define LLVM_LIBC_SRC___SUPPORT_THREADS_LINUX_MUTEX_H

#include "hdr/types/pid_t.h"
#include "src/__support/CPP/optional.h"
#include "src/__support/libc_assert.h"
#include "src/__support/macros/config.h"
#include "src/__support/threads/linux/futex_utils.h"
#include "src/__support/threads/linux/raw_mutex.h"
#include "src/__support/threads/mutex_common.h"

namespace LIBC_NAMESPACE_DECL {

// TODO: support shared/recursive/robust mutexes.
class Mutex final : private RawMutex {
  // reserved timed, may be useful when combined with other flags.
  unsigned char timed;
  unsigned char recursive;
  unsigned char robust;
  unsigned char pshared;

  // TLS address may not work across forked processes. Use thread id instead.
  pid_t owner;
  unsigned long long lock_count;

public:
  LIBC_INLINE constexpr Mutex(bool is_timed, bool is_recursive, bool is_robust,
                              bool is_pshared)
      : RawMutex(), timed(is_timed), recursive(is_recursive), robust(is_robust),
        pshared(is_pshared), owner(0), lock_count(0) {}

  LIBC_INLINE static MutexError init(Mutex *mutex, bool is_timed, bool isrecur,
                                     bool isrobust, bool is_pshared) {
    RawMutex::init(mutex);
    mutex->timed = is_timed;
    mutex->recursive = isrecur;
    mutex->robust = isrobust;
    mutex->pshared = is_pshared;
    mutex->owner = 0;
    mutex->lock_count = 0;
    return MutexError::NONE;
  }

  LIBC_INLINE static MutexError destroy(Mutex *lock) {
    LIBC_ASSERT(lock->owner == 0 && lock->lock_count == 0 &&
                "Mutex destroyed while being locked.");
    RawMutex::destroy(lock);
    return MutexError::NONE;
  }

  // TODO: record owner and lock count.
  LIBC_INLINE MutexError lock() {
    // Since timeout is not specified, we do not need to check the return value.
    this->RawMutex::lock(
        /* timeout=*/cpp::nullopt, this->pshared);
    return MutexError::NONE;
  }

  // TODO: record owner and lock count.
  LIBC_INLINE MutexError timed_lock(internal::AbsTimeout abs_time) {
    if (this->RawMutex::lock(abs_time, this->pshared))
      return MutexError::NONE;
    return MutexError::TIMEOUT;
  }

  LIBC_INLINE MutexError unlock() {
    if (this->RawMutex::unlock(this->pshared))
      return MutexError::NONE;
    return MutexError::UNLOCK_WITHOUT_LOCK;
  }

  // TODO: record owner and lock count.
  LIBC_INLINE MutexError try_lock() {
    if (this->RawMutex::try_lock())
      return MutexError::NONE;
    return MutexError::BUSY;
  }
};

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC___SUPPORT_THREADS_LINUX_MUTEX_H
