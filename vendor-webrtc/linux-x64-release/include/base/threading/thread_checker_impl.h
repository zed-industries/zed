// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_THREADING_THREAD_CHECKER_IMPL_H_
#define BASE_THREADING_THREAD_CHECKER_IMPL_H_

#include <memory>

#include "base/base_export.h"
#include "base/sequence_token.h"
#include "base/synchronization/lock.h"
#include "base/thread_annotations.h"
#include "base/threading/platform_thread_ref.h"

namespace base {
namespace debug {
class StackTrace;
}

class SequenceCheckerImpl;

// Real implementation of ThreadChecker, for use in debug mode, or for temporary
// use in release mode (e.g. to CHECK on a threading issue seen only in the
// wild).
//
// Note: You should almost always use the ThreadChecker class to get the right
// version for your build configuration.
// Note: This is only a check, not a "lock". It is marked "LOCKABLE" only in
// order to support thread_annotations.h.
class LOCKABLE BASE_EXPORT ThreadCheckerImpl {
 public:
  ThreadCheckerImpl();
  ~ThreadCheckerImpl();

  // Allow move construct/assign. This must be called on |other|'s associated
  // thread and assignment can only be made into a ThreadCheckerImpl which is
  // detached or already associated with the current thread. This isn't
  // thread-safe (|this| and |other| shouldn't be in use while this move is
  // performed). If the assignment was legal, the resulting ThreadCheckerImpl
  // will be bound to the current thread and |other| will be detached.
  ThreadCheckerImpl(ThreadCheckerImpl&& other);
  ThreadCheckerImpl& operator=(ThreadCheckerImpl&& other);

  // On returning false, if logging is enabled with EnableStackLogging() and
  // `out_bound_at` is not null, this method allocates a StackTrace and returns
  // it in the out-parameter, storing inside it the stack from where the failing
  // ThreadChecker was bound to its thread.
  [[nodiscard]] bool CalledOnValidThread(
      std::unique_ptr<debug::StackTrace>* out_bound_at = nullptr) const
      LOCKS_EXCLUDED(lock_);

  // Changes the thread that is checked for in CalledOnValidThread.  This may
  // be useful when an object may be created on one thread and then used
  // exclusively on another thread.
  void DetachFromThread() LOCKS_EXCLUDED(lock_);

 private:
  friend class SequenceCheckerImpl;

  // Private because it's only called by
  // `SequenceCheckerImpl::EnableStackLogging()`.
  static void EnableStackLogging();

  // Returns ownership of a pointer to StackTrace where the ThreadCheckerImpl
  // was bound for debug logs, or nullptr if such logging was not enabled at
  // the time.
  std::unique_ptr<debug::StackTrace> GetBoundAt() const
      EXCLUSIVE_LOCKS_REQUIRED(lock_);

  void EnsureAssigned() const EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Members are mutable so that CalledOnValidThread() can set them.

  // Synchronizes access to all members.
  mutable base::Lock lock_;

  // Stack from which this was bound (set if `EnableStackLogging()` was called).
  mutable std::unique_ptr<debug::StackTrace> bound_at_ GUARDED_BY(lock_);

  // Thread on which CalledOnValidThread() may return true.
  mutable PlatformThreadRef thread_ref_ GUARDED_BY(lock_);

  // TaskToken for which CalledOnValidThread() always returns true. This allows
  // CalledOnValidThread() to return true when called multiple times from the
  // same task, even if it's not running in a single-threaded context itself
  // (allowing usage of ThreadChecker objects on the stack in the scope of one-
  // off tasks). Note: CalledOnValidThread() may return true even if the current
  // TaskToken is not equal to this.
  mutable internal::TaskToken task_token_ GUARDED_BY(lock_);

  // SequenceToken for which CalledOnValidThread() may return true. Used to
  // ensure that CalledOnValidThread() doesn't return true for ThreadPool
  // tasks that happen to run on the same thread but weren't posted to the same
  // SingleThreadTaskRunner.
  mutable internal::SequenceToken sequence_token_ GUARDED_BY(lock_);
};

}  // namespace base

#endif  // BASE_THREADING_THREAD_CHECKER_IMPL_H_
