// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This is a low level implementation of atomic semantics for reference
// counting.  Please use base/memory/ref_counted.h directly instead.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_ATOMIC_REF_COUNT_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_ATOMIC_REF_COUNT_H_

#include <atomic>

namespace partition_alloc::internal::base {

class AtomicRefCount {
 public:
  constexpr AtomicRefCount() : ref_count_(0) {}
  explicit constexpr AtomicRefCount(int initial_value)
      : ref_count_(initial_value) {}

  // Increment a reference count.
  // Returns the previous value of the count.
  int Increment() { return Increment(1); }

  // Increment a reference count by "increment", which must exceed 0.
  // Returns the previous value of the count.
  int Increment(int increment) {
    return ref_count_.fetch_add(increment, std::memory_order_relaxed);
  }

  // Decrement a reference count, and return whether the result is non-zero.
  // Insert barriers to ensure that state written before the reference count
  // became zero will be visible to a thread that has just made the count zero.
  bool Decrement() {
    // TODO(jbroman): Technically this doesn't need to be an acquire operation
    // unless the result is 1 (i.e., the ref count did indeed reach zero).
    // However, there are toolchain issues that make that not work as well at
    // present (notably TSAN doesn't like it).
    return ref_count_.fetch_sub(1, std::memory_order_acq_rel) != 1;
  }

  // Return whether the reference count is one.  If the reference count is used
  // in the conventional way, a reference count of 1 implies that the current
  // thread owns the reference and no other thread shares it.  This call
  // performs the test for a reference count of one, and performs the memory
  // barrier needed for the owning thread to act on the object, knowing that it
  // has exclusive access to the object.
  bool IsOne() const { return ref_count_.load(std::memory_order_acquire) == 1; }

  // Return whether the reference count is zero.  With conventional object
  // referencing counting, the object will be destroyed, so the reference count
  // should never be zero.  Hence this is generally used for a debug check.
  bool IsZero() const {
    return ref_count_.load(std::memory_order_acquire) == 0;
  }

  // Returns the current reference count (with no barriers). This is subtle, and
  // should be used only for debugging.
  int SubtleRefCountForDebug() const {
    return ref_count_.load(std::memory_order_relaxed);
  }

 private:
  std::atomic_int ref_count_;
};

}  // namespace partition_alloc::internal::base

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_ATOMIC_REF_COUNT_H_
