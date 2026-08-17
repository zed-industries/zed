// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_POINTER_EVENT_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_POINTER_EVENT_UTIL_H_

#include <cstdint>

#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

class CORE_EXPORT PointerEventUtil {
 public:
  PointerEventUtil() = delete;

  static double AzimuthFromTilt(double tilt_x_degrees, double tilt_y_degrees);
  static double AltitudeFromTilt(double tilt_x_degrees, double tilt_y_degrees);
  static int32_t TiltXFromSpherical(double azimuth_radians,
                                    double altitude_radians);
  static int32_t TiltYFromSpherical(double azimuth_radians,
                                    double altitude_radians);

  // Returns tilt in range [-90, 90] by using formula
  // tilt = tilt_degrees - k*180
  static int32_t TransformToTiltInValidRange(int32_t tilt_degrees);
  // Returns azimuth in range [0,2*PI] by using formula
  // azimuth = azimuth_radians - 2*k*PI
  static double TransformToAzimuthInValidRange(double azimuth_radians);
  // Returns altitude in range [0,PI/2]
  // altitude = altitude_radians - k*PI/2
  static double TransformToAltitudeInValidRange(double altitude_radians);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_POINTER_EVENT_UTIL_H_
