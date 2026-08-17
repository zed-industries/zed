// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_UNDERLYING_SINK_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_UNDERLYING_SINK_BASE_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/streams/stream_algorithms.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

class ExceptionState;
class ScriptState;
class WritableStreamDefaultController;

class CORE_EXPORT UnderlyingSinkBase
    : public GarbageCollected<UnderlyingSinkBase> {
 public:
  virtual ~UnderlyingSinkBase() = default;

  // We define non-virtual |start| and |write| which take ScriptValue for
  // |controller| and are called from IDL. Also we define virtual |start| and
  // |write| which take WritableStreamDefaultController.
  virtual ScriptPromise<IDLUndefined> start(ScriptState*,
                                            WritableStreamDefaultController*,
                                            ExceptionState&) = 0;
  virtual ScriptPromise<IDLUndefined> write(ScriptState*,
                                            ScriptValue chunk,
                                            WritableStreamDefaultController*,
                                            ExceptionState&) = 0;
  virtual ScriptPromise<IDLUndefined> close(ScriptState*, ExceptionState&) = 0;
  virtual ScriptPromise<IDLUndefined> abort(ScriptState*,
                                            ScriptValue reason,
                                            ExceptionState&) = 0;

  ScriptPromise<IDLUndefined> start(ScriptState* script_state,
                                    ExceptionState& exception_state) {
    return start(script_state, controller_, exception_state);
  }

  ScriptPromise<IDLUndefined> write(ScriptState* script_state,
                                    ScriptValue chunk,
                                    ExceptionState& exception_state) {
    DCHECK(controller_);
    return write(script_state, chunk, controller_, exception_state);
  }

  virtual void Trace(Visitor*) const;

 protected:
  WritableStreamDefaultController* Controller() const {
    return controller_.Get();
  }

 private:
  friend class UnderlyingSinkStartAlgorithm;
  Member<WritableStreamDefaultController> controller_;
};

class UnderlyingSinkStartAlgorithm final : public StreamStartAlgorithm {
 public:
  UnderlyingSinkStartAlgorithm(UnderlyingSinkBase* sink,
                               WritableStreamDefaultController* controller)
      : sink_(sink) {
    sink_->controller_ = controller;
  }

  ScriptPromise<IDLUndefined> Run(ScriptState* script_state) final;
  void Trace(Visitor* visitor) const final;

 private:
  Member<UnderlyingSinkBase> sink_;
};

class UnderlyingSinkWriteAlgorithm final : public StreamAlgorithm {
 public:
  explicit UnderlyingSinkWriteAlgorithm(UnderlyingSinkBase* sink)
      : sink_(sink) {}

  ScriptPromise<IDLUndefined> Run(ScriptState*,
                                  int argc,
                                  v8::Local<v8::Value> argv[]) final;
  void Trace(Visitor* visitor) const final;

 private:
  Member<UnderlyingSinkBase> sink_;
};

class UnderlyingSinkCloseAlgorithm final : public StreamAlgorithm {
 public:
  explicit UnderlyingSinkCloseAlgorithm(UnderlyingSinkBase* sink)
      : sink_(sink) {}

  ScriptPromise<IDLUndefined> Run(ScriptState*,
                                  int argc,
                                  v8::Local<v8::Value> argv[]) final;
  void Trace(Visitor* visitor) const final;

 private:
  Member<UnderlyingSinkBase> sink_;
};

class UnderlyingSinkAbortAlgorithm final : public StreamAlgorithm {
 public:
  explicit UnderlyingSinkAbortAlgorithm(UnderlyingSinkBase* sink)
      : sink_(sink) {}

  ScriptPromise<IDLUndefined> Run(ScriptState*,
                                  int argc,
                                  v8::Local<v8::Value> argv[]) final;
  void Trace(Visitor* visitor) const final;

 private:
  Member<UnderlyingSinkBase> sink_;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_UNDERLYING_SINK_BASE_H_
