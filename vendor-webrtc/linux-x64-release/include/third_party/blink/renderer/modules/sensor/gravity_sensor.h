// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_GRAVITY_SENSOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_GRAVITY_SENSOR_H_

#include "third_party/blink/renderer/modules/sensor/accelerometer.h"

namespace blink {

class GravitySensor final : public Accelerometer {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static GravitySensor* Create(ExecutionContext*,
                               const SpatialSensorOptions*,
                               ExceptionState&);
  static GravitySensor* Create(ExecutionContext*, ExceptionState&);

  GravitySensor(ExecutionContext*,
                const SpatialSensorOptions*,
                ExceptionState&);

  void Trace(Visitor*) const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_GRAVITY_SENSOR_H_
