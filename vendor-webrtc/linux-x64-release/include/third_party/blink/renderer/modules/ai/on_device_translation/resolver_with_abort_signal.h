
// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_AI_ON_DEVICE_TRANSLATION_RESOLVER_WITH_ABORT_SIGNAL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_AI_ON_DEVICE_TRANSLATION_RESOLVER_WITH_ABORT_SIGNAL_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/dom/abort_signal.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/platform/bindings/script_state.h"
#include "third_party/blink/renderer/platform/context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

// Wraps a `ScriptPromiseResolver` with an `AbortSignal`. Rejects the
// `ScriptPromiseResolver` if the AbortSignal is aborted, and `Resolve` becomes
// a no-op.
template <typename T>
class ResolverWithAbortSignal final
    : public GarbageCollected<ResolverWithAbortSignal<T>>,
      public ContextLifecycleObserver {
 public:
  ResolverWithAbortSignal(ScriptState* script_state, AbortSignal* abort_signal)
      : script_state_(script_state),
        resolver_(MakeGarbageCollected<ScriptPromiseResolver<T>>(script_state)),
        abort_signal_(abort_signal) {
    SetContextLifecycleNotifier(ExecutionContext::From(script_state));
    if (abort_signal_) {
      CHECK(!abort_signal_->aborted());
      abort_handle_ = abort_signal_->AddAlgorithm(WTF::BindOnce(
          &ResolverWithAbortSignal<T>::OnAborted, WrapWeakPersistent(this)));
    }
  }
  ~ResolverWithAbortSignal() override = default;
  ResolverWithAbortSignal(const ResolverWithAbortSignal&) = delete;
  ResolverWithAbortSignal& operator=(const ResolverWithAbortSignal&) = delete;

  // `GarbageCollectedMixin` implementation
  void Trace(Visitor* visitor) const override {
    ContextLifecycleObserver::Trace(visitor);
    visitor->Trace(script_state_);
    visitor->Trace(resolver_);
    visitor->Trace(abort_signal_);
    visitor->Trace(abort_handle_);
  }

  bool aborted() { return !resolver_; }

  ScriptPromise<T> Promise() {
    if (!resolver_) {
      return EmptyPromise();
    }

    return resolver_->Promise();
  }

  template <typename BlinkType>
  void Resolve(BlinkType value) {
    if (!resolver_) {
      return;
    }

    resolver_->Resolve(value);

    Cleanup();
  }

  template <typename BlinkType>
  void Reject(BlinkType value) {
    if (!resolver_) {
      return;
    }

    resolver_->Reject(value);

    Cleanup();
  }

 private:
  // `ContextLifecycleObserver` implementation
  void ContextDestroyed() override { Cleanup(); }

  void OnAborted() {
    if (!resolver_) {
      return;
    }

    auto reason = abort_signal_->reason(script_state_);
    if (reason.IsEmpty()) {
      resolver_->Reject(DOMException::Create(
          "The request has been aborted.",
          DOMException::GetErrorName(DOMExceptionCode::kAbortError)));
    } else {
      resolver_->Reject(reason.V8Value());
    }

    Cleanup();
  }

  void Cleanup() { resolver_ = nullptr; }

  Member<ScriptState> script_state_;

  Member<ScriptPromiseResolver<T>> resolver_;

  Member<AbortSignal::AlgorithmHandle> abort_handle_;
  Member<AbortSignal> abort_signal_;
};
}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_AI_ON_DEVICE_TRANSLATION_RESOLVER_WITH_ABORT_SIGNAL_H_
