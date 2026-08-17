// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SERIAL_SERIAL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SERIAL_SERIAL_H_

#include "third_party/blink/public/mojom/serial/serial.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"

namespace blink {

class ExecutionContext;
class NavigatorBase;
class ScriptState;
class SerialPort;
class SerialPortRequestOptions;
class SerialPortFilter;

class MODULES_EXPORT Serial final : public EventTarget,
                                    public Supplement<NavigatorBase>,
                                    public ExecutionContextLifecycleObserver,
                                    public mojom::blink::SerialServiceClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kSupplementName[];

  // Web-exposed navigator.serial
  static Serial* serial(NavigatorBase&);

  // Given a JavaScript `filter` object create a Mojo filter object.
  // Upon error an exception will be thrown (using `exception_state`) and a
  // null Mojo filter object will be returned.
  static mojom::blink::SerialPortFilterPtr CreateMojoFilter(
      const SerialPortFilter* filter,
      ExceptionState& exception_state);

  explicit Serial(NavigatorBase&);

  // EventTarget
  ExecutionContext* GetExecutionContext() const override;
  const AtomicString& InterfaceName() const override;

  // ExecutionContextLifecycleObserver
  void ContextDestroyed() override;

  // SerialServiceClient
  void OnPortConnectedStateChanged(
      mojom::blink::SerialPortInfoPtr port_info) override;

  // Web-exposed interfaces
  DEFINE_ATTRIBUTE_EVENT_LISTENER(connect, kConnect)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(disconnect, kDisconnect)
  ScriptPromise<IDLSequence<SerialPort>> getPorts(ScriptState*,
                                                  ExceptionState&);
  ScriptPromise<SerialPort> requestPort(ScriptState*,
                                        const SerialPortRequestOptions*,
                                        ExceptionState&);

  void OpenPort(
      const base::UnguessableToken& token,
      device::mojom::blink::SerialConnectionOptionsPtr options,
      mojo::PendingRemote<device::mojom::blink::SerialPortClient> client,
      mojom::blink::SerialService::OpenPortCallback callback);
  void ForgetPort(const base::UnguessableToken& token,
                  mojom::blink::SerialService::ForgetPortCallback callback);
  void Trace(Visitor*) const override;

 protected:
  // EventTarget
  void AddedEventListener(const AtomicString& event_type,
                          RegisteredEventListener&) override;

 private:
  void EnsureServiceConnection();
  void OnServiceConnectionError();
  SerialPort* GetOrCreatePort(mojom::blink::SerialPortInfoPtr);
  void OnGetPorts(ScriptPromiseResolver<IDLSequence<SerialPort>>*,
                  Vector<mojom::blink::SerialPortInfoPtr>);
  void OnRequestPort(ScriptPromiseResolver<SerialPort>*,
                     mojom::blink::SerialPortInfoPtr);

  HeapMojoRemote<mojom::blink::SerialService> service_;
  HeapMojoReceiver<mojom::blink::SerialServiceClient, Serial> receiver_;
  HeapHashSet<Member<ScriptPromiseResolver<IDLSequence<SerialPort>>>>
      get_ports_promises_;
  HeapHashSet<Member<ScriptPromiseResolverBase>> request_port_promises_;
  HeapHashMap<String, WeakMember<SerialPort>> port_cache_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SERIAL_SERIAL_H_
