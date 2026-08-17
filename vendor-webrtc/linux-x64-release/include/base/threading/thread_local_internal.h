// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_THREADING_THREAD_LOCAL_INTERNAL_H_
#define BASE_THREADING_THREAD_LOCAL_INTERNAL_H_

#include "base/dcheck_is_on.h"

#if DCHECK_IS_ON()

#include <atomic>
#include <memory>
#include <ostream>

#include "base/check_op.h"
#include "base/memory/raw_ptr.h"
#include "base/threading/thread_local_storage.h"

namespace base {
namespace internal {

// A version of ThreadLocalOwnedPointer which verifies that it's only destroyed
// when no threads, other than the one it is destroyed on, have remaining state
// set in it. A ThreadLocalOwnedPointer instance being destroyed too early would
// result in leaks per unregistering the TLS slot (and thus the DeleteTlsPtr
// hook).
template <typename T>
class CheckedThreadLocalOwnedPointer {
 public:
  CheckedThreadLocalOwnedPointer() = default;

  CheckedThreadLocalOwnedPointer(const CheckedThreadLocalOwnedPointer<T>&) =
      delete;
  CheckedThreadLocalOwnedPointer<T>& operator=(
      const CheckedThreadLocalOwnedPointer<T>&) = delete;

  ~CheckedThreadLocalOwnedPointer() {
    Set(nullptr);

    DCHECK_EQ(num_assigned_threads_.load(std::memory_order_relaxed), 0)
        << "Memory leak: Must join all threads or release all associated "
           "thread-local slots before ~ThreadLocalOwnedPointer";
  }

  T* Get() const {
    PtrTracker* const ptr_tracker = static_cast<PtrTracker*>(slot_.Get());
    return ptr_tracker ? ptr_tracker->ptr_.get() : nullptr;
  }

  std::unique_ptr<T> Set(std::unique_ptr<T> ptr) {
    std::unique_ptr<T> existing_ptr;
    auto existing_tracker = static_cast<PtrTracker*>(slot_.Get());
    if (existing_tracker) {
      existing_ptr = std::move(existing_tracker->ptr_);
      delete existing_tracker;
    }

    if (ptr) {
      slot_.Set(new PtrTracker(this, std::move(ptr)));
    } else {
      slot_.Set(nullptr);
    }

    return existing_ptr;
  }

  T& operator*() { return *Get(); }

 private:
  struct PtrTracker {
   public:
    PtrTracker(CheckedThreadLocalOwnedPointer<T>* outer, std::unique_ptr<T> ptr)
        : outer_(outer), ptr_(std::move(ptr)) {
      outer_->num_assigned_threads_.fetch_add(1, std::memory_order_relaxed);
    }

    ~PtrTracker() {
      outer_->num_assigned_threads_.fetch_sub(1, std::memory_order_relaxed);
    }

    const raw_ptr<CheckedThreadLocalOwnedPointer<T>> outer_;
    std::unique_ptr<T> ptr_;
  };

  static void DeleteTlsPtr(void* ptr) { delete static_cast<PtrTracker*>(ptr); }

  ThreadLocalStorage::Slot slot_{&DeleteTlsPtr};

  std::atomic_int num_assigned_threads_{0};
};

}  // namespace internal
}  // namespace base

#endif  // DCHECK_IS_ON()

#endif  // BASE_THREADING_THREAD_LOCAL_INTERNAL_H_
