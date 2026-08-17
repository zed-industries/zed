// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_SENSOR_READING_REMAPPER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_SENSOR_READING_REMAPPER_H_

#include "services/device/public/cpp/generic_sensor/sensor_reading.h"
#include "services/device/public/mojom/sensor.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class SensorReadingRemapper {
  STATIC_ONLY(SensorReadingRemapper);

 public:
  // For spatial sensors, remaps coordinates to the
  // screen coordinate system, i.e. performs clockwise
  // rotation |orientation_angle| degrees around Z-axis:
  // |  cos(a) sin(a)  0||x|
  // | -sin(a) cos(a)  0||y|
  // |      0      0   1||z|
  static void RemapToScreenCoords(device::mojom::blink::SensorType,
                                  uint16_t orientation_angle,
                                  device::SensorReading*);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_SENSOR_READING_REMAPPER_H_
