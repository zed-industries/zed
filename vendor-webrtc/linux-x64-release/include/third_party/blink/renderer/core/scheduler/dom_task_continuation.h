// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCHEDULER_DOM_TASK_CONTINUATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCHEDULER_DOM_TASK_CONTINUATION_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/dom/abort_signal.h"
#include "third_party/blink/renderer/core/probe/async_task_context.h"
#include "third_party/blink/renderer/core/scheduler/dom_scheduler.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cancellable_task.h"

namespace blink {

// `DOMTaskContinuation` represents the continuation of a task, e.g. tasks
// scheduled with postTask or setTimeout, event listeners, rAF, etc.  It will
// keep itself alive until `DOMTaskContinuation::Invoke` is called, which may be
// after the promise resolver' v8 context is invalid, in which case, the
// continuation will not be run.
class DOMTaskContinuation final : public GarbageCollected<DOMTaskContinuation> {
 public:
  DOMTaskContinuation(ScriptPromiseResolver<IDLUndefined>*,
                      AbortSignal*,
                      DOMScheduler::DOMTaskQueue*,
                      uint64_t task_id_for_tracing);

  void Trace(Visitor*) const;

 private:
  // Entry point for running this continuation.
  void Invoke();

  // Abort algorithm callback.
  void OnAbort();

  TaskHandle task_handle_;
  Member<ScriptPromiseResolver<IDLUndefined>> resolver_;
  probe::AsyncTaskContext async_task_context_;
  Member<AbortSignal> signal_;
  Member<AbortSignal::AlgorithmHandle> abort_handle_;
  // Do not remove. For dynamic priority task queues, `task_queue_` ensures that
  // the associated WebSchedulingTaskQueue stays alive until after this task
  // runs, which is necessary to ensure throttling works correctly.
  Member<DOMScheduler::DOMTaskQueue> task_queue_;
  const uint64_t task_id_for_tracing_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCHEDULER_DOM_TASK_CONTINUATION_H_
