// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_WORKER_NON_MAIN_THREAD_WEB_SCHEDULING_TASK_QUEUE_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_WORKER_NON_MAIN_THREAD_WEB_SCHEDULING_TASK_QUEUE_IMPL_H_

#include "third_party/blink/renderer/platform/scheduler/public/web_scheduling_task_queue.h"

#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/public/web_scheduling_priority.h"

namespace blink {
namespace scheduler {

class NonMainThreadTaskQueue;

class PLATFORM_EXPORT NonMainThreadWebSchedulingTaskQueueImpl
    : public WebSchedulingTaskQueue {
 public:
  explicit NonMainThreadWebSchedulingTaskQueueImpl(
      scoped_refptr<NonMainThreadTaskQueue>);
  ~NonMainThreadWebSchedulingTaskQueueImpl() override = default;

  void SetPriority(WebSchedulingPriority) override;

  scoped_refptr<base::SingleThreadTaskRunner> GetTaskRunner() override;

 private:
  const scoped_refptr<base::SingleThreadTaskRunner> task_runner_;

  // NonMainThreadWebSchedulingTaskQueueImpl owns this NonMainThreadTaskQueue
  // and has the sole reference to it after its creation.
  const scoped_refptr<NonMainThreadTaskQueue> task_queue_;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_WORKER_NON_MAIN_THREAD_WEB_SCHEDULING_TASK_QUEUE_IMPL_H_
