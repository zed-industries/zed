// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_ABSOLUTE_ORIENTATION_SENSOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_ABSOLUTE_ORIENTATION_SENSOR_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_spatial_sensor_options.h"
#include "third_party/blink/renderer/modules/sensor/orientation_sensor.h"

namespace blink {

class AbsoluteOrientationSensor final : public OrientationSensor {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static AbsoluteOrientationSensor* Create(ExecutionContext*,
                                           const SpatialSensorOptions*,
                                           ExceptionState&);
  static AbsoluteOrientationSensor* Create(ExecutionContext*, ExceptionState&);

  AbsoluteOrientationSensor(ExecutionContext*,
                            const SpatialSensorOptions*,
                            ExceptionState&);

  void Trace(Visitor*) const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_ABSOLUTE_ORIENTATION_SENSOR_H_
