// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_WORKER_WORKER_SCHEDULER_PROXY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_WORKER_WORKER_SCHEDULER_PROXY_H_

#include <optional>

#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/thread_checker.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/main_thread/frame_origin_type.h"
#include "third_party/blink/renderer/platform/scheduler/public/frame_or_worker_scheduler.h"
#include "third_party/blink/renderer/platform/scheduler/public/frame_status.h"
#include "third_party/blink/renderer/platform/wtf/wtf.h"

namespace blink {
namespace scheduler {
class WorkerScheduler;

// Helper class for communication between parent scheduler (may be a frame
// scheduler on the main thread or another woker scheduler on a worker thread)
// and worker scheduler (worker thread).
//
// It's owned by DedicatedWorkerThread and is created and destroyed
// on the parent thread. It's passed to WorkerScheduler during its
// construction. Given that DedicatedWorkerThread object outlives worker thread,
// this class outlives worker thread too.
class PLATFORM_EXPORT WorkerSchedulerProxy {
 public:
  explicit WorkerSchedulerProxy(FrameOrWorkerScheduler* scheduler);
  WorkerSchedulerProxy(const WorkerSchedulerProxy&) = delete;
  WorkerSchedulerProxy& operator=(const WorkerSchedulerProxy&) = delete;
  ~WorkerSchedulerProxy();

  void OnWorkerSchedulerCreated(
      base::WeakPtr<WorkerScheduler> worker_scheduler);

  void OnLifecycleStateChanged(SchedulingLifecycleState lifecycle_state);

  // Accessed only during init.
  SchedulingLifecycleState lifecycle_state() const {
    DCHECK(!initialized_);
    return lifecycle_state_;
  }

 private:
  // Can be accessed only from the worker thread.
  base::WeakPtr<WorkerScheduler> worker_scheduler_;

  // Const after init on the worker thread.
  scoped_refptr<base::SingleThreadTaskRunner> worker_thread_task_runner_;

  SchedulingLifecycleState lifecycle_state_ =
      SchedulingLifecycleState::kNotThrottled;

  std::unique_ptr<FrameOrWorkerScheduler::LifecycleObserverHandle>
      throttling_observer_handle_;

  bool initialized_ = false;

  THREAD_CHECKER(parent_thread_checker_);
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_WORKER_WORKER_SCHEDULER_PROXY_H_
