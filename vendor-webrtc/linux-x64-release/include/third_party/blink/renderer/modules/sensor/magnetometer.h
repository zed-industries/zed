// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_MAGNETOMETER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_MAGNETOMETER_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_spatial_sensor_options.h"
#include "third_party/blink/renderer/modules/sensor/sensor.h"

namespace blink {

class Magnetometer final : public Sensor {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static Magnetometer* Create(ExecutionContext*,
                              const SpatialSensorOptions*,
                              ExceptionState&);
  static Magnetometer* Create(ExecutionContext*, ExceptionState&);

  Magnetometer(ExecutionContext*, const SpatialSensorOptions*, ExceptionState&);

  std::optional<double> x() const;
  std::optional<double> y() const;
  std::optional<double> z() const;

  void Trace(Visitor*) const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_MAGNETOMETER_H_
