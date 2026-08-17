// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_PAUSABLE_SCRIPT_EXECUTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_PAUSABLE_SCRIPT_EXECUTOR_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/public/mojom/script/script_evaluation_params.mojom-blink.h"
#include "third_party/blink/public/web/web_script_execution_callback.h"
#include "third_party/blink/public/web/web_script_source.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/bindings/dom_wrapper_world.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/self_keep_alive.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cancellable_task.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "v8/include/v8.h"

namespace blink {

class ScriptState;
enum class ExecuteScriptPolicy;

// PausableScriptExecutor executes scripts possibly with various capabilities:
// - Executes with user activation (`UserActivationOption`, Window only)
// - Executes sync/async (`EvaluationTiming`)
// - Delays the load event (`LoadEventBlockingOption`, Window only)
// - Converts results to `base::Value` (`WantResultOption`)
// - Waits for promise resolution (`PromiseResultOption`)
class CORE_EXPORT PausableScriptExecutor final
    : public GarbageCollected<PausableScriptExecutor>,
      public ExecutionContextLifecycleObserver {
 public:
  static void CreateAndRun(v8::Local<v8::Context>,
                           v8::Local<v8::Function>,
                           v8::Local<v8::Value> receiver,
                           int argc,
                           v8::Local<v8::Value> argv[],
                           mojom::blink::WantResultOption,
                           WebScriptExecutionCallback);
  static void CreateAndRun(ScriptState*,
                           Vector<WebScriptSource>,
                           ExecuteScriptPolicy,
                           mojom::blink::UserActivationOption,
                           mojom::blink::EvaluationTiming,
                           mojom::blink::LoadEventBlockingOption,
                           mojom::blink::WantResultOption,
                           mojom::blink::PromiseResultOption,
                           WebScriptExecutionCallback);

  class Executor : public GarbageCollected<Executor> {
   public:
    virtual ~Executor() = default;

    virtual v8::LocalVector<v8::Value> Execute(ScriptState*) = 0;

    virtual void Trace(Visitor* visitor) const {}
  };

  PausableScriptExecutor(ScriptState*,
                         mojom::blink::UserActivationOption,
                         mojom::blink::LoadEventBlockingOption,
                         mojom::blink::WantResultOption,
                         mojom::blink::PromiseResultOption,
                         WebScriptExecutionCallback,
                         Executor*);
  ~PausableScriptExecutor() override;

  void ContextDestroyed() override;

  void Trace(Visitor*) const override;

 private:
  void Run();
  void RunAsync();
  void PostExecuteAndDestroySelf(ExecutionContext* context);
  void ExecuteAndDestroySelf();
  void Dispose();

  void HandleResults(const v8::LocalVector<v8::Value>& results);

  Member<ScriptState> script_state_;
  WebScriptExecutionCallback callback_;
  base::TimeTicks start_time_;
  const mojom::blink::UserActivationOption user_activation_option_;
  const mojom::blink::LoadEventBlockingOption blocking_option_;
  const mojom::blink::WantResultOption want_result_option_;
  // Whether to wait for a promise to resolve, if the executed script evaluates
  // to a promise.
  const mojom::blink::PromiseResultOption wait_for_promise_;

  TaskHandle task_handle_;

  Member<Executor> executor_;

  // A keepalive used when waiting on promises to settle.
  SelfKeepAlive<PausableScriptExecutor> keep_alive_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_PAUSABLE_SCRIPT_EXECUTOR_H_
