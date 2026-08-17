// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_WORKER_COMPOSITOR_THREAD_SCHEDULER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_WORKER_COMPOSITOR_THREAD_SCHEDULER_IMPL_H_

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/common/single_thread_idle_task_runner.h"
#include "third_party/blink/renderer/platform/scheduler/public/compositor_thread_scheduler.h"
#include "third_party/blink/renderer/platform/scheduler/public/thread_type.h"
#include "third_party/blink/renderer/platform/scheduler/worker/non_main_thread_scheduler_base.h"

namespace base {
class LazyNow;
class TaskObserver;
}  // namespace base

namespace blink {
namespace scheduler {

class PLATFORM_EXPORT CompositorThreadSchedulerImpl
    : public CompositorThreadScheduler,
      public NonMainThreadSchedulerBase,
      public SingleThreadIdleTaskRunner::Delegate {
 public:
  explicit CompositorThreadSchedulerImpl(
      base::sequence_manager::SequenceManager* sequence_manager);
  CompositorThreadSchedulerImpl(const CompositorThreadSchedulerImpl&) = delete;
  CompositorThreadSchedulerImpl& operator=(
      const CompositorThreadSchedulerImpl&) = delete;

  ~CompositorThreadSchedulerImpl() override;

  // NonMainThreadSchedulerImpl:
  scoped_refptr<NonMainThreadTaskQueue> DefaultTaskQueue() override;
  void OnTaskCompleted(
      NonMainThreadTaskQueue* worker_task_queue,
      const base::sequence_manager::Task& task,
      base::sequence_manager::TaskQueue::TaskTiming* task_timing,
      base::LazyNow* lazy_now) override;

  // WebThreadScheduler:
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

  // CompositorThreadScheduler:
  scoped_refptr<base::SingleThreadTaskRunner> InputTaskRunner() override;
  scoped_refptr<base::SingleThreadTaskRunner> DefaultTaskRunner() override;

  scoped_refptr<scheduler::SingleThreadIdleTaskRunner> IdleTaskRunner();

  // SingleThreadIdleTaskRunner::Delegate:
  void OnIdleTaskPosted() override;
  base::TimeTicks WillProcessIdleTask() override;
  void DidProcessIdleTask() override;
  base::TimeTicks NowTicks() override;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_WORKER_COMPOSITOR_THREAD_SCHEDULER_IMPL_H_
