// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_SERVICE_DATA_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_SERVICE_DATA_MAP_H_

#include "third_party/blink/renderer/bindings/core/v8/maplike.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_sync_iterator_bluetooth_service_data_map.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_data_view.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class BluetoothServiceDataMap final : public ScriptWrappable,
                                      public Maplike<BluetoothServiceDataMap> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  using MapType = HashMap<String, WTF::Vector<uint8_t>>;

  explicit BluetoothServiceDataMap(const MapType&);

  ~BluetoothServiceDataMap() override;

  const MapType& Map() const { return parameter_map_; }

  // IDL attributes / methods
  uint32_t size() const { return parameter_map_.size(); }

 private:
  PairSyncIterable<BluetoothServiceDataMap>::IterationSource*
  CreateIterationSource(ScriptState*, ExceptionState&) override;
  bool GetMapEntry(ScriptState*,
                   const String& key,
                   NotShared<DOMDataView>& value,
                   ExceptionState&) override;

  const MapType parameter_map_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_SERVICE_DATA_MAP_H_
