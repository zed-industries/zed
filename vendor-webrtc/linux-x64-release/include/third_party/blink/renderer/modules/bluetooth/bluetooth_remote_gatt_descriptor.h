// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_REMOTE_GATT_DESCRIPTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_REMOTE_GATT_DESCRIPTOR_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/typed_arrays/array_buffer_view_helpers.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_array_piece.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_data_view.h"
#include "third_party/blink/renderer/modules/bluetooth/bluetooth.h"
#include "third_party/blink/renderer/modules/bluetooth/bluetooth_remote_gatt_characteristic.h"
#include "third_party/blink/renderer/modules/bluetooth/bluetooth_remote_gatt_service.h"
#include "third_party/blink/renderer/modules/event_target_modules.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {
class ExceptionState;
class BluetoothRemoteGATTCharacteristic;
class ScriptState;

// BluetoothRemoteGATTDescriptor represents a GATT Descriptor, which is
// a basic data element that provides further information about a peripheral's
// characteristic.
class BluetoothRemoteGATTDescriptor final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  BluetoothRemoteGATTDescriptor(
      mojom::blink::WebBluetoothRemoteGATTDescriptorPtr,
      BluetoothRemoteGATTCharacteristic*);

  // IDL exposed interface:
  BluetoothRemoteGATTCharacteristic* characteristic() {
    return characteristic_.Get();
  }
  String uuid() { return descriptor_->uuid; }
  NotShared<DOMDataView> value() const { return value_; }
  ScriptPromise<NotShared<DOMDataView>> readValue(ScriptState*,
                                                  ExceptionState&);
  ScriptPromise<IDLUndefined> writeValue(ScriptState*,
                                         base::span<const uint8_t> value,
                                         ExceptionState&);

  // Interface required by garbage collection.
  void Trace(Visitor*) const override;

 private:
  friend class DescriptorReadValueCallback;

  BluetoothRemoteGATTServer* GetGatt() const {
    return characteristic_->GetGatt();
  }
  Bluetooth* GetBluetooth() const {
    return characteristic_->device_->GetBluetooth();
  }

  void ReadValueCallback(ScriptPromiseResolver<NotShared<DOMDataView>>*,
                         mojom::blink::WebBluetoothResult,
                         base::span<const uint8_t>);

  void WriteValueCallback(ScriptPromiseResolver<IDLUndefined>*,
                          DOMDataView* new_value,
                          mojom::blink::WebBluetoothResult);

  String CreateInvalidDescriptorErrorMessage();

  mojom::blink::WebBluetoothRemoteGATTDescriptorPtr descriptor_;
  Member<BluetoothRemoteGATTCharacteristic> characteristic_;
  NotShared<DOMDataView> value_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_REMOTE_GATT_DESCRIPTOR_H_
