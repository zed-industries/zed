// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_WORKER_NON_MAIN_THREAD_SCHEDULER_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_WORKER_NON_MAIN_THREAD_SCHEDULER_HELPER_H_

#include "base/memory/raw_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/platform/scheduler/common/scheduler_helper.h"

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/platform/task_type.h"
#include "third_party/blink/renderer/platform/scheduler/worker/non_main_thread_task_queue.h"

namespace blink {
namespace scheduler {

class NonMainThreadSchedulerBase;

// TODO(carlscab): This class is not really needed and should be removed
class PLATFORM_EXPORT NonMainThreadSchedulerHelper : public SchedulerHelper {
 public:
  // |sequence_manager| must remain valid until Shutdown() is called or the
  // object is destroyed. |main_thread_scheduler| must remain valid for the
  // entire lifetime of this object.
  NonMainThreadSchedulerHelper(
      base::sequence_manager::SequenceManager* manager,
      NonMainThreadSchedulerBase* non_main_thread_scheduler,
      TaskType default_task_type);
  NonMainThreadSchedulerHelper(const NonMainThreadSchedulerHelper&) = delete;
  NonMainThreadSchedulerHelper& operator=(const NonMainThreadSchedulerHelper&) =
      delete;
  ~NonMainThreadSchedulerHelper() override;

  scoped_refptr<NonMainThreadTaskQueue> NewTaskQueue(
      const base::sequence_manager::TaskQueue::Spec& spec,
      NonMainThreadTaskQueue::QueueCreationParams params =
          NonMainThreadTaskQueue::QueueCreationParams());

  scoped_refptr<NonMainThreadTaskQueue> DefaultNonMainThreadTaskQueue();
  scoped_refptr<NonMainThreadTaskQueue> ControlNonMainThreadTaskQueue();

  const scoped_refptr<base::SingleThreadTaskRunner>& ControlTaskRunner()
      override;

  const scoped_refptr<base::SingleThreadTaskRunner>& InputTaskRunner();

 protected:
  void ShutdownAllQueues() override;

 private:
  // Creates a task queue managed by this class. These queues are created
  // without providing a backup thread task runner used for object deletion,
  // which is unnecessary for internal queues since they match the lifetime of
  // the default thread task runner.
  scoped_refptr<NonMainThreadTaskQueue> NewTaskQueueInternal(
      const base::sequence_manager::TaskQueue::Spec& spec,
      NonMainThreadTaskQueue::QueueCreationParams params =
          NonMainThreadTaskQueue::QueueCreationParams());

  raw_ptr<NonMainThreadSchedulerBase> non_main_thread_scheduler_;  // NOT OWNED
  const scoped_refptr<NonMainThreadTaskQueue> default_task_queue_;
  const scoped_refptr<NonMainThreadTaskQueue> input_task_queue_;
  const scoped_refptr<NonMainThreadTaskQueue> control_task_queue_;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_WORKER_NON_MAIN_THREAD_SCHEDULER_HELPER_H_
