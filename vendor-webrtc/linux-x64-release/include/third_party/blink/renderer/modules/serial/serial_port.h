// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SERIAL_SERIAL_PORT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SERIAL_SERIAL_PORT_H_

#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "services/device/public/mojom/serial.mojom-blink.h"
#include "third_party/blink/public/mojom/serial/serial.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/scheduler/public/frame_scheduler.h"

namespace base {
class UnguessableToken;
}

namespace blink {

class ReadableStream;
class ScriptState;
class Serial;
class SerialInputSignals;
class SerialOptions;
class SerialOutputSignals;
class SerialPortInfo;
class SerialPortUnderlyingSink;
class SerialPortUnderlyingSource;
class WritableStream;

class SerialPort final : public EventTarget,
                         public ActiveScriptWrappable<SerialPort>,
                         public device::mojom::blink::SerialPortClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit SerialPort(Serial* parent, mojom::blink::SerialPortInfoPtr info);
  ~SerialPort() override;

  // Web-exposed functions
  DEFINE_ATTRIBUTE_EVENT_LISTENER(connect, kConnect)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(disconnect, kDisconnect)
  SerialPortInfo* getInfo();
  ScriptPromise<IDLUndefined> open(ScriptState*,
                                   const SerialOptions* options,
                                   ExceptionState&);
  bool connected() { return connected_; }
  ReadableStream* readable(ScriptState*, ExceptionState&);
  WritableStream* writable(ScriptState*, ExceptionState&);
  ScriptPromise<SerialInputSignals> getSignals(ScriptState*, ExceptionState&);
  ScriptPromise<IDLUndefined> setSignals(ScriptState*,
                                         const SerialOutputSignals*,
                                         ExceptionState&);
  ScriptPromise<IDLUndefined> close(ScriptState*, ExceptionState&);
  ScriptPromise<IDLUndefined> forget(ScriptState*, ExceptionState&);

  const base::UnguessableToken& token() const { return info_->token; }

  void set_connected(bool connected) { connected_ = connected; }
  ScriptPromise<IDLUndefined> ContinueClose(ScriptState*);
  void AbortClose();
  void StreamsClosed();
  bool IsClosing() const { return close_resolver_ != nullptr; }

  void Flush(device::mojom::blink::SerialPortFlushMode mode,
             device::mojom::blink::SerialPort::FlushCallback callback);
  void Drain(device::mojom::blink::SerialPort::DrainCallback callback);
  void UnderlyingSourceClosed();
  void UnderlyingSinkClosed();

  void ContextDestroyed();
  void Trace(Visitor*) const override;

  // ActiveScriptWrappable
  bool HasPendingActivity() const override;

  // EventTarget
  ExecutionContext* GetExecutionContext() const override;
  const AtomicString& InterfaceName() const override;
  DispatchEventResult DispatchEventInternal(Event& event) override;

  // SerialPortClient
  void OnReadError(device::mojom::blink::SerialReceiveError) override;
  void OnSendError(device::mojom::blink::SerialSendError) override;

 private:
  bool CreateDataPipe(mojo::ScopedDataPipeProducerHandle* producer,
                      mojo::ScopedDataPipeConsumerHandle* consumer);
  void OnConnectionError();
  void OnOpen(mojo::PendingReceiver<device::mojom::blink::SerialPortClient>,
              mojo::PendingRemote<device::mojom::blink::SerialPort>);
  void OnGetSignals(ScriptPromiseResolver<SerialInputSignals>*,
                    device::mojom::blink::SerialPortControlSignalsPtr);
  void OnSetSignals(ScriptPromiseResolver<IDLUndefined>*, bool success);
  void OnClose();

  const mojom::blink::SerialPortInfoPtr info_;
  bool connected_;
  const Member<Serial> parent_;

  uint32_t buffer_size_ = 0;
  HeapMojoRemote<device::mojom::blink::SerialPort> port_;
  HeapMojoReceiver<device::mojom::blink::SerialPortClient, SerialPort>
      client_receiver_;

  Member<ReadableStream> readable_;
  Member<SerialPortUnderlyingSource> underlying_source_;
  Member<WritableStream> writable_;
  Member<SerialPortUnderlyingSink> underlying_sink_;

  // Indicates that the read or write streams have encountered a fatal error and
  // should not be reopened.
  bool read_fatal_ = false;
  bool write_fatal_ = false;

  // The port was opened with { flowControl: "hardware" }.
  bool hardware_flow_control_ = false;

  // Resolver for the Promise returned by open().
  Member<ScriptPromiseResolver<IDLUndefined>> open_resolver_;
  // Resolvers for the Promises returned by getSignals() and setSignals() to
  // reject them on Mojo connection failure.
  HeapHashSet<Member<ScriptPromiseResolverBase>> signal_resolvers_;
  // Resolver for the Promise returned by close().
  Member<ScriptPromiseResolver<IDLUndefined>> close_resolver_;

  FrameScheduler::SchedulingAffectingFeatureHandle
      feature_handle_for_scheduler_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SERIAL_SERIAL_PORT_H_
