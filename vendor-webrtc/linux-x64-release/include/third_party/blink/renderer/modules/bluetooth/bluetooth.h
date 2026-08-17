// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_H_

#include "third_party/blink/public/mojom/bluetooth/web_bluetooth.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/page/page_visibility_observer.h"
#include "third_party/blink/renderer/modules/bluetooth/bluetooth_advertising_event.h"
#include "third_party/blink/renderer/modules/bluetooth/bluetooth_device.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_receiver_set.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {
class BluetoothDevice;
class BluetoothLEScan;
class BluetoothLEScanOptions;
class ExceptionState;
class RequestDeviceOptions;
class Navigator;
class ScriptState;

class Bluetooth final : public EventTarget,
                        public Supplement<Navigator>,
                        public PageVisibilityObserver,
                        public mojom::blink::WebBluetoothAdvertisementClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kSupplementName[];

  // IDL exposed as navigator.bluetooth
  static Bluetooth* bluetooth(Navigator&);

  explicit Bluetooth(Navigator&);
  ~Bluetooth() override;

  // IDL exposed bluetooth interface:
  ScriptPromise<IDLBoolean> getAvailability(ScriptState*, ExceptionState&);
  ScriptPromise<IDLSequence<BluetoothDevice>> getDevices(ScriptState*,
                                                         ExceptionState&);
  ScriptPromise<BluetoothDevice> requestDevice(ScriptState*,
                                               const RequestDeviceOptions*,
                                               ExceptionState&);

  ScriptPromise<BluetoothLEScan> requestLEScan(ScriptState*,
                                               const BluetoothLEScanOptions*,
                                               ExceptionState&);

  bool IsServiceBound() const { return service_.is_bound(); }
  mojom::blink::WebBluetoothService* Service() { return service_.get(); }

  // WebBluetoothAdvertisementClient
  void AdvertisingEvent(mojom::blink::WebBluetoothAdvertisingEventPtr) override;

  // EventTarget
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  // GC
  void Trace(Visitor*) const override;

  DEFINE_ATTRIBUTE_EVENT_LISTENER(advertisementreceived, kAdvertisementreceived)

  // PageVisibilityObserver
  void PageVisibilityChanged() override;

  void CancelScan(mojo::ReceiverId);
  bool IsScanActive(mojo::ReceiverId) const;

  BluetoothAdvertisingEvent* CreateBluetoothAdvertisingEvent(
      mojom::blink::WebBluetoothAdvertisingEventPtr advertising_event);

 private:
  BluetoothDevice* GetBluetoothDeviceRepresentingDevice(
      mojom::blink::WebBluetoothDevicePtr,
      ExecutionContext*);

  void GetDevicesCallback(ScriptPromiseResolver<IDLSequence<BluetoothDevice>>*,
                          Vector<mojom::blink::WebBluetoothDevicePtr>);

  void RequestDeviceCallback(ScriptPromiseResolver<BluetoothDevice>*,
                             mojom::blink::WebBluetoothResult,
                             mojom::blink::WebBluetoothDevicePtr);

  void RequestScanningCallback(
      ScriptPromiseResolver<BluetoothLEScan>*,
      mojo::ReceiverId,
      mojom::blink::WebBluetoothRequestLEScanOptionsPtr,
      mojom::blink::WebBluetoothResult);

  void EnsureServiceConnection(ExecutionContext*);

  // Map of device ids to BluetoothDevice objects.
  // Ensures only one BluetoothDevice instance represents each
  // Bluetooth device inside a single global object.
  HeapHashMap<String, Member<BluetoothDevice>> device_instance_map_;

  HeapMojoAssociatedReceiverSet<mojom::blink::WebBluetoothAdvertisementClient,
                                Bluetooth>
      client_receivers_;

  // HeapMojoRemote objects are associated with a ContextLifecycleNotifier and
  // cleaned up automatically when it is destroyed.
  HeapMojoRemote<mojom::blink::WebBluetoothService> service_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_H_
