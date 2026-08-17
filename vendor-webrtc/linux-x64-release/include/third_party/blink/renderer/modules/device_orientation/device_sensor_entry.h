// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_ORIENTATION_DEVICE_SENSOR_ENTRY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_ORIENTATION_DEVICE_SENSOR_ENTRY_H_

#include "services/device/public/mojom/sensor.mojom-blink.h"
#include "services/device/public/mojom/sensor_provider.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/sensor/web_sensor_provider.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"

namespace device {
union SensorReading;
class SensorReadingSharedBufferReader;
}  // namespace device

namespace blink {

class DeviceSensorEventPump;

class DeviceSensorEntry : public GarbageCollected<DeviceSensorEntry>,
                          public device::mojom::blink::SensorClient {
 public:
  // The sensor state is an automaton with allowed transitions as follows:
  // kNotInitialized -> kInitializing
  // kInitializing -> kActive
  // kInitializing -> kShouldSuspend
  // kActive -> kSuspended
  // kShouldSuspend -> kInitializing
  // kShouldSuspend -> kSuspended
  // kSuspended -> kActive
  // { kInitializing, kActive, kShouldSuspend, kSuspended } -> kNotInitialized
  enum class State {
    kNotInitialized,
    kInitializing,
    kActive,
    kShouldSuspend,
    kSuspended
  };

  DeviceSensorEntry(DeviceSensorEventPump* pump,
                    ExecutionContext* context,
                    device::mojom::blink::SensorType sensor_type);
  ~DeviceSensorEntry() override;

  void Start(mojom::blink::WebSensorProvider* sensor_provider);
  void Stop();
  bool IsConnected() const;
  bool ReadyOrErrored() const;
  bool GetReading(device::SensorReading* reading);

  State state() const { return state_; }

  void Trace(Visitor* visitor) const;

 private:
  // device::mojom::SensorClient:
  void RaiseError() override;
  void SensorReadingChanged() override;

  // Mojo callback for SensorProvider::GetSensor().
  void OnSensorCreated(device::mojom::blink::SensorCreationResult result,
                       device::mojom::blink::SensorInitParamsPtr params);

  // Mojo callback for Sensor::AddConfiguration().
  void OnSensorAddConfiguration(bool success);

  void HandleSensorError();

  Member<DeviceSensorEventPump> event_pump_;

  State state_ = State::kNotInitialized;

  HeapMojoRemote<device::mojom::blink::Sensor> sensor_remote_;
  HeapMojoReceiver<device::mojom::blink::SensorClient, DeviceSensorEntry>
      client_receiver_;

  device::mojom::blink::SensorType type_;

  std::unique_ptr<device::SensorReadingSharedBufferReader>
      shared_buffer_reader_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_ORIENTATION_DEVICE_SENSOR_ENTRY_H_
