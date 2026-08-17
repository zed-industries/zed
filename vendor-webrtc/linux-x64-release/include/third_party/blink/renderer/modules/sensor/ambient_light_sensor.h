// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_AMBIENT_LIGHT_SENSOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_AMBIENT_LIGHT_SENSOR_H_

#include <optional>

#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/sensor/sensor.h"

namespace blink {

class MODULES_EXPORT AmbientLightSensor final : public Sensor {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static AmbientLightSensor* Create(ExecutionContext*,
                                    const SensorOptions*,
                                    ExceptionState&);
  static AmbientLightSensor* Create(ExecutionContext*, ExceptionState&);

  AmbientLightSensor(ExecutionContext*, const SensorOptions*, ExceptionState&);

  std::optional<double> illuminance() const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_AMBIENT_LIGHT_SENSOR_H_
