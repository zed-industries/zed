// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_UNDERLYING_SOURCE_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_UNDERLYING_SOURCE_BASE_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/streams/stream_algorithms.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ReadableStreamDefaultController;
class ReadableStreamDefaultControllerWithScriptScope;
class ScriptState;

class CORE_EXPORT UnderlyingSourceBase
    : public GarbageCollected<UnderlyingSourceBase>,
      public ExecutionContextLifecycleObserver {
 public:
  void Trace(Visitor*) const override;
  ~UnderlyingSourceBase() override = default;

  ScriptPromise<IDLUndefined> StartWrapper(ScriptState*,
                                           ReadableStreamDefaultController*);
  virtual ScriptPromise<IDLUndefined> Start(ScriptState*);

  virtual ScriptPromise<IDLUndefined> Pull(ScriptState*, ExceptionState&);

  ScriptPromise<IDLUndefined> CancelWrapper(ScriptState*,
                                            ScriptValue reason,
                                            ExceptionState&);
  virtual ScriptPromise<IDLUndefined> Cancel(ScriptState*,
                                             ScriptValue reason,
                                             ExceptionState&);

  // ExecutionContextLifecycleObserver implementation:

  // This is needed to prevent stream operations being performed after the
  // window or worker is destroyed.
  void ContextDestroyed() override;

 protected:
  explicit UnderlyingSourceBase(ScriptState* script_state)
      : ExecutionContextLifecycleObserver(
            ExecutionContext::From(script_state)) {}

  ReadableStreamDefaultControllerWithScriptScope* Controller() const {
    return controller_.Get();
  }

 private:
  Member<ReadableStreamDefaultControllerWithScriptScope> controller_;
};

class UnderlyingStartAlgorithm final : public StreamStartAlgorithm {
 public:
  UnderlyingStartAlgorithm(UnderlyingSourceBase* source,
                           ReadableStreamDefaultController* controller)
      : source_(source), controller_(controller) {}

  ScriptPromise<IDLUndefined> Run(ScriptState* script_state) final;
  void Trace(Visitor* visitor) const final;

 private:
  Member<UnderlyingSourceBase> source_;
  Member<ReadableStreamDefaultController> controller_;
};

class UnderlyingPullAlgorithm final : public StreamAlgorithm {
 public:
  explicit UnderlyingPullAlgorithm(UnderlyingSourceBase* source)
      : source_(source) {}

  ScriptPromise<IDLUndefined> Run(ScriptState* script_state,
                                  int argc,
                                  v8::Local<v8::Value> argv[]) final;
  void Trace(Visitor* visitor) const final;

 private:
  Member<UnderlyingSourceBase> source_;
};

class UnderlyingCancelAlgorithm final : public StreamAlgorithm {
 public:
  explicit UnderlyingCancelAlgorithm(UnderlyingSourceBase* source)
      : source_(source) {}

  ScriptPromise<IDLUndefined> Run(ScriptState* script_state,
                                  int argc,
                                  v8::Local<v8::Value> argv[]) final;
  void Trace(Visitor* visitor) const final;

 private:
  Member<UnderlyingSourceBase> source_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_UNDERLYING_SOURCE_BASE_H_
