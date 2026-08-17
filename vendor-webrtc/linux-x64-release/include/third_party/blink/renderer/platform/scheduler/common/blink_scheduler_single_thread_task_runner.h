// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_BLINK_SCHEDULER_SINGLE_THREAD_TASK_RUNNER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_BLINK_SCHEDULER_SINGLE_THREAD_TASK_RUNNER_H_

#include "base/functional/callback_forward.h"
#include "base/memory/scoped_refptr.h"
#include "base/task/delayed_task_handle.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace base {
class Location;
}  // namespace base

namespace blink::scheduler {

// TaskRunner used in the scheduler.
//
// This class specializes the DeleteSoon/ReleaseSoon implementation to prevent
// the object from leaking when possible (see DeleteOrReleaseSoonInternal). This
// is needed in Blink since frame and worker schedulers can get torn down long
// before the process shuts down.
//
// All other task-posting functionality is forwarded to to an underlying task
// runner.
class PLATFORM_EXPORT BlinkSchedulerSingleThreadTaskRunner
    : public base::SingleThreadTaskRunner {
 public:
  // `wrapped_task_runner` is the task runner used for scheduling tasks.
  // `thread_task_runner` is used to schedule deleter tasks if
  // `wrapped_task_runner`'s task queue is already shut down.
  BlinkSchedulerSingleThreadTaskRunner(
      scoped_refptr<base::SingleThreadTaskRunner> wrapped_task_runner,
      scoped_refptr<base::SingleThreadTaskRunner> thread_task_runner);
  ~BlinkSchedulerSingleThreadTaskRunner() override;

  BlinkSchedulerSingleThreadTaskRunner(BlinkSchedulerSingleThreadTaskRunner&&) =
      delete;
  BlinkSchedulerSingleThreadTaskRunner& operator=(
      BlinkSchedulerSingleThreadTaskRunner&&) = delete;

  // base::TaskRunner overrides:
  bool PostDelayedTask(const base::Location& from_here,
                       base::OnceClosure task,
                       base::TimeDelta delay) override {
    return outer_->PostDelayedTask(from_here, std::move(task), delay);
  }

  // base::SequencedTaskRunner overrides:
  bool PostNonNestableDelayedTask(const base::Location& from_here,
                                  base::OnceClosure task,
                                  base::TimeDelta delay) override {
    return outer_->PostNonNestableDelayedTask(from_here, std::move(task),
                                              delay);
  }

  base::DelayedTaskHandle PostCancelableDelayedTask(
      base::subtle::PostDelayedTaskPassKey pass_key,
      const base::Location& from_here,
      base::OnceClosure task,
      base::TimeDelta delay) override {
    return outer_->PostCancelableDelayedTask(pass_key, from_here,
                                             std::move(task), delay);
  }

  [[nodiscard]] base::DelayedTaskHandle PostCancelableDelayedTaskAt(
      base::subtle::PostDelayedTaskPassKey pass_key,
      const base::Location& from_here,
      base::OnceClosure task,
      base::TimeTicks delayed_run_time,
      base::subtle::DelayPolicy delay_policy) override {
    return outer_->PostCancelableDelayedTaskAt(
        pass_key, from_here, std::move(task), delayed_run_time, delay_policy);
  }

  bool PostDelayedTaskAt(base::subtle::PostDelayedTaskPassKey pass_key,
                         const base::Location& from_here,
                         base::OnceClosure task,
                         base::TimeTicks delayed_run_time,
                         base::subtle::DelayPolicy delay_policy) override {
    return outer_->PostDelayedTaskAt(pass_key, from_here, std::move(task),
                                     delayed_run_time, delay_policy);
  }

  bool RunsTasksInCurrentSequence() const override {
    return outer_->RunsTasksInCurrentSequence();
  }

 protected:
  // This always returns true, even if `object` gets leaked because the deleter
  // task was not posted.
  // TODO(crbug.com/1376851): Determine if leaking still occurs and whether to
  // CHECK or handle at callsites.
  bool DeleteOrReleaseSoonInternal(const base::Location& from_here,
                                   void (*deleter)(const void*),
                                   const void* object) override;

 private:
  // The task runner this object forwards all non-delete/release tasks to.
  scoped_refptr<base::SingleThreadTaskRunner> outer_;
  // Backup task runner used for delete/release tasks if `outer_`'s task queue
  // is already shut down when `DeleteOrReleaseSoonInternal()` is called.
  scoped_refptr<base::SingleThreadTaskRunner> thread_task_runner_;
};

}  // namespace blink::scheduler

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_BLINK_SCHEDULER_SINGLE_THREAD_TASK_RUNNER_H_
