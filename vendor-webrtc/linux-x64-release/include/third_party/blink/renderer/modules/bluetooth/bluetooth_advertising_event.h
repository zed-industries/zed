// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_ADVERTISING_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_ADVERTISING_EVENT_H_

#include <optional>

#include "third_party/blink/public/mojom/bluetooth/web_bluetooth.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/dom/events/event.h"

namespace blink {

class BluetoothDevice;
class BluetoothManufacturerDataMap;
class BluetoothServiceDataMap;

class BluetoothAdvertisingEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  BluetoothAdvertisingEvent(
      const AtomicString& event_type,
      BluetoothDevice* device,
      mojom::blink::WebBluetoothAdvertisingEventPtr advertising_event);

  ~BluetoothAdvertisingEvent() override;

  void Trace(Visitor*) const override;

  const AtomicString& InterfaceName() const override;

  BluetoothDevice* device() const;
  const String& name() const;
  const Vector<String>& uuids() const;
  std::optional<uint16_t> appearance() const { return appearance_; }
  std::optional<int8_t> txPower() const { return txPower_; }
  std::optional<int8_t> rssi() const { return rssi_; }
  BluetoothManufacturerDataMap* manufacturerData() const;
  BluetoothServiceDataMap* serviceData() const;

 private:
  Member<BluetoothDevice> device_;
  String name_;
  Vector<String> uuids_;
  std::optional<uint16_t> appearance_;
  std::optional<int8_t> txPower_;
  std::optional<int8_t> rssi_;
  const Member<BluetoothManufacturerDataMap> manufacturer_data_map_;
  const Member<BluetoothServiceDataMap> service_data_map_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_ADVERTISING_EVENT_H_
