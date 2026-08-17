// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_ORIENTATION_DEVICE_ORIENTATION_EVENT_PUMP_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_ORIENTATION_DEVICE_ORIENTATION_EVENT_PUMP_H_

#include "third_party/blink/renderer/modules/device_orientation/device_sensor_event_pump.h"

#include "third_party/blink/renderer/modules/device_orientation/device_sensor_entry.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class DeviceOrientationData;
class DeviceSensorEntry;
class PlatformEventController;

class MODULES_EXPORT DeviceOrientationEventPump
    : public GarbageCollected<DeviceOrientationEventPump>,
      public DeviceSensorEventPump {
 public:
  // Angle threshold beyond which two orientation events are considered
  // sufficiently different.
  static const double kOrientationThreshold;

  DeviceOrientationEventPump(LocalFrame&, bool absolute);

  DeviceOrientationEventPump(const DeviceOrientationEventPump&) = delete;
  DeviceOrientationEventPump& operator=(const DeviceOrientationEventPump&) =
      delete;

  ~DeviceOrientationEventPump() override;

  void SetController(PlatformEventController*);
  void RemoveController();

  // Note that the returned object is owned by this class.
  DeviceOrientationData* LatestDeviceOrientationData();

  void Trace(Visitor*) const override;

  // DeviceSensorEventPump:
  void SendStartMessage(LocalFrame& frame) override;
  void SendStopMessage() override;

  DeviceSensorEntry::State GetRelativeSensorStateForTesting() {
    return relative_orientation_sensor_->state();
  }

  DeviceSensorEntry::State GetAbsoluteSensorStateForTesting() {
    return absolute_orientation_sensor_->state();
  }

 protected:
  // DeviceSensorEventPump:
  void FireEvent(TimerBase*) override;
  void DidStartIfPossible() override;

  Member<DeviceSensorEntry> relative_orientation_sensor_;
  Member<DeviceSensorEntry> absolute_orientation_sensor_;

 private:
  friend class DeviceOrientationEventPumpTest;
  friend class DeviceAbsoluteOrientationEventPumpTest;

  void NotifyController();

  // DeviceSensorEventPump:
  bool SensorsReadyOrErrored() const override;

  DeviceOrientationData* GetDataFromSharedMemory();

  bool ShouldFireEvent(const DeviceOrientationData* data) const;

  bool absolute_;
  // If relative_orientation_sensor_ is requested but fails to initialize then
  // attempt to fall back to absolute_orientation_sensor_ once.
  bool attempted_to_fall_back_to_absolute_orientation_sensor_;
  Member<DeviceOrientationData> data_;
  Member<PlatformEventController> controller_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_ORIENTATION_DEVICE_ORIENTATION_EVENT_PUMP_H_
