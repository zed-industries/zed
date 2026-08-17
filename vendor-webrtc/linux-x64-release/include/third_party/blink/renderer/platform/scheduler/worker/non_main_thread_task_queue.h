// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_WORKER_NON_MAIN_THREAD_TASK_QUEUE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_WORKER_NON_MAIN_THREAD_TASK_QUEUE_H_

#include <optional>

#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "base/task/common/lazy_now.h"
#include "base/task/sequence_manager/task_queue.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/platform/task_type.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/common/blink_scheduler_single_thread_task_runner.h"
#include "third_party/blink/renderer/platform/scheduler/common/task_priority.h"
#include "third_party/blink/renderer/platform/scheduler/common/throttling/task_queue_throttler.h"
#include "third_party/blink/renderer/platform/scheduler/public/web_scheduling_priority.h"
#include "third_party/blink/renderer/platform/scheduler/public/web_scheduling_queue_type.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"

namespace base::sequence_manager {
class SequenceManager;
}  // namespace base::sequence_manager

namespace blink::scheduler {

using TaskQueue = base::sequence_manager::TaskQueue;

class NonMainThreadSchedulerBase;

class PLATFORM_EXPORT NonMainThreadTaskQueue
    : public ThreadSafeRefCounted<NonMainThreadTaskQueue> {
 public:
  struct QueueCreationParams {
    QueueCreationParams() = default;

    QueueCreationParams SetCanBeThrottled(bool value) {
      can_be_throttled = value;
      return *this;
    }

    QueueCreationParams SetWebSchedulingQueueType(
        std::optional<WebSchedulingQueueType> type) {
      web_scheduling_queue_type = type;
      return *this;
    }

    QueueCreationParams SetWebSchedulingPriority(
        std::optional<WebSchedulingPriority> priority) {
      web_scheduling_priority = priority;
      return *this;
    }

    bool can_be_throttled = false;
    std::optional<WebSchedulingQueueType> web_scheduling_queue_type;
    std::optional<WebSchedulingPriority> web_scheduling_priority;
  };

  NonMainThreadTaskQueue(
      base::sequence_manager::SequenceManager& sequence_manager,
      const TaskQueue::Spec& spec,
      NonMainThreadSchedulerBase* non_main_thread_scheduler,
      QueueCreationParams params,
      scoped_refptr<base::SingleThreadTaskRunner> thread_task_runner);

  void OnTaskCompleted(
      const base::sequence_manager::Task& task,
      base::sequence_manager::TaskQueue::TaskTiming* task_timing,
      base::LazyNow* lazy_now);

  scoped_refptr<base::SingleThreadTaskRunner> CreateTaskRunner(
      TaskType task_type);

  bool IsThrottled() const { return throttler_->IsThrottled(); }

  // Methods for setting and resetting budget pools for this task queue.
  // Note that a task queue can be in multiple budget pools so a pool must
  // be specified when removing.
  void AddToBudgetPool(base::TimeTicks now, BudgetPool* pool);
  void RemoveFromBudgetPool(base::TimeTicks now, BudgetPool* pool);

  void IncreaseThrottleRefCount();
  void DecreaseThrottleRefCount();

  void SetQueuePriority(TaskPriority priority) {
    task_queue_->SetQueuePriority(priority);
  }

  TaskPriority GetQueuePriority() const {
    return static_cast<TaskPriority>(task_queue_->GetQueuePriority());
  }

  std::unique_ptr<TaskQueue::QueueEnabledVoter> CreateQueueEnabledVoter() {
    return task_queue_->CreateQueueEnabledVoter();
  }

  void ShutdownTaskQueue();

  // This method returns the default task runner with task type kTaskTypeNone
  // and is mostly used for tests. For most use cases, you'll want a more
  // specific task runner and should use the 'CreateTaskRunner' method and pass
  // the desired task type.
  const scoped_refptr<base::SingleThreadTaskRunner>&
  GetTaskRunnerWithDefaultTaskType() const {
    return task_queue_->task_runner();
  }

  void SetWebSchedulingPriority(WebSchedulingPriority priority);

  void OnTaskRunTimeReported(TaskQueue::TaskTiming* task_timing);

  // TODO(crbug.com/1143007): Improve MTTQ API surface so that we no longer
  // need to expose the raw pointer to the queue.
  TaskQueue* GetTaskQueue() { return task_queue_.get(); }

  // This method returns the default task runner with task type kTaskTypeNone
  // and is mostly used for tests. For most use cases, you'll want a more
  // specific task runner and should use the 'CreateTaskRunner' method and pass
  // the desired task type.
  const scoped_refptr<base::SingleThreadTaskRunner>&
  GetTaskRunnerWithDefaultTaskType() {
    return task_runner_with_default_task_type_;
  }

 private:
  friend class ThreadSafeRefCounted<NonMainThreadTaskQueue>;
  ~NonMainThreadTaskQueue();

  void OnWebSchedulingPriorityChanged();

  scoped_refptr<BlinkSchedulerSingleThreadTaskRunner> WrapTaskRunner(
      scoped_refptr<base::SingleThreadTaskRunner>);

  TaskQueue::Handle task_queue_;
  std::optional<TaskQueueThrottler> throttler_;

  // Not owned.
  raw_ptr<NonMainThreadSchedulerBase> non_main_thread_scheduler_;

  // Set if this is queue is used for the web-exposed scheduling API. Used to
  // differentiate initial tasks from continuations for prioritization.
  const std::optional<WebSchedulingQueueType> web_scheduling_queue_type_;

  // |web_scheduling_priority_| is the priority of the task queue within the web
  // scheduling API. This priority is used to determine the task queue priority.
  std::optional<WebSchedulingPriority> web_scheduling_priority_;

  scoped_refptr<base::SingleThreadTaskRunner> thread_task_runner_;
  scoped_refptr<base::SingleThreadTaskRunner>
      task_runner_with_default_task_type_;
};

}  // namespace blink::scheduler

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_WORKER_NON_MAIN_THREAD_TASK_QUEUE_H_
