// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_THREAD_POOL_TASK_H_
#define BASE_TASK_THREAD_POOL_TASK_H_

#include "base/base_export.h"
#include "base/containers/intrusive_heap.h"
#include "base/functional/callback.h"
#include "base/location.h"
#include "base/pending_task.h"
#include "base/task/sequenced_task_runner.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"

namespace base {
namespace internal {

// A task is a unit of work inside the thread pool. Support for tracing and
// profiling inherited from PendingTask.
// TODO(etiennep): This class is now equivalent to PendingTask, remove it.
struct BASE_EXPORT Task : public PendingTask {
  Task() = default;

  // |posted_from| is the site the task was posted from. |task| is the closure
  // to run. |delay| is a delay that must expire before the Task runs.
  Task(const Location& posted_from,
       OnceClosure task,
       TimeTicks queue_time,
       TimeDelta delay,
       TimeDelta leeway = TimeDelta(),
       int sequence_num = 0);
  // |delayed_run_time| is the time when the task should be run.
  Task(
      const Location& posted_from,
      OnceClosure task,
      TimeTicks queue_time,
      TimeTicks delayed_run_time,
      TimeDelta leeway = TimeDelta(),
      subtle::DelayPolicy delay_policy = subtle::DelayPolicy::kFlexibleNoSooner,
      int sequence_num = 0);
  Task(const TaskMetadata& metadata, OnceClosure task);

  // Task is move-only to avoid mistakes that cause reference counts to be
  // accidentally bumped.
  Task(Task&& other) noexcept;

  Task(const Task&) = delete;
  Task& operator=(const Task&) = delete;

  ~Task() = default;

  Task& operator=(Task&& other);

  // Required by IntrusiveHeap.
  void SetHeapHandle(const HeapHandle& handle) {}

  // Required by IntrusiveHeap.
  void ClearHeapHandle() {}

  // Required by IntrusiveHeap.
  HeapHandle GetHeapHandle() const { return HeapHandle::Invalid(); }
};

}  // namespace internal
}  // namespace base

#endif  // BASE_TASK_THREAD_POOL_TASK_H_
