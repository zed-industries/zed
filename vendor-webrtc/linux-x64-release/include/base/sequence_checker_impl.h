// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_SEQUENCE_CHECKER_IMPL_H_
#define BASE_SEQUENCE_CHECKER_IMPL_H_

#include <cstdint>
#include <memory>
#include <vector>

#include "base/base_export.h"
#include "base/sequence_token.h"
#include "base/synchronization/lock.h"
#include "base/thread_annotations.h"
#include "base/threading/platform_thread_ref.h"

namespace base {
namespace debug {
class StackTrace;
}

// Real implementation of SequenceChecker.
//
// In most cases, SEQUENCE_CHECKER should be used instead of this, get the right
// implementation for the build configuration. It's possible to temporarily use
// this directly to get sequence checking in production builds, which can be
// handy to debug issues only seen in the field. However, when used in a
// non-DCHECK build, SequenceCheckerImpl::CalledOnValidSequence() will not
// consider locks as a valid way to guarantee mutual exclusion (returns false if
// not invoked from the bound sequence, even if all calls are made under the
// same lock).

// Marked with "context" capability to support thread_annotations.h.
class THREAD_ANNOTATION_ATTRIBUTE__(capability("context"))
    BASE_EXPORT SequenceCheckerImpl {
 public:
  static void EnableStackLogging();

  SequenceCheckerImpl();

  // Allow move construct/assign. This must be called on |other|'s associated
  // sequence and assignment can only be made into a SequenceCheckerImpl which
  // is detached or already associated with the current sequence. This isn't
  // thread-safe (|this| and |other| shouldn't be in use while this move is
  // performed). If the assignment was legal, the resulting SequenceCheckerImpl
  // will be bound to the current sequence and |other| will be detached.
  SequenceCheckerImpl(SequenceCheckerImpl&& other);
  SequenceCheckerImpl& operator=(SequenceCheckerImpl&& other);
  SequenceCheckerImpl(const SequenceCheckerImpl&) = delete;
  SequenceCheckerImpl& operator=(const SequenceCheckerImpl&) = delete;
  ~SequenceCheckerImpl();

  // Returns true if called in sequence with previous calls to this method and
  // the constructor.
  // On returning false, if logging is enabled with EnableStackLogging() and
  // `out_bound_at` is not null, this method allocates a StackTrace and returns
  // it in the out-parameter, storing inside it the stack from where the failing
  // SequenceChecker was bound to its sequence. Otherwise, out_bound_at is left
  // untouched.
  [[nodiscard]] bool CalledOnValidSequence(
      std::unique_ptr<debug::StackTrace>* out_bound_at = nullptr) const;

  // Unbinds the checker from the currently associated sequence. The checker
  // will be re-bound on the next call to CalledOnValidSequence().
  void DetachFromSequence();

 private:
  void EnsureAssigned() const EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Members are mutable so that `CalledOnValidSequence()` can set them.

  mutable Lock lock_;

  // Stack from which this was bound (set if `EnableStackLogging()` was called).
  mutable std::unique_ptr<debug::StackTrace> bound_at_ GUARDED_BY(lock_);

  // Sequence to which this is bound.
  mutable internal::SequenceToken sequence_token_ GUARDED_BY(lock_);

#if DCHECK_IS_ON()
  // Locks to which this is bound.
  mutable std::vector<uintptr_t> locks_ GUARDED_BY(lock_);
#endif  // DCHECK_IS_ON()

  // Thread to which this is bound. Only used to evaluate
  // `CalledOnValidSequence()` after TLS destruction.
  mutable PlatformThreadRef thread_ref_ GUARDED_BY(lock_);
};

}  // namespace base

#endif  // BASE_SEQUENCE_CHECKER_IMPL_H_
