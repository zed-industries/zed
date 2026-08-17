// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_SEQUENCE_CHECKER_H_
#define BASE_SEQUENCE_CHECKER_H_

#include "base/base_export.h"
#include "base/dcheck_is_on.h"
#include "base/macros/uniquify.h"
#include "base/sequence_checker_impl.h"

// SequenceChecker verifies mutual exclusion between calls to its
// `CalledOnValidSequence()` method. Mutual exclusion is guaranteed if all calls
// are made from the same thread, from the same sequence (see
// `SequencedTaskRunner`) or under the same lock acquired with
// `base::subtle::LockTracking::kEnabled`. SequenceChecker supports thread
// safety annotations (see base/thread_annotations.h).
//
// Use the macros below instead of the SequenceChecker directly so that the
// unused member doesn't result in an extra byte (four when padded) per instance
// in production.
//
// This class is much prefered to ThreadChecker for thread-safety checks.
// ThreadChecker should only be used for classes that are truly thread-affine
// (use thread-local-storage or a third-party API that does).
//
// Debugging:
//   If SequenceChecker::EnableStackLogging() is called beforehand, then when
//   SequenceChecker fails, in addition to crashing with a stack trace of where
//   the violation occurred, it will also dump a stack trace of where the
//   checker was bound to a sequence.
//
// Usage:
//   class MyClass {
//    public:
//     MyClass() {
//       // Detaching on construction is necessary for objects that are
//       // constructed on one sequence and forever after used from another
//       // sequence.
//       DETACH_FROM_SEQUENCE(sequence_checker_);
//     }
//
//     ~MyClass() {
//       // SequenceChecker doesn't automatically check it's destroyed on origin
//       // sequence for the same reason it's sometimes detached in the
//       // constructor. It's okay to destroy off sequence if the owner
//       // otherwise knows usage on the associated sequence is done. If you're
//       // not detaching in the constructor, you probably want to explicitly
//       // check in the destructor.
//       DCHECK_CALLED_ON_VALID_SEQUENCE(sequence_checker_);
//     }
//     void MyMethod() {
//       DCHECK_CALLED_ON_VALID_SEQUENCE(sequence_checker_);
//       ... (do stuff) ...
//       MyOtherMethod();
//     }
//
//     void MyOtherMethod() VALID_CONTEXT_REQUIRED(sequence_checker_) {
//       foo_ = 42;
//     }
//
//    private:
//      // GUARDED_BY_CONTEXT() enforces that this member is only
//      // accessed from a scope that invokes DCHECK_CALLED_ON_VALID_SEQUENCE()
//      // or from a function annotated with VALID_CONTEXT_REQUIRED(). A
//      // DCHECK build will not compile if the member is accessed and these
//      // conditions are not met.
//     int foo_ GUARDED_BY_CONTEXT(sequence_checker_);
//
//     SEQUENCE_CHECKER(sequence_checker_);
//   }

#if DCHECK_IS_ON()
#define SEQUENCE_CHECKER(name) base::SequenceChecker name
#define DCHECK_CALLED_ON_VALID_SEQUENCE(name, ...)   \
  base::ScopedValidateSequenceChecker BASE_UNIQUIFY( \
      scoped_validate_sequence_checker_)(name, ##__VA_ARGS__)
#define DETACH_FROM_SEQUENCE(name) (name).DetachFromSequence()
#else  // DCHECK_IS_ON()
// A no-op expansion that can be followed by a semicolon at class level.
#define SEQUENCE_CHECKER(name) static_assert(true, "")
#define DCHECK_CALLED_ON_VALID_SEQUENCE(name, ...) EAT_CHECK_STREAM_PARAMS()
#define DETACH_FROM_SEQUENCE(name)
#endif  // DCHECK_IS_ON()

namespace base {

// Do nothing implementation, for use in release mode.
//
// Note: You should almost always use the SequenceChecker class (through the
// above macros) to get the right version for your build configuration.
// Note: This is marked with "context" capability in order to support
// thread_annotations.h.
class THREAD_ANNOTATION_ATTRIBUTE__(capability("context"))
    SequenceCheckerDoNothing {
 public:
  static void EnableStackLogging() {}

  SequenceCheckerDoNothing() = default;

  // Moving between matching sequences is allowed to help classes with
  // SequenceCheckers that want a default move-construct/assign.
  SequenceCheckerDoNothing(SequenceCheckerDoNothing&& other) = default;
  SequenceCheckerDoNothing& operator=(SequenceCheckerDoNothing&& other) =
      default;
  SequenceCheckerDoNothing(const SequenceCheckerDoNothing&) = delete;
  SequenceCheckerDoNothing& operator=(const SequenceCheckerDoNothing&) = delete;

  [[nodiscard]] bool CalledOnValidSequence(void* = nullptr) const {
    return true;
  }
  void DetachFromSequence() {}
};

#if DCHECK_IS_ON()
using SequenceChecker = SequenceCheckerImpl;
#else
using SequenceChecker = SequenceCheckerDoNothing;
#endif  // DCHECK_IS_ON()

#if DCHECK_IS_ON()
class BASE_EXPORT SCOPED_LOCKABLE ScopedValidateSequenceChecker {
 public:
  explicit ScopedValidateSequenceChecker(const SequenceChecker& checker)
      EXCLUSIVE_LOCK_FUNCTION(checker);
  ~ScopedValidateSequenceChecker() UNLOCK_FUNCTION();
};
#endif

}  // namespace base

#endif  // BASE_SEQUENCE_CHECKER_H_
