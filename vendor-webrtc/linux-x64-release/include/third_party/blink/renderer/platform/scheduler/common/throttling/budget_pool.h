// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_THROTTLING_BUDGET_POOL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_THROTTLING_BUDGET_POOL_H_

#include "base/functional/callback.h"
#include "base/task/common/lazy_now.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/common/throttling/task_queue_throttler.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/perfetto/include/perfetto/tracing/traced_value_forward.h"

namespace base {
namespace trace_event {
class TracedValue;
}
}  // namespace base

namespace blink {
namespace scheduler {

enum class QueueBlockType;

// BudgetPool represents a group of task queues which share a limit
// on a resource. This limit applies when task queues are already throttled
// by TaskQueueThrottler.
class PLATFORM_EXPORT BudgetPool {
  USING_FAST_MALLOC(BudgetPool);

 public:
  virtual ~BudgetPool();

  const char* Name() const;

  // Report task run time to the budget pool.
  virtual void RecordTaskRunTime(base::TimeTicks start_time,
                                 base::TimeTicks end_time) = 0;

  // Returns the earliest time when the next wake-up can be scheduled to run
  // new tasks.
  virtual base::TimeTicks GetNextAllowedRunTime(
      base::TimeTicks desired_run_time) const = 0;

  // Returns true if a task can run at the given time.
  virtual bool CanRunTasksAt(base::TimeTicks moment) const = 0;

  // Returns a point in time until which tasks are allowed to run.
  // base::TimeTicks::Max() means that there are no known limits.
  virtual base::TimeTicks GetTimeTasksCanRunUntil(
      base::TimeTicks now) const = 0;

  // Invoked as part of a global wake up if any of the task queues associated
  // with the budget pool has reached its next allowed run time. The next
  // allowed run time of a queue is the maximum value returned from
  // GetNextAllowedRunTime() among all the budget pools it is part of.
  virtual void OnWakeUp(base::TimeTicks now) = 0;

  // Specify how this budget pool should block affected queues.
  virtual QueueBlockType GetBlockType() const = 0;

  // Records state for tracing.
  virtual void WriteIntoTrace(perfetto::TracedValue context,
                              base::TimeTicks now) const = 0;

  // Adds |throttler| to given pool and invokes
  // TaskQueueThrottler::UpdateQueueState().
  void AddThrottler(base::TimeTicks now, TaskQueueThrottler* throttler);

  // Removes |throttler| from given pool and invokes
  // TaskQueueThrottler::UpdateQueueState().
  void RemoveThrottler(base::TimeTicks now, TaskQueueThrottler* throttler);

  // Unlike RemoveThrottler, does not update the queue's state.
  void UnregisterThrottler(TaskQueueThrottler* throttler);

  // Enables this budget pool, allowing it to enforce its policies on its
  // queues.
  void EnableThrottling(base::LazyNow* now);

  // Disables this budget pool, stopping it from enforcing its policies on its
  // queues. UpdateQueueState() is invoked on all queues to update their wake up
  // times and fences.
  void DisableThrottling(base::LazyNow* now);

  bool IsThrottlingEnabled() const;

  // All queues should be removed before calling Close().
  void Close();

  // Ensures that a wake-up is scheduled and that a fence is installed for all
  // queues in this pool, based on state of those queues and latest values from
  // CanRunTasksAt/GetTimeTasksCanRunUntil/GetNextAllowedRunTime.
  void UpdateStateForAllThrottlers(base::TimeTicks now);

 protected:
  explicit BudgetPool(const char* name);

  const char* name_;  // NOT OWNED

  HashSet<TaskQueueThrottler*> associated_throttlers_;
  bool is_enabled_;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_THROTTLING_BUDGET_POOL_H_
