// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_SVG_PATH_SEG_INTERPOLATION_FUNCTIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_SVG_PATH_SEG_INTERPOLATION_FUNCTIONS_H_

#include <memory>
#include "third_party/blink/renderer/core/animation/interpolable_value.h"
#include "third_party/blink/renderer/core/svg/svg_path_data.h"

namespace blink {

struct PathCoordinates {
  double initial_x = 0;
  double initial_y = 0;
  double current_x = 0;
  double current_y = 0;
};

class SVGPathSegInterpolationFunctions {
  STATIC_ONLY(SVGPathSegInterpolationFunctions);

 public:
  static InterpolableValue* ConsumePathSeg(
      const PathSegmentData&,
      PathCoordinates& current_coordinates);
  static PathSegmentData ConsumeInterpolablePathSeg(
      const InterpolableValue&,
      SVGPathSegType,
      PathCoordinates& current_coordinates);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_SVG_PATH_SEG_INTERPOLATION_FUNCTIONS_H_
