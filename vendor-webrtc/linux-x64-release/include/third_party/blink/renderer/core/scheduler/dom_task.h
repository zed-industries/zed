// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCHEDULER_DOM_TASK_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCHEDULER_DOM_TASK_H_

#include <optional>

#include "base/time/time.h"
#include "third_party/blink/public/common/scheduler/task_attribution_id.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/probe/async_task_context.h"
#include "third_party/blink/renderer/core/scheduler/dom_scheduler.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cancellable_task.h"

namespace blink {
class SchedulerTaskContext;
class ScriptState;
class V8SchedulerPostTaskCallback;

namespace scheduler {
class TaskAttributionInfo;
}

// DOMTask represents a task scheduled via the web scheduling API. It will
// keep itself alive until DOMTask::Invoke is called, which may be after the
// callback's v8 context is invalid, in which case, the task will not be run.
class DOMTask final : public GarbageCollected<DOMTask> {
 public:
  DOMTask(ScriptPromiseResolver<IDLAny>*,
          V8SchedulerPostTaskCallback*,
          SchedulerTaskContext*,
          DOMScheduler::DOMTaskQueue*,
          base::TimeDelta delay,
          uint64_t task_id_for_tracing);

  void Trace(Visitor*) const;

  void OnPendingPromiseSettled();

 private:
  enum class ExecutionState {
    kNotStarted,
    kRunningSync,
    kRunningAsync,
    kFinished,
  };

  // Entry point for running this DOMTask's |callback_|.
  void Invoke();
  // Internal step of Invoke that handles invoking the callback, including
  // catching any errors and retrieving the result.
  void InvokeInternal(ScriptState*);
  void OnAbort();
  void RemoveAbortAlgorithm();

  TaskHandle task_handle_;
  Member<V8SchedulerPostTaskCallback> callback_;
  Member<ScriptPromiseResolver<IDLAny>> resolver_;
  probe::AsyncTaskContext async_task_context_;
  Member<SchedulerTaskContext> scheduler_task_context_;
  Member<AbortSignal::AlgorithmHandle> abort_handle_;
  // Do not remove. For dynamic priority task queues, |task_queue_| ensures that
  // the associated WebSchedulingTaskQueue stays alive until after this task
  // runs, which is necessary to ensure throttling works correctly.
  Member<DOMScheduler::DOMTaskQueue> task_queue_;
  const base::TimeDelta delay_;
  const uint64_t task_id_for_tracing_;
  ExecutionState execution_state_ = ExecutionState::kNotStarted;
  Member<scheduler::TaskAttributionInfo> parent_task_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCHEDULER_DOM_TASK_H_
