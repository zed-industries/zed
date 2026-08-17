// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_COMMON_CHECKED_LOCK_H_
#define BASE_TASK_COMMON_CHECKED_LOCK_H_

#include <optional>

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "base/memory/stack_allocated.h"
#include "base/synchronization/condition_variable.h"
#include "base/synchronization/lock.h"
#include "base/task/common/checked_lock_impl.h"
#include "base/thread_annotations.h"

namespace base {
namespace internal {

// CheckedLock should be used anywhere a Lock would be used in the base/task
// impl. When DCHECK_IS_ON(), lock checking occurs. Otherwise, CheckedLock is
// equivalent to base::Lock.
//
// The shape of CheckedLock is as follows:
// CheckedLock()
//     Default constructor, no predecessor lock.
//     DCHECKs
//         On Acquisition if any CheckedLock is acquired on this thread.
//             Okay if a universal predecessor is acquired.
//
// CheckedLock(const CheckedLock* predecessor)
//     Constructor that specifies an allowed predecessor for that lock.
//     DCHECKs
//         On Construction if |predecessor| forms a predecessor lock cycle or
//             is a universal successor.
//         On Acquisition if the previous lock acquired on the thread is not
//             either |predecessor| or a universal predecessor. Okay if there
//             was no previous lock acquired.
//
// CheckedLock(UniversalPredecessor universal_predecessor)
//     Constructor for a lock that will allow the acquisition of any lock after
//     it, without needing to explicitly be named a predecessor (e.g. a root in
//     a lock chain). Can only be acquired if no locks are currently held by
//     this thread. DCHECKs
//         On Acquisition if any CheckedLock is acquired on this thread.
//
// CheckedLock(UniversalSuccessor universal_successor)
//     Constructor for a lock that will allow its acquisition after any other
//     lock, without needing to explicitly name its predecessor (e.g. a leaf in
//     a lock chain). Can not be acquired after another UniversalSuccessor lock.
//     DCHECKs
//         On Acquisition if there was a previously acquired lock on the thread
//             and it was also a universal successor.
//
// void Acquire()
//     Acquires the lock.
//
// void Release()
//     Releases the lock.
//
// void AssertAcquired().
//     DCHECKs if the lock is not acquired.
//
// ConditionVariable CreateConditionVariable()
//     Creates a condition variable using this as a lock.

#if DCHECK_IS_ON()
class LOCKABLE CheckedLock : public CheckedLockImpl {
 public:
  CheckedLock() = default;
  explicit CheckedLock(const CheckedLock* predecessor)
      : CheckedLockImpl(predecessor) {}
  explicit CheckedLock(UniversalPredecessor universal_predecessor)
      : CheckedLockImpl(universal_predecessor) {}
  explicit CheckedLock(UniversalSuccessor universal_successor)
      : CheckedLockImpl(universal_successor) {}
};
#else   // DCHECK_IS_ON()
class LOCKABLE CheckedLock : public Lock {
 public:
  CheckedLock() = default;
  explicit CheckedLock(const CheckedLock*) {}
  explicit CheckedLock(UniversalPredecessor) {}
  explicit CheckedLock(UniversalSuccessor) {}
  static void AssertNoLockHeldOnCurrentThread() {}

  ConditionVariable CreateConditionVariable() {
    return ConditionVariable(this);
  }
  void CreateConditionVariableAndEmplace(
      std::optional<ConditionVariable>& opt) {
    opt.emplace(this);
  }
};
#endif  // DCHECK_IS_ON()

// Provides the same functionality as base::AutoLock for CheckedLock.
using CheckedAutoLock = internal::BasicAutoLock<CheckedLock>;

// Provides the same functionality as base::AutoUnlock for CheckedLock.
using CheckedAutoUnlock = internal::BasicAutoUnlock<CheckedLock>;

// Provides the same functionality as base::AutoLockMaybe for CheckedLock.
using CheckedAutoLockMaybe = internal::BasicAutoLockMaybe<CheckedLock>;

// Informs the clang thread safety analysis that an aliased lock is acquired.
// Because the clang thread safety analysis doesn't understand aliased locks
// [1], this code wouldn't compile without AnnotateAcquiredLockAlias:
//
// class Example {
//  public:
//    CheckedLock lock_;
//    int value = 0 GUARDED_BY(lock_);
// };
//
// Example example;
// CheckedLock* acquired = &example.lock_;
// CheckedAutoLock auto_lock(*acquired);
// AnnotateAcquiredLockAlias annotate(*acquired, example.lock_);
// example.value = 42;  // Doesn't compile without |annotate|.
//
// [1] https://clang.llvm.org/docs/ThreadSafetyAnalysis.html#no-alias-analysis
class SCOPED_LOCKABLE AnnotateAcquiredLockAlias {
  STACK_ALLOCATED();

 public:
  // |acquired_lock| is an acquired lock. |lock_alias| is an alias of
  // |acquired_lock|.
  AnnotateAcquiredLockAlias(const CheckedLock& acquired_lock,
                            const CheckedLock& lock_alias)
      EXCLUSIVE_LOCK_FUNCTION(lock_alias)
      : acquired_lock_(acquired_lock) {
    DCHECK_EQ(&acquired_lock, &lock_alias);
    acquired_lock_.AssertAcquired();
  }

  AnnotateAcquiredLockAlias(const AnnotateAcquiredLockAlias&) = delete;
  AnnotateAcquiredLockAlias& operator=(const AnnotateAcquiredLockAlias&) =
      delete;

  ~AnnotateAcquiredLockAlias() UNLOCK_FUNCTION() {
    acquired_lock_.AssertAcquired();
  }

 private:
  const CheckedLock& acquired_lock_;
};

}  // namespace internal
}  // namespace base

#endif  // BASE_TASK_COMMON_CHECKED_LOCK_H_
