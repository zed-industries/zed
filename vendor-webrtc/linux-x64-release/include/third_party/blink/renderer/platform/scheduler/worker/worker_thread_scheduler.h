// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_WORKER_WORKER_THREAD_SCHEDULER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_WORKER_WORKER_THREAD_SCHEDULER_H_

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/platform/scheduler/common/idle_helper.h"
#include "third_party/blink/renderer/platform/scheduler/public/frame_scheduler.h"
#include "third_party/blink/renderer/platform/scheduler/public/frame_status.h"
#include "third_party/blink/renderer/platform/scheduler/public/thread_type.h"
#include "third_party/blink/renderer/platform/scheduler/worker/non_main_thread_scheduler_base.h"

namespace base {
class LazyNow;
class TaskObserver;
namespace sequence_manager {
class SequenceManager;
}
}  // namespace base

namespace blink {
namespace scheduler {

class WorkerSchedulerImpl;
class WorkerSchedulerProxy;
class WakeUpBudgetPool;
class CPUTimeBudgetPool;

class PLATFORM_EXPORT WorkerThreadScheduler : public NonMainThreadSchedulerBase,
                                              public ThreadScheduler,
                                              public IdleHelper::Delegate {
 public:
  // |sequence_manager| and |proxy| must remain valid for the entire lifetime of
  // this object.
  WorkerThreadScheduler(
      ThreadType thread_type,
      base::sequence_manager::SequenceManager* sequence_manager,
      WorkerSchedulerProxy* proxy);
  WorkerThreadScheduler(const WorkerThreadScheduler&) = delete;
  WorkerThreadScheduler& operator=(const WorkerThreadScheduler&) = delete;
  ~WorkerThreadScheduler() override;

  // ThreadScheduler implementation:
  scoped_refptr<base::SingleThreadTaskRunner> V8TaskRunner() override;
  scoped_refptr<base::SingleThreadTaskRunner> CleanupTaskRunner() override;
  bool ShouldYieldForHighPriorityWork() override;
  void AddTaskObserver(base::TaskObserver* task_observer) override;
  void RemoveTaskObserver(base::TaskObserver* task_observer) override;
  void PostIdleTask(const base::Location&, Thread::IdleTask) override;
  void PostDelayedIdleTask(const base::Location&,
                           base::TimeDelta delay,
                           Thread::IdleTask) override;
  void RemoveCancelledIdleTasks() override;
  base::TimeTicks MonotonicallyIncreasingVirtualTime() override;
  void SetV8Isolate(v8::Isolate* isolate) override;
  void Shutdown() override;

  // NonMainThreadSchedulerImpl implementation:
  void Init() override;
  scoped_refptr<NonMainThreadTaskQueue> DefaultTaskQueue() override;
  void OnTaskCompleted(
      NonMainThreadTaskQueue* worker_task_queue,
      const base::sequence_manager::Task& task,
      base::sequence_manager::TaskQueue::TaskTiming* task_timing,
      base::LazyNow* lazy_now) override;

  SchedulerHelper* GetSchedulerHelperForTesting();
  base::TimeTicks CurrentIdleTaskDeadlineForTesting() const;

  scoped_refptr<SingleThreadIdleTaskRunner> IdleTaskRunner();
  scoped_refptr<base::SingleThreadTaskRunner> CompositorTaskRunner();

  // Virtual for test.
  virtual void OnLifecycleStateChanged(
      SchedulingLifecycleState lifecycle_state);

  SchedulingLifecycleState lifecycle_state() const { return lifecycle_state_; }

  // Each WorkerScheduler should notify NonMainThreadSchedulerImpl when it is
  // created or destroyed.
  void RegisterWorkerScheduler(WorkerSchedulerImpl* worker_scheduler);
  void UnregisterWorkerScheduler(WorkerSchedulerImpl* worker_scheduler);

  // Returns the control task queue.  Tasks posted to this queue are executed
  // with the highest priority. Care must be taken to avoid starvation of other
  // task queues.
  scoped_refptr<NonMainThreadTaskQueue> ControlTaskQueue();

  WakeUpBudgetPool* wake_up_budget_pool() const {
    return wake_up_budget_pool_.get();
  }
  CPUTimeBudgetPool* cpu_time_budget_pool() const {
    return cpu_time_budget_pool_.get();
  }

 protected:
  // IdleHelper::Delegate implementation:
  bool CanEnterLongIdlePeriod(
      base::TimeTicks now,
      base::TimeDelta* next_long_idle_period_delay_out) override;
  void IsNotQuiescent() override {}
  void OnPendingTasksChanged(bool new_state) override {}

  void CreateBudgetPools();

  void SetCPUTimeBudgetPoolForTesting(
      std::unique_ptr<CPUTimeBudgetPool> cpu_time_budget_pool);

  HashSet<WorkerSchedulerImpl*>& GetWorkerSchedulersForTesting();

  virtual void PerformMicrotaskCheckpoint();

 private:
  // ThreadSchedulerBase overrides
  base::SequencedTaskRunner* GetVirtualTimeTaskRunner() override;
  void OnVirtualTimeDisabled() override;
  void OnVirtualTimePaused() override;
  void OnVirtualTimeResumed() override;

  void MaybeStartLongIdlePeriod();

  const ThreadType thread_type_;
  scoped_refptr<NonMainThreadTaskQueue> idle_helper_queue_;
  IdleHelper idle_helper_;
  bool initialized_ = false;
  scoped_refptr<base::SingleThreadTaskRunner> v8_task_runner_;
  scoped_refptr<base::SingleThreadTaskRunner> compositor_task_runner_;
  SchedulingLifecycleState lifecycle_state_;

  // This controller should be initialized before any TraceableVariables
  // because they require one to initialize themselves.
  TraceableVariableController traceable_variable_controller_;

  // Worker schedulers associated with this thread.
  HashSet<WorkerSchedulerImpl*> worker_schedulers_;

  std::unique_ptr<WakeUpBudgetPool> wake_up_budget_pool_;
  std::unique_ptr<CPUTimeBudgetPool> cpu_time_budget_pool_;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_WORKER_WORKER_THREAD_SCHEDULER_H_
