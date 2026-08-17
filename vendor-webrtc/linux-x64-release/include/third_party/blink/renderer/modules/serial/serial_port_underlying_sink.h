// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SERIAL_SERIAL_PORT_UNDERLYING_SINK_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SERIAL_SERIAL_PORT_UNDERLYING_SINK_H_

#include "mojo/public/cpp/system/data_pipe.h"
#include "mojo/public/cpp/system/simple_watcher.h"
#include "services/device/public/mojom/serial.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_typedefs.h"
#include "third_party/blink/renderer/core/dom/abort_signal.h"
#include "third_party/blink/renderer/core/streams/underlying_sink_base.h"

namespace blink {

class ExceptionState;
class SerialPort;
class WritableStreamDefaultController;

class SerialPortUnderlyingSink final : public UnderlyingSinkBase {
  USING_PRE_FINALIZER(SerialPortUnderlyingSink, Dispose);

 public:
  SerialPortUnderlyingSink(SerialPort*, mojo::ScopedDataPipeProducerHandle);

  // UnderlyingSinkBase
  ScriptPromise<IDLUndefined> start(ScriptState*,
                                    WritableStreamDefaultController*,
                                    ExceptionState&) override;
  ScriptPromise<IDLUndefined> write(ScriptState*,
                                    ScriptValue chunk,
                                    WritableStreamDefaultController*,
                                    ExceptionState&) override;
  ScriptPromise<IDLUndefined> close(ScriptState*, ExceptionState&) override;
  ScriptPromise<IDLUndefined> abort(ScriptState*,
                                    ScriptValue reason,
                                    ExceptionState&) override;

  void SignalError(device::mojom::blink::SerialSendError);

  void Trace(Visitor*) const override;

 private:
  void OnAborted();
  void OnHandleReady(MojoResult, const mojo::HandleSignalsState&);
  void OnFlushOrDrain();
  void WriteData();
  void PipeClosed();
  void Dispose();

  mojo::ScopedDataPipeProducerHandle data_pipe_;
  mojo::SimpleWatcher watcher_;
  Member<SerialPort> serial_port_;
  Member<ScriptState> script_state_;
  Member<WritableStreamDefaultController> controller_;
  Member<AbortSignal::AlgorithmHandle> abort_handle_;

  Member<V8BufferSource> buffer_source_;
  size_t offset_ = 0;

  // Only one outstanding call to write(), close() or abort() is allowed at a
  // time. This holds the resolver for the Promise returned by any
  // of these functions.
  Member<ScriptPromiseResolver<IDLUndefined>> pending_operation_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SERIAL_SERIAL_PORT_UNDERLYING_SINK_H_
