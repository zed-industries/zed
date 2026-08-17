// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_DEADLINE_TASK_RUNNER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_DEADLINE_TASK_RUNNER_H_

#include "base/functional/callback.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/common/cancelable_closure_holder.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {
namespace scheduler {

// Runs a posted task at latest by a given deadline, but possibly sooner.
class PLATFORM_EXPORT DeadlineTaskRunner {
  USING_FAST_MALLOC(DeadlineTaskRunner);

 public:
  DeadlineTaskRunner(const base::RepeatingClosure& callback,
                     scoped_refptr<base::SingleThreadTaskRunner> task_runner);
  DeadlineTaskRunner(const DeadlineTaskRunner&) = delete;
  DeadlineTaskRunner& operator=(const DeadlineTaskRunner&) = delete;

  ~DeadlineTaskRunner();

  // If there is no outstanding task then a task is posted to run after |delay|.
  // If there is an outstanding task which is scheduled to run:
  //   a) sooner - then this is a NOP.
  //   b) later - then the outstanding task is cancelled and a new task is
  //              posted to run after |delay|.
  //
  // Once the deadline task has run, we reset.
  void SetDeadline(const base::Location& from_here,
                   base::TimeDelta delay,
                   base::TimeTicks now);

 private:
  void RunInternal();

  CancelableClosureHolder cancelable_run_internal_;
  base::RepeatingClosure callback_;
  base::TimeTicks deadline_;
  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_DEADLINE_TASK_RUNNER_H_
