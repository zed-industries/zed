// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_ORIENTATION_DEVICE_MOTION_EVENT_PUMP_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_ORIENTATION_DEVICE_MOTION_EVENT_PUMP_H_

#include "third_party/blink/renderer/modules/device_orientation/device_sensor_event_pump.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class DeviceMotionData;
class DeviceSensorEntry;
class PlatformEventController;

class MODULES_EXPORT DeviceMotionEventPump
    : public GarbageCollected<DeviceMotionEventPump>,
      public DeviceSensorEventPump {
 public:
  explicit DeviceMotionEventPump(LocalFrame&);

  DeviceMotionEventPump(const DeviceMotionEventPump&) = delete;
  DeviceMotionEventPump& operator=(const DeviceMotionEventPump&) = delete;

  ~DeviceMotionEventPump() override;

  void SetController(PlatformEventController*);
  void RemoveController();

  // Note that the returned object is owned by this class.
  DeviceMotionData* LatestDeviceMotionData();

  void Trace(Visitor*) const override;

  // DeviceSensorEventPump:
  void SendStartMessage(LocalFrame& frame) override;
  void SendStopMessage() override;

 protected:
  // DeviceSensorEventPump:
  void FireEvent(TimerBase*) override;

  Member<DeviceSensorEntry> accelerometer_;
  Member<DeviceSensorEntry> linear_acceleration_sensor_;
  Member<DeviceSensorEntry> gyroscope_;

 private:
  friend class DeviceMotionEventPumpTest;

  void StartListening(LocalFrame&);
  void StopListening();
  void NotifyController();

  // DeviceSensorEventPump:
  bool SensorsReadyOrErrored() const override;

  DeviceMotionData* GetDataFromSharedMemory();

  Member<DeviceMotionData> data_;
  Member<PlatformEventController> controller_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_ORIENTATION_DEVICE_MOTION_EVENT_PUMP_H_
