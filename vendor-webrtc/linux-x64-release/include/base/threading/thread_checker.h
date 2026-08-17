// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_THREADING_THREAD_CHECKER_H_
#define BASE_THREADING_THREAD_CHECKER_H_

#include <string_view>

#include "base/base_export.h"
#include "base/dcheck_is_on.h"
#include "base/macros/uniquify.h"
#include "base/thread_annotations.h"
#include "base/threading/thread_checker_impl.h"

// ThreadChecker is a helper class used to help verify that some methods of a
// class are called from the same thread (for thread-affinity).  It supports
// thread safety annotations (see base/thread_annotations.h).
//
// Use the macros below instead of the ThreadChecker directly so that the unused
// member doesn't result in an extra byte (four when padded) per instance in
// production.
//
// Usage of this class should be *rare* as most classes require thread-safety
// but not thread-affinity. Prefer base::SequenceChecker to verify thread-safe
// access.
//
// Thread-affinity checks should only be required in classes that use thread-
// local-storage or a third-party API that does.
//
// Prefer to encode the minimum requirements of each class instead of the
// environment it happens to run in today. e.g. if a class requires thread-
// safety but not thread-affinity, use a SequenceChecker even if it happens to
// run on a SingleThreadTaskRunner today. That makes it easier to understand
// what would need to change to turn that SingleThreadTaskRunner into a
// SequencedTaskRunner for ease of scheduling as well as minimizes side-effects
// if that change is made.
//
// Debugging:
//   If ThreadChecker::EnableStackLogging() is called beforehand, then when
//   ThreadChecker fails, in addition to crashing with a stack trace of where
//   the violation occurred, it will also dump a stack trace of where the
//   checker was bound to a thread.
//
// Usage:
//   class MyClass {
//    public:
//     MyClass() {
//       // It's sometimes useful to detach on construction for objects that are
//       // constructed in one place and forever after used from another
//       // thread.
//       DETACH_FROM_THREAD(thread_checker_);
//     }
//
//     ~MyClass() {
//       // ThreadChecker doesn't automatically check it's destroyed on origin
//       // thread for the same reason it's sometimes detached in the
//       // constructor. It's okay to destroy off thread if the owner otherwise
//       // knows usage on the associated thread is done. If you're not
//       // detaching in the constructor, you probably want to explicitly check
//       // in the destructor.
//       DCHECK_CALLED_ON_VALID_THREAD(thread_checker_);
//     }
//
//     void MyMethod() {
//       DCHECK_CALLED_ON_VALID_THREAD(thread_checker_);
//       ... (do stuff) ...
//     }
//
//     void MyOtherMethod()
//         VALID_CONTEXT_REQUIRED(thread_checker_) {
//       foo_ = 42;
//     }
//
//    private:
//     int foo_ GUARDED_BY_CONTEXT(thread_checker_);
//
//     THREAD_CHECKER(thread_checker_);
//   }

#if DCHECK_IS_ON()
#define THREAD_CHECKER(name) base::ThreadChecker name
#define DCHECK_CALLED_ON_VALID_THREAD(name, ...)   \
  base::ScopedValidateThreadChecker BASE_UNIQUIFY( \
      scoped_validate_thread_checker_)(name, ##__VA_ARGS__);
#define DETACH_FROM_THREAD(name) (name).DetachFromThread()
#else  // DCHECK_IS_ON()
#define THREAD_CHECKER(name) static_assert(true, "")
#define DCHECK_CALLED_ON_VALID_THREAD(name, ...) EAT_CHECK_STREAM_PARAMS()
#define DETACH_FROM_THREAD(name)
#endif  // DCHECK_IS_ON()

namespace base {

// Do nothing implementation, for use in release mode.
//
// Note: You should almost always use the ThreadChecker class (through the above
// macros) to get the right version for your build configuration.
// Note: This is only a check, not a "lock". It is marked "LOCKABLE" only in
// order to support thread_annotations.h.
class LOCKABLE ThreadCheckerDoNothing {
 public:
  static void EnableStackLogging() {}

  ThreadCheckerDoNothing() = default;

  ThreadCheckerDoNothing(const ThreadCheckerDoNothing&) = delete;
  ThreadCheckerDoNothing& operator=(const ThreadCheckerDoNothing&) = delete;

  // Moving between matching threads is allowed to help classes with
  // ThreadCheckers that want a default move-construct/assign.
  ThreadCheckerDoNothing(ThreadCheckerDoNothing&& other) = default;
  ThreadCheckerDoNothing& operator=(ThreadCheckerDoNothing&& other) = default;

  [[nodiscard]] bool CalledOnValidThread(
      std::unique_ptr<void*> = nullptr) const {
    return true;
  }
  void DetachFromThread() {}
};

// Note that ThreadCheckerImpl::CalledOnValidThread() returns false when called
// from tasks posted to SingleThreadTaskRunners bound to different sequences,
// even if the tasks happen to run on the same thread (e.g. two independent
// SingleThreadTaskRunners on the ThreadPool that happen to share a thread).
#if DCHECK_IS_ON()
class ThreadChecker : public ThreadCheckerImpl {};
#else
class ThreadChecker : public ThreadCheckerDoNothing {};
#endif  // DCHECK_IS_ON()

#if DCHECK_IS_ON()
class BASE_EXPORT SCOPED_LOCKABLE ScopedValidateThreadChecker {
 public:
  explicit ScopedValidateThreadChecker(const ThreadChecker& checker)
      EXCLUSIVE_LOCK_FUNCTION(checker);
  ScopedValidateThreadChecker(const ThreadChecker& checker,
                              std::string_view msg)
      EXCLUSIVE_LOCK_FUNCTION(checker);

  ScopedValidateThreadChecker(const ScopedValidateThreadChecker&) = delete;
  ScopedValidateThreadChecker& operator=(const ScopedValidateThreadChecker&) =
      delete;

  ~ScopedValidateThreadChecker() UNLOCK_FUNCTION();
};
#endif

}  // namespace base

#endif  // BASE_THREADING_THREAD_CHECKER_H_
