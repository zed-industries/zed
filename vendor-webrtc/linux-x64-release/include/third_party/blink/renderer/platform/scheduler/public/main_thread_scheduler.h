// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_MAIN_THREAD_SCHEDULER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_MAIN_THREAD_SCHEDULER_H_

#include <memory>

#include "base/functional/callback_forward.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/common/input/web_input_event_attribution.h"
#include "third_party/blink/renderer/platform/scheduler/public/thread_scheduler.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {
class AgentGroupScheduler;

namespace scheduler {
class WebThreadScheduler;
}  // namespace scheduler

namespace test {
class TaskEnvironment;
}  // namespace test

class RAILModeObserver;

class ExecuteAfterCurrentTaskRestricted {
 private:
  // Permitted users of `ThreadScheduler::ExecuteAfterCurrentTaskForTesting()`.
  friend class DOMScheduler;

  ExecuteAfterCurrentTaskRestricted() = default;
};

// This class is used to submit tasks and pass other information from Blink to
// the platform's main thread scheduler.
class PLATFORM_EXPORT MainThreadScheduler : public ThreadScheduler {
 public:
  // RAII handle for pausing the renderer. Renderer is paused while
  // at least one pause handle exists.
  class PLATFORM_EXPORT RendererPauseHandle {
   public:
    RendererPauseHandle() = default;
    RendererPauseHandle(const RendererPauseHandle&) = delete;
    RendererPauseHandle& operator=(const RendererPauseHandle&) = delete;
    virtual ~RendererPauseHandle() = default;
  };

  // Tells the scheduler that the renderer process should be paused.
  // Pausing means that all javascript callbacks should not fire.
  // https://html.spec.whatwg.org/#pause
  //
  // Renderer will be resumed when the handle is destroyed.
  // Handle should be destroyed before the renderer.
  [[nodiscard]] virtual std::unique_ptr<RendererPauseHandle>
  PauseScheduler() = 0;

  // Returns a task runner which does not generate system wakeups on its own.
  // This means that if a delayed task is posted to it, it will run when
  // the delay expires AND another task runs.
  virtual scoped_refptr<base::SingleThreadTaskRunner> NonWakingTaskRunner() = 0;

  // Creates a AgentGroupScheduler implementation.
  virtual AgentGroupScheduler* CreateAgentGroupScheduler() = 0;

  // The current active AgentGroupScheduler is set when the task gets
  // started (i.e., OnTaskStarted) and unset when the task gets
  // finished (i.e., OnTaskCompleted). GetCurrentAgentGroupScheduler()
  // returns nullptr in task observers.
  virtual AgentGroupScheduler* GetCurrentAgentGroupScheduler() = 0;

  virtual void AddRAILModeObserver(RAILModeObserver* observer) = 0;

  virtual void RemoveRAILModeObserver(RAILModeObserver const* observer) = 0;

  // Calls the callback for each unique isolate that bound to the main thread.
  virtual void ForEachMainThreadIsolate(
      base::RepeatingCallback<void(v8::Isolate* isolate)> callback) = 0;

  // Returns a list of all unique attributions that are marked for event
  // dispatch. If |include_continuous| is true, include event types from
  // "continuous" sources (see PendingUserInput::IsContinuousEventTypes).
  virtual Vector<WebInputEventAttribution> GetPendingUserInputInfo(
      bool include_continuous) const {
    return {};
  }

  // Test helpers

  // Runs `on_completion_task` after the current task has finished.
  virtual void ExecuteAfterCurrentTaskForTesting(
      base::OnceClosure on_completion_task,
      ExecuteAfterCurrentTaskRestricted) = 0;

  // Starts an idle period, allowing pending idle tasks to run. Idle tasks can
  // only run within an idle period, which is determined based on compositor
  // signals. This method enables idle tasks to run in tests outside of a
  // detected idle period. The idle period ends once all idle tasks scheduled
  // before this method was called have run.
  virtual void StartIdlePeriodForTesting() = 0;

  // See WebThreadScheduler::SetRendererBackgrounded().
  virtual void SetRendererBackgroundedForTesting(bool backgrounded) = 0;

 private:
  // For `ToWebMainThreadScheduler`.
  friend class scheduler::WebThreadScheduler;

  // For `Isolate`.
  friend class ScopedMainThreadOverrider;
  friend class test::TaskEnvironment;

  // Get the isolate previously set with `SetV8Isolate`. This method is scoped
  // private so only friends can use it. Other users should use
  // `WebAgentGroupScheduler::Isolate` instead.
  virtual v8::Isolate* Isolate() = 0;

  // Return a reference to an underlying main thread WebThreadScheduler object.
  // This will be null if the `MainThreadScheduler` object doesn't support this,
  // which can happen in tests if not using a real scheduler.
  virtual scheduler::WebThreadScheduler* ToWebMainThreadScheduler() {
    return nullptr;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_MAIN_THREAD_SCHEDULER_H_
