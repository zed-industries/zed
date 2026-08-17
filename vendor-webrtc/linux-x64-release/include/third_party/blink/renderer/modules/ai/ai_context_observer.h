// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_AI_AI_CONTEXT_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_AI_AI_CONTEXT_OBSERVER_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/dom/abort_signal.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/ai/exception_helpers.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

// AIContextObserver is a base class for the renderer to send a mojo IPC to the
// AI component. It adds observers for the execution context lifecycle and the
// abort signal. The resources will be freed when the execution context gets
// destroyed or the user explicitly aborts.
template <typename V8SessionObjectType>
class AIContextObserver : public ContextLifecycleObserver {
 public:
  AIContextObserver(ScriptState* script_state,
                    ExecutionContextClient* context_client,
                    ScriptPromiseResolver<V8SessionObjectType>* resolver,
                    AbortSignal* abort_signal)
      : script_state_(script_state),
        context_client_(context_client),
        resolver_(resolver),
        abort_signal_(abort_signal) {
    CHECK(resolver);
    SetContextLifecycleNotifier(context_client_->GetExecutionContext());
    if (abort_signal_) {
      CHECK(!abort_signal_->aborted());
      abort_handle_ = abort_signal_->AddAlgorithm(WTF::BindOnce(
          &AIContextObserver::OnAborted, WrapWeakPersistent(this)));
    }
  }

  // `GarbageCollectedMixin` implementation
  void Trace(Visitor* visitor) const override {
    ContextLifecycleObserver::Trace(visitor);
    visitor->Trace(script_state_);
    visitor->Trace(context_client_);
    visitor->Trace(resolver_);
    visitor->Trace(abort_signal_);
    visitor->Trace(abort_handle_);
  }

  ~AIContextObserver() override = default;

 protected:
  ScriptState* GetScriptState() { return script_state_; }
  ScriptPromiseResolver<V8SessionObjectType>* GetResolver() {
    return resolver_;
  }
  AbortSignal* GetAbortSignal() { return abort_signal_; }

  void Cleanup() {
    ResetReceiver();
    resolver_ = nullptr;
    keep_alive_.Clear();
    if (abort_handle_) {
      abort_signal_->RemoveAlgorithm(abort_handle_);
      abort_handle_ = nullptr;
    }
  }

  virtual void ResetReceiver() = 0;

 private:
  // `ContextLifecycleObserver` implementation
  void ContextDestroyed() override { Cleanup(); }

  void OnAborted() {
    if (!resolver_) {
      return;
    }
    resolver_->Reject(abort_signal_->reason(script_state_));
    Cleanup();
  }

  Member<ScriptState> script_state_;
  Member<ExecutionContextClient> context_client_;
  Member<ScriptPromiseResolver<V8SessionObjectType>> resolver_;
  Member<AbortSignal> abort_signal_;
  Member<AbortSignal::AlgorithmHandle> abort_handle_;
  SelfKeepAlive<AIContextObserver> keep_alive_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_AI_AI_CONTEXT_OBSERVER_H_
