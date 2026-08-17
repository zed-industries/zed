// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_REMOTE_GATT_SERVICE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_REMOTE_GATT_SERVICE_H_

#include "third_party/blink/public/mojom/bluetooth/web_bluetooth.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_typedefs.h"
#include "third_party/blink/renderer/modules/bluetooth/bluetooth_device.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ExceptionState;
class ScriptState;

// Represents a GATT Service within a Bluetooth Peripheral, a collection of
// characteristics and relationships to other services that encapsulate the
// behavior of part of a device.
class BluetoothRemoteGATTService final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  BluetoothRemoteGATTService(mojom::blink::WebBluetoothRemoteGATTServicePtr,
                             bool is_primary,
                             const String& device_instance_id,
                             BluetoothDevice*);

  // Interface required by garbage collection.
  void Trace(Visitor*) const override;

  // IDL exposed interface:
  String uuid() { return service_->uuid; }
  bool isPrimary() { return is_primary_; }
  BluetoothDevice* device() { return device_.Get(); }
  ScriptPromise<BluetoothRemoteGATTCharacteristic> getCharacteristic(
      ScriptState* script_state,
      const V8BluetoothCharacteristicUUID* characteristic,
      ExceptionState& exception_state);
  ScriptPromise<IDLSequence<BluetoothRemoteGATTCharacteristic>>
  getCharacteristics(ScriptState* script_state,
                     const V8BluetoothCharacteristicUUID* characteristic,
                     ExceptionState& exception_state);
  ScriptPromise<IDLSequence<BluetoothRemoteGATTCharacteristic>>
  getCharacteristics(ScriptState*, ExceptionState&);

 private:
  void GetCharacteristicsCallback(
      const String& service_instance_id,
      const String& requested_characteristic_uuid,
      mojom::blink::WebBluetoothGATTQueryQuantity,
      ScriptPromiseResolverBase*,
      mojom::blink::WebBluetoothResult,
      std::optional<
          Vector<mojom::blink::WebBluetoothRemoteGATTCharacteristicPtr>>
          characteristics);

  ScriptPromise<IDLSequence<BluetoothRemoteGATTCharacteristic>>
  GetCharacteristicsImpl(ScriptState*,
                         ExceptionState&,
                         const String& characteristic_uuid = String());

  mojom::blink::WebBluetoothRemoteGATTServicePtr service_;
  const bool is_primary_;
  const String device_instance_id_;
  Member<BluetoothDevice> device_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_REMOTE_GATT_SERVICE_H_
