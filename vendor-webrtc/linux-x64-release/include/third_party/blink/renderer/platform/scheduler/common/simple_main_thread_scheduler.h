// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_SIMPLE_MAIN_THREAD_SCHEDULER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_SIMPLE_MAIN_THREAD_SCHEDULER_H_

#include <memory>

#include "base/memory/raw_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/platform/scheduler/web_agent_group_scheduler.h"
#include "third_party/blink/renderer/platform/scheduler/public/main_thread_scheduler.h"

namespace blink {
namespace scheduler {

// ThreadScheduler implementation that should be used when you don't have a
// dedicated ThreadScheduler instance. This is essentially a scheduler doesn't
// really schedule anything -- it just runs tasks as they come.
//
// This scheduler does not implement several features (see comments at each
// function). If you invoke a Blink functionality that relies on an
// unimplemented scheduler function, you might observe crashes or misbehavior.
// Apparently we don't rely on those missing features at least for things that
// use this scheduler.

class SimpleMainThreadScheduler : public MainThreadScheduler {
 public:
  SimpleMainThreadScheduler();
  SimpleMainThreadScheduler(const SimpleMainThreadScheduler&) = delete;
  SimpleMainThreadScheduler& operator=(const SimpleMainThreadScheduler&) =
      delete;
  ~SimpleMainThreadScheduler() override;

  // Do nothing.
  void Shutdown() override;

  // Always return false.
  bool ShouldYieldForHighPriorityWork() override;

  // Those tasks are simply ignored (we assume there's no idle period).
  void PostIdleTask(const base::Location&, Thread::IdleTask) override;
  void PostDelayedIdleTask(const base::Location&,
                           base::TimeDelta delay,
                           Thread::IdleTask) override;
  void RemoveCancelledIdleTasks() override;

  // Do nothing (the observer won't get notified).
  void AddRAILModeObserver(RAILModeObserver*) override;

  // Do nothing.
  void RemoveRAILModeObserver(RAILModeObserver const*) override;

  void ForEachMainThreadIsolate(
      base::RepeatingCallback<void(v8::Isolate* isolate)> callback) override;

  // Return the thread task runner (there's no separate task runner for them).
  scoped_refptr<base::SingleThreadTaskRunner> V8TaskRunner() override;
  scoped_refptr<base::SingleThreadTaskRunner> CleanupTaskRunner() override;
  scoped_refptr<base::SingleThreadTaskRunner> NonWakingTaskRunner() override;

  // Unsupported. Return nullptr.
  AgentGroupScheduler* CreateAgentGroupScheduler() override;

  // Return nullptr
  AgentGroupScheduler* GetCurrentAgentGroupScheduler() override;

  // Return the current time.
  base::TimeTicks MonotonicallyIncreasingVirtualTime() override;

  // Unsupported. The observer won't get called. May break some functionalities
  // that rely on the task observer.
  void AddTaskObserver(base::TaskObserver*) override;
  void RemoveTaskObserver(base::TaskObserver*) override;

  // Return pointer to this.
  MainThreadScheduler* ToMainThreadScheduler() override;

  void SetV8Isolate(v8::Isolate* isolate) override;
  v8::Isolate* Isolate() override;
  std::unique_ptr<RendererPauseHandle> PauseScheduler() override;

  // After-task callbacks are dropped, so this is a no-op.
  void ExecuteAfterCurrentTaskForTesting(
      base::OnceClosure on_completion_task,
      ExecuteAfterCurrentTaskRestricted) override;

  // Idle tasks are dropped in `PostIdleTask()` and friends, so this is a no-op.
  void StartIdlePeriodForTesting() override;

  // Do nothing. This class does not differentiate between foregrounded and
  // backgrounded renderers.
  void SetRendererBackgroundedForTesting(bool) override;

 private:
  raw_ptr<v8::Isolate> isolate_ = nullptr;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_SIMPLE_MAIN_THREAD_SCHEDULER_H_
