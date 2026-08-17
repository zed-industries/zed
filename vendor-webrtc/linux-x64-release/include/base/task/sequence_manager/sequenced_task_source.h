// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_SEQUENCE_MANAGER_SEQUENCED_TASK_SOURCE_H_
#define BASE_TASK_SEQUENCE_MANAGER_SEQUENCED_TASK_SOURCE_H_

#include <optional>

#include "base/base_export.h"
#include "base/functional/callback_helpers.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/pending_task.h"
#include "base/task/common/lazy_now.h"
#include "base/task/sequence_manager/task_queue.h"
#include "base/task/sequence_manager/tasks.h"
#include "build/build_config.h"

namespace perfetto {
class EventContext;
}

namespace base {
namespace sequence_manager {
namespace internal {

// Interface to pass tasks to ThreadController.
class SequencedTaskSource {
 public:
  enum class SelectTaskOption { kDefault, kSkipDelayedTask };

  using TaskExecutionTraceLogger =
      RepeatingCallback<void(perfetto::EventContext&, const Task&)>;

  struct BASE_EXPORT SelectedTask {
    SelectedTask(const SelectedTask&);
    SelectedTask(Task& task,
                 TaskExecutionTraceLogger task_execution_trace_logger,
                 TaskQueue::QueuePriority priority,
                 QueueName task_queue_name);
    ~SelectedTask();

    // RAW_PTR_EXCLUSION: Performance reasons: based on this sampling profiler
    // result on Mac. go/brp-mac-prof-diff-20230403
    RAW_PTR_EXCLUSION Task& task;
    // Callback to fill trace event arguments associated with the task
    // execution. Can be null
    TaskExecutionTraceLogger task_execution_trace_logger =
        TaskExecutionTraceLogger();
    TaskQueue::QueuePriority priority;
    QueueName task_queue_name;
  };

  virtual ~SequencedTaskSource() = default;

  // Controls whether a `SequencedTaskRunner` associated with this source can
  // run a task synchronously in `RunOrPostTask`. Enable this to indicate that
  // there isn't any pending or running work that has mutual exclusion or
  // ordering expectations with tasks from this source, outside of
  // `SelectNextTask()` or `OnBeginWork()` -> `OnIdle()` (those prevent tasks
  // from running synchronously, irrespective of the state set here).
  virtual void SetRunTaskSynchronouslyAllowed(
      bool can_run_tasks_synchronously) = 0;

  // Returns the next task to run from this source or nullopt if
  // there're no more tasks ready to run. If a task is returned,
  // DidRunTask() must be invoked before the next call to SelectNextTask().
  // |option| allows control on which kind of tasks can be selected.
  virtual std::optional<SelectedTask> SelectNextTask(
      LazyNow& lazy_now,
      SelectTaskOption option) = 0;
  std::optional<SelectedTask> SelectNextTask(LazyNow& lazy_now) {
    return SelectNextTask(lazy_now, SelectTaskOption::kDefault);
  }

  // Notifies this source that the task previously obtained
  // from SelectNextTask() has been completed.
  virtual void DidRunTask(LazyNow& lazy_now) = 0;

  // Returns a WakeUp for the next pending task, is_immediate() if the
  // next task can run immediately, or nullopt if there are no more immediate or
  // delayed tasks. |option| allows control on which kind of tasks can be
  // selected. May delete canceled tasks.
  virtual std::optional<WakeUp> GetPendingWakeUp(LazyNow* lazy_now,
                                                 SelectTaskOption option) = 0;
  std::optional<WakeUp> GetPendingWakeUp(LazyNow* lazy_now) {
    return GetPendingWakeUp(lazy_now, SelectTaskOption::kDefault);
  }

#if BUILDFLAG(IS_WIN)
  // Return true if the next wakeup requires Windows' high-resolution timer to
  // be enabled.
  virtual bool NextWakeUpNeedsHighRes() = 0;
#endif

  // Indicates that work that has mutual exclusion expectations with tasks from
  // this `SequencedTaskSource` will start running.
  virtual void OnBeginWork() = 0;

  // Called when we have run out of immediate work.  If more immediate work
  // becomes available as a result of any processing done by this callback,
  // return true to schedule a future DoWork.
  virtual bool OnIdle() = 0;

  // Called prior to running `selected_task` to emit trace event data for it.
  virtual void MaybeEmitTaskDetails(
      perfetto::EventContext& ctx,
      const SelectedTask& selected_task) const = 0;
};

}  // namespace internal
}  // namespace sequence_manager
}  // namespace base

#endif  // BASE_TASK_SEQUENCE_MANAGER_SEQUENCED_TASK_SOURCE_H_
