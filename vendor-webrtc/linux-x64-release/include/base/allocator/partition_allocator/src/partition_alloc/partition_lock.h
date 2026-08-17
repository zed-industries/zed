// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_LOCK_H_
#define PARTITION_ALLOC_PARTITION_LOCK_H_

#include <atomic>
#include <type_traits>

#include "partition_alloc/build_config.h"
#include "partition_alloc/buildflags.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/immediate_crash.h"
#include "partition_alloc/partition_alloc_base/thread_annotations.h"
#include "partition_alloc/partition_alloc_base/threading/platform_thread.h"
#include "partition_alloc/partition_alloc_check.h"
#include "partition_alloc/spinning_mutex.h"
#include "partition_alloc/thread_isolation/thread_isolation.h"

namespace partition_alloc::internal {

class PA_LOCKABLE Lock {
 public:
  inline constexpr Lock();
  void Acquire() PA_EXCLUSIVE_LOCK_FUNCTION() {
#if PA_BUILDFLAG(DCHECKS_ARE_ON) || \
    PA_BUILDFLAG(ENABLE_PARTITION_LOCK_REENTRANCY_CHECK)
#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
    LiftThreadIsolationScope lift_thread_isolation_restrictions;
#endif

    // When PartitionAlloc is malloc(), it can easily become reentrant. For
    // instance, a DCHECK() triggers in external code (such as
    // base::Lock). DCHECK() error message formatting allocates, which triggers
    // PartitionAlloc, and then we get reentrancy, and in this case infinite
    // recursion.
    //
    // To avoid that, crash quickly when the code becomes reentrant.
    base::PlatformThreadRef current_thread = base::PlatformThread::CurrentRef();
    if (!lock_.Try()) {
      // The lock wasn't free when we tried to acquire it. This can be because
      // another thread or *this* thread was holding it.
      //
      // If it's this thread holding it, then it cannot have become free in the
      // meantime, and the current value of |owning_thread_ref_| is valid, as it
      // was set by this thread. Assuming that writes to |owning_thread_ref_|
      // are atomic, then if it's us, we are trying to recursively acquire a
      // non-recursive lock.
      //
      // Note that we don't rely on a DCHECK() in base::Lock(), as it would
      // itself allocate. Meaning that without this code, a reentrancy issue
      // hangs on Linux.
      if (owning_thread_ref_.load(std::memory_order_acquire) == current_thread)
          [[unlikely]] {
        // Trying to acquire lock while it's held by this thread: reentrancy
        // issue.
        ReentrancyIssueDetected();
      }
      lock_.Acquire();
    }
    owning_thread_ref_.store(current_thread, std::memory_order_release);
#else
    lock_.Acquire();
#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON) ||
        // PA_BUILDFLAG(ENABLE_PARTITION_LOCK_REENTRANCY_CHECK)
  }

  void Release() PA_UNLOCK_FUNCTION() {
#if PA_BUILDFLAG(DCHECKS_ARE_ON) || \
    PA_BUILDFLAG(ENABLE_PARTITION_LOCK_REENTRANCY_CHECK)
#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
    LiftThreadIsolationScope lift_thread_isolation_restrictions;
#endif
    owning_thread_ref_.store(base::PlatformThreadRef(),
                             std::memory_order_release);
#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON) ||
        // PA_BUILDFLAG(ENABLE_PARTITION_LOCK_REENTRANCY_CHECK)
    lock_.Release();
  }

  bool TryAcquire() PA_EXCLUSIVE_TRYLOCK_FUNCTION(true) {
    // Unlike `Acquire()`, `TryAcquire()` does not provide any reentrancy
    // protection. Instead, it just fails and returns `false`.
    // Hence this should not be used in (de)allocation code path.
    if (!lock_.Try()) {
      return false;
    }

#if PA_BUILDFLAG(DCHECKS_ARE_ON) || \
    PA_BUILDFLAG(ENABLE_PARTITION_LOCK_REENTRANCY_CHECK)
#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
    LiftThreadIsolationScope lift_thread_isolation_restrictions;
#endif

    base::PlatformThreadRef current_thread = base::PlatformThread::CurrentRef();
    owning_thread_ref_.store(current_thread, std::memory_order_release);
#endif

    return true;
  }

  void AssertAcquired() const PA_ASSERT_EXCLUSIVE_LOCK() {
    lock_.AssertAcquired();
#if PA_BUILDFLAG(DCHECKS_ARE_ON) || \
    PA_BUILDFLAG(ENABLE_PARTITION_LOCK_REENTRANCY_CHECK)
#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
    LiftThreadIsolationScope lift_thread_isolation_restrictions;
#endif
    PA_DCHECK(owning_thread_ref_.load(std ::memory_order_acquire) ==
              base::PlatformThread::CurrentRef());
#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON) ||
        // PA_BUILDFLAG(ENABLE_PARTITION_LOCK_REENTRANCY_CHECK)
  }

  void Reinit() PA_UNLOCK_FUNCTION() {
    lock_.AssertAcquired();
#if PA_BUILDFLAG(DCHECKS_ARE_ON) || \
    PA_BUILDFLAG(ENABLE_PARTITION_LOCK_REENTRANCY_CHECK)
    owning_thread_ref_.store(base::PlatformThreadRef(),
                             std::memory_order_release);
#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON) ||
        // PA_BUILDFLAG(ENABLE_PARTITION_LOCK_REENTRANCY_CHECK)
    lock_.Reinit();
  }

#if PA_BUILDFLAG(ENABLE_PARTITION_LOCK_PRIORITY_INHERITANCE)
  bool HasWaitersForTesting() const { return lock_.HasWaitersForTesting(); }
#endif

 private:
  [[noreturn]] PA_NOINLINE PA_NOT_TAIL_CALLED void ReentrancyIssueDetected() {
    PA_NO_CODE_FOLDING();
    PA_IMMEDIATE_CRASH();
  }

  SpinningMutex lock_;

#if PA_BUILDFLAG(DCHECKS_ARE_ON) || \
    PA_BUILDFLAG(ENABLE_PARTITION_LOCK_REENTRANCY_CHECK)
  // Should in theory be protected by |lock_|, but we need to read it to detect
  // recursive lock acquisition (and thus, the allocator becoming reentrant).
  std::atomic<base::PlatformThreadRef> owning_thread_ref_ =
      base::PlatformThreadRef();
#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON) ||
        // PA_BUILDFLAG(ENABLE_PARTITION_LOCK_REENTRANCY_CHECK)
};

class PA_SCOPED_LOCKABLE ScopedGuard {
 public:
  explicit ScopedGuard(Lock& lock) PA_EXCLUSIVE_LOCK_FUNCTION(lock)
      : lock_(lock) {
    lock_.Acquire();
  }
  ~ScopedGuard() PA_UNLOCK_FUNCTION() { lock_.Release(); }

 private:
  Lock& lock_;
};

class PA_SCOPED_LOCKABLE ScopedUnlockGuard {
 public:
  explicit ScopedUnlockGuard(Lock& lock) PA_UNLOCK_FUNCTION(lock)
      : lock_(lock) {
    lock_.Release();
  }
  ~ScopedUnlockGuard() PA_EXCLUSIVE_LOCK_FUNCTION() { lock_.Acquire(); }

 private:
  Lock& lock_;
};

constexpr Lock::Lock() = default;

// We want PartitionRoot to not have a global destructor, so this should not
// have one.
static_assert(std::is_trivially_destructible_v<Lock>, "");

}  // namespace partition_alloc::internal

namespace base {
namespace internal {

using PartitionLock = ::partition_alloc::internal::Lock;
using PartitionAutoLock = ::partition_alloc::internal::ScopedGuard;

}  // namespace internal
}  // namespace base

#endif  // PARTITION_ALLOC_PARTITION_LOCK_H_
