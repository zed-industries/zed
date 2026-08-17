// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BATTERY_BATTERY_DISPATCHER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BATTERY_BATTERY_DISPATCHER_H_

#include "services/device/public/mojom/battery_monitor.mojom-blink.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/frame/platform_event_dispatcher.h"
#include "third_party/blink/renderer/modules/battery/battery_status.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"

namespace blink {

class MODULES_EXPORT BatteryDispatcher final
    : public GarbageCollected<BatteryDispatcher>,
      public PlatformEventDispatcher {
 public:
  explicit BatteryDispatcher(ExecutionContext*);

  BatteryDispatcher(const BatteryDispatcher&) = delete;
  BatteryDispatcher& operator=(const BatteryDispatcher&) = delete;

  const BatteryStatus* LatestData() const {
    return has_latest_data_ ? &battery_status_ : nullptr;
  }

  void Trace(Visitor*) const override;

 private:
  void QueryNextStatus();
  void OnDidChange(device::mojom::blink::BatteryStatusPtr);
  void UpdateBatteryStatus(const BatteryStatus&);

  // Inherited from PlatformEventDispatcher.
  void StartListening(LocalDOMWindow*) override;
  void StopListening() override;

  HeapMojoRemote<device::mojom::blink::BatteryMonitor> monitor_;
  BatteryStatus battery_status_;
  bool has_latest_data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BATTERY_BATTERY_DISPATCHER_H_
