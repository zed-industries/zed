// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_SYNCHRONIZATION_LOCK_H_
#define BASE_SYNCHRONIZATION_LOCK_H_

#include <type_traits>

#include "base/base_export.h"
#include "base/dcheck_is_on.h"
#include "base/synchronization/lock_impl.h"
#include "base/synchronization/lock_subtle.h"
#include "base/thread_annotations.h"
#include "build/build_config.h"

#if DCHECK_IS_ON()
#include <memory>

#include "base/compiler_specific.h"
#include "base/threading/platform_thread_ref.h"
#endif

namespace base {

// Foward-declare to avoid circular #includes.
template <typename T>
class FunctionRef;

// A convenient wrapper for an OS specific critical section.  The only real
// intelligence in this class is in debug mode for the support for the
// AssertAcquired() method and invariant debugging.
class LOCKABLE BASE_EXPORT Lock {
 public:
  Lock(const Lock&) = delete;
  Lock& operator=(const Lock&) = delete;

#if !DCHECK_IS_ON()
  // Optimized wrapper implementation

  Lock() = default;
  // The provided `check_invariants` will be ignored. Using a templated method
  // here instead of `explicit Lock(FunctionRef<void()>)` avoids a compile error
  // about instantiation of an undefined template when code that neither
  // #includes function_ref.h nor calls this constructor #includes this header.
  template <typename T>
    requires(std::is_convertible_v<T, FunctionRef<void()>>)
  explicit Lock(T check_invariants) : Lock() {}
  ~Lock() = default;

  void Acquire(subtle::LockTracking tracking = subtle::LockTracking::kDisabled)
      EXCLUSIVE_LOCK_FUNCTION() {
    lock_.Lock();
  }
  void Release() UNLOCK_FUNCTION() { lock_.Unlock(); }

  // If the lock is not held, take it and return true. If the lock is already
  // held by another thread, immediately return false. This must not be called
  // by a thread already holding the lock (what happens is undefined and an
  // assertion may fail).
  bool Try(subtle::LockTracking tracking = subtle::LockTracking::kDisabled)
      EXCLUSIVE_TRYLOCK_FUNCTION(true) {
    return lock_.Try();
  }

  // Null implementation if not debug.
  void AssertAcquired() const ASSERT_EXCLUSIVE_LOCK() {}
  void AssertNotHeld() const {}
#else
  Lock();
  // The provided `check_invariants` will be invoked just after the mutex is
  // acquired and just before the mutex is released. It should have no
  // side-effects and should DCHECK whatever holder-specific invariants
  // regarding this lock may exist.
  explicit Lock(FunctionRef<void()> check_invariants LIFETIME_BOUND);
  ~Lock();

  // Note: Acquiring a lock that is already held by the calling thread is not
  // supported and results in a CHECK() failure.
  void Acquire(subtle::LockTracking tracking = subtle::LockTracking::kDisabled)
      EXCLUSIVE_LOCK_FUNCTION();
  void Release() UNLOCK_FUNCTION();
  bool Try(subtle::LockTracking tracking = subtle::LockTracking::kDisabled)
      EXCLUSIVE_TRYLOCK_FUNCTION(true);

  void AssertAcquired() const ASSERT_EXCLUSIVE_LOCK();
  void AssertNotHeld() const;
#endif  // DCHECK_IS_ON()

  // Whether Lock mitigates priority inversion when used from different thread
  // priorities.
  static bool HandlesMultipleThreadPriorities() {
#if BUILDFLAG(IS_WIN)
    // Prior to Windows 11, Windows mitigated priority inversion by randomly
    // boosting the priority of ready threads. From Windows 11 onwards, priority
    // inversion mitigation works similar to POSIX through a facility called
    // AutoBoost which sets the priority floor of the thread holding the lock to
    // the maximum priority of its waiters.
    // https://github.com/MicrosoftDocs/win32/commit/a43cb3b5039c5cfc53642bfcea174003a2f1168f
    return true;
#elif BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)
    // POSIX mitigates priority inversion by setting the priority of a thread
    // holding a Lock to the maximum priority of any other thread waiting on it.
    return internal::LockImpl::PriorityInheritanceAvailable();
#else
#error Unsupported platform
#endif
  }

  // Both Windows and POSIX implementations of ConditionVariable need to be
  // able to see our lock and tweak our debugging counters, as they release and
  // acquire locks inside of their condition variable APIs.
  friend class ConditionVariable;

 private:
#if DCHECK_IS_ON()
  // Check that `owning_thread_ref_` refers to the current thread and unset it.
  void CheckHeldAndUnmark();
  // Check that `owning_thread_ref_` is null and set it to the current thread.
  void CheckUnheldAndMark();

  // Adds/removes this lock to/from the thread-local list returned by
  // `subtle::GetLocksHeldByCurrentThread()`, unless tracking is disabled.
  void AddToLocksHeldOnCurrentThread();
  void RemoveFromLocksHeldOnCurrentThread();

  // Reference to the thread holding the lock. Protected by `lock_`.
  base::PlatformThreadRef owning_thread_ref_;

  // Whether the lock is currently in the list of locks held by a thread. When
  // true, the lock is removed from the list upon `Release()`.
  bool in_tracked_locks_held_by_current_thread_ = false;

  std::unique_ptr<FunctionRef<void()>> check_invariants_;
#endif  // DCHECK_IS_ON()

  // Platform specific underlying lock implementation.
  internal::LockImpl lock_;
};

// A helper class that acquires the given Lock while the AutoLock is in scope.
using AutoLock = internal::BasicAutoLock<Lock>;

// A helper class that acquires the given Lock while the MovableAutoLock is in
// scope. Unlike AutoLock, the lock can be moved out of MovableAutoLock. Unlike
// AutoLockMaybe, the passed in lock is always valid, so need to check only on
// destruction.
using MovableAutoLock = internal::BasicMovableAutoLock<Lock>;

// A helper class that tries to acquire the given Lock while the AutoTryLock is
// in scope.
using AutoTryLock = internal::BasicAutoTryLock<Lock>;

// AutoUnlock is a helper that will Release() the |lock| argument in the
// constructor, and re-Acquire() it in the destructor.
using AutoUnlock = internal::BasicAutoUnlock<Lock>;

// Like AutoLock but is a no-op when the provided Lock* is null. Inspired from
// absl::MutexLockMaybe. Use this instead of std::optional<AutoLock> to get
// around -Wthread-safety-analysis warnings for conditional locking.
using AutoLockMaybe = internal::BasicAutoLockMaybe<Lock>;

// Like AutoLock but permits Release() of its mutex before destruction.
// Release() may be called at most once. Inspired from
// absl::ReleasableMutexLock. Use this instead of std::optional<AutoLock> to get
// around -Wthread-safety-analysis warnings for AutoLocks that are explicitly
// released early (prefer proper scoping to this).
using ReleasableAutoLock = internal::BasicReleasableAutoLock<Lock>;

}  // namespace base

#endif  // BASE_SYNCHRONIZATION_LOCK_H_
