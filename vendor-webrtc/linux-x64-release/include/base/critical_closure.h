// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_CRITICAL_CLOSURE_H_
#define BASE_CRITICAL_CLOSURE_H_

#include <string_view>
#include <utility>

#include "base/functional/callback.h"
#include "base/location.h"
#include "build/build_config.h"
#include "build/ios_buildflags.h"

#if BUILDFLAG(IS_IOS) && !BUILDFLAG(IS_IOS_APP_EXTENSION)
#include <optional>

#include "base/functional/bind.h"
#include "base/ios/scoped_critical_action.h"
#endif

namespace base {

namespace internal {

#if BUILDFLAG(IS_IOS) && !BUILDFLAG(IS_IOS_APP_EXTENSION)
// This class wraps a closure so it can continue to run for a period of time
// when the application goes to the background by using
// |ios::ScopedCriticalAction|.
class ImmediateCriticalClosure {
 public:
  explicit ImmediateCriticalClosure(std::string_view task_name,
                                    OnceClosure closure);
  ImmediateCriticalClosure(const ImmediateCriticalClosure&) = delete;
  ImmediateCriticalClosure& operator=(const ImmediateCriticalClosure&) = delete;
  ~ImmediateCriticalClosure();
  void Run();

 private:
  ios::ScopedCriticalAction critical_action_;
  OnceClosure closure_;
};

// This class is identical to ImmediateCriticalClosure, but the critical action
// is started when the action runs, not when the CriticalAction is created.
class PendingCriticalClosure {
 public:
  explicit PendingCriticalClosure(std::string_view task_name,
                                  OnceClosure closure);
  PendingCriticalClosure(const PendingCriticalClosure&) = delete;
  PendingCriticalClosure& operator=(const PendingCriticalClosure&) = delete;
  ~PendingCriticalClosure();
  void Run();

 private:
  std::optional<ios::ScopedCriticalAction> critical_action_;
  std::string task_name_;
  OnceClosure closure_;
};
#endif  // BUILDFLAG(IS_IOS)

}  // namespace internal

// Returns a closure that will continue to run for a period of time when the
// application goes to the background if possible on platforms where
// applications don't execute while backgrounded, otherwise the original task is
// returned. If |is_immediate| is true, the closure will immediately prevent
// background suspension. Otherwise, the closure will wait to request background
// permission until it is run.
//
// Example:
//   file_task_runner_->PostTask(
//       FROM_HERE,
//       MakeCriticalClosure(task_name,
//                           base::BindOnce(&WriteToDiskTask, path_, data)));
//
// Note new closures might be posted in this closure. If the new closures need
// background running time, |MakeCriticalClosure| should be applied on them
// before posting. |task_name| is used by the platform to identify any tasks
// that do not complete in time for suspension.
//
// This function is used automatically for tasks posted to a sequence runner
// using TaskShutdownBehavior::BLOCK_SHUTDOWN.
#if BUILDFLAG(IS_IOS) && !BUILDFLAG(IS_IOS_APP_EXTENSION)
inline OnceClosure MakeCriticalClosure(std::string_view task_name,
                                       OnceClosure closure,
                                       bool is_immediate) {
  // Wrapping a null closure in a critical closure has unclear semantics and
  // most likely indicates a bug. CHECK-ing early allows detecting and
  // investigating these cases more easily.
  CHECK(!closure.is_null());
  if (is_immediate) {
    return base::BindOnce(&internal::ImmediateCriticalClosure::Run,
                          Owned(new internal::ImmediateCriticalClosure(
                              task_name, std::move(closure))));
  } else {
    return base::BindOnce(&internal::PendingCriticalClosure::Run,
                          Owned(new internal::PendingCriticalClosure(
                              task_name, std::move(closure))));
  }
}

inline OnceClosure MakeCriticalClosure(const Location& posted_from,
                                       OnceClosure closure,
                                       bool is_immediate) {
  return MakeCriticalClosure(posted_from.ToString(), std::move(closure),
                             is_immediate);
}

#else  // BUILDFLAG(IS_IOS) && !BUILDFLAG(IS_IOS_APP_EXTENSION)

inline OnceClosure MakeCriticalClosure(std::string_view task_name,
                                       OnceClosure closure,
                                       bool is_immediate) {
  // No-op for platforms where the application does not need to acquire
  // background time for closures to finish when it goes into the background.
  return closure;
}

inline OnceClosure MakeCriticalClosure(const Location& posted_from,
                                       OnceClosure closure,
                                       bool is_immediate) {
  return closure;
}

#endif  // BUILDFLAG(IS_IOS) && !BUILDFLAG(IS_IOS_APP_EXTENSION)

}  // namespace base

#endif  // BASE_CRITICAL_CLOSURE_H_
