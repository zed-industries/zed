// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_REMOTE_GATT_CHARACTERISTIC_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_REMOTE_GATT_CHARACTERISTIC_H_

#include "third_party/blink/public/mojom/bluetooth/web_bluetooth.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_typedefs.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_array_piece.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_data_view.h"
#include "third_party/blink/renderer/modules/bluetooth/bluetooth_remote_gatt_service.h"
#include "third_party/blink/renderer/modules/event_target_modules.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_receiver_set.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class BluetoothCharacteristicProperties;
class BluetoothDevice;
class ExceptionState;
class ExecutionContext;
class ScriptState;

// BluetoothRemoteGATTCharacteristic represents a GATT Characteristic, which is
// a basic data element that provides further information about a peripheral's
// service.
class BluetoothRemoteGATTCharacteristic final
    : public EventTarget,
      public ActiveScriptWrappable<BluetoothRemoteGATTCharacteristic>,
      public ExecutionContextLifecycleObserver,
      public mojom::blink::WebBluetoothCharacteristicClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit BluetoothRemoteGATTCharacteristic(
      ExecutionContext*,
      mojom::blink::WebBluetoothRemoteGATTCharacteristicPtr,
      BluetoothRemoteGATTService*,
      BluetoothDevice*);

  // mojom::blink::WebBluetoothCharacteristicClient:
  void RemoteCharacteristicValueChanged(
      base::span<const uint8_t> value) override;

  // ExecutionContextLifecycleObserver interface.
  void ContextDestroyed() override {}

  // EventTarget methods:
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  // ActiveScriptWrappable methods:
  bool HasPendingActivity() const override;

  // Interface required by garbage collection.
  void Trace(Visitor*) const override;

  // IDL exposed interface:
  BluetoothRemoteGATTService* service() { return service_.Get(); }
  String uuid() { return characteristic_->uuid; }
  BluetoothCharacteristicProperties* properties() { return properties_.Get(); }
  NotShared<DOMDataView> value() const { return value_; }
  ScriptPromise<BluetoothRemoteGATTDescriptor> getDescriptor(
      ScriptState* script_state,
      const V8BluetoothDescriptorUUID* descriptor_uuid,
      ExceptionState& exception_state);
  ScriptPromise<IDLSequence<BluetoothRemoteGATTDescriptor>> getDescriptors(
      ScriptState*,
      ExceptionState&);
  ScriptPromise<IDLSequence<BluetoothRemoteGATTDescriptor>> getDescriptors(
      ScriptState* script_state,
      const V8BluetoothDescriptorUUID* descriptor_uuid,
      ExceptionState& exception_state);
  ScriptPromise<NotShared<DOMDataView>> readValue(ScriptState*,
                                                  ExceptionState&);
  ScriptPromise<IDLUndefined> writeValue(ScriptState*,
                                         base::span<const uint8_t> value,
                                         ExceptionState&);
  ScriptPromise<IDLUndefined> writeValueWithResponse(
      ScriptState*,
      base::span<const uint8_t> value,
      ExceptionState&);
  ScriptPromise<IDLUndefined> writeValueWithoutResponse(
      ScriptState*,
      base::span<const uint8_t> value,
      ExceptionState&);
  ScriptPromise<BluetoothRemoteGATTCharacteristic> startNotifications(
      ScriptState*,
      ExceptionState&);
  ScriptPromise<BluetoothRemoteGATTCharacteristic> stopNotifications(
      ScriptState*,
      ExceptionState&);

  DEFINE_ATTRIBUTE_EVENT_LISTENER(characteristicvaluechanged,
                                  kCharacteristicvaluechanged)

 protected:
  // EventTarget overrides.
  void AddedEventListener(const AtomicString& event_type,
                          RegisteredEventListener&) override;

 private:
  friend class BluetoothRemoteGATTDescriptor;

  struct DeferredValueChange : public GarbageCollected<DeferredValueChange> {
    DeferredValueChange(Member<Event> event,
                        NotShared<DOMDataView> dom_data_view,
                        ScriptPromiseResolver<NotShared<DOMDataView>>* resolver)
        : event(event), dom_data_view(dom_data_view), resolver(resolver) {}

    // GarbageCollectedMixin:
    void Trace(Visitor*) const;

    Member<Event> event;  // Event to dispatch before resolving promise.
    NotShared<DOMDataView> dom_data_view;

    // Possibly null.
    Member<ScriptPromiseResolver<NotShared<DOMDataView>>> resolver;
  };

  BluetoothRemoteGATTServer* GetGatt() const {
    return service_->device()->gatt();
  }
  Bluetooth* GetBluetooth() const { return device_->GetBluetooth(); }

  void ReadValueCallback(ScriptPromiseResolver<NotShared<DOMDataView>>*,
                         mojom::blink::WebBluetoothResult,
                         base::span<const uint8_t> value);
  void WriteValueCallback(ScriptPromiseResolver<IDLUndefined>*,
                          DOMDataView* new_value,
                          mojom::blink::WebBluetoothResult);

  // Callback for startNotifictions/stopNotifications.
  // |started| is true if called as a result of startNotifictions() and
  // false if called as a result of stopNotifications().
  void NotificationsCallback(
      ScriptPromiseResolver<BluetoothRemoteGATTCharacteristic>*,
      bool started,
      mojom::blink::WebBluetoothResult);

  ScriptPromise<IDLUndefined> WriteCharacteristicValue(
      ScriptState*,
      base::span<const uint8_t> value,
      mojom::blink::WebBluetoothWriteType,
      ExceptionState&);

  ScriptPromise<IDLSequence<BluetoothRemoteGATTDescriptor>> GetDescriptorsImpl(
      ScriptState*,
      ExceptionState&,
      const String& descriptor_uuid = String());

  void GetDescriptorsCallback(
      const String& requested_descriptor_uuid,
      const String& characteristic_instance_id,
      mojom::blink::WebBluetoothGATTQueryQuantity,
      ScriptPromiseResolverBase*,
      mojom::blink::WebBluetoothResult,
      std::optional<Vector<mojom::blink::WebBluetoothRemoteGATTDescriptorPtr>>
          descriptors);

  String CreateInvalidCharacteristicErrorMessage();

  // Still waiting for acknowledgement from device of request for notifications?
  bool notification_registration_in_progress() const {
    return num_in_flight_notification_registrations_ > 0;
  }

  mojom::blink::WebBluetoothRemoteGATTCharacteristicPtr characteristic_;
  Member<BluetoothRemoteGATTService> service_;
  Member<BluetoothCharacteristicProperties> properties_;
  NotShared<DOMDataView> value_;
  Member<BluetoothDevice> device_;
  HeapMojoAssociatedReceiverSet<mojom::blink::WebBluetoothCharacteristicClient,
                                BluetoothRemoteGATTCharacteristic>
      receivers_;

  uint32_t num_in_flight_notification_registrations_ = 0;

  // Queue of characteristicvaluechanged events created if a value changes
  // while startNotificications() is in the process of registering a listener.
  HeapVector<Member<DeferredValueChange>> deferred_value_change_data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_REMOTE_GATT_CHARACTERISTIC_H_
