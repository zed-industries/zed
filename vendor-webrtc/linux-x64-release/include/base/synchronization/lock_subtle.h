// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_SYNCHRONIZATION_LOCK_SUBTLE_H_
#define BASE_SYNCHRONIZATION_LOCK_SUBTLE_H_

#include "base/base_export.h"
#include "base/containers/span.h"
#include "base/dcheck_is_on.h"

namespace base::subtle {

#if DCHECK_IS_ON()
// Returns addresses of locks acquired by the current thread with
// `subtle::LockTracking::kEnabled`. `uintptr_t` is used because addresses are
// meant to be used as unique identifiers but not to be dereferenced.
BASE_EXPORT span<const uintptr_t> GetTrackedLocksHeldByCurrentThread();
#endif

// Whether to add a lock to the list returned by
// `subtle::GetLocksHeldByCurrentThread()` upon acquisition. This has no effect
// in non-DCHECK builds because tracking is always disabled. This is disabled by
// default to avoid exceeding the fixed-size storage backing
// `GetTrackedLocksHeldByCurrentThread()` and to avoid reentrancy, e.g.:
//
//     thread_local implementation
//     Add lock to the thread_local array of locks held by current thread
//     base::Lock::Acquire from allocator shim
//     ... Allocator shim ...
//     thread_local implementation
//     Access to a thread_local variable
//
// A lock acquired with `subtle::LockTracking::kEnabled` can be used to provide
// a mutual exclusion guarantee for SEQUENCE_CHECKER.
enum class LockTracking {
  kDisabled,
  kEnabled,
};

}  // namespace base::subtle

#endif  // BASE_SYNCHRONIZATION_LOCK_SUBTLE_H_
