// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_SPINNING_MUTEX_H_
#define PARTITION_ALLOC_SPINNING_MUTEX_H_

#include <atomic>
#include <cstdint>

#include "partition_alloc/build_config.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_base/cxx_wrapper/algorithm.h"
#include "partition_alloc/partition_alloc_base/thread_annotations.h"
#include "partition_alloc/partition_alloc_base/threading/platform_thread.h"
#include "partition_alloc/partition_alloc_check.h"
#include "partition_alloc/partition_alloc_config.h"
#include "partition_alloc/yield_processor.h"

#if PA_BUILDFLAG(IS_WIN)
#include "partition_alloc/partition_alloc_base/win/windows_types.h"
#endif

#if PA_BUILDFLAG(IS_POSIX)
#include <pthread.h>

#include <cerrno>
#endif

#if PA_BUILDFLAG(IS_APPLE)
#include <os/lock.h>
#endif  // PA_BUILDFLAG(IS_APPLE)

#if PA_BUILDFLAG(IS_FUCHSIA)
#include <lib/sync/mutex.h>
#endif

#if PA_CONFIG(HAS_LINUX_KERNEL) && \
    PA_BUILDFLAG(ENABLE_PARTITION_LOCK_PRIORITY_INHERITANCE)
#include <linux/futex.h>
#endif

namespace partition_alloc::internal {

// The behavior of this class depends on platform support:
// 1. When platform supports is available:
//
// Simple spinning lock. It will spin in user space a set number of times before
// going into the kernel to sleep.
//
// This is intended to give "the best of both worlds" between a SpinLock and
// base::Lock:
// - SpinLock: Inlined fast path, no external function calls, just
//   compare-and-swap. Short waits do not go into the kernel. Good behavior in
//   low contention cases.
// - base::Lock: Good behavior in case of contention.
//
// We don't rely on base::Lock which we could make spin (by calling Try() in a
// loop), as performance is below a custom spinlock as seen on high-level
// benchmarks. Instead this implements a simple non-recursive mutex on top of:
// - Linux   : futex()
// - Windows : SRWLock
// - MacOS   : os_unfair_lock
// - POSIX   : pthread_mutex_trylock()
//
// The main difference between this and a libc implementation is that it only
// supports the simplest path: private (to a process), non-recursive mutexes
// with no priority inheritance, no timed waits.
//
// As an interesting side-effect to be used in the allocator, this code does not
// make any allocations, locks are small with a constexpr constructor and no
// destructor.
//
// 2. Otherwise: This is a simple SpinLock, in the sense that it does not have
//    any awareness of other threads' behavior.
class PA_LOCKABLE PA_COMPONENT_EXPORT(PARTITION_ALLOC) SpinningMutex {
 public:
  inline constexpr SpinningMutex();
  PA_ALWAYS_INLINE void Acquire() PA_EXCLUSIVE_LOCK_FUNCTION();
  PA_ALWAYS_INLINE void Release() PA_UNLOCK_FUNCTION();
  PA_ALWAYS_INLINE bool Try() PA_EXCLUSIVE_TRYLOCK_FUNCTION(true);
  void AssertAcquired() const {}  // Not supported.
  void Reinit() PA_UNLOCK_FUNCTION();
#if PA_BUILDFLAG(ENABLE_PARTITION_LOCK_PRIORITY_INHERITANCE)
  static void EnableUsePriorityInheritance();
  inline bool HasWaitersForTesting() const;
#endif  // PA_BUILDFLAG(ENABLE_PARTITION_LOCK_PRIORITY_INHERITANCE)

 private:
  PA_NOINLINE void AcquireSpinThenBlock() PA_EXCLUSIVE_LOCK_FUNCTION();
  void LockSlow() PA_EXCLUSIVE_LOCK_FUNCTION();

  // See below, the latency of PA_YIELD_PROCESSOR can be as high as ~150
  // cycles. Meanwhile, sleeping costs a few us. Spinning 64 times at 3GHz would
  // cost 150 * 64 / 3e9 ~= 3.2us.
  //
  // This applies to Linux kernels, on x86_64. On ARM we might want to spin
  // more.
  static constexpr int kSpinCount = 64;

#if PA_CONFIG(HAS_LINUX_KERNEL)
  void FutexWait();
  void FutexWake();

  static constexpr int kUnlocked = 0;
  static constexpr int kLockedUncontended = 1;
  static constexpr int kLockedContended = 2;

  std::atomic<int32_t> state_{kUnlocked};
#if PA_BUILDFLAG(ENABLE_PARTITION_LOCK_PRIORITY_INHERITANCE)
  static constexpr int kMigrated = 0xdead;
  static std::atomic<bool> s_use_pi_futex;

  std::atomic<bool> migrated_{false};
  std::atomic<int32_t> state_pi_{kUnlocked};

  void FutexLockPI() PA_EXCLUSIVE_LOCK_FUNCTION();
  void FutexUnlockPI() PA_UNLOCK_FUNCTION();
  void FutexMigrate() PA_UNLOCK_FUNCTION();

  PA_ALWAYS_INLINE static bool ShouldUsePriorityInheritance();
  PA_ALWAYS_INLINE bool IsLockMigrated() const;
#endif  // PA_BUILDFLAG(ENABLE_PARTITION_LOCK_PRIORITY_INHERITANCE)
#elif PA_BUILDFLAG(IS_WIN)
  PA_CHROME_SRWLOCK lock_ = SRWLOCK_INIT;
#elif PA_BUILDFLAG(IS_APPLE)
  os_unfair_lock unfair_lock_ = OS_UNFAIR_LOCK_INIT;
#elif PA_BUILDFLAG(IS_POSIX)
  pthread_mutex_t lock_ = PTHREAD_MUTEX_INITIALIZER;
#elif PA_BUILDFLAG(IS_FUCHSIA)
  sync_mutex lock_;
#else
  std::atomic<bool> lock_{false};
#endif
};

PA_ALWAYS_INLINE void SpinningMutex::Acquire() {
  // Not marked `[[likely]]`, as:
  // 1. We don't know how much contention the lock would experience
  // 2. This may lead to weird-looking code layout when inlined into a caller
  // with `[[(un)likely]]` attributes.
  if (Try()) {
    return;
  }

  return AcquireSpinThenBlock();
}

inline constexpr SpinningMutex::SpinningMutex() = default;

#if PA_CONFIG(HAS_LINUX_KERNEL)

#if PA_BUILDFLAG(ENABLE_PARTITION_LOCK_PRIORITY_INHERITANCE)
// static
PA_ALWAYS_INLINE bool SpinningMutex::ShouldUsePriorityInheritance() {
  return s_use_pi_futex.load(std::memory_order_relaxed);
}

PA_ALWAYS_INLINE bool SpinningMutex::IsLockMigrated() const {
  return migrated_.load(std::memory_order_acquire);
}

inline bool SpinningMutex::HasWaitersForTesting() const {
  if (IsLockMigrated()) {
    return (static_cast<uint32_t>(state_pi_.load(std::memory_order_relaxed)) &
            FUTEX_WAITERS) == FUTEX_WAITERS;
  } else {
    return state_.load(std::memory_order_relaxed) == kLockedContended;
  }
}

#endif  // PA_BUILDFLAG(ENABLE_PARTITION_LOCK_PRIORITY_INHERITANCE)

PA_ALWAYS_INLINE bool SpinningMutex::Try() {
  int expected = kUnlocked, desired = kLockedUncontended;
  std::atomic<int32_t>* state = &state_;
#if PA_BUILDFLAG(ENABLE_PARTITION_LOCK_PRIORITY_INHERITANCE)
  if (IsLockMigrated()) {
    desired = base::PlatformThread::CurrentId();
    state = &state_pi_;
  }
#endif  // PA_BUILDFLAG(ENABLE_PARTITION_LOCK_PRIORITY_INHERITANCE)
  // Using the weak variant of compare_exchange(), which may fail spuriously. On
  // some architectures such as ARM, CAS is typically performed as a LDREX/STREX
  // pair, where the store may fail. In the strong version, there is a loop
  // inserted by the compiler to retry in these cases.
  //
  // Since we are retrying in AcquireSpinThenBlock() anyway, there is no point
  // having two nested loops.
  return ((state->load(std::memory_order_relaxed) == expected) &&
          state->compare_exchange_weak(expected, desired,
                                       std::memory_order_acquire,
                                       std::memory_order_relaxed));
}

PA_ALWAYS_INLINE void SpinningMutex::Release() {
#if PA_BUILDFLAG(ENABLE_PARTITION_LOCK_PRIORITY_INHERITANCE)
  if (ShouldUsePriorityInheritance()) {
    // We check if the lock should be migrated while releasing the lock since
    // migrating the non-PI futex to the PI futex effectively unlocks the non-PI
    // futex and therefore the lock itself. The migration happens in the release
    // path only once, with one corner case handled in |LockSlow()|.
    if (!IsLockMigrated()) [[unlikely]] {
      FutexMigrate();
      return;
    }

    // In the fast path of the PI futex, the value of the futex is still set to
    // the thread ID of the current thread. If there are waiters, the kernel
    // will set the |FUTEX_WAITERS| bit which will cause the compare-exchange to
    // fail and force the current thread to call into the kernel and assign the
    // futex to one of the waiters.
    //
    // Note that we cannot pessimize in the PI futex case as we do in the non-PI
    // futex case by marking the futex as unlocked and then calling into the
    // kernel. The kernel expects that a PI-futex must have an owner if it has
    // waiters in order for the priority inheritance to work as expected.
    int expected = base::PlatformThread::CurrentId();
    if (!((state_pi_.load(std::memory_order_relaxed) == expected) &&
          state_pi_.compare_exchange_strong(
              expected, kUnlocked, std::memory_order_release,
              std::memory_order_relaxed))) [[unlikely]] {
      FutexUnlockPI();
    }

    return;
  }
#endif  // PA_BUILDFLAG(ENABLE_PARTITION_LOCK_PRIORITY_INHERITANCE)
  if (state_.exchange(kUnlocked, std::memory_order_release) == kLockedContended)
      [[unlikely]] {
    // |kLockedContended|: there is a waiter to wake up.
    //
    // Here there is a window where the lock is unlocked, since we just set it
    // to |kUnlocked| above. Meaning that another thread can grab the lock
    // in-between now and |FutexWake()| waking up a waiter. Aside from
    // potentially fairness, this is not an issue, as the newly-awaken thread
    // will check that the lock is still free.
    //
    // There is a small pessimization here though: if we have a single waiter,
    // then when it wakes up, the lock will be set to |kLockedContended|, so
    // when this waiter releases the lock, it will needlessly call
    // |FutexWake()|, even though there are no waiters. This is supported by the
    // kernel, and is what bionic (Android's libc) also does.
    FutexWake();
  }
}

#elif PA_BUILDFLAG(IS_WIN)

PA_ALWAYS_INLINE bool SpinningMutex::Try() {
  return !!::TryAcquireSRWLockExclusive(reinterpret_cast<PSRWLOCK>(&lock_));
}

PA_ALWAYS_INLINE void SpinningMutex::Release() {
  ::ReleaseSRWLockExclusive(reinterpret_cast<PSRWLOCK>(&lock_));
}

#elif PA_BUILDFLAG(IS_APPLE)

PA_ALWAYS_INLINE bool SpinningMutex::Try() {
  return os_unfair_lock_trylock(&unfair_lock_);
}

PA_ALWAYS_INLINE void SpinningMutex::Release() {
  return os_unfair_lock_unlock(&unfair_lock_);
}

#elif PA_BUILDFLAG(IS_POSIX)

PA_ALWAYS_INLINE bool SpinningMutex::Try() {
  int retval = pthread_mutex_trylock(&lock_);
  PA_DCHECK(retval == 0 || retval == EBUSY);
  return retval == 0;
}

PA_ALWAYS_INLINE void SpinningMutex::Release() {
  int retval = pthread_mutex_unlock(&lock_);
  PA_DCHECK(retval == 0);
}

#elif PA_BUILDFLAG(IS_FUCHSIA)

PA_ALWAYS_INLINE bool SpinningMutex::Try() {
  return sync_mutex_trylock(&lock_) == ZX_OK;
}

PA_ALWAYS_INLINE void SpinningMutex::Release() {
  sync_mutex_unlock(&lock_);
}

#else

PA_ALWAYS_INLINE bool SpinningMutex::Try() {
  // Possibly faster than CAS. The theory is that if the cacheline is shared,
  // then it can stay shared, for the contended case.
  return !lock_.load(std::memory_order_relaxed) &&
         !lock_.exchange(true, std::memory_order_acquire);
}

PA_ALWAYS_INLINE void SpinningMutex::Release() {
  lock_.store(false, std::memory_order_release);
}

#endif

}  // namespace partition_alloc::internal

#endif  // PARTITION_ALLOC_SPINNING_MUTEX_H_
