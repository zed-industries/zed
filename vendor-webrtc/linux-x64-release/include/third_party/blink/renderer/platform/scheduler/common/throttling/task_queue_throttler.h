// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_THROTTLING_TASK_QUEUE_THROTTLER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_THROTTLING_TASK_QUEUE_THROTTLER_H_

#include <optional>

#include "base/memory/raw_ptr.h"
#include "base/task/sequence_manager/task_queue.h"
#include "base/threading/thread_checker.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/common/tracing_helper.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/perfetto/include/perfetto/tracing/traced_value_forward.h"

namespace base {
class LazyNow;
namespace trace_event {
class TracedValue;
}
}  // namespace base

namespace blink {
namespace scheduler {

class BudgetPool;

// kNewTasksOnly prevents new tasks from running (old tasks can run normally),
// kAllTasks block queue completely.
// kAllTasks-type block always blocks the queue completely.
// kNewTasksOnly-type block does nothing when queue is already blocked by
// kAllTasks, and overrides previous kNewTasksOnly block if any, which may
// unblock some tasks.
enum class QueueBlockType { kAllTasks, kNewTasksOnly };

// The job of the TaskQueueThrottler is to control when tasks posted on a
// throttled queue get to run.
//
// This is done by inserting a fence in the queue when it cannot run and
// removing or updating the fence when the next allowed wake up time is reached.
//
// TaskQueueThrottler assumes that it's the only system inserting fences in
// its associated TaskQueue.
//
// There may be more than one system that wishes to throttle a queue (e.g.
// renderer suspension vs tab level suspension) so the TaskQueueThrottler keeps
// a count of the number of systems that wish the queue to be throttled. See
// IncreaseThrottleRefCount & DecreaseThrottleRefCount.
//
// This class is main-thread only.
class PLATFORM_EXPORT TaskQueueThrottler final
    : public base::sequence_manager::TaskQueue::Throttler {
 public:
  TaskQueueThrottler(base::sequence_manager::TaskQueue* task_queue,
                     const base::TickClock* tick_clock);
  TaskQueueThrottler(const TaskQueueThrottler&) = delete;
  TaskQueueThrottler& operator=(const TaskQueueThrottler&) = delete;
  ~TaskQueueThrottler();

  // Updates the queue's next wake up and fence based on policies enforced
  // by this TaskQueueThrottler's budget pools.
  void UpdateQueueState(base::TimeTicks now);

  // Returns true if the queue is throttled.
  bool IsThrottled() const;

  // Increments the throttled refcount and causes |task_queue| to be throttled
  // if its not already throttled.
  void IncreaseThrottleRefCount();
  // If the refcouint is non-zero it's decremented.  If the throttled refcount
  // becomes zero then |task_queue| is unthrottled.  If the refcount was already
  // zero this function does nothing.
  void DecreaseThrottleRefCount();

  // Accounts for given task for cpu-based throttling needs.
  void OnTaskRunTimeReported(base::TimeTicks start_time,
                             base::TimeTicks end_time);

  void WriteIntoTrace(perfetto::TracedValue context) const;

 private:
  friend class BudgetPool;

  std::optional<QueueBlockType> GetBlockType(base::TimeTicks now) const;

  // To be used by BudgetPool only, use BudgetPool::{Add,Remove}Queue
  // methods instead.
  void AddBudgetPool(BudgetPool* budget_pool);
  void RemoveBudgetPool(BudgetPool* budget_pool);

  bool CanRunTasksAt(base::TimeTicks moment);
  // Returns the next allowed runtime, given |desired_runtime| if it was
  // affected by any BudgetPool, of TimeTicks() otherwise.
  base::TimeTicks GetNextAllowedRunTime(base::TimeTicks desired_runtime) const;

  // Returns the time until which tasks in |queue| can run. TimeTicks::Max()
  // means that there are no known limits.
  base::TimeTicks GetTimeTasksCanRunUntil(base::TimeTicks now) const;

  // See GetNextAllowedWakeUp().
  std::optional<base::sequence_manager::WakeUp> GetNextAllowedWakeUpImpl(
      base::LazyNow* lazy_now,
      std::optional<base::sequence_manager::WakeUp> next_wake_up,
      bool has_ready_task);

  // TaskQueue::Throttler implementation:
  std::optional<base::sequence_manager::WakeUp> GetNextAllowedWakeUp(
      base::LazyNow* lazy_now,
      std::optional<base::sequence_manager::WakeUp> next_wake_up,
      bool has_ready_task) override;
  void OnWakeUp(base::LazyNow* lazy_now) override;
  void OnHasImmediateTask() override;

  void UpdateFence(base::TimeTicks now);

  void DisableThrottling();

  const raw_ptr<base::sequence_manager::TaskQueue> task_queue_;
  size_t throttling_ref_count_ = 0;
  HashSet<BudgetPool*> budget_pools_;
  raw_ptr<const base::TickClock, DanglingUntriaged> tick_clock_;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_THROTTLING_TASK_QUEUE_THROTTLER_H_
