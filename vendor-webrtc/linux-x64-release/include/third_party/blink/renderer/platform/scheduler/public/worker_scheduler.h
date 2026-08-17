// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_WORKER_SCHEDULER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_WORKER_SCHEDULER_H_

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/platform/task_type.h"
#include "third_party/blink/renderer/platform/scheduler/public/frame_or_worker_scheduler.h"

namespace blink {
class VirtualTimeController;
namespace scheduler {

class WorkerSchedulerProxy;
class WorkerThreadScheduler;

// A scheduler provides per-global-scope task queues.
//
// Unless stated otherwise, all methods must be called on the worker thread.
class PLATFORM_EXPORT WorkerScheduler : public FrameOrWorkerScheduler {
 public:
  // Represents RAII handle for pausing the scheduler. The scheduler is paused
  // as long as one PauseHandle lives.
  class PLATFORM_EXPORT PauseHandle {
    USING_FAST_MALLOC(PauseHandle);

   public:
    PauseHandle(const PauseHandle&) = delete;
    PauseHandle& operator=(const PauseHandle&) = delete;
    virtual ~PauseHandle() = default;

   protected:
    PauseHandle() = default;
  };

  // Creates a new WorkerScheduler.
  static std::unique_ptr<WorkerScheduler> CreateWorkerScheduler(
      WorkerThreadScheduler* worker_thread_scheduler,
      WorkerSchedulerProxy* proxy);

  // Unregisters the task queues and cancels tasks in them.
  virtual void Dispose() = 0;

  // Returns a task runner that is suitable with the given task type. This can
  // be called from any thread.
  //
  // This must be called only from WorkerThread::GetTaskRunner().
  virtual scoped_refptr<base::SingleThreadTaskRunner> GetTaskRunner(
      TaskType) const = 0;

  virtual WorkerThreadScheduler* GetWorkerThreadScheduler() const = 0;

  virtual void OnLifecycleStateChanged(
      SchedulingLifecycleState lifecycle_state) = 0;

  // Pauses the scheduler. The scheduler is paused as long as PauseHandle lives.
  [[nodiscard]] virtual std::unique_ptr<PauseHandle> Pause() = 0;

  // Initializes this on a worker thread. This must not be called twice or more.
  // `delegate` must outlive this.
  virtual void InitializeOnWorkerThread(Delegate* delegate) = 0;

  virtual VirtualTimeController* GetVirtualTimeController() = 0;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_WORKER_SCHEDULER_H_
