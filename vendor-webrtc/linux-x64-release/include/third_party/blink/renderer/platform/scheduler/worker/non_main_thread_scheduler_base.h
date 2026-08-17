// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_WORKER_NON_MAIN_THREAD_SCHEDULER_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_WORKER_NON_MAIN_THREAD_SCHEDULER_BASE_H_

#include "base/task/sequence_manager/sequence_manager.h"
#include "base/task/sequence_manager/task_queue.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/platform/scheduler/web_agent_group_scheduler.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/common/single_thread_idle_task_runner.h"
#include "third_party/blink/renderer/platform/scheduler/common/thread_scheduler_base.h"
#include "third_party/blink/renderer/platform/scheduler/common/tracing_helper.h"
#include "third_party/blink/renderer/platform/scheduler/public/thread_scheduler.h"
#include "third_party/blink/renderer/platform/scheduler/public/thread_type.h"
#include "third_party/blink/renderer/platform/scheduler/worker/non_main_thread_scheduler_helper.h"
#include "third_party/blink/renderer/platform/scheduler/worker/non_main_thread_task_queue.h"

namespace base {
class LazyNow;
}

namespace blink::scheduler {

class PLATFORM_EXPORT NonMainThreadSchedulerBase : public ThreadSchedulerBase {
 public:
  NonMainThreadSchedulerBase(const NonMainThreadSchedulerBase&) = delete;
  NonMainThreadSchedulerBase& operator=(const NonMainThreadSchedulerBase&) =
      delete;
  ~NonMainThreadSchedulerBase() override;

  // Performs initialization that must occur after the constructor of all
  // subclasses has run. Must be invoked before any other method. Must be
  // invoked on the same sequence as the constructor.
  virtual void Init() {}

  // Attaches the scheduler to the current thread. Must be invoked on the thread
  // that runs tasks from this scheduler, before running tasks from this
  // scheduler.
  void AttachToCurrentThread();

  virtual scoped_refptr<NonMainThreadTaskQueue> DefaultTaskQueue() = 0;

  virtual void OnTaskCompleted(
      NonMainThreadTaskQueue* worker_task_queue,
      const base::sequence_manager::Task& task,
      base::sequence_manager::TaskQueue::TaskTiming* task_timing,
      base::LazyNow* lazy_now) = 0;

  // ThreadSchedulerBase:
  scoped_refptr<base::SingleThreadTaskRunner> ControlTaskRunner() override;
  const base::TickClock* GetTickClock() const override;

  // Returns base::TimeTicks::Now() by default.
  base::TimeTicks MonotonicallyIncreasingVirtualTime();

  scoped_refptr<NonMainThreadTaskQueue> CreateTaskQueue(
      base::sequence_manager::QueueName name,
      NonMainThreadTaskQueue::QueueCreationParams params =
          NonMainThreadTaskQueue::QueueCreationParams());

 protected:
  // ThreadSchedulerBase:
  WTF::Vector<base::OnceClosure>& GetOnTaskCompletionCallbacks() override;

  // |sequence_manager| must remain valid for the entire lifetime of
  // this object.
  explicit NonMainThreadSchedulerBase(
      base::sequence_manager::SequenceManager* sequence_manager,
      TaskType default_task_type);

  friend class WorkerSchedulerImpl;

  NonMainThreadSchedulerHelper& GetHelper() override { return helper_; }

 private:
  NonMainThreadSchedulerHelper helper_;

  // List of callbacks to execute after the current task.
  WTF::Vector<base::OnceClosure> on_task_completion_callbacks_;
};

}  // namespace blink::scheduler

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_WORKER_NON_MAIN_THREAD_SCHEDULER_BASE_H_
