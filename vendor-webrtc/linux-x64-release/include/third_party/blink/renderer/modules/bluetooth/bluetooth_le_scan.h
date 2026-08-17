// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_LE_SCAN_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_LE_SCAN_H_

#include "mojo/public/cpp/bindings/receiver_set.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_bluetooth_le_scan_filter_init.h"
#include "third_party/blink/renderer/modules/bluetooth/bluetooth.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class BluetoothLEScan final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  BluetoothLEScan(mojo::ReceiverId,
                  Bluetooth*,
                  mojom::blink::WebBluetoothRequestLEScanOptionsPtr);

  // IDL exposed interface:
  const HeapVector<Member<BluetoothLEScanFilterInit>>& filters() const;
  bool keepRepeatedDevices() const;
  bool acceptAllAdvertisements() const;
  bool active() const;
  bool stop();

  // Interface required by garbage collection.
  void Trace(Visitor*) const override;

 private:
  mojo::ReceiverId id_;
  HeapVector<Member<BluetoothLEScanFilterInit>> filters_;
  Member<Bluetooth> bluetooth_;
  const bool keep_repeated_devices_;
  const bool accept_all_advertisements_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_LE_SCAN_H_
