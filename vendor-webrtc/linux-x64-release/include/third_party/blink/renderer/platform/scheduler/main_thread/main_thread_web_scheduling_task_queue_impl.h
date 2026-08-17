// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_MAIN_THREAD_WEB_SCHEDULING_TASK_QUEUE_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_MAIN_THREAD_WEB_SCHEDULING_TASK_QUEUE_IMPL_H_

#include "third_party/blink/renderer/platform/scheduler/public/web_scheduling_task_queue.h"

#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/public/web_scheduling_priority.h"

namespace blink {
namespace scheduler {

class MainThreadTaskQueue;

class PLATFORM_EXPORT MainThreadWebSchedulingTaskQueueImpl
    : public WebSchedulingTaskQueue {
 public:
  MainThreadWebSchedulingTaskQueueImpl(
      base::WeakPtr<MainThreadTaskQueue> immediate_task_queue,
      base::WeakPtr<MainThreadTaskQueue> delayed_task_queue);
  ~MainThreadWebSchedulingTaskQueueImpl() override;

  void SetPriority(WebSchedulingPriority) override;

  scoped_refptr<base::SingleThreadTaskRunner> GetTaskRunner() override;

 private:
  // In order to throttle delayed tasks in the background, we manage two
  // MainThreadTaskQueues and their associated TaskRunners---one for delayed
  // tasks and one for non-delayed tasks (immediate). Rather than exposing this
  // to the web scheduling layer, we implement a simple custom TaskRunner that
  // handles picking the appropriate underlying TaskRunner based on the delay
  // value and return that in GetTaskRunner().
  class WebSchedulingTaskRunner : public base::SingleThreadTaskRunner {
   public:
    // `delayed_task_runner` can be null if this is a continuation task queue
    // (continuations are always immediate). `immediate_task_runner` must be
    // non-null.
    WebSchedulingTaskRunner(
        scoped_refptr<base::SingleThreadTaskRunner> immediate_task_runner,
        scoped_refptr<base::SingleThreadTaskRunner> delayed_task_runner);

    bool PostDelayedTask(const base::Location& location,
                         base::OnceClosure task,
                         base::TimeDelta delay) override;
    bool PostNonNestableDelayedTask(const base::Location& from_here,
                                    base::OnceClosure task,
                                    base::TimeDelta delay) override;
    bool RunsTasksInCurrentSequence() const override;

   private:
    base::SingleThreadTaskRunner* GetTaskRunnerForDelay(base::TimeDelta delay);

    const scoped_refptr<base::SingleThreadTaskRunner> immediate_task_runner_;
    // Null for continuation task queues.
    const scoped_refptr<base::SingleThreadTaskRunner> delayed_task_runner_;
  };

  scoped_refptr<WebSchedulingTaskRunner> task_runner_;
  base::WeakPtr<MainThreadTaskQueue> immediate_task_queue_;
  base::WeakPtr<MainThreadTaskQueue> delayed_task_queue_;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_MAIN_THREAD_WEB_SCHEDULING_TASK_QUEUE_IMPL_H_
