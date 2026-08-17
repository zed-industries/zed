// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_MAIN_THREAD_SCHEDULER_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_MAIN_THREAD_SCHEDULER_HELPER_H_

#include "base/memory/raw_ptr.h"
#include "third_party/blink/renderer/platform/scheduler/common/scheduler_helper.h"

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/platform/scheduler/main_thread/main_thread_task_queue.h"

namespace blink {
namespace scheduler {

class MainThreadSchedulerImpl;

// TODO(carlscab): This class is not really needed and should be removed
class PLATFORM_EXPORT MainThreadSchedulerHelper : public SchedulerHelper {
 public:
  // |sequence_manager| must remain valid until Shutdown() is called or the
  // object is destroyed. |main_thread_scheduler| must remain valid for the
  // entire lifetime of this object.
  MainThreadSchedulerHelper(
      base::sequence_manager::SequenceManager* sequence_manager,
      MainThreadSchedulerImpl* main_thread_scheduler);
  MainThreadSchedulerHelper(const MainThreadSchedulerHelper&) = delete;
  MainThreadSchedulerHelper& operator=(const MainThreadSchedulerHelper&) =
      delete;
  ~MainThreadSchedulerHelper() override;

  scoped_refptr<MainThreadTaskQueue> NewTaskQueue(
      const MainThreadTaskQueue::QueueCreationParams& params);

  scoped_refptr<MainThreadTaskQueue> DefaultMainThreadTaskQueue();
  scoped_refptr<MainThreadTaskQueue> ControlMainThreadTaskQueue();
  scoped_refptr<base::SingleThreadTaskRunner> DeprecatedDefaultTaskRunner();

  const scoped_refptr<base::SingleThreadTaskRunner>& ControlTaskRunner()
      override;

 protected:
  void ShutdownAllQueues() override;

 private:
  raw_ptr<MainThreadSchedulerImpl> main_thread_scheduler_;  // NOT OWNED

  const scoped_refptr<MainThreadTaskQueue> default_task_queue_;
  const scoped_refptr<MainThreadTaskQueue> control_task_queue_;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_MAIN_THREAD_SCHEDULER_HELPER_H_
