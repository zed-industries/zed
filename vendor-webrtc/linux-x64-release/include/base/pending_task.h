// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PENDING_TASK_H_
#define BASE_PENDING_TASK_H_

#include <array>

#include "base/base_export.h"
#include "base/functional/callback.h"
#include "base/location.h"
#include "base/task/delay_policy.h"
#include "base/time/time.h"

namespace base {

enum class Nestable : uint8_t {
  kNonNestable,
  kNestable,
};

// Copyable data part of PendingTask.
struct BASE_EXPORT TaskMetadata {
  TaskMetadata();
  explicit TaskMetadata(const Location& posted_from,
                        TimeTicks queue_time = TimeTicks(),
                        TimeTicks delayed_run_time = TimeTicks(),
                        TimeDelta leeway = TimeDelta(),
                        subtle::DelayPolicy delay_policy =
                            subtle::DelayPolicy::kFlexibleNoSooner);
  TaskMetadata(TaskMetadata&& other);
  TaskMetadata(const TaskMetadata& other);
  ~TaskMetadata();

  TaskMetadata& operator=(TaskMetadata&& other);
  TaskMetadata& operator=(const TaskMetadata& other);

  // Returns the time at which this task should run. This is |delayed_run_time|
  // for a delayed task, |queue_time| otherwise.
  base::TimeTicks GetDesiredExecutionTime() const;

  TimeTicks earliest_delayed_run_time() const;
  TimeTicks latest_delayed_run_time() const;

  // The site this PendingTask was posted from.
  Location posted_from;

  // The time at which the task was queued, which happens at post time. For
  // deferred non-nestable tasks, this is reset when the nested loop exits and
  // the deferred tasks are pushed back at the front of the queue. This is not
  // set for immediate SequenceManager tasks unless SetAddQueueTimeToTasks(true)
  // was called. This defaults to a null TimeTicks if the task hasn't been
  // inserted in a sequence yet.
  TimeTicks queue_time;

  // The time when the task should be run. This is null for an immediate task.
  base::TimeTicks delayed_run_time;

  // |leeway| and |delay_policy| determine the preferred time range for running
  // the delayed task. A larger leeway provides more freedom to run the task at
  // an optimal time for power consumption. These fields are ignored for an
  // immediate (non-delayed) task.
  TimeDelta leeway;
  subtle::DelayPolicy delay_policy = subtle::DelayPolicy::kFlexibleNoSooner;

  // Chain of symbols of the parent tasks which led to this one being posted.
  static constexpr size_t kTaskBacktraceLength = 4;
  std::array<const void*, kTaskBacktraceLength> task_backtrace = {};

  // The context of the IPC message that was being handled when this task was
  // posted. This is a hash of the IPC message name that is set within the scope
  // of an IPC handler and when symbolized uniquely identifies the message being
  // processed. This property is not propagated from one PendingTask to the
  // next. For example, if pending task A was posted while handling an IPC,
  // and pending task B was posted from within pending task A, then pending task
  // B will not inherit the |ipc_hash| of pending task A.
  uint32_t ipc_hash = 0;
  const char* ipc_interface_name = nullptr;

  // Secondary sort key for run time.
  int sequence_num = 0;

  bool task_backtrace_overflow = false;
};

// Contains data about a pending task. Stored in TaskQueue and DelayedTaskQueue
// for use by classes that queue and execute tasks.
struct BASE_EXPORT PendingTask : public TaskMetadata {
  PendingTask();
  PendingTask(const Location& posted_from,
              OnceClosure task,
              TimeTicks queue_time = TimeTicks(),
              TimeTicks delayed_run_time = TimeTicks(),
              TimeDelta leeway = TimeDelta(),
              subtle::DelayPolicy delay_policy =
                  subtle::DelayPolicy::kFlexibleNoSooner);
  PendingTask(const TaskMetadata& metadata, OnceClosure task);
  PendingTask(PendingTask&& other);
  ~PendingTask();

  PendingTask& operator=(PendingTask&& other);

  // The task to run.
  OnceClosure task;
};

}  // namespace base

#endif  // BASE_PENDING_TASK_H_
