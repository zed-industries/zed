// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_SHARED_MUTEX_H_
#define PARTITION_ALLOC_PARTITION_SHARED_MUTEX_H_

#include "partition_alloc/partition_alloc_base/thread_annotations.h"
#include "partition_alloc/partition_lock.h"

namespace partition_alloc::internal {

// A partial implementation of `std::shared_mutex` for PartitionAllocator.
// Since `std::shared_mutex` allocates memory, we cannot use it inside
// PartitionAllocator. The difference between `std::shared_mutex` and this
// SharedMutex, this SharedMutex doesn't support try_lock() and
// try_lock_shared(), because no code uses the methods.
class PA_LOCKABLE SharedMutex {
 public:
  inline constexpr SharedMutex() = default;

  void lock() PA_EXCLUSIVE_LOCK_FUNCTION() { writer_lock_.Acquire(); }

  void unlock() PA_UNLOCK_FUNCTION() { writer_lock_.Release(); }

  void lock_shared() PA_SHARED_LOCK_FUNCTION() {
    ScopedGuard lock(reader_lock_);
    ++counter_;
    if (counter_ == 1u) {
      writer_lock_.Acquire();
    }
  }

  void unlock_shared() PA_UNLOCK_FUNCTION() {
    ScopedGuard lock(reader_lock_);
    --counter_;
    if (counter_ == 0u) {
      writer_lock_.Release();
    }
  }

 private:
  Lock reader_lock_;
  Lock writer_lock_;
  size_t counter_ PA_GUARDED_BY(reader_lock_) = 0;
};

static_assert(std::is_trivially_destructible_v<SharedMutex>,
              "SharedMutex must be trivally destructible.");

// A partial implementation of `std::unique_lock` for PartitionAllocator.
// Locking a UniqueLock locks the associated shared mutex in exclusive mode.
class PA_SCOPED_LOCKABLE UniqueLock {
 public:
  explicit UniqueLock(SharedMutex& mutex) PA_EXCLUSIVE_LOCK_FUNCTION(mutex)
      : mutex_(mutex) {
    mutex_.lock();
  }
  ~UniqueLock() PA_UNLOCK_FUNCTION() { mutex_.unlock(); }

 private:
  SharedMutex& mutex_;
};

// A partial implementation of `std::shared_lock` for PartitionAllocator.
// Locking a SharedLock locks the associated shared mutex in shared mode.
// (like std::shared_lock).
class PA_SCOPED_LOCKABLE SharedLock {
 public:
  explicit SharedLock(SharedMutex& mutex) PA_SHARED_LOCK_FUNCTION(mutex)
      : mutex_(mutex) {
    mutex_.lock_shared();
  }
  ~SharedLock() PA_UNLOCK_FUNCTION() { mutex_.unlock_shared(); }

 private:
  SharedMutex& mutex_;
};

}  // namespace partition_alloc::internal

#endif  // PARTITION_ALLOC_PARTITION_SHARED_MUTEX_H_
