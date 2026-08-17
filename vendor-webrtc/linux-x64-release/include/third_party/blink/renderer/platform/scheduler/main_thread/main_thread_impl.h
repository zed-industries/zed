// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_MAIN_THREAD_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_MAIN_THREAD_IMPL_H_

#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/public/main_thread.h"

namespace blink {
class ThreadScheduler;
}

namespace blink {
namespace scheduler {
class MainThreadSchedulerImpl;

class PLATFORM_EXPORT MainThreadImpl : public MainThread {
 public:
  explicit MainThreadImpl(MainThreadSchedulerImpl* scheduler);
  ~MainThreadImpl() override;

  // MainThread implementation.
  scoped_refptr<base::SingleThreadTaskRunner> GetTaskRunner(
      MainThreadTaskRunnerRestricted) const override;

  // Thread implementation.
  ThreadScheduler* Scheduler() override;
  void AddTaskTimeObserver(base::sequence_manager::TaskTimeObserver*) override;
  void RemoveTaskTimeObserver(
      base::sequence_manager::TaskTimeObserver*) override;
  base::TimeTicks CurrentTaskStartTime() const override;

 private:
  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
  raw_ptr<MainThreadSchedulerImpl, DanglingUntriaged> scheduler_;  // Not owned.
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_MAIN_THREAD_IMPL_H_
