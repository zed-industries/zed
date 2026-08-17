// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_OFFSET_ROTATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_OFFSET_ROTATION_H_

#include "third_party/blink/renderer/core/style/computed_style_constants.h"

namespace blink {

struct StyleOffsetRotation {
  StyleOffsetRotation(float angle, OffsetRotationType type)
      : angle(angle), type(type) {}

  bool operator==(const StyleOffsetRotation& other) const {
    return angle == other.angle && type == other.type;
  }
  bool operator!=(const StyleOffsetRotation& other) const {
    return !(*this == other);
  }

  float angle;
  OffsetRotationType type;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_OFFSET_ROTATION_H_
