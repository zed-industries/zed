// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_UTIL_MULTI_MUTEX_LOCK_H_
#define IPCZ_SRC_UTIL_MULTI_MUTEX_LOCK_H_

#include <algorithm>
#include <array>
#include <cstddef>

#include "third_party/abseil-cpp/absl/synchronization/mutex.h"

namespace ipcz {

// MultiMutexLock is a scoped mutex locker capable of locking between two and
// four mutexes simultaneously. Locks are always acquired in a globally
// consistent order  based on the address of each Mutex.
template <size_t N>
class ABSL_SCOPED_LOCKABLE MultiMutexLock {
 public:
  MultiMutexLock(absl::Mutex* a, absl::Mutex* b)
      ABSL_EXCLUSIVE_LOCK_FUNCTION(*a, *b)
      : mutexes_({a, b}) {
    SortAndLock();
  }

  MultiMutexLock(absl::Mutex* a, absl::Mutex* b, absl::Mutex* c)
      ABSL_EXCLUSIVE_LOCK_FUNCTION(*a, *b, *c)
      : mutexes_({a, b, c}) {
    SortAndLock();
  }

  MultiMutexLock(absl::Mutex* a, absl::Mutex* b, absl::Mutex* c, absl::Mutex* d)
      ABSL_EXCLUSIVE_LOCK_FUNCTION(*a, *b, *c, *d)
      : mutexes_({a, b, c, d}) {
    SortAndLock();
  }

  ~MultiMutexLock() ABSL_UNLOCK_FUNCTION() {
    for (absl::Mutex* mutex : mutexes_) {
      mutex->Unlock();
    }
  }

 private:
  void SortAndLock() ABSL_NO_THREAD_SAFETY_ANALYSIS {
    std::sort(mutexes_.begin(), mutexes_.end());
    for (absl::Mutex* mutex : mutexes_) {
      mutex->Lock();
    }
  }

  std::array<absl::Mutex*, N> mutexes_;
};

// Helpful deduction guides so instantiations can omit a template argument.
MultiMutexLock(absl::Mutex*, absl::Mutex*)->MultiMutexLock<2>;
MultiMutexLock(absl::Mutex*, absl::Mutex*, absl::Mutex*)->MultiMutexLock<3>;
MultiMutexLock(absl::Mutex*, absl::Mutex*, absl::Mutex*, absl::Mutex*)
    ->MultiMutexLock<4>;

}  // namespace ipcz

#endif  // IPCZ_SRC_UTIL_MULTI_MUTEX_LOCK_H_
