// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_DEVICE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_DEVICE_H_

#include "third_party/blink/public/mojom/bluetooth/web_bluetooth.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/dom/abort_signal.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/bluetooth/bluetooth_remote_gatt_server.h"
#include "third_party/blink/renderer/modules/event_target_modules.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_receiver.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class Bluetooth;
class BluetoothAttributeInstanceMap;
class BluetoothRemoteGATTCharacteristic;
class BluetoothRemoteGATTDescriptor;
class BluetoothRemoteGATTServer;
class BluetoothRemoteGATTService;
class WatchAdvertisementsOptions;

// BluetoothDevice represents a physical bluetooth device in the DOM. See IDL.
class BluetoothDevice final
    : public EventTarget,
      public ExecutionContextClient,
      public ActiveScriptWrappable<BluetoothDevice>,
      public mojom::blink::WebBluetoothAdvertisementClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  BluetoothDevice(ExecutionContext*,
                  mojom::blink::WebBluetoothDevicePtr,
                  Bluetooth*);

  BluetoothRemoteGATTService* GetOrCreateRemoteGATTService(
      mojom::blink::WebBluetoothRemoteGATTServicePtr,
      bool is_primary,
      const String& device_instance_id);
  bool IsValidService(const String& service_instance_id);

  BluetoothRemoteGATTCharacteristic* GetOrCreateRemoteGATTCharacteristic(
      ExecutionContext*,
      mojom::blink::WebBluetoothRemoteGATTCharacteristicPtr,
      BluetoothRemoteGATTService*);
  bool IsValidCharacteristic(const String& characteristic_instance_id);

  BluetoothRemoteGATTDescriptor* GetOrCreateBluetoothRemoteGATTDescriptor(
      mojom::blink::WebBluetoothRemoteGATTDescriptorPtr,
      BluetoothRemoteGATTCharacteristic*);
  bool IsValidDescriptor(const String& descriptor_instance_id);

  // We should disconnect from the device in all of the following cases:
  // 1. When the object gets GarbageCollected e.g. it went out of scope.
  // dispose() is called in this case.
  // 2. When the parent document gets detached e.g. reloading a page.
  // stop() is called in this case.
  // TODO(ortuno): Users should be able to turn on notifications for
  // events on navigator.bluetooth and still remain connected even if the
  // BluetoothDevice object is garbage collected.

  // Performs necessary cleanup when a device disconnects and fires
  // gattserverdisconnected event.
  void ClearAttributeInstanceMapAndFireEvent();

  // EventTarget methods:
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  Bluetooth* GetBluetooth() { return bluetooth_.Get(); }

  const mojom::blink::WebBluetoothDevicePtr& GetDevice() const {
    return device_;
  }

  // Interface required by Garbage Collection:
  void Trace(Visitor*) const override;

  // IDL exposed interface:
  ScriptPromise<IDLUndefined> watchAdvertisements(
      ScriptState*,
      const WatchAdvertisementsOptions*,
      ExceptionState&);
  ScriptPromise<IDLUndefined> forget(ScriptState*, ExceptionState&);
  String id() { return device_->id.DeviceIdInBase64().c_str(); }
  String name() { return device_->name; }
  BluetoothRemoteGATTServer* gatt() { return gatt_.Get(); }
  bool watchingAdvertisements() { return client_receiver_.is_bound(); }

  void AbortWatchAdvertisements(AbortSignal* signal);

  // WebBluetoothAdvertisementClient:
  void AdvertisingEvent(mojom::blink::WebBluetoothAdvertisingEventPtr) override;

  // ActiveScriptWrappable:
  bool HasPendingActivity() const override;

  DEFINE_ATTRIBUTE_EVENT_LISTENER(advertisementreceived, kAdvertisementreceived)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(gattserverdisconnected,
                                  kGattserverdisconnected)

 protected:
  // EventTarget overrides:
  void AddedEventListener(const AtomicString& eventType,
                          RegisteredEventListener&) override;

 private:
  void WatchAdvertisementsCallback(mojom::blink::WebBluetoothResult);

  // Holds all GATT Attributes associated with this BluetoothDevice.
  Member<BluetoothAttributeInstanceMap> attribute_instance_map_;

  mojom::blink::WebBluetoothDevicePtr device_;
  Member<BluetoothRemoteGATTServer> gatt_;
  Member<Bluetooth> bluetooth_;

  Member<ScriptPromiseResolver<IDLUndefined>> watch_advertisements_resolver_;

  HeapMojoAssociatedReceiver<mojom::blink::WebBluetoothAdvertisementClient,
                             BluetoothDevice>
      client_receiver_;

  // Any unaborted signal passed to watchAdvertisements() can abort
  // advertisements for this device, regardless of previous aborts from other
  // signals. Abort algorithm handles therefore need to remain alive as long as
  // both the device and associated signal remain alive, so we store the handles
  // in a map keyed weakly by AbortSignal.
  HeapHashMap<WeakMember<AbortSignal>, Member<AbortSignal::AlgorithmHandle>>
      abort_handle_map_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_DEVICE_H_
