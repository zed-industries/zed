// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_THREAD_SCHEDULER_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_THREAD_SCHEDULER_BASE_H_

#include "base/memory/raw_ptr.h"
#include "third_party/blink/renderer/platform/platform_export.h"

#include "base/task/sequenced_task_runner.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/platform/scheduler/common/scheduler_helper.h"
#include "third_party/blink/renderer/platform/scheduler/public/virtual_time_controller.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/perfetto/include/perfetto/tracing/traced_value_forward.h"

namespace base {
class TickClock;
}  // namespace base

namespace v8 {
class Isolate;
}

namespace blink {
namespace scheduler {
class AutoAdvancingVirtualTimeDomain;

// Scheduler-internal interface for the common methods between
// MainThreadSchedulerImpl and NonMainThreadSchedulerImpl which should
// not be exposed outside the scheduler.
// This class does not implement the public ThreadScheduler interface
// but provides functionality so that subclasses such as MainThreadScheduler
// can extend ThreadScheduler and not end up in with diamond inheritenance.
class PLATFORM_EXPORT ThreadSchedulerBase : public VirtualTimeController,
                                            public SchedulerHelper::Observer {
 public:
  virtual scoped_refptr<base::SingleThreadTaskRunner> ControlTaskRunner() = 0;

  virtual const base::TickClock* GetTickClock() const = 0;

  // Allow places in the scheduler to do some work after the current task.
  // The primary use case here is batching – to allow updates to be processed
  // only once per task.
  void ExecuteAfterCurrentTask(base::OnceClosure on_completion_task);

  void SetV8Isolate(v8::Isolate* isolate) { isolate_ = isolate; }
  v8::Isolate* isolate() const { return isolate_; }

  void Shutdown();

  // VirtualTimeController implementation.
  base::TimeTicks EnableVirtualTime(base::Time initial_time) override;
  void DisableVirtualTimeForTesting() override;
  bool VirtualTimeAllowedToAdvance() const override;
  void GrantVirtualTimeBudget(
      base::TimeDelta budget,
      base::OnceClosure budget_exhausted_callback) override;
  void SetVirtualTimePolicy(VirtualTimePolicy virtual_time_policy) override;
  void SetMaxVirtualTimeTaskStarvationCount(
      int max_task_starvation_count) override;
  WebScopedVirtualTimePauser CreateWebScopedVirtualTimePauser(
      const WTF::String& name,
      WebScopedVirtualTimePauser::VirtualTaskDuration) override;

  bool IsVirtualTimeEnabled() const;
  base::TimeTicks IncrementVirtualTimePauseCount();
  void DecrementVirtualTimePauseCount();
  void MaybeAdvanceVirtualTime(base::TimeTicks new_virtual_time);
  AutoAdvancingVirtualTimeDomain* GetVirtualTimeDomain();
  VirtualTimePolicy GetVirtualTimePolicyForTest() const {
    return virtual_time_policy_;
  }

 protected:
  ThreadSchedulerBase();
  ~ThreadSchedulerBase() override;

  // Returns the list of callbacks to execute after the current task.
  virtual WTF::Vector<base::OnceClosure>& GetOnTaskCompletionCallbacks() = 0;

  // Returns instance of specific helper instantiated by a subclass.
  virtual SchedulerHelper& GetHelper() = 0;

  // Dispatch the callbacks which requested to be executed after the current
  // task.
  void DispatchOnTaskCompletionCallbacks();

  void WriteVirtualTimeInfoIntoTrace(perfetto::TracedDictionary& dict) const;

  // A derived implementation should provide a task runner associated for
  // virtual time control tasks (when VT budget is exhausted, callback will be
  // posted there).
  virtual base::SequencedTaskRunner* GetVirtualTimeTaskRunner() {
    NOTREACHED();
  }
  virtual void OnVirtualTimeEnabled() {}
  virtual void OnVirtualTimeDisabled() {}

  // Tells the derived implementation that VT is now paused and it has to
  // insert fences into its task queues as required.
  virtual void OnVirtualTimePaused() {}
  // Tells the derived implementation that VT is now resumed and it has to
  // remove fences added when time was paused from the queues it manages.
  virtual void OnVirtualTimeResumed() {}

 private:
  void NotifyVirtualTimePaused();
  void SetVirtualTimeStopped(bool virtual_time_stopped);
  void ApplyVirtualTimePolicy();

  // SchedulerHelper::Observer implementation:
  void OnBeginNestedRunLoop() override;
  void OnExitNestedRunLoop() override;

  raw_ptr<v8::Isolate, DanglingUntriaged> isolate_ = nullptr;

  // Note |virtual_time_domain_| is only present iff virtual time is enabled.
  std::unique_ptr<AutoAdvancingVirtualTimeDomain> virtual_time_domain_;
  VirtualTimePolicy virtual_time_policy_ = VirtualTimePolicy::kAdvance;

  // In VirtualTimePolicy::kDeterministicLoading virtual time is only allowed
  // to advance if this is zero.
  int virtual_time_pause_count_ = 0;

  // The maximum number amount of delayed task starvation we will allow in
  // VirtualTimePolicy::kAdvance or VirtualTimePolicy::kDeterministicLoading
  // unless the run_loop is nested (in which case infinite starvation is
  // allowed). NB a value of 0 allows infinite starvation.
  int max_virtual_time_task_starvation_count_ = 0;
  bool virtual_time_stopped_ = false;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_THREAD_SCHEDULER_BASE_H_
